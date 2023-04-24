#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use sea_orm_rocket::Database;

use medata::routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(medata::Db::init())
        .attach(AdHoc::try_on_ignite("Migrate", medata::run_migrations))
        .mount("/api/kawasaki", routes![get_all_patients, get_patient, new_patient, update_patient])
}
