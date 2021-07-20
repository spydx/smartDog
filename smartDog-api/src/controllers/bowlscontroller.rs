use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::models::bowls::{NewBowl, WaterLevel};
use uuid::Uuid;

type DbPool = String;

#[get("/bowls")]
pub async fn get_bowls(_pool: web::Data<DbPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/bowls/{id}")]
pub async fn get_bowl_id(_pool: web::Data<DbPool>, _id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[put("/bowls/{id}")]
pub async fn put_bowl_id(
    _pool: web::Data<DbPool>,
    _id: web::Path<Uuid>,
    _data: web::Json<WaterLevel>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[delete("/bowls/{id}")]
pub async fn del_bowl_id(_pool: web::Data<DbPool>, _id: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/bowls")]
pub async fn post_bowl_id(_pool: web::Data<DbPool>, _form: web::Json<NewBowl>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
