use rocket::form::Form;
use rocket::response::status::{Accepted, BadRequest, NotFound};
use rocket::serde::json::{json, Json, Value};
use sea_orm_rocket::Connection;

use crate::entity::patient;
use crate::mutation::Patient;
use crate::Db;

#[get("/patients?<page>&<page_size>")]
pub async fn get_all_patients(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    page_size: Option<u64>,
) -> Result<Value, NotFound<String>> {
    let db = conn.into_inner();
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let (patients, num_page) = Patient::all(db, page, page_size)
        .await
        .map_err(|e| NotFound(e.to_string()))?;

    Ok(json!({
        "page": page,
        "page_size": page_size,
        "total_page": num_page,
        "patients": patients,
    }))
}

#[get("/patients/<id>")]
pub async fn get_patient(
    conn: Connection<'_, Db>,
    id: i32,
) -> Result<Json<patient::Model>, NotFound<String>> {
    let patient = Patient::one(conn.into_inner(), id)
        .await
        .map_err(|e| NotFound(e.to_string()))?;

    Ok(Json(patient))
}

#[post("/patients/new", data = "<formdata>")]
pub async fn new_patient(
    conn: Connection<'_, Db>,
    formdata: Form<patient::Model>,
) -> Result<Accepted<String>, BadRequest<String>> {
    match Patient::create(conn.into_inner(), formdata.into_inner()).await {
        Ok(_) => Ok(Accepted(Some("patient created".to_owned()))),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}

#[post("/patient/<id>", data = "<formdata>")]
pub async fn update_patient(
    conn: Connection<'_, Db>,
    id: i32,
    formdata: Form<patient::Model>,
) -> Result<Accepted<String>, BadRequest<String>> {
    match Patient::update(conn.into_inner(), id, formdata.into_inner()).await {
        Ok(_) => Ok(Accepted(Some("patient updated".to_owned()))),
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}
