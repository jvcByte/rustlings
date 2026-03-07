mod api;
mod shared;

use crate::shared::config::postgres::check_connection;
use crate::shared::{AppState, config::postgres};
use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use dotenvy::dotenv;
use env_logger::Env;
use log::{error, info};
use migration::{Migrator, MigratorTrait};
use std::env;

/// Simple health-check endpoint so you can verify the server is running.
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

async fn check_db_connection(state: web::Data<AppState>) -> impl Responder {
    match check_connection(&state.db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Database connection successful"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database connection failed: {}", e)
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env (if present) and initialize logging.
    dotenv().ok();
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env).init();

    // Initialize DB connection via the postgres module. This requires the
    // `DATABASE_URL` environment variable to be set. No secrets are hardcoded here.
    let db = match postgres::init_db().await {
        Ok(db) => db,
        Err(e) => {
            error!("failed to initialize database: {}", e);
            // Exit with non-zero status so orchestrators/CI notice startup failure.
            std::process::exit(1);
        }
    };
    if let Err(e) = Migrator::up(&db, None).await {
        error!("failed to run migrations: {}", e);
        std::process::exit(1);
    }

    // Build application state and start server.
    let state = web::Data::new(AppState::new(db));

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    info!("starting server at {}", &bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(health))
            .route("/db", web::get().to(check_db_connection))
            .configure(api::routes)
    })
    .bind(bind_addr)?
    .run()
    .await
}
