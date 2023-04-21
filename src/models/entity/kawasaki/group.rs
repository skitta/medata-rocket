//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "kawasaki_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::patient::Entity")]
    KawasakiPatient,
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::KawasakiPatient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
