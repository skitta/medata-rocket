pub use sea_orm_migration::prelude::*;

mod m20230419_065718_kawasaki_group;
mod m20230419_073327_kawasaki_patient;
mod m20230420_010437_create_kawasaki_liverfunction;
mod m20230420_011308_create_kawasaki_echocardiography;
mod m20230420_012910_create_kawasaki_samples;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230419_065718_kawasaki_group::Migration),
            Box::new(m20230419_073327_kawasaki_patient::Migration),
            Box::new(m20230420_010437_create_kawasaki_liverfunction::Migration),
            Box::new(m20230420_011308_create_kawasaki_echocardiography::Migration),
            Box::new(m20230420_012910_create_kawasaki_samples::Migration)
        ]
    }
}
