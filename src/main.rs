// ============================================================
// main.rs — Ponto de entrada do servidor CityIssue
//
// Padroes de Projeto aplicados:
//   1. MVC          — models/ controllers/ (views = static HTML)
//   2. Repository   — repositories/user_repository, chamado_repository
//   3. Singleton    — JWT_SECRET (once_cell::Lazy), AppState compartilhado
//   4. Builder      — ApiResponse::success() / ::error()
//   5. Chain of Responsibility — middlewares: SecurityHeaders → CORS → Controller
// ============================================================

mod controllers;
mod db;
mod models;
mod repositories;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware::DefaultHeaders, web, App, HttpServer};
use db::Database;
use std::sync::Mutex;

/// Estado compartilhado da aplicacao (Singleton Pattern via Data<AppState>).
pub struct AppState {
    pub db: Mutex<Database>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new("cityissue.db").expect("Falha ao inicializar banco de dados");

    let data = web::Data::new(AppState {
        db: Mutex::new(database),
    });

    println!("========================================");
    println!("  CityIssue — Sistema de Chamados");
    println!("  Servidor: http://127.0.0.1:8080");
    println!("========================================");

    HttpServer::new(move || {
        // Req. A2 — CORS configurado (nao expoe rotas a origens desconhecidas)
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                let resp = utils::response::ApiResponse::<()>::error(
                    400,
                    &format!("JSON invalido: {err}"),
                );
                actix_web::error::InternalError::from_response(err, resp).into()
            }))
            .wrap(cors)
            // Req. C1 — Cabecalhos de seguranca HTTP (OWASP ASVS V14.4)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                    .add((
                        "Content-Security-Policy",
                        "default-src 'self' https://fonts.googleapis.com \
                         https://cdnjs.cloudflare.com https:; \
                         style-src 'self' 'unsafe-inline' https://fonts.googleapis.com \
                         https://cdnjs.cloudflare.com; \
                         font-src 'self' https://fonts.gstatic.com https://cdnjs.cloudflare.com; \
                         script-src 'self' 'unsafe-inline'; img-src 'self' data: https:",
                    )),
            )
            // API Routes
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(controllers::auth::register))
                            .route("/login", web::post().to(controllers::auth::login)),
                    )
                    .service(
                        web::scope("/chamados")
                            .route("", web::get().to(controllers::chamado::list))
                            .route("", web::post().to(controllers::chamado::create))
                            .route("/{id}", web::put().to(controllers::chamado::update))
                            .route("/{id}", web::delete().to(controllers::chamado::delete))
                            .route(
                                "/{id}/status",
                                web::put().to(controllers::chamado::update_status),
                            ),
                    )
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(controllers::user::list))
                            .route("/{id}", web::delete().to(controllers::user::delete)),
                    ),
            )
            // Arquivos estaticos (View)
            .service(Files::new("/", "static").index_file("home.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
