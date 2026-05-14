// ============================================================
// models/user.rs — Entidade Usuario (MVC - Model)
// ============================================================

use serde::{Deserialize, Serialize};

/// Entidade persistida no banco de dados.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub nome: String,
    pub email: String,
    #[serde(skip_serializing)] // NUNCA serializar a senha hash na resposta
    pub senha_hash: String,
    pub role: String,
    pub criado_em: String,
    pub ativo: bool,
}

/// DTO de entrada para cadastro (Factory Pattern: cria User a partir deste DTO).
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

/// DTO de entrada para login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

/// DTO de resposta de autenticacao bem-sucedida.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub nome: String,
    pub role: String,
}
