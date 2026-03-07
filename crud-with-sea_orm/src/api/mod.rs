//! API root module — wires feature routes under a common `/api` prefix.
//!
//! This module exposes a single `routes` function that your `main` can pass to
//! `App::configure(...)`. It delegates to feature modules (e.g. `users`) which
//! should each provide their own `pub fn routes(cfg: &mut web::ServiceConfig)`.

use actix_web::{Responder, web};

mod users;
use crate::api::users::routes::user_routes;

async fn available_routes() -> impl Responder {
    web::Json(serde_json::json!({
        "available_routes": [
            "/api/users (GET, POST)",
            "/api/users/{id} (GET, PUT, DELETE)"
        ]
    }))
}

/// Mount all API routes under `/api`.
///
/// Example usage from `main.rs`:
///     .configure(crud_with_sea_orm::api::routes)
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("", web::get().to(available_routes))
            // Keep the API surface stable by grouping feature scopes under `/api`.
            // Each feature module (e.g. `users`) should expose `routes`.
            .configure(user_routes),
    );
}
