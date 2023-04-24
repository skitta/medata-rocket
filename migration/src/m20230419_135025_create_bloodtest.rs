use sea_orm_migration::prelude::*;

use super::m20230423_102741_create_document::Document;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(BloodTest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BloodTest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BloodTest::Document).integer().not_null())
                    .col(ColumnDef::new(BloodTest::Date).date().not_null())
                    .col(ColumnDef::new(BloodTest::Wbc).float())
                    .col(ColumnDef::new(BloodTest::Ne).float())
                    .col(ColumnDef::new(BloodTest::Ly).float())
                    .col(ColumnDef::new(BloodTest::Mo).float())
                    .col(ColumnDef::new(BloodTest::Rbc).float())
                    .col(ColumnDef::new(BloodTest::Plt).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-blood_test_document_id")
                            .from(BloodTest::Table, BloodTest::Document)
                            .to(Document::Table, Document::Id)
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
            .drop_table(Table::drop().table(BloodTest::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum BloodTest{
    Table,
    Id,
    Document,
    Date,
    Wbc,
    Ne,
    Ly,
    Mo,
    Rbc,
    Plt,
}
