use actix_web::{
    HttpResponse,
    web::{Data, Form, Path},
};

use crate::{
    context::Context,
    features::book::{
        model::Book,
        usecase::{self, UpdateDto},
    },
    presentation::shared::html::{self, HtmlResponse},
};

pub fn find_success(item: Book) -> String {
    let action = format!("/books/{}", item.id().value());
    html::put_form(&action, html::input("name", item.name().value()))
}

pub async fn query(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let item = usecase::get_book(data.book.as_ref(), id).await;
    let response = match item {
        Result::Ok(item) => find_success(item),
        Result::Err(_) => "ng".to_string(),
    };
    response.html()
}

pub async fn command(
    data: Data<Context>,
    path: Path<String>,
    form: Form<UpdateDto>,
) -> HttpResponse {
    let id = path.into_inner();
    let result = usecase::update_book(data.book.as_ref(), id, form.into_inner()).await;
    match result {
        Result::Ok(_) => html::redirect("/books"),
        Result::Err(_) => html::redirect("/books"),
    }
}
