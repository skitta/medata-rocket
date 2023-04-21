use sea_orm_migration::prelude::*;

use super::m20230419_073327_kawasaki_patient::KawasakiPatient;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(KawasakiSamples::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KawasakiSamples::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KawasakiSamples::Patient).integer().not_null())
                    .col(ColumnDef::new(KawasakiSamples::Date).date().not_null())
                    .col(ColumnDef::new(KawasakiSamples::Type).string().not_null())
                    .col(ColumnDef::new(KawasakiSamples::Label).string().not_null())
                    .col(ColumnDef::new(KawasakiSamples::Status).string().not_null())
                    .col(ColumnDef::new(KawasakiSamples::Note).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-kawasaki_samples_patient_id")
                            .from(KawasakiSamples::Table, KawasakiSamples::Patient)
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
            .drop_table(Table::drop().table(KawasakiSamples::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum KawasakiSamples {
    Table,
    Id,
    Patient,
    Date,
    Type,
    Label,
    Status,
    Note,
}
