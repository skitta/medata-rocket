#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;
use sea_orm_rocket::Database;

use medata::routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(medata::Db::init())
        .attach(AdHoc::try_on_ignite("Migrate", medata::run_migrations))
        .mount(
            "/api/kawasaki",
            routes![
                new_group,
                get_all_groups,
                new_patient,
                get_all_patients,
                get_patient,
                update_patient
            ]
        )
}
