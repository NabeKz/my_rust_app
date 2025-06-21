use actix_web::HttpRequest;
use actix_web::web::{Path, Query};
use actix_web::{HttpResponse, web::Data};
use serde::Deserialize;

use crate::context::Context;
use crate::features::book::model::Book;
use crate::features::book::usecase::GetBooksInput;
use crate::presentation::shared::html::{self, HtmlResponse, input, post_form, put_form};

#[derive(Deserialize)]
pub struct GetBookQuery {
    name: Option<String>,
}

impl From<GetBookQuery> for GetBooksInput {
    fn from(value: GetBookQuery) -> Self {
        GetBooksInput { name: value.name }
    }
}

fn td(book: &Book) -> String {
    format!(
        "
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        ",
        book.id().value(),
        book.name().value(),
        html::link(format!("/books/{}", book.id().value()), "edit".to_string()),
        html::delete_form(format!("/books/{}", book.id().value()), "")
    )
}

pub async fn list(data: Data<Context>, query: Query<GetBookQuery>) -> HttpResponse {
    let query = query.into_inner().into();
    let books = data.book_usecase.get_books(query).await;
    let table = html::table(vec!["id", "name", "edit", "delete"], books, td);
    let body = r"
    <div style=display:flex;gap:16px;>
        <a href=/>back</a>
        <a href=/books/create>create</a>
    </div>"
        .to_string()
        + &table;

    body.html()
}

pub async fn create(req: HttpRequest) -> HttpResponse {
    let cookie = req.cookie("error");
    let error = match cookie {
        Some(cookie) => cookie.value().to_string(),
        None => "".to_string(),
    };

    let body = format!("<div>{}</div>", error);
    let form = post_form("/books/create", input("name", ""));
    (body + &form).flush("error")
}

pub async fn edit(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let item = data.book_usecase.get_book(id).await;
    let response = match item {
        Result::Ok(item) => put_form(
            format!("/books/{}", item.id().value()),
            input("name", item.name().value()),
        ),
        Result::Err(_) => "ng".to_string(),
    };
    response.html()
}
