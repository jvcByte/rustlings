//
// SeaORM entity definition (refresh_tokens table)
//
pub mod refresh_token {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "refresh_tokens")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub user_id: Uuid,
        /// Hashed refresh token (do NOT store plaintext refresh tokens)
        pub token_hash: String,
        /// Expiration timestamp for the refresh token (timestamptz)
        pub expires_at: Option<sea_orm::prelude::DateTimeWithTimeZone>,
        /// When the refresh token was created
        pub created_at: Option<sea_orm::prelude::DateTimeWithTimeZone>,
        /// Whether this token has been revoked
        pub revoked: bool,
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

pub use refresh_token::Entity as RefreshToken;
