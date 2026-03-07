//
// Handlers
//
//
use crate::AppState;
use crate::api::users::dto::{CreateUser, UpdateUser, UserResponse};
use crate::shared::models::users::{User, user};
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub async fn list_users(state: web::Data<AppState>) -> Result<HttpResponse> {
    let db: &DatabaseConnection = &state.db;
    let users = User::find()
        .all(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    // Map to response DTOs
    let resp: Vec<UserResponse> = users
        .into_iter()
        .map(|m| UserResponse {
            id: m.id,
            name: m.name,
            email: m.email,
        })
        .collect();

    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_user(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    match User::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?
    {
        Some(user) => {
            let resp = UserResponse {
                id: user.id,
                name: user.name,
                email: user.email,
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        None => Ok(HttpResponse::NotFound().body(format!("user {} not found", id))),
    }
}

pub async fn create_user(
    body: web::Json<CreateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let db: &DatabaseConnection = &state.db;
    let generated_id = Uuid::new_v4();

    let active = user::ActiveModel {
        id: Set(generated_id),
        name: Set(body.name.clone()),
        email: Set(body.email.clone()),
        // created_at default handled by DB or set to None
        created_at: Set(None),
        ..Default::default()
    };

    User::insert(active.clone())
        .exec(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    let resp = UserResponse {
        id: generated_id,
        name: active.name.unwrap(),
        email: active.email.unwrap(),
    };
    let message = format!(
        "User Created: {}",
        serde_json::to_string_pretty(&resp).unwrap()
    );
    Ok(HttpResponse::Created().body(message))
}

pub async fn update_user(
    path: web::Path<Uuid>,
    body: web::Json<UpdateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    // Fetch existing
    let existing = User::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    if let Some(model) = existing {
        let mut active: user::ActiveModel = model.into();

        if let Some(name) = &body.name {
            active.name = Set(name.clone());
        }
        if let Some(email) = &body.email {
            active.email = Set(email.clone());
        }

        let updated = active
            .update(db)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

        let resp = UserResponse {
            id: updated.id,
            name: updated.name,
            email: updated.email,
        };
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::NotFound().body(format!("user {} not found", id)))
    }
}

pub async fn delete_user(
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let db: &DatabaseConnection = &state.db;

    let res = User::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    if res.rows_affected > 0 {
        Ok(HttpResponse::Ok().body(format!("deleted user {}", id)))
    } else {
        Ok(HttpResponse::NotFound().body(format!("user {} not found", id)))
    }
}
