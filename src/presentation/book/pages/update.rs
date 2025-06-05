use actix_web::{
    HttpResponse,
    web::{Data, Form, Path},
};

use crate::{
    features::book::{
        model::Book,
        usecase::{self, UpdateDto},
    },
    handler::Context,
    presentation::shared::html::{self, HtmlResponse},
};

pub fn find_success(item: Book) -> String {
    let action = format!("/books/update/{}", item.id);
    html::put_form(&action, html::input("name", &item.name.to_string()))
}

pub async fn query(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let item = usecase::get_user(data.book_update.as_ref(), id).await;
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
    let result = usecase::update_user(data.book_update.as_ref(), id, form.into_inner()).await;
    match result {
        Result::Ok(_) => html::redirect("/books"),
        Result::Err(_) => html::redirect("/books"),
    }
}
