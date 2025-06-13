use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::{
    context::Context,
    features::book::usecase::{CreateBookInput, UpdateBookInput},
};

#[derive(Deserialize)]
pub struct CreateBookApiRequest {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateBookApiRequest {
    pub name: String,
}

impl From<CreateBookApiRequest> for CreateBookInput {
    fn from(req: CreateBookApiRequest) -> Self {
        CreateBookInput { name: req.name }
    }
}
impl From<UpdateBookApiRequest> for UpdateBookInput {
    fn from(req: UpdateBookApiRequest) -> Self {
        UpdateBookInput { name: req.name }
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
pub async fn put(
    data: Data<Context>,
    path: Path<String>,
    json: Json<UpdateBookApiRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let input = json.into_inner().into();
    let result = data.book_usecase.update_book(id, input).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().json(()),
        Result::Err(_) => HttpResponse::BadRequest().json(()),
    }
}
