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
                    .table(Echocardiography::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Echocardiography::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Echocardiography::Document).integer().not_null())
                    .col(ColumnDef::new(Echocardiography::Date).date().not_null())
                    .col(ColumnDef::new(Echocardiography::Rca).float())
                    .col(ColumnDef::new(Echocardiography::Lmca).float())
                    .col(ColumnDef::new(Echocardiography::Lad).float())
                    .col(ColumnDef::new(Echocardiography::Lcx).float())
                    .col(ColumnDef::new(Echocardiography::RcaZ).float())
                    .col(ColumnDef::new(Echocardiography::LmcaZ).float())
                    .col(ColumnDef::new(Echocardiography::LadZ).float())
                    .col(ColumnDef::new(Echocardiography::LcxZ).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-echocardiography_document_id")
                            .from(Echocardiography::Table, Echocardiography::Document)
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
            .drop_table(Table::drop().table(Echocardiography::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Echocardiography {
    Table,
    Id,
    Document,
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
