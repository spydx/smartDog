#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, middleware, web, HttpResponse};
use actix_web_static_files;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use crate::controllers::systemcontroller::aroof;
use crate::controllers::bowlscontroller::{*};

mod models;
mod schema;
mod services;
mod controllers;
mod build;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone,Debug)]
pub struct Fcm {
    id: String,
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let con = std::env::var("DATABASE_URL")
                                .expect("DATABASE_URL");
    let fcmapikey = std::env::var("FCM_TOKEN")
                                .expect("FCM_TOKEN");
    let fcmid = std::env::var("FCM_SNDID")
                                .expect("FCM_SNDID");

    let binding = "127.0.0.1:8080";
    let root = "/api";

    println!("Creating server at: {}", &binding);
    dbg!(&con, &fcmapikey, &fcmid);

    let fcm = Fcm { id: fcmid, key: fcmapikey};

    let mgr = ConnectionManager::<SqliteConnection>::new(con);
    let pool = r2d2::Pool::builder()
        .build(mgr)
        .expect("Failed to create pool");

    HttpServer::new( move || {
        let gen = generate();
        App::new()
            .data(pool.clone())
            .data(fcm.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope(&root)
                    .service(aroof)
                    .service(post_bowl_id)//create
                    .service(get_bowls) //get all bowls
                    .service(get_bowl_id)//fetch
                    .service(put_bowl_id) //update
                    .service(del_bowl_id) //delete a bowl
                ,)
            .service(
                actix_web_static_files::ResourceFiles::new(
                    "/static", gen,
            ))
            .route("/", web::get().to(index))

    })
    .bind(&binding)?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<a href=/static/index.html>swagger</a>")
}
