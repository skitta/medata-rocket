use sea_orm_migration::prelude::*;

use super::m20230419_065718_kawasaki_group::KawasakiGroup;

#[derive(Iden)]
pub enum KawasakiPatient {
    Table,
    Id,
    RegisteredId,
    DocumentId,
    InDate,
    Name,
    Gender,
    Age,
    Weight,
    Height,
    Relapse,
    Resistance,
    Group,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(KawasakiPatient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KawasakiPatient::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(KawasakiPatient::RegisteredId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KawasakiPatient::DocumentId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(KawasakiPatient::InDate).date().not_null())
                    .col(ColumnDef::new(KawasakiPatient::Name).string().not_null())
                    .col(
                        ColumnDef::new(KawasakiPatient::Gender).string().not_null(),
                    )
                    .col(ColumnDef::new(KawasakiPatient::Age).integer().not_null())
                    .col(ColumnDef::new(KawasakiPatient::Weight).float().not_null())
                    .col(ColumnDef::new(KawasakiPatient::Height).float().not_null())
                    .col(
                        ColumnDef::new(KawasakiPatient::Relapse)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(KawasakiPatient::Resistance)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(KawasakiPatient::Group).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-kawasaki_patient_group_id")
                            .from(KawasakiPatient::Table, KawasakiPatient::Group)
                            .to(KawasakiGroup::Table, KawasakiGroup::Id)
                            .on_update(ForeignKeyAction::SetNull)
                            .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-kawasaki_patient_registered_id")
                    .table(KawasakiPatient::Table)
                    .col(KawasakiPatient::RegisteredId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-kawasaki_patient_document_id")
                    .table(KawasakiPatient::Table)
                    .col(KawasakiPatient::DocumentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(KawasakiPatient::Table).to_owned())
            .await
    }
}
