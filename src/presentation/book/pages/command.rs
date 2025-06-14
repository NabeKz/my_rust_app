use actix_web::{
    HttpResponse,
    web::{Data, Form, Path},
};
use serde::Deserialize;

use crate::{
    context::Context,
    features::book::usecase::{CreateBookInput, UpdateBookInput},
    presentation::shared::html::{redirect, redirect_with_error},
};

#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateBookRequest {
    pub name: String,
}

impl From<CreateBookRequest> for CreateBookInput {
    fn from(req: CreateBookRequest) -> Self {
        CreateBookInput { name: req.name }
    }
}

impl From<UpdateBookRequest> for UpdateBookInput {
    fn from(req: UpdateBookRequest) -> Self {
        UpdateBookInput { name: req.name }
    }
}

pub async fn create(data: Data<Context>, form: Form<CreateBookRequest>) -> HttpResponse {
    let input = form.into_inner().into();
    let result = data.book_usecase.create_book(input).await;

    match result {
        Result::Ok(_) => redirect("/books"),
        Result::Err(err) => redirect_with_error("/books", err),
    }
}

pub async fn update(
    data: Data<Context>,
    path: Path<String>,
    form: Form<UpdateBookRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let input = form.into_inner().into();
    let result = data.book_usecase.update_book(id, input).await;
    match result {
        Result::Ok(_) => redirect("/books"),
        Result::Err(_) => redirect("/books"),
    }
}

pub async fn delete(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let _ = data.book_usecase.delete_book(id).await;

    redirect("/books")
}
