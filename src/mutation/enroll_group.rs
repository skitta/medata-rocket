use sea_orm::{ DbConn, Set, ActiveModelTrait, EntityTrait, error::DbErr };
use crate::entity::enroll_group;

pub struct EnrollGroup;

impl EnrollGroup {
    pub async fn create(db: &DbConn, name: &str) -> Result<(), DbErr> {
        enroll_group::ActiveModel {
            name: Set(name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await?;

        Ok(())
    }

    pub async fn one(db: &DbConn, id: i32) -> Result<enroll_group::Model, DbErr> {
        enroll_group::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find group.".to_owned()))
    }

    pub async fn all(db: &DbConn) -> Result<Vec<enroll_group::Model>, DbErr> {
        enroll_group::Entity::find()
            .all(db)
            .await
    }
}