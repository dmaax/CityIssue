// ============================================================
// controllers/chamado.rs — MVC Controller de Chamados (CRUD)
// Modelo de Dominio Principal: Chamado (relato de problema urbano)
// ============================================================

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::models::chamado::{
    Chamado, CreateChamadoRequest, UpdateChamadoRequest, UpdateStatusRequest,
};
use crate::repositories::chamado_repository::ChamadoRepository;
use crate::utils::{
    jwt::validate_token,
    response::ApiResponse,
    validation::{is_valid_status, is_valid_tipo, sanitize_text},
};
use crate::AppState;

/// Extrai e valida o Bearer token do header Authorization.
fn extract_claims(req: &HttpRequest) -> Result<crate::utils::jwt::Claims, HttpResponse> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !header.starts_with("Bearer ") {
        return Err(ApiResponse::<()>::error(401, "Token de autenticacao ausente."));
    }

    validate_token(&header[7..]).map_err(|e| ApiResponse::<()>::error(401, &e))
}

// -------- LIST --------
pub async fn list(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    // Req. A3 — Controle de acesso: apenas autenticados listam chamados
    if let Err(resp) = extract_claims(&req) {
        return resp;
    }
    let db = state.db.lock().unwrap();
    let chamados = ChamadoRepository::list(&db);
    ApiResponse::success("OK", Some(chamados))
}

// -------- CREATE --------
pub async fn create(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateChamadoRequest>,
) -> HttpResponse {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    // Req. B2 — Validar e sanitizar entradas
    let titulo = sanitize_text(body.titulo.trim());
    let descricao = sanitize_text(body.descricao.trim());
    let endereco = sanitize_text(body.endereco.trim());

    if titulo.is_empty() || titulo.len() > 200 {
        return ApiResponse::<()>::error(400, "Titulo invalido (1-200 caracteres).");
    }
    if descricao.is_empty() || descricao.len() > 2000 {
        return ApiResponse::<()>::error(400, "Descricao invalida (1-2000 caracteres).");
    }
    if !is_valid_tipo(&body.tipo) {
        return ApiResponse::<()>::error(400, "Tipo de chamado invalido.");
    }

    let db = state.db.lock().unwrap();

    // Recuperar nome do usuario
    let user = crate::repositories::user_repository::UserRepository::find_by_id(
        &db, &claims.sub,
    );
    let user_nome = user.map(|u| u.nome).unwrap_or_else(|| claims.email.clone());

    let now = Utc::now().to_rfc3339();
    let chamado = Chamado {
        id: Uuid::new_v4().to_string(),
        titulo,
        tipo: body.tipo.clone(),
        descricao,
        endereco,
        foto_url: body.foto_url.clone(),
        status: "pending".to_string(),
        user_id: claims.sub.clone(),
        user_nome,
        criado_em: now.clone(),
        atualizado_em: now,
    };

    match ChamadoRepository::create(&db, &chamado) {
        Ok(_) => ApiResponse::success("Chamado criado com sucesso!", Some(chamado)),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}

// -------- UPDATE --------
pub async fn update(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateChamadoRequest>,
) -> HttpResponse {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    let id = path.into_inner();
    let db = state.db.lock().unwrap();

    let mut chamado = match ChamadoRepository::find_by_id(&db, &id) {
        Some(c) => c,
        None => return ApiResponse::<()>::error(404, "Chamado nao encontrado."),
    };

    // Req. A3 — Apenas dono ou admin podem editar
    if chamado.user_id != claims.sub && claims.role != "admin" {
        return ApiResponse::<()>::error(403, "Sem permissao para editar este chamado.");
    }

    // Aplicar atualizacoes parciais (campos opcionais)
    if let Some(t) = &body.titulo {
        chamado.titulo = sanitize_text(t.trim());
    }
    if let Some(t) = &body.tipo {
        if !is_valid_tipo(t) {
            return ApiResponse::<()>::error(400, "Tipo invalido.");
        }
        chamado.tipo = t.clone();
    }
    if let Some(d) = &body.descricao {
        chamado.descricao = sanitize_text(d.trim());
    }
    if let Some(e) = &body.endereco {
        chamado.endereco = sanitize_text(e.trim());
    }
    if let Some(f) = &body.foto_url {
        chamado.foto_url = Some(f.clone());
    }
    if let Some(s) = &body.status {
        if !is_valid_status(s) {
            return ApiResponse::<()>::error(400, "Status invalido.");
        }
        chamado.status = s.clone();
    }
    chamado.atualizado_em = Utc::now().to_rfc3339();

    match ChamadoRepository::update(&db, &id, &chamado) {
        Ok(_) => ApiResponse::success("Chamado atualizado.", Some(chamado)),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}

// -------- UPDATE STATUS --------
pub async fn update_status(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateStatusRequest>,
) -> HttpResponse {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    // Req. A3 — Apenas admin muda status
    if claims.role != "admin" {
        return ApiResponse::<()>::error(403, "Apenas administradores podem alterar o status.");
    }

    if !is_valid_status(&body.status) {
        return ApiResponse::<()>::error(400, "Status invalido.");
    }

    let id = path.into_inner();
    let db = state.db.lock().unwrap();

    match ChamadoRepository::update_status(&db, &id, &body.status) {
        Ok(_) => ApiResponse::<String>::success(
            &format!("Status atualizado para '{}'.", body.status),
            None,
        ),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}

// -------- DELETE --------
pub async fn delete(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let claims = match extract_claims(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    let id = path.into_inner();
    let db = state.db.lock().unwrap();

    let chamado = match ChamadoRepository::find_by_id(&db, &id) {
        Some(c) => c,
        None => return ApiResponse::<()>::error(404, "Chamado nao encontrado."),
    };

    // Req. A3 — Apenas dono ou admin podem deletar
    if chamado.user_id != claims.sub && claims.role != "admin" {
        return ApiResponse::<()>::error(403, "Sem permissao para deletar este chamado.");
    }

    match ChamadoRepository::delete(&db, &id) {
        Ok(_) => ApiResponse::<String>::success("Chamado removido.", None),
        Err(e) => ApiResponse::<()>::error(500, &e),
    }
}
