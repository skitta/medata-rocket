use sea_orm_migration::prelude::*;

use super::m20230423_102741_create_document::Document;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration script
        manager
            .create_table(
                Table::create()
                    .table(LiverFunction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LiverFunction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LiverFunction::Document).integer().not_null())
                    .col(ColumnDef::new(LiverFunction::Date).date().not_null())
                    .col(ColumnDef::new(LiverFunction::Ast).float())
                    .col(ColumnDef::new(LiverFunction::Alt).float())
                    .col(ColumnDef::new(LiverFunction::Pa).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-liverfunction_document_id")
                            .from(LiverFunction::Table, LiverFunction::Document)
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
            .drop_table(Table::drop().table(LiverFunction::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum LiverFunction {
    Table,
    Id,
    Document,
    Date,
    Ast,
    Alt,
    Pa,
}
