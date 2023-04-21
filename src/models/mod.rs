mod entity;
pub mod mutation;

use async_trait::async_trait;
use sea_orm::{ ConnectOptions, DatabaseConnection, DbErr };
use std::time::Duration;
use sea_orm_rocket::{ Pool, Config, Database };
use sea_orm_rocket::rocket::figment::Figment;

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
