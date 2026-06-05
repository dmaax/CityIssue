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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_gerado_com_sucesso() {
        let result = hash_password("Abcdef1!");
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn hash_nao_contem_senha_em_texto_plano() {
        let senha = "SuperSecreta99!";
        let hash = hash_password(senha).unwrap();
        assert!(!hash.contains(senha));
    }

    #[test]
    fn verificacao_senha_correta() {
        let senha = "Abcdef1!";
        let hash = hash_password(senha).unwrap();
        assert!(verify_password(senha, &hash));
    }

    #[test]
    fn verificacao_senha_errada() {
        let hash = hash_password("Abcdef1!").unwrap();
        assert!(!verify_password("SenhaErrada1!", &hash));
    }

    #[test]
    fn hash_invalido_retorna_false() {
        assert!(!verify_password("Abcdef1!", "hash_invalido"));
    }

    #[test]
    fn dois_hashes_mesma_senha_sao_diferentes() {
        let senha = "Abcdef1!";
        let hash1 = hash_password(senha).unwrap();
        let hash2 = hash_password(senha).unwrap();
        assert_ne!(hash1, hash2, "Hashes devem diferir devido ao salt aleatorio");
    }
}
