/*!
Library entry for the crate so integration tests and other consumers can
depend on this package as a library (e.g. `crud_with_sea_orm::...`).

This file exposes the main application modules that tests and other crates
need to access:

- `api`    — HTTP routes and feature modules (re-exports `routes` configuration).
- `shared` — shared application state, configuration and helpers (re-exports `AppState`).

The binary `src/main.rs` remains the application entry point for running the server.
*/

pub mod api;
pub mod shared;

// Re-export commonly used items for convenience in integration tests.
pub use api::routes;
pub use shared::AppState;
