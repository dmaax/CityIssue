// ============================================================
// utils/validation.rs — Validacao de entradas (Req. B2)
// Referencia: OWASP ASVS V5.1.1, SEI CERT IDS00-J
// ============================================================

use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap());

/// Valida formato de e-mail.
pub fn is_valid_email(email: &str) -> bool {
    email.len() <= 254 && EMAIL_REGEX.is_match(email)
}

/// Valida forca da senha (min 8 chars, upper, lower, digit, especial).
/// OWASP ASVS V2.1.1
pub fn is_strong_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("A senha deve ter pelo menos 8 caracteres.");
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("A senha deve conter ao menos uma letra maiuscula.");
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("A senha deve conter ao menos uma letra minuscula.");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("A senha deve conter ao menos um digito.");
    }
    if !password
        .chars()
        .any(|c| "!@#$%^&*()-_=+[]{}|;:',.<>?/`~".contains(c))
    {
        return Err("A senha deve conter ao menos um caractere especial.");
    }
    Ok(())
}

/// Sanitiza string removendo caracteres de controle HTML para prevenir XSS.
/// OWASP ASVS V5.2.1, CWE-79
pub fn sanitize_text(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Valida status de chamado contra lista de valores permitidos.
pub fn is_valid_status(status: &str) -> bool {
    matches!(status, "pending" | "in-progress" | "done" | "closed")
}

/// Valida tipo de chamado.
pub fn is_valid_tipo(tipo: &str) -> bool {
    matches!(
        tipo,
        "iluminacao" | "sinalizacao" | "limpeza" | "areas_verdes" | "agua" | "outro"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_valid_email ──────────────────────────────────────────

    #[test]
    fn email_valido_simples() {
        assert!(is_valid_email("user@example.com"));
    }

    #[test]
    fn email_valido_com_subdominio() {
        assert!(is_valid_email("user@mail.example.co.uk"));
    }

    #[test]
    fn email_valido_com_caracteres_especiais() {
        assert!(is_valid_email("user.name+tag@example.com"));
    }

    #[test]
    fn email_invalido_sem_arroba() {
        assert!(!is_valid_email("userexample.com"));
    }

    #[test]
    fn email_invalido_sem_dominio() {
        assert!(!is_valid_email("user@"));
    }

    #[test]
    fn email_invalido_sem_tld() {
        assert!(!is_valid_email("user@example"));
    }

    #[test]
    fn email_invalido_vazio() {
        assert!(!is_valid_email(""));
    }

    #[test]
    fn email_invalido_excede_254_caracteres() {
        let email = format!("{}@example.com", "a".repeat(243)); // 243+1+11 = 255
        assert!(!is_valid_email(&email));
    }

    // ── is_strong_password ──────────────────────────────────────

    #[test]
    fn senha_forte_aceita() {
        assert!(is_strong_password("Abcdef1!").is_ok());
    }

    #[test]
    fn senha_curta_rejeitada() {
        let err = is_strong_password("Ab1!").unwrap_err();
        assert!(err.contains("8 caracteres"));
    }

    #[test]
    fn senha_sem_maiuscula_rejeitada() {
        let err = is_strong_password("abcdef1!").unwrap_err();
        assert!(err.contains("maiuscula"));
    }

    #[test]
    fn senha_sem_minuscula_rejeitada() {
        let err = is_strong_password("ABCDEF1!").unwrap_err();
        assert!(err.contains("minuscula"));
    }

    #[test]
    fn senha_sem_digito_rejeitada() {
        let err = is_strong_password("Abcdefg!").unwrap_err();
        assert!(err.contains("digito"));
    }

    #[test]
    fn senha_sem_especial_rejeitada() {
        let err = is_strong_password("Abcdefg1").unwrap_err();
        assert!(err.contains("especial"));
    }

    // ── sanitize_text ───────────────────────────────────────────

    #[test]
    fn sanitiza_tags_html() {
        assert_eq!(sanitize_text("<script>alert('xss')</script>"),
                   "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
    }

    #[test]
    fn sanitiza_ampersand() {
        assert_eq!(sanitize_text("a & b"), "a &amp; b");
    }

    #[test]
    fn sanitiza_aspas_duplas() {
        assert_eq!(sanitize_text(r#"valor="x""#), "valor=&quot;x&quot;");
    }

    #[test]
    fn sanitiza_aspas_simples() {
        assert_eq!(sanitize_text("it's"), "it&#x27;s");
    }

    #[test]
    fn sanitiza_texto_sem_caracteres_perigosos() {
        assert_eq!(sanitize_text("texto normal 123"), "texto normal 123");
    }

    // ── is_valid_status ─────────────────────────────────────────

    #[test]
    fn status_pending_valido() {
        assert!(is_valid_status("pending"));
    }

    #[test]
    fn status_in_progress_valido() {
        assert!(is_valid_status("in-progress"));
    }

    #[test]
    fn status_done_valido() {
        assert!(is_valid_status("done"));
    }

    #[test]
    fn status_closed_valido() {
        assert!(is_valid_status("closed"));
    }

    #[test]
    fn status_invalido() {
        assert!(!is_valid_status("aberto"));
    }

    #[test]
    fn status_vazio_invalido() {
        assert!(!is_valid_status(""));
    }

    // ── is_valid_tipo ───────────────────────────────────────────

    #[test]
    fn tipo_iluminacao_valido() {
        assert!(is_valid_tipo("iluminacao"));
    }

    #[test]
    fn tipo_sinalizacao_valido() {
        assert!(is_valid_tipo("sinalizacao"));
    }

    #[test]
    fn tipo_limpeza_valido() {
        assert!(is_valid_tipo("limpeza"));
    }

    #[test]
    fn tipo_areas_verdes_valido() {
        assert!(is_valid_tipo("areas_verdes"));
    }

    #[test]
    fn tipo_agua_valido() {
        assert!(is_valid_tipo("agua"));
    }

    #[test]
    fn tipo_outro_valido() {
        assert!(is_valid_tipo("outro"));
    }

    #[test]
    fn tipo_invalido() {
        assert!(!is_valid_tipo("esgoto"));
    }

    #[test]
    fn tipo_vazio_invalido() {
        assert!(!is_valid_tipo(""));
    }
}
