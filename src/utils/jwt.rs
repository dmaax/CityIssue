// ============================================================
// utils/jwt.rs — Geracao e validacao de JWT (Req. A1 / B3)
// Singleton Pattern: JWT_SECRET e inicializado uma unica vez com once_cell.
// ============================================================

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Segredo JWT — Singleton via once_cell.
/// Em producao, carregar de variavel de ambiente.
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "cityissue_super_secret_key_2026_change_in_prod".to_string())
});

/// Claims do token JWT.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub role: String,
    pub exp: usize, // timestamp de expiracao
}

/// Gera token JWT com expiracao de 8 horas.
pub fn generate_token(user_id: &str, email: &str, role: &str) -> Result<String, String> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 8 * 3600; // 8 horas

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|e| format!("Erro ao gerar token: {e}"))
}

/// Valida e decodifica token JWT. Retorna Claims se valido.
pub fn validate_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|e| format!("Token invalido: {e}"))
}
