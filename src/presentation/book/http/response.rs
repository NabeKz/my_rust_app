use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::{
    context::Context,
    features::book::{
        model::Book,
        usecase::{CreateBookInput, UpdateBookInput},
    },
};

#[derive(Serialize)]
pub struct GetBookApiResponse {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateBookApiRequest {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateBookApiRequest {
    pub name: String,
}

impl From<&Book> for GetBookApiResponse {
    fn from(res: &Book) -> Self {
        GetBookApiResponse {
            id: res.id().value().to_string(),
            name: res.name().value().to_string(),
        }
    }
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

pub async fn get(data: Data<Context>) -> HttpResponse {
    let result = data.book_usecase.get_books().await;
    let res = result
        .iter()
        .map(GetBookApiResponse::from)
        .collect::<Vec<GetBookApiResponse>>();

    HttpResponse::Ok().json(res)
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

pub async fn delete(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let result = data.book_usecase.delete_book(id).await;

    match result {
        Result::Ok(_) => HttpResponse::Ok().json(()),
        Result::Err(_) => HttpResponse::BadRequest().json(()),
    }
}
