#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use controllers::*;

mod controllers;
mod models;
mod schema;
mod services;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let con = std::env::var("DATABASE_URL")
                                .expect("DATABASE_URL");
    let mgr = ConnectionManager::<SqliteConnection>::new(con);
    let pool = r2d2::Pool::builder()
        .build(mgr)
        .expect("Failed to create pool");

    let binding = "127.0.0.1:8080";
    let root = "/api";

    println!("Creating server at: {}", &binding);

    HttpServer::new( move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope(&root)
                    .service(aroof)
                    .service(post_bowl_id)//create
                    .service(get_bowl_id)//fetch
                    .service(put_bowl_id) //update

                ,)

    })
    .bind(&binding)?
    .run()
    .await
}
