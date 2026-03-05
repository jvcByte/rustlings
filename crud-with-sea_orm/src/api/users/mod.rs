//! Users feature module: routes, handlers and SeaORM entity definitions.
//!
//! This file defines:
//! - a `routes` function that mounts the users endpoints under a scope (used by the top-level router)
//! - Actix handlers for CRUD operations (list, get, create, update, delete)
//! - a simple SeaORM entity definition for `users`
//!
//! Notes:
//! - Handlers expect a `web::Data<crate::AppState>` with a `db: sea_orm::DatabaseConnection` field.
//! - Error handling is intentionally simple: SeaORM errors are converted to 500 Internal Server Error responses.
//! - This is a compact, single-file example. For a larger project you may want to split entity/service/repo/handlers across files.

pub mod dto;
pub mod handlers;
pub mod repository;
pub mod routes;
pub mod service;
