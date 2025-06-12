use actix_web::{
    HttpResponse,
    web::{Data, Form, Path},
};
use serde::Deserialize;

use crate::{
    context::Context,
    features::book::{model::Book, usecase::UpdateBookInput},
    presentation::shared::html::{self, HtmlResponse},
};

#[derive(Deserialize)]
pub struct UpdateBookRequest {
    pub name: String,
}

impl From<UpdateBookRequest> for UpdateBookInput {
    fn from(req: UpdateBookRequest) -> Self {
        UpdateBookInput { name: req.name }
    }
}

pub fn find_success(item: Book) -> String {
    let action = format!("/books/{}", item.id().value());
    html::put_form(&action, html::input("name", item.name().value()))
}

pub async fn query(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let item = data.book_usecase.get_book(id).await;
    let response = match item {
        Result::Ok(item) => find_success(item),
        Result::Err(_) => "ng".to_string(),
    };
    response.html()
}

pub async fn command(
    data: Data<Context>,
    path: Path<String>,
    form: Form<UpdateBookRequest>,
) -> HttpResponse {
    let id = path.into_inner();
    let input = form.into_inner().into();
    let result = data.book_usecase.update_book(id, input).await;
    match result {
        Result::Ok(_) => html::redirect("/books"),
        Result::Err(_) => html::redirect("/books"),
    }
}
