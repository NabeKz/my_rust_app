use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use serde::Serialize;

use crate::{context::Context, features::book::usecase::CreateDto};

#[derive(Serialize)]
pub struct JsonResponse {
    name: String,
}

pub async fn response() -> HttpResponse {
    HttpResponse::Ok().json(JsonResponse {
        name: String::from(""),
    })
}

pub async fn post(data: Data<Context>, json: Json<CreateDto>) -> HttpResponse {
    let result = data.book_usecase.create_book(json.into_inner()).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().json(()),
        Result::Err(_) => HttpResponse::BadRequest().json(()),
    }
}
