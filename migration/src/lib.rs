pub use sea_orm_migration::prelude::*;

mod m20230419_065718_create_enroll_group;
mod m20230419_073327_create_patient;
mod m20230419_135025_create_bloodtest;
mod m20230420_010437_create_liverfunction;
mod m20230420_011308_create_echocardiography;
mod m20230420_012910_create_samples;
mod m20230423_102741_create_document;
mod m20230423_113325_create_physique;
mod m20230423_124626_create_kawasaki;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230419_065718_create_enroll_group::Migration),
            Box::new(m20230419_073327_create_patient::Migration),
            Box::new(m20230419_135025_create_bloodtest::Migration),
            Box::new(m20230420_010437_create_liverfunction::Migration),
            Box::new(m20230420_011308_create_echocardiography::Migration),
            Box::new(m20230420_012910_create_samples::Migration),
            Box::new(m20230423_102741_create_document::Migration),
            Box::new(m20230423_113325_create_physique::Migration),
            Box::new(m20230423_124626_create_kawasaki::Migration)
        ]
    }
}
