use sea_orm::{ DbConn, error::DbErr, Set, ActiveModelTrait, EntityTrait, DeleteResult, QueryOrder, PaginatorTrait };
use crate::entity::prelude::*;

pub struct Patient;

impl Patient {
    pub async fn create(db: &DbConn, data: KawasakiPatientModel) -> Result<KawasakiPatientActiveModel, DbErr> {
        KawasakiPatientActiveModel {
            registered_id: Set(data.registered_id.to_owned()),
            document_id: Set(data.document_id.to_owned()),
            in_date: Set(data.in_date.to_owned()),
            name: Set(data.name.to_owned()),
            gender: Set(data.gender.to_owned()),
            age: Set(data.age.to_owned()),
            weight: Set(data.weight.to_owned()),
            height: Set(data.height.to_owned()),
            relapse: Set(data.relapse.to_owned()),
            resistance: Set(data.resistance.to_owned()),
            group: Set(data.group.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update(db: &DbConn, id: i32, data: KawasakiPatientModel) -> Result<KawasakiPatientModel, DbErr> {
        let p: KawasakiPatientActiveModel = KawasakiPatient::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find patient.".to_owned()))
            .map(Into::into)?;

        KawasakiPatientActiveModel {
            id: p.id,
            registered_id: Set(data.registered_id.to_owned()),
            document_id: Set(data.document_id.to_owned()),
            in_date: Set(data.in_date.to_owned()),
            name: Set(data.name.to_owned()),
            gender: Set(data.gender.to_owned()),
            age: Set(data.age.to_owned()),
            weight: Set(data.weight.to_owned()),
            height: Set(data.height.to_owned()),
            relapse: Set(data.relapse.to_owned()),
            resistance: Set(data.resistance.to_owned()),
            group: Set(data.group.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        KawasakiPatient::delete_by_id(id).exec(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        KawasakiPatient::delete_many().exec(db).await
    }

    pub async fn all(db: &DbConn, page: u64, page_size: u64) -> Result<(Vec<KawasakiPatientModel>, u64), DbErr> {
        let paginator = KawasakiPatient::find()
            .order_by_asc(KawasakiPatientColumn::Id)
            .paginate(db, page_size);
        let num_page = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_page))
    }
}
