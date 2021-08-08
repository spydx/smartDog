use actix_web::HttpResponse;

pub async fn get_history_for_id() -> HttpResponse {
    HttpResponse::Ok().finish()
}