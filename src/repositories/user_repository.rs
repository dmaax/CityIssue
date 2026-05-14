// ============================================================
// repositories/user_repository.rs — Repository Pattern
// Abstrai todo acesso a dados de Usuario.
// Todas as queries usam parametros posicionais (?) evitando SQL Injection (Req. B2).
// ============================================================

use crate::db::Database;
use crate::models::user::User;
use rusqlite::params;

pub struct UserRepository;

impl UserRepository {
    /// Cria um novo usuario. Retorna o id criado.
    pub fn create(db: &Database, user: &User) -> Result<(), String> {
        db.conn
            .execute(
                "INSERT INTO users (id, nome, email, senha_hash, role, criado_em, ativo)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    user.id,
                    user.nome,
                    user.email,
                    user.senha_hash,
                    user.role,
                    user.criado_em,
                    user.ativo as i32,
                ],
            )
            .map(|_| ())
            .map_err(|e| format!("Erro ao criar usuario: {e}"))
    }

    /// Busca usuario por email (usado no login).
    pub fn find_by_email(db: &Database, email: &str) -> Option<User> {
        db.conn
            .query_row(
                "SELECT id, nome, email, senha_hash, role, criado_em, ativo
                 FROM users WHERE email = ?1 AND ativo = 1",
                params![email],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        nome: row.get(1)?,
                        email: row.get(2)?,
                        senha_hash: row.get(3)?,
                        role: row.get(4)?,
                        criado_em: row.get(5)?,
                        ativo: row.get::<_, i32>(6)? != 0,
                    })
                },
            )
            .ok()
    }

    /// Busca usuario por id.
    pub fn find_by_id(db: &Database, id: &str) -> Option<User> {
        db.conn
            .query_row(
                "SELECT id, nome, email, senha_hash, role, criado_em, ativo
                 FROM users WHERE id = ?1",
                params![id],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        nome: row.get(1)?,
                        email: row.get(2)?,
                        senha_hash: row.get(3)?,
                        role: row.get(4)?,
                        criado_em: row.get(5)?,
                        ativo: row.get::<_, i32>(6)? != 0,
                    })
                },
            )
            .ok()
    }

    /// Lista todos os usuarios.
    pub fn list(db: &Database) -> Vec<User> {
        let mut stmt = match db.conn.prepare(
            "SELECT id, nome, email, senha_hash, role, criado_em, ativo FROM users ORDER BY criado_em DESC",
        ) {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let result: Vec<User> = match stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                nome: row.get(1)?,
                email: row.get(2)?,
                senha_hash: row.get(3)?,
                role: row.get(4)?,
                criado_em: row.get(5)?,
                ativo: row.get::<_, i32>(6)? != 0,
            })
        }) {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => vec![],
        };
        result
    }

    /// Desativa (soft-delete) usuario pelo id.
    pub fn deactivate(db: &Database, id: &str) -> Result<(), String> {
        db.conn
            .execute("UPDATE users SET ativo = 0 WHERE id = ?1", params![id])
            .map(|_| ())
            .map_err(|e| format!("Erro ao desativar usuario: {e}"))
    }

    /// Verifica se email ja existe.
    pub fn email_exists(db: &Database, email: &str) -> bool {
        db.conn
            .query_row(
                "SELECT COUNT(*) FROM users WHERE email = ?1",
                params![email],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0)
            > 0
    }

    // ---- Rate limiting / auditoria de login ----

    /// Registra tentativa de login.
    pub fn log_attempt(db: &Database, email: &str, ip: &str, sucesso: bool) {
        let _ = db.conn.execute(
            "INSERT INTO login_attempts (email, ip, tentativa_em, sucesso)
             VALUES (?1, ?2, datetime('now'), ?3)",
            params![email, ip, sucesso as i32],
        );
    }

    /// Conta falhas de login nos ultimos N minutos para o email.
    /// Req. C4: bloquear apos 5 tentativas em 15 minutos.
    pub fn recent_failures(db: &Database, email: &str, minutes: i64) -> i64 {
        db.conn
            .query_row(
                "SELECT COUNT(*) FROM login_attempts
                 WHERE email = ?1
                   AND sucesso = 0
                   AND tentativa_em >= datetime('now', ?2)",
                params![email, format!("-{minutes} minutes")],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0)
    }
}
