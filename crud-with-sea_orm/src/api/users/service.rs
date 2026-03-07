use crate::api::users::dto::{CreateUser, UpdateUser, UserResponse};
use crate::api::users::repository::UserRepository;
use crate::shared::errors::api_errors::ApiError;
use crate::shared::models::users::user::ActiveModel;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn create_user(db: &DatabaseConnection, input: CreateUser) -> Result<Uuid, ApiError> {
        // Validation
        if input.name.trim().is_empty() {
            return Err(ApiError::BadRequest("Name cannot be empty".into()));
        }

        if input.email.trim().is_empty() {
            return Err(ApiError::BadRequest("Email cannot be empty".into()));
        }

        if UserRepository::find_by_email(db, &input.email)
            .await
            .map_err(|_| ApiError::InternalError("DB error".to_string()))?
            .is_some()
        {
            return Err(ApiError::Conflict("Email already exists".into()));
        }

        let id = Uuid::new_v4();
        let active = ActiveModel {
            id: Set(id),
            name: Set(input.name),
            email: Set(input.email),
            created_at: Set(None),
            ..Default::default()
        };

        UserRepository::insert(db, active)
            .await
            .map_err(|_| ApiError::InternalError("DB insert failed".to_string()))?;

        Ok(id)
    }

    pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<UserResponse>, ApiError> {
        let users = UserRepository::find_all(db)
            .await
            .map_err(|_| ApiError::InternalError("DB error".to_string()))?;

        Ok(users
            .into_iter()
            .map(|m| UserResponse {
                id: m.id,
                name: m.name,
                email: m.email,
            })
            .collect())
    }

    pub async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<UserResponse, ApiError> {
        if id == Uuid::nil() {
            return Err(ApiError::BadRequest("Invalid UUID".into()));
        }

        let user = UserRepository::find_by_id(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB error".to_string()))?
            .ok_or_else(|| ApiError::NotFound(format!("User {} not found", id)))?;

        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        id: Uuid,
        input: UpdateUser,
    ) -> Result<UserResponse, ApiError> {
        let existing = UserRepository::find_by_id(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB error".to_string()))?
            .ok_or_else(|| ApiError::NotFound(format!("User {} not found", id)))?;

        let mut active: ActiveModel = existing.into();

        if let Some(name) = input.name {
            if name.trim().is_empty() {
                return Err(ApiError::BadRequest("Name cannot be empty".into()));
            }
            active.name = Set(name);
        }

        if let Some(email) = input.email {
            if email.trim().is_empty() {
                return Err(ApiError::BadRequest("Email cannot be empty".into()));
            }
            if UserRepository::find_by_email(db, &email)
                .await
                .map_err(|_| ApiError::InternalError("DB error".to_string()))?
                .filter(|u| u.id != id)
                .is_some()
            {
                return Err(ApiError::Conflict("Email already exists".into()));
            }
            active.email = Set(email);
        }

        let updated = UserRepository::update(db, active)
            .await
            .map_err(|_| ApiError::InternalError("DB update failed".to_string()))?;

        Ok(UserResponse {
            id: updated.id,
            name: updated.name,
            email: updated.email,
        })
    }

    pub async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
        if id == Uuid::nil() {
            return Err(ApiError::BadRequest("Invalid UUID".into()));
        }

        let rows = UserRepository::delete(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB delete failed".to_string()))?;

        if rows == 0 {
            return Err(ApiError::NotFound(format!("User {} not found", id)));
        }

        Ok(())
    }

    // Similarly implement update, delete, get, list with validation
}
