//
// Handlers
//
//
use crate::AppState;
use crate::api::users::dto::{CreateUser, UpdateUser, UserResponse};
use crate::shared::models::users::{User, user};
use actix_web::{HttpResponse, Result, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

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

pub async fn get_user(path: web::Path<i32>, state: web::Data<AppState>) -> Result<HttpResponse> {
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

    let active = user::ActiveModel {
        // id is auto-increment primary key; leave as NotSet
        name: Set(body.name.clone()),
        email: Set(body.email.clone()),
        // created_at default handled by DB or set to None
        created_at: Set(None),
        ..Default::default()
    };

    let res = User::insert(active)
        .exec(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    // SeaORM's InsertResult may not return the full model; fetch it back.
    let created = User::find_by_id(res.last_insert_id)
        .one(db)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("db error: {}", e)))?;

    match created {
        Some(m) => {
            let resp = UserResponse {
                id: m.id,
                name: m.name,
                email: m.email,
            };
            Ok(HttpResponse::Created().json(resp))
        }
        None => Ok(HttpResponse::InternalServerError().body("failed to fetch created user")),
    }
}

pub async fn update_user(
    path: web::Path<i32>,
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

pub async fn delete_user(path: web::Path<i32>, state: web::Data<AppState>) -> Result<HttpResponse> {
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
