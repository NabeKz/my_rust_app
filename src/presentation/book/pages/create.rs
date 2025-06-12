use actix_web::{
    HttpRequest, HttpResponse,
    http::header,
    web::{Data, Form},
};
use serde::Deserialize;

use crate::{
    context::Context,
    features::book::usecase::CreateBookInput,
    presentation::shared::html::{HtmlResponse, input, post_form},
};

#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub name: String,
}

impl From<CreateBookRequest> for CreateBookInput {
    fn from(req: CreateBookRequest) -> Self {
        CreateBookInput { name: req.name }
    }
}

pub async fn query(req: HttpRequest) -> HttpResponse {
    let cookie = req.cookie("error");
    let error = match cookie {
        Some(cookie) => cookie.value().to_string(),
        None => "".to_string(),
    };

    let body = format!("<div>{}</div>", error);
    let form = post_form("/books/create", input("name", ""));
    (body + &form).flush("error")
}

fn success() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}

fn failure(err: String) -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books/create"))
        .append_header((header::SET_COOKIE, "error=".to_owned() + &err))
        .finish()
}

pub async fn command(data: Data<Context>, form: Form<CreateBookRequest>) -> HttpResponse {
    let input = form.into_inner().into();
    let result = data.book_usecase.create_book(input).await;

    match result {
        Result::Ok(_) => success(),
        Result::Err(err) => failure(err),
    }
}
