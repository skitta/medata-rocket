use sea_orm_migration::prelude::*;

use super::m20230419_073327_kawasaki_patient::KawasakiPatient;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration script
        manager
            .create_table(
                Table::create()
                    .table(KawasakiLiverFunction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KawasakiLiverFunction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KawasakiLiverFunction::Patient).integer().not_null())
                    .col(ColumnDef::new(KawasakiLiverFunction::Date).date().not_null())
                    .col(ColumnDef::new(KawasakiLiverFunction::Ast).float())
                    .col(ColumnDef::new(KawasakiLiverFunction::Alt).float())
                    .col(ColumnDef::new(KawasakiLiverFunction::Pa).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-kawasaki_liverfunction_patient_id")
                            .from(KawasakiLiverFunction::Table, KawasakiLiverFunction::Patient)
                            .to(KawasakiPatient::Table, KawasakiPatient::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(KawasakiLiverFunction::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum KawasakiLiverFunction {
    Table,
    Id,
    Patient,
    Date,
    Ast,
    Alt,
    Pa,
}
