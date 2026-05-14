// ============================================================
// utils/validation.rs — Validacao de entradas (Req. B2)
// Referencia: OWASP ASVS V5.1.1, SEI CERT IDS00-J
// ============================================================

use regex::Regex;
use once_cell::sync::Lazy;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap()
});

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
    if !password.chars().any(|c| "!@#$%^&*()-_=+[]{}|;:',.<>?/`~".contains(c)) {
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
