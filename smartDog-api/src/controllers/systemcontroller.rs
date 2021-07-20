use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn aroof() -> impl Responder {
    // return health of the system
    HttpResponse::Ok().json("Arroff Arooff!")
}
