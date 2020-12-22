use actix_web::{get, post, web, HttpResponse, HttpServer, Responder};

use crate::models::{BowlContent, AlertLevel};
use chrono::{DateTime, Utc, Date};

#[get("/health")]
pub async fn aroof() -> impl Responder {
    // return health of the system
    HttpResponse::Ok().json("Arroff Arooff!")
}


#[get("/bowls/{id}")]
pub async fn get_bowl_id(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("bowl: {}", path.into_inner().0))
}

#[post("/bowls/{id}")]
pub async fn post_bowl_id(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[get("/bowls/status/")]
pub async fn get_bowl_status() -> impl Responder {
    let time = Utc::now();
    let status = BowlContent {
        name: "Boisy".to_string(),
        alert: AlertLevel::LowWater,
        waterlevel: 20,
        time: time.to_rfc3339() };

    dbg!("{}", &status);
    HttpResponse::Ok().json(&status)
}

