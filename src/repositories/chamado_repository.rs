// ============================================================
// repositories/chamado_repository.rs — Repository Pattern
// Abstrai todo acesso a dados de Chamado.
// Queries parametrizadas — prevencao de SQL Injection (Req. B2).
// ============================================================

use crate::db::Database;
use crate::models::chamado::Chamado;
use rusqlite::params;

pub struct ChamadoRepository;

impl ChamadoRepository {
    pub fn create(db: &Database, c: &Chamado) -> Result<(), String> {
        db.conn
            .execute(
                "INSERT INTO chamados (id, titulo, tipo, descricao, endereco, foto_url,
                  status, user_id, user_nome, criado_em, atualizado_em)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
                params![
                    c.id, c.titulo, c.tipo, c.descricao, c.endereco,
                    c.foto_url, c.status, c.user_id, c.user_nome,
                    c.criado_em, c.atualizado_em,
                ],
            )
            .map(|_| ())
            .map_err(|e| format!("Erro ao criar chamado: {e}"))
    }

    pub fn list(db: &Database) -> Vec<Chamado> {
        let mut stmt = match db.conn.prepare(
            "SELECT id, titulo, tipo, descricao, endereco, foto_url, status,
                    user_id, user_nome, criado_em, atualizado_em
             FROM chamados ORDER BY criado_em DESC",
        ) {
            Ok(s) => s,
            Err(_) => return vec![],
        };
        let result: Vec<Chamado> = match stmt.query_map([], Self::row_to_chamado) {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => vec![],
        };
        result
    }

    pub fn find_by_id(db: &Database, id: &str) -> Option<Chamado> {
        db.conn
            .query_row(
                "SELECT id, titulo, tipo, descricao, endereco, foto_url, status,
                        user_id, user_nome, criado_em, atualizado_em
                 FROM chamados WHERE id = ?1",
                params![id],
                Self::row_to_chamado,
            )
            .ok()
    }

    pub fn update(db: &Database, id: &str, c: &Chamado) -> Result<(), String> {
        db.conn
            .execute(
                "UPDATE chamados SET titulo=?1, tipo=?2, descricao=?3, endereco=?4,
                  foto_url=?5, status=?6, atualizado_em=?7
                 WHERE id = ?8",
                params![
                    c.titulo, c.tipo, c.descricao, c.endereco,
                    c.foto_url, c.status, c.atualizado_em, id,
                ],
            )
            .map(|_| ())
            .map_err(|e| format!("Erro ao atualizar chamado: {e}"))
    }

    pub fn update_status(db: &Database, id: &str, status: &str) -> Result<(), String> {
        db.conn
            .execute(
                "UPDATE chamados SET status=?1, atualizado_em=datetime('now') WHERE id=?2",
                params![status, id],
            )
            .map(|_| ())
            .map_err(|e| format!("Erro ao atualizar status: {e}"))
    }

    pub fn delete(db: &Database, id: &str) -> Result<(), String> {
        db.conn
            .execute("DELETE FROM chamados WHERE id = ?1", params![id])
            .map(|_| ())
            .map_err(|e| format!("Erro ao deletar chamado: {e}"))
    }

    fn row_to_chamado(row: &rusqlite::Row<'_>) -> rusqlite::Result<Chamado> {
        Ok(Chamado {
            id: row.get(0)?,
            titulo: row.get(1)?,
            tipo: row.get(2)?,
            descricao: row.get(3)?,
            endereco: row.get(4)?,
            foto_url: row.get(5)?,
            status: row.get(6)?,
            user_id: row.get(7)?,
            user_nome: row.get(8)?,
            criado_em: row.get(9)?,
            atualizado_em: row.get(10)?,
        })
    }
}
