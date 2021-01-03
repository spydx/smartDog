use actix_web::{get, post, put, web, Error, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::{Bowls};
use crate::{services, models};
use chrono::{Utc};
use crate::DbPool;

#[get("/bowls/{id}")]
pub async fn get_bowl_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    ) -> Result<HttpResponse,Error> {

    let bowl_uuid = id.into_inner();
    let con = pool.get().expect("db connect error");
    let bowl = web::block(move || services::find_bowl_by_uuid(bowl_uuid, &con))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    if let Some(bowl) = bowl {
        Ok(HttpResponse::Ok().json(bowl))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No bowl with thtat UUID: {}", bowl_uuid));
        Ok(res)
    }
}

#[put("/bowls/{id}")]
pub async fn put_bowl_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    data: web::Json<models::WaterLevel>,
    ) -> Result<HttpResponse, Error> {

    let bowl_uuid = id.into_inner();
    let con = pool.get().expect("db connec error");

    let bowl_update = web::block(move || services::update_bowl_id(bowl_uuid,&con, data.waterlevel))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    if let Some(bowl_update) = bowl_update {
        Ok(HttpResponse::Ok().json(bowl_update))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No bowl with that UUID: {}", bowl_uuid));
        Ok(res)
    }
}

#[post("/bowls/")]
pub async fn post_bowl_id(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewBowl>,
) -> Result<HttpResponse, Error> {

    let con = pool.get().expect("db connect error");

    let bowl = Bowls {
        id: Uuid::new_v4().to_string(),
        name: form.name.clone(),
        waterlevel: 0,
        timestamp: Utc::now().to_string()
    };
    dbg!(&bowl);
    let addbowl = web::block(move ||services::insert_bowl(bowl,&con))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(addbowl) = addbowl {
        Ok(HttpResponse::Ok().json(addbowl))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("Unable to add a new bowl with name: {}", form.name));
        Ok(res)
    }

}


#[get("/health")]
pub async fn aroof() -> impl Responder {
    // return health of the system
    HttpResponse::Ok().json("Arroff Arooff!")
}
