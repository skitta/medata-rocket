#[macro_use] extern crate rocket;

pub mod entity;
pub mod mutation;
pub mod routes;

use migration::MigratorTrait;

use std::time::Duration;
use async_trait::async_trait;

use sea_orm::{ ConnectOptions, DatabaseConnection, DbErr };
use sea_orm_rocket::{ Pool, Config, Database };
use sea_orm_rocket::rocket::figment::Figment;

use rocket::{Rocket, Build, fairing};

#[derive(Database)]
#[database("medata")]
pub struct Db(SeaOrmPool);

#[derive(Clone)]
pub struct SeaOrmPool {
    pub conn: DatabaseConnection
}

#[async_trait]
impl Pool for SeaOrmPool {
    type Error = DbErr;
    type Connection = DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let mut options: ConnectOptions = config.url.into();
        options
            .max_connections(config.max_connections as u32)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging(config.sqlx_logging);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}
