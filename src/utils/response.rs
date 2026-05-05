// ============================================================
// utils/response.rs — Builder de respostas da API (Builder Pattern)
// Builder Pattern: ApiResponse::success() / ::error() constroem
// a resposta padronizada sem expor o construtor interno.
// ============================================================

use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Constroi resposta de sucesso com dados opcionais.
    pub fn success(message: &str, data: Option<T>) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: message.to_string(),
            data,
        })
    }

    /// Constroi resposta de erro com codigo HTTP customizavel.
    pub fn error(status: u16, message: &str) -> HttpResponse {
        let body: ApiResponse<()> = ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        };
        match status {
            400 => HttpResponse::BadRequest().json(body),
            401 => HttpResponse::Unauthorized().json(body),
            403 => HttpResponse::Forbidden().json(body),
            404 => HttpResponse::NotFound().json(body),
            429 => HttpResponse::TooManyRequests().json(body),
            _ => HttpResponse::InternalServerError().json(body),
        }
    }
}
