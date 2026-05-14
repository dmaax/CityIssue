// ============================================================
// controllers/user.rs — MVC Controller de Usuarios
// ============================================================

use actix_web::{web, HttpRequest, HttpResponse};

use crate::repositories::user_repository::UserRepository;
use crate::utils::{jwt::validate_token, response::ApiResponse};
use crate::AppState;

fn require_admin(req: &HttpRequest) -> Result<(), HttpResponse> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !header.starts_with("Bearer ") {
        return Err(ApiResponse::<()>::error(401, "Token ausente."));
    }

    let claims = validate_token(&header[7..]).map_err(|e| ApiResponse::<()>::error(401, &e))?;

    if claims.role != "admin" {
        return Err(ApiResponse::<()>::error(
            403,
            "Acesso restrito a administradores.",
        ));
    }

    Ok(())
}

/// Lista todos os usuarios. Requer role admin.
pub async fn list(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    if let Err(resp) = require_admin(&req) {
        return resp;
    }
    let db = state.db.lock().unwrap();
    let users = UserRepository::list(&db);
    ApiResponse::success("OK", Some(users))
}

/// Desativa um usuario pelo id. Requer role admin.
pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    if let Err(resp) = require_admin(&req) {
        return resp;
    }
    let id = path.into_inner();
    let db = state.db.lock().unwrap();
    match UserRepository::deactivate(&db, &id) {
        Ok(_) => ApiResponse::<String>::success("Usuario desativado.", None),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}
