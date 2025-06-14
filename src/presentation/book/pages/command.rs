use actix_web::{HttpResponse, web::Data};

use crate::context::Context;
use crate::features::book::model::Book;
use crate::presentation::shared::html::{self, HtmlResponse};

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

pub async fn query(data: Data<Context>) -> HttpResponse {
    let books = data.book_usecase.get_books().await;
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
