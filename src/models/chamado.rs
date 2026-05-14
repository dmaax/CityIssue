// ============================================================
// models/chamado.rs — Entidade Chamado / modelo de dominio principal (MVC - Model)
// ============================================================

use serde::{Deserialize, Serialize};

/// Entidade persistida no banco de dados.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chamado {
    pub id: String,
    pub titulo: String,
    pub tipo: String,
    pub descricao: String,
    pub endereco: String,
    pub foto_url: Option<String>,
    pub status: String,
    pub user_id: String,
    pub user_nome: String,
    pub criado_em: String,
    pub atualizado_em: String,
}

/// DTO de entrada para criacao de chamado.
#[derive(Debug, Deserialize)]
pub struct CreateChamadoRequest {
    pub titulo: String,
    pub tipo: String,
    pub descricao: String,
    pub endereco: String,
    pub foto_url: Option<String>,
}

/// DTO de entrada para atualizacao completa de chamado.
#[derive(Debug, Deserialize)]
pub struct UpdateChamadoRequest {
    pub titulo: Option<String>,
    pub tipo: Option<String>,
    pub descricao: Option<String>,
    pub endereco: Option<String>,
    pub foto_url: Option<String>,
    pub status: Option<String>,
}

/// DTO para atualizacao apenas de status.
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}
