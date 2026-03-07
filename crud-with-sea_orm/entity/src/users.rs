//
// SeaORM entity definition (users table)
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
