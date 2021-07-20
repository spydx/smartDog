use actix_web::{middleware, web, App, HttpServer};
use smart_dog_api::controllers::bowlscontroller::get_bowls;
use smart_dog_api::controllers::systemcontroller::aroof;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let binding = "127.0.0.1:8080";
    let root = "/api";

    println!("Creating server at: {}", &binding);

    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope(&root).service(aroof).service(get_bowls), //get all bowls
        )
    })
    .bind(&binding)?
    .run()
    .await
}