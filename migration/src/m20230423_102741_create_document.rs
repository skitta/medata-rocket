use sea_orm_migration::prelude::*;

use super::m20230419_073327_create_patient::Patient;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Document::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Document::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Document::Patient).integer().not_null())
                    .col(ColumnDef::new(Document::CreatedAt).date().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-document_patient_id")
                            .from(Document::Table, Document::Patient)
                            .to(Patient::Table, Patient::Id)
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
            .drop_table(Table::drop().table(Document::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
/// 医疗记录模型
#[derive(Iden)]
pub enum Document {
    Table,
    Id,
    Patient,
    // 入院日期
    CreatedAt,
}
