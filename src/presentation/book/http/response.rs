use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};

use crate::{context::Context, features::book::usecase::CreateBookInput};

#[derive(Deserialize)]
pub struct CreateBookApiRequest {
    pub name: String,
}

impl From<CreateBookApiRequest> for CreateBookInput {
    fn from(req: CreateBookApiRequest) -> Self {
        CreateBookInput { name: req.name }
    }
}

#[derive(Serialize)]
pub struct JsonResponse {
    name: String,
}

pub async fn response() -> HttpResponse {
    HttpResponse::Ok().json(JsonResponse {
        name: String::from(""),
    })
}

pub async fn post(data: Data<Context>, json: Json<CreateBookApiRequest>) -> HttpResponse {
    let input = json.into_inner().into();
    let result = data.book_usecase.create_book(input).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().json(()),
        Result::Err(_) => HttpResponse::BadRequest().json(()),
    }
}
