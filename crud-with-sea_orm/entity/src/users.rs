//
// SeaORM entity definition (users table) with production-grade auth fields.
//
// Notes:
// - `password_hash` stores a secure Argon2 (or similar) hash; do NOT store plaintext.
// - `token_version` is used to invalidate issued JWTs when you want to revoke sessions
//   (increment the token_version on password change / global logout).
// - `is_active` allows disabling accounts without deleting them.
// - `last_login` is optional and can be updated on successful auth.
//
// When adding these fields, also add appropriate DB migration changes (unique index on email,
// non-null constraints, defaults for is_active and token_version, and an index if needed).
//
pub mod user {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub name: String,
        pub email: String,
        /// Argon2 password hash (never store plaintext passwords).
        pub password_hash: String,
        /// Token version to support revoking tokens (increment on logout/password change).
        pub token_version: i32,
        /// Whether the account is active.
        pub is_active: bool,
        /// Optional last login timestamp (timestamptz).
        pub last_login: Option<sea_orm::prelude::DateTimeWithTimeZone>,
        /// Optional created_at field. The actual DB column type should match your DB (e.g. timestamptz).
        pub created_at: Option<sea_orm::prelude::DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation {}

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            panic!("No Relations")
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub use user::Entity as User;
