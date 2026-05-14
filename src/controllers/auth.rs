// ============================================================
// controllers/auth.rs — MVC Controller de Autenticacao
// Implementa: Req. A1 (JWT Spoofing), B1 (Argon2), B3 (JWT),
//             C4 (Rate Limiting), B2 (Validacao de entrada).
// ============================================================

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, User};
use crate::repositories::user_repository::UserRepository;
use crate::utils::{
    jwt::generate_token,
    password::{hash_password, verify_password},
    response::ApiResponse,
    validation::{is_strong_password, is_valid_email},
};
use crate::AppState;

// -------- CADASTRO --------

pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let db = state.db.lock().unwrap();

    // Req. B2 — Validacao de entrada (OWASP ASVS V5.1.1)
    let nome = body.nome.trim().to_string();
    let email = body.email.trim().to_lowercase();

    if nome.is_empty() || nome.len() > 100 {
        return ApiResponse::<()>::error(400, "Nome invalido (1-100 caracteres).");
    }
    if !is_valid_email(&email) {
        return ApiResponse::<()>::error(400, "Formato de e-mail invalido.");
    }
    if let Err(msg) = is_strong_password(&body.senha) {
        return ApiResponse::<()>::error(400, msg);
    }

    // Verificar duplicidade de e-mail
    if UserRepository::email_exists(&db, &email) {
        return ApiResponse::<()>::error(400, "Este e-mail ja esta cadastrado.");
    }

    // Req. B1 — Hash Argon2id (OWASP ASVS V2.4.1)
    let senha_hash = match hash_password(&body.senha) {
        Ok(h) => h,
        Err(e) => return ApiResponse::<()>::error(500, &e),
    };

    let user = User {
        id: Uuid::new_v4().to_string(),
        nome,
        email: email.clone(),
        senha_hash,
        role: "user".to_string(),
        criado_em: Utc::now().to_rfc3339(),
        ativo: true,
    };

    match UserRepository::create(&db, &user) {
        Ok(_) => {
            // Req. B3 — gerar JWT imediatamente apos cadastro
            let token = generate_token(&user.id, &email, &user.role).unwrap_or_default();
            ApiResponse::success(
                "Cadastro realizado com sucesso!",
                Some(AuthResponse {
                    token,
                    user_id: user.id,
                    nome: user.nome,
                    role: user.role,
                }),
            )
        }
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}

// -------- LOGIN --------

pub async fn login(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let db = state.db.lock().unwrap();

    let email = body.email.trim().to_lowercase();
    let ip = req
        .connection_info()
        .peer_addr()
        .unwrap_or("unknown")
        .to_string();

    // Req. C4 — Rate limiting: bloquear apos 5 falhas em 15 min
    let failures = UserRepository::recent_failures(&db, &email, 15);
    if failures >= 5 {
        return ApiResponse::<()>::error(
            429,
            "Conta temporariamente bloqueada por excesso de tentativas. Tente em 15 minutos.",
        );
    }

    // Verificar credenciais
    let user = match UserRepository::find_by_email(&db, &email) {
        Some(u) => u,
        None => {
            UserRepository::log_attempt(&db, &email, &ip, false);
            // Req. A2 — Mensagem generica para nao revelar existencia do e-mail
            return ApiResponse::<()>::error(401, "E-mail ou senha incorretos.");
        }
    };

    // Req. B1 — Verificar hash Argon2
    if !verify_password(&body.senha, &user.senha_hash) {
        UserRepository::log_attempt(&db, &email, &ip, false);
        return ApiResponse::<()>::error(401, "E-mail ou senha incorretos.");
    }

    UserRepository::log_attempt(&db, &email, &ip, true);

    // Req. B3 — Emitir JWT assinado HS256
    match generate_token(&user.id, &user.email, &user.role) {
        Ok(token) => ApiResponse::success(
            "Login realizado com sucesso!",
            Some(AuthResponse {
                token,
                user_id: user.id,
                nome: user.nome,
                role: user.role,
            }),
        ),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}
