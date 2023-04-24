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
                    .table(Kawasaki::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Kawasaki::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Kawasaki::Document).integer().not_null())
                    .col(ColumnDef::new(Kawasaki::OnSet).date().not_null())
                    .col(ColumnDef::new(Kawasaki::FeverTime).integer().not_null())
                    .col(ColumnDef::new(Kawasaki::Relapse).boolean().default(false))
                    .col(ColumnDef::new(Kawasaki::Resistance).boolean().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-Kawasaki_document_id")
                            .from(Kawasaki::Table, Kawasaki::Document)
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
            .drop_table(Table::drop().table(Kawasaki::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Kawasaki {
    Table,
    Id,
    Document,
    // 发病日期
    OnSet,
    // 发热时间
    FeverTime,
    // 是否复发
    Relapse,
    // 是否IVIG抵抗
    Resistance,
}
