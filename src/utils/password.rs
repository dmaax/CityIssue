// ============================================================
// utils/password.rs — Hashing e verificacao com Argon2 (Req. B1)
// Referencia: OWASP ASVS V2.4.1 — Argon2id para armazenamento de senhas
// ============================================================

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use password_hash::rand_core::OsRng;

/// Gera hash Argon2id da senha. Cada chamada gera um salt unico.
/// ASVS V2.4.1: usar Argon2id com parametros seguros (default Argon2 ja atende).
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("Erro ao gerar hash: {e}"))
}

/// Verifica se a senha corresponde ao hash armazenado.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}
