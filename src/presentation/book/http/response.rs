use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse {
    name: String,
}

pub async fn response() -> HttpResponse {
    HttpResponse::Ok().json(JsonResponse {
        name: String::from(""),
    })
}
