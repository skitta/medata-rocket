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
                    .table(KawasakiBloodTest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KawasakiBloodTest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KawasakiBloodTest::Patient).integer().not_null())
                    .col(ColumnDef::new(KawasakiBloodTest::Date).date().not_null())
                    .col(ColumnDef::new(KawasakiBloodTest::Wbc).float())
                    .col(ColumnDef::new(KawasakiBloodTest::Ne).float())
                    .col(ColumnDef::new(KawasakiBloodTest::Ly).float())
                    .col(ColumnDef::new(KawasakiBloodTest::Mo).float())
                    .col(ColumnDef::new(KawasakiBloodTest::Rbc).float())
                    .col(ColumnDef::new(KawasakiBloodTest::Plt).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-kawasaki_blood_test_patient_id")
                            .from(KawasakiBloodTest::Table, KawasakiBloodTest::Patient)
                            .to(KawasakiPatient::Table, KawasakiPatient::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(KawasakiBloodTest::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum KawasakiBloodTest{
    Table,
    Id,
    Patient,
    Date,
    Wbc,
    Ne,
    Ly,
    Mo,
    Rbc,
    Plt,
}
