use std::str::FromStr;

use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Form, Path},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    features::book::model::Book, handler::Context, presentation::shared::Html, router::html,
};

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

pub fn find_success(item: Book) -> String {
    let action = format!("/books/update/{}", item.id);
    html::put_form(&action, html::input("name", &item.name.to_string()))
}

pub async fn query(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let uuid = Uuid::from_str(&id).unwrap();
    let item = data.book_update.find(uuid);
    let response = match item {
        Result::Ok(item) => find_success(item),
        Result::Err(_) => "ng".to_string(),
    };
    response.html()
}

pub async fn command(
    data: Data<Context>,
    path: Path<String>,
    form: Form<FormData>,
) -> HttpResponse {
    let id = path.into_inner();
    let uuid = Uuid::from_str(&id).unwrap();
    let item = data.book_update.find(uuid);
    if item.is_err() {
        return HttpResponse::SeeOther()
            .append_header((header::LOCATION, "/books"))
            .finish();
    }
    let mut item = item.unwrap();
    item.name = form.name.clone();

    let _ = data.book_update.update(item);

    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}
