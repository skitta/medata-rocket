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
                    .table(Samples::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Samples::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Samples::Document).integer().not_null())
                    .col(ColumnDef::new(Samples::Date).date().not_null())
                    .col(ColumnDef::new(Samples::Type).string().not_null())
                    .col(ColumnDef::new(Samples::Label).string().not_null())
                    .col(ColumnDef::new(Samples::Status).string().not_null())
                    .col(ColumnDef::new(Samples::Note).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-samples_document_id")
                            .from(Samples::Table, Samples::Document)
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
            .drop_table(Table::drop().table(Samples::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Samples {
    Table,
    Id,
    Document,
    Date,
    Type,
    Label,
    Status,
    Note,
}
