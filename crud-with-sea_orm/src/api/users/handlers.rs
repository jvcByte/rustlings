//
// Handlers
//
//
use super::service::UserService;
use crate::AppState;
use crate::api::users::dto::{CreateUser, UpdateUser};
use crate::shared::errors::api_errors::ApiError;
use actix_web::{HttpResponse, Result, web};
use uuid::Uuid;

/// Create (register) a new user.
///
/// This mirrors the existing `create_user` behavior but is provided as an explicit
/// `register` endpoint to make the authentication flow clearer when adding auth later.
pub async fn register_user(
    body: web::Json<CreateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    // Reuse the service layer for creation/validation.
    let id = UserService::create_user(&state.db, body.into_inner()).await?;
    Ok(HttpResponse::Created().body(format!("User created with id {}", id)))
}

/// Existing handlers retained for backward compatibility / completeness.
pub async fn create_user(
    body: web::Json<CreateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let id = UserService::create_user(&state.db, body.into_inner()).await?;
    Ok(HttpResponse::Created().body(format!("User created with id {}", id)))
}

pub async fn list_users(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let users = UserService::list_users(&state.db).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let user = UserService::get_user(&state.db, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(
    path: web::Path<Uuid>,
    body: web::Json<UpdateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let updated_user = UserService::update_user(&state.db, id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_user))
}

pub async fn delete_user(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    UserService::delete_user(&state.db, id).await?;
    Ok(HttpResponse::NoContent().finish())
}
