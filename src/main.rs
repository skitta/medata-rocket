#[macro_use] extern crate rocket;

use rocket::{Rocket, Build, fairing::{self, AdHoc}};
use sea_orm_rocket::Database;
use migration::MigratorTrait;

use medata::models::Db;
use medata::routes::patients;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrate", run_migrations))
        .mount("/api", routes![patients])
}
