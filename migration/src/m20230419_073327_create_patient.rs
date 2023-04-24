use sea_orm_migration::prelude::*;

use super::m20230419_065718_create_enroll_group::EnrollGroup;

#[derive(Iden)]
pub enum Patient {
    Table,
    Id,
    Name,
    Gender,
    Birth,
    Location,
    Group,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Patient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Patient::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Patient::Name).string().not_null())
                    .col(ColumnDef::new(Patient::Gender).string().not_null())
                    .col(ColumnDef::new(Patient::Location).string().not_null())
                    .col(ColumnDef::new(Patient::Birth).date().not_null())
                    .col(ColumnDef::new(Patient::Group).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("pk-patient_group_id")
                            .from(Patient::Table, Patient::Group)
                            .to(EnrollGroup::Table, EnrollGroup::Id)
                            .on_update(ForeignKeyAction::SetNull)
                            .on_delete(ForeignKeyAction::SetNull)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Patient::Table).to_owned())
            .await
    }
}
