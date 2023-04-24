use sea_orm::{ DbConn, error::DbErr, Set, ActiveModelTrait, EntityTrait, DeleteResult, QueryOrder, PaginatorTrait };

use crate::entity::patient;

pub struct Patient;

impl Patient {
    pub async fn create(db: &DbConn, data: patient::Model) -> Result<(), DbErr> {
        patient::ActiveModel {
            id: Set(data.id.to_owned()),
            name: Set(data.name.to_owned()),
            gender: Set(data.gender.to_owned()),
            location: Set(data.location.to_owned()),
            birth: Set(data.birth.to_owned()),
            group: Set(data.group.to_owned()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    pub async fn update(db: &DbConn, id: i32, data: patient::Model) -> Result<(), DbErr> {
        let p: patient::ActiveModel = Self::one(db, id).await.map(Into::into)?;

        patient::ActiveModel {
            id: p.id,
            name: Set(data.name.to_owned()),
            gender: Set(data.gender.to_owned()),
            location: Set(data.location.to_owned()),
            birth: Set(data.birth.to_owned()),
            group: Set(data.group.to_owned()),
        }
        .update(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        patient::Entity::delete_by_id(id).exec(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        patient::Entity::delete_many().exec(db).await
    }

    pub async fn one(db: &DbConn, id: i32) -> Result<patient::Model, DbErr> {
        patient::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find patient.".to_owned()))
    }

    pub async fn all(db: &DbConn, page: u64, page_size: u64) -> Result<(Vec<patient::Model>, u64), DbErr> {
        let paginator = patient::Entity::find()
            .order_by_asc(patient::Column::Id)
            .paginate(db, page_size);
        let num_page = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_page))
    }
}
