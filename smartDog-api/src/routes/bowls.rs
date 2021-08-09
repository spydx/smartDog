use actix_http::header::HttpDate;
use actix_web::web::{self, Form};
use actix_web::{HttpRequest, HttpResponse};
use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::Waterbowl;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Params {
    pub id: String,
}
pub async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn create_bowl(form: Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let _bowl_id = match insert_new_bowl(&form.name, &pool).await {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().finish()
}

pub async fn post_to_id() -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub async fn get_id(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let bowl_id_str = match req.match_info().get("bowlid") {
        Some(v) => v,
        None => return HttpResponse::NoContent().finish(),
    };
    let id: i32 = bowl_id_str.to_string().parse().unwrap();
    let bowl = match get_bowl_from_id(id, &pool).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    // todo: missing returning data
    HttpResponse::Ok().finish()
}

async fn get_bowl_from_id(id: i32, pool: &PgPool) -> Result<Waterbowl, sqlx::Error> {
    let row = sqlx::query!(r#"SELECT id, name, disabled FROM bowls WHERE id = $1;"#, id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e)?;
    let r = row
        .map(|r| Waterbowl {
            bowl_id: r.id,
            name: r.name,
            disabled: r.disabled,
        })
        .unwrap();

    Ok(r)
}

async fn insert_new_bowl(name: &str, pool: &PgPool) -> anyhow::Result<i32, sqlx::Error> {
    let row_id = sqlx::query!(r#"SELECT currval(pg_get_serial_sequence('bowls','id'))"#)
        .fetch_optional(pool)
        .await
        .map_err(|e| e)?;

    let id = match row_id.map(|r| r.currval).unwrap() {
        Some(id) => id as i32,
        None => 1_i32,
    };

    sqlx::query!(
        r#"INSERT INTO bowls (id, name, disabled) VALUES ($1, $2, $3);"#,
        id,
        name,
        "enabled",
    )
    .execute(pool)
    .await
    .map_err(|e| e)?;

    Ok(id)
}
