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
                    .table(Physique::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Physique::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Physique::Document).integer().not_null())
                    .col(ColumnDef::new(Physique::Date).date().not_null())
                    .col(ColumnDef::new(Physique::Height).float().not_null())
                    .col(ColumnDef::new(Physique::Weight).float().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-physique_document_id")
                            .from(Physique::Table, Physique::Document)
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
            .drop_table(Table::drop().table(Physique::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
/// 
/// 体格测量
#[derive(Iden)]
enum Physique {
    Table,
    Id,
    Document,
    Date,
    Height,
    Weight,
}
