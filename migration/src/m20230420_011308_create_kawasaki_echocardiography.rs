use sea_orm_migration::prelude::*;

use super::m20230419_073327_create_kawasaki_patient::KawasakiPatient;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(KawasakiEchocardiography::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KawasakiEchocardiography::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KawasakiEchocardiography::Patient).integer().not_null())
                    .col(ColumnDef::new(KawasakiEchocardiography::Date).date().not_null())
                    .col(ColumnDef::new(KawasakiEchocardiography::Rca).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::Lmca).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::Lad).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::Lcx).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::RcaZ).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::LmcaZ).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::LadZ).float())
                    .col(ColumnDef::new(KawasakiEchocardiography::LcxZ).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-kawasaki_echocardiography_patient_id")
                            .from(KawasakiEchocardiography::Table, KawasakiEchocardiography::Patient)
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
            .drop_table(Table::drop().table(KawasakiEchocardiography::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum KawasakiEchocardiography {
    Table,
    Id,
    Patient,
    Date,
    Rca,
    Lmca,
    Lad,
    Lcx,
    LmcaZ,
    RcaZ,
    LadZ,
    LcxZ,
}
