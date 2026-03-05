use actix_web::web;

use super::handlers::{create_user, delete_user, get_user, list_users, update_user};

/// Re-exported so `api::mod` can call `users::routes`.
pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}
