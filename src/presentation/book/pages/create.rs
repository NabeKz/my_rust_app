use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Form},
};

use crate::{
    features::book::model::Book,
    handler::Context,
    presentation::{book::handler::FormData, shared::Html},
    router::html::{input, post_form},
};

pub async fn index() -> HttpResponse {
    let content = input("name", "");
    post_form("/books/create", content).to_string().html()
}

pub async fn command(data: Data<Context>, form: Form<FormData>) -> HttpResponse {
    let book = Book::from_values(form.name.clone());
    let _ = data.book_create.save(book);

    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}
