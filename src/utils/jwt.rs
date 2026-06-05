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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_token_returns_ok() {
        let result = generate_token("user123", "user@example.com", "admin");
        assert!(result.is_ok(), "generate_token deve retornar Ok");
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn roundtrip_generate_then_validate() {
        let token = generate_token("user456", "test@mail.com", "citizen")
            .expect("falha ao gerar token");
        let claims = validate_token(&token).expect("falha ao validar token gerado");
        assert_eq!(claims.sub, "user456");
    }

    #[test]
    fn claims_match_input_data() {
        let token = generate_token("id_789", "hello@world.com", "moderator")
            .expect("falha ao gerar token");
        let claims = validate_token(&token).expect("falha ao validar token");

        assert_eq!(claims.sub, "id_789");
        assert_eq!(claims.email, "hello@world.com");
        assert_eq!(claims.role, "moderator");
        assert!(claims.exp > 0, "exp deve ser um timestamp positivo");
    }

    #[test]
    fn invalid_token_returns_err() {
        let result = validate_token("isto.nao.eh.um.token.valido");
        assert!(result.is_err(), "token corrompido deve retornar Err");
    }

    #[test]
    fn empty_token_returns_err() {
        let result = validate_token("");
        assert!(result.is_err(), "token vazio deve retornar Err");
    }

    #[test]
    fn tampered_signature_returns_err() {
        let token = generate_token("user_tamper", "tamper@test.com", "admin")
            .expect("falha ao gerar token");

        // Corrompe o ultimo caractere da assinatura
        let mut tampered = token.clone();
        let last = tampered.pop().unwrap();
        let replacement = if last == 'A' { 'B' } else { 'A' };
        tampered.push(replacement);

        let result = validate_token(&tampered);
        assert!(result.is_err(), "token com assinatura adulterada deve retornar Err");
    }
}
