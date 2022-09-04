use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

// Import service module
use crate::services;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

#[get("/date/get-current-date")]
pub fn get_current_date() -> Json<Date> {
    Json(services::date::get_current_date())
}
