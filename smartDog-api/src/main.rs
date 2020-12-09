use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Arroff arooff, world!");
    HttpServer::new(|| {
        App::new()
            .service(aroof)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn aroof() -> impl Responder {
    HttpResponse::Ok().body("Arroff Arooff!")
}


