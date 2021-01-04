use actix_web::{Error, HttpResponse,get, delete, post, put, web};
use chrono::Utc;
use uuid::Uuid;

use crate::{services, Fcm};
use crate::DbPool;
use crate::models::bowls::{Bowls, WaterLevel, NewBowl};
use crate::services::msgservice::push_msg;

#[get("/bowls")]
pub async fn get_bowls(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let con = pool.get().expect("db connect error");

    let bowls = web::block(move || services::bowlservice::find_all_bowls(&con))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    if let Ok(bowls) = bowls {
        Ok(HttpResponse::Ok().json(bowls))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("sorry"));
        Ok(res)
    }
}

#[get("/bowls/{id}")]
pub async fn get_bowl_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    ) -> Result<HttpResponse,Error> {

    let bowl_uuid = id.into_inner();
    let con = pool.get().expect("db connect error");
    let bowl = web::block(move || services::bowlservice::find_bowl_by_uuid(bowl_uuid, &con))
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
    msg: web::Data<Fcm>,
    id: web::Path<Uuid>,
    data: web::Json<WaterLevel>,
    ) -> Result<HttpResponse, Error> {

    let bowl_uuid = id.into_inner();
    let con = pool.get().expect("db connect error");

    let bowl_update = web::block(move || services::bowlservice::update_bowl_id(
        bowl_uuid,
        &con,
        data.waterlevel,
        ))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    if let Some(bowl_update) = bowl_update {
        push_msg(msg).await;
        Ok(HttpResponse::Ok().json(bowl_update))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No bowl with that UUID: {}", bowl_uuid));
        Ok(res)
    }
}

#[delete("/bowls/{id}")]
pub async fn del_bowl_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let bowl_uuid = id.into_inner();
    let con = pool.get().expect("db connect error");

    let bowl_delete = web::block(move || services::bowlservice::delete_bowl_id(bowl_uuid,&con))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    match bowl_delete {
        1 => Ok(HttpResponse::Ok().body("OK")),
        _ => Ok(HttpResponse::NotFound().body(format!("No bowl id: {}", bowl_uuid)))
    }
}

#[post("/bowls")]
pub async fn post_bowl_id(
    pool: web::Data<DbPool>,
    form: web::Json<NewBowl>,
) -> Result<HttpResponse, Error> {

    let con = pool.get().expect("db connect error");

    let bowl = Bowls {
        id: Uuid::new_v4().to_string(),
        name: form.name.clone(),
        waterlevel: 0,
        timestamp: Utc::now().to_string()
    };
    dbg!(&bowl);
    let addbowl = web::block(move ||services::bowlservice::insert_bowl(bowl,&con))
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