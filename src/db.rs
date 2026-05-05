// ============================================================
// db.rs — Database singleton (Singleton Pattern)
// Responsavel por criar e manter a conexao SQLite unica.
// ============================================================

use rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Abre (ou cria) o banco SQLite e inicializa as tabelas.
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        // WAL mode para melhor concorrencia
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS users (
                id          TEXT PRIMARY KEY,
                nome        TEXT NOT NULL,
                email       TEXT UNIQUE NOT NULL,
                senha_hash  TEXT NOT NULL,
                role        TEXT NOT NULL DEFAULT 'user',
                criado_em   TEXT NOT NULL,
                ativo       INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS chamados (
                id              TEXT PRIMARY KEY,
                titulo          TEXT NOT NULL,
                tipo            TEXT NOT NULL,
                descricao       TEXT NOT NULL,
                endereco        TEXT NOT NULL,
                foto_url        TEXT,
                status          TEXT NOT NULL DEFAULT 'pending',
                user_id         TEXT NOT NULL,
                user_nome       TEXT NOT NULL,
                criado_em       TEXT NOT NULL,
                atualizado_em   TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            );

            -- Tabela de tentativas de login para rate-limiting (Req. C4)
            CREATE TABLE IF NOT EXISTS login_attempts (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                email       TEXT NOT NULL,
                ip          TEXT NOT NULL,
                tentativa_em TEXT NOT NULL,
                sucesso     INTEGER NOT NULL DEFAULT 0
            );
            ",
        )?;
        Ok(())
    }
}
