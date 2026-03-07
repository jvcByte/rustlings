use sea_orm::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Default, Serialize)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    content: String,
}

#[derive(Debug, EnumIter, DeriveRelation, Clone)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
