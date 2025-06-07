use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Form},
};

use crate::{
    context::Context,
    features::book::usecase::{self, CreateDto},
    presentation::shared::html::{HtmlResponse, input, post_form},
};

pub async fn query() -> HttpResponse {
    let content = input("name", "");
    post_form("/books/create", content).to_string().html()
}

fn success() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}
pub async fn command(data: Data<Context>, form: Form<CreateDto>) -> HttpResponse {
    let result = usecase::create_book(data.book.as_ref(), form.into_inner()).await;

    match result {
        Result::Ok(_) => success(),
        Result::Err(_) => success(),
    }
}
