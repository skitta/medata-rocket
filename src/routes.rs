use rocket::get;
use rocket::request::FlashMessage;
use sea_orm_rocket::Connection;
use serde_json::{ json, Value };

use crate::models::Db;
use crate::models::mutation::kawasaki;

#[get("/patients?<page>&<page_size>")]
pub async fn patients(
    conn: Connection<'_, Db>,
    page: Option<u64>,
    page_size: Option<u64>,
    flash: Option<FlashMessage<'_>>
) -> Value {
    let db = conn.into_inner();
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);
    let (patients, num_page) = kawasaki::Patient::all(db, page, page_size).await.expect("cannot find patients in page");
    
    json!({
        "page": page,
        "page_size": page_size,
        "total_page": num_page,
        "patients": patients,
        "flash": flash.map(FlashMessage::into_inner)
    })
}
