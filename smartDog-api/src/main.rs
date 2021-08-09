use std::time::Duration;

use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};

use smartdog_api::routes::bowls::{create_bowl, get_all, get_id, post_to_id};
use smartdog_api::routes::health::health_check;
use smartdog_api::routes::history::get_history_for_id;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let binding = "127.0.0.1:8080";
    let connection_string = "postgres://veko:password@localhost:5432/smartdog";

    let db_pool: PgPool = get_db_pool(connection_string)
        .await
        .expect("Failed to get DB Pool");

    println!("Creating server at: {}", &binding);
    let db_pool = Data::new(db_pool);
    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api")
                .route("/health_check", web::get().to(health_check))
                .route("/bowls", web::get().to(get_all))
                .route("/bowls", web::post().to(create_bowl))
                .route("/bowls/{bowlid}", web::get().to(get_id))
                .route("/bowls/{bowlid}", web::post().to(post_to_id))
                .route("/history/{bowlid}", web::post().to(get_history_for_id))
                .app_data(db_pool.clone()),
        )
    })
    .bind(&binding)?
    .run()
    .await
}

pub async fn get_db_pool(connection: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect(connection)
        .await
}
