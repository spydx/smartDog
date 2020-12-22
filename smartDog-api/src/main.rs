use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use serde::{Serialize, Deserialize};
use r2d2_sqlite::{self, SqliteConnectionManager};


mod db;
use db::{Pool};

mod api;
use api::*;

mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mgr = SqliteConnectionManager::file("smartBowl.db");
    let pool = Pool::new(mgr).unwrap();
    println!("Arroff arooff, world!");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(aroof)
            .service(
                web::scope("/api")
                    .service(aroof)
                    .service(get_bowl_status)
                    .service(get_bowl_id)
                    .service(post_bowl_id)
                ,)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}