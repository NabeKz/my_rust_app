use actix_web::{HttpResponse, web::Data};

use super::super::handler;
use crate::features::book::model::Book;
use crate::handler::Context;
use crate::presentation::shared::Html;
use crate::router::html;

fn td(book: &Book) -> String {
    format!(
        "
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        ",
        book.id,
        book.name,
        html::link(book.id.to_string(), "edit".to_string()),
        html::delete_form(format!("/books/delete/{}", book.id), "".to_string())
    )
}

pub async fn index(data: Data<Context>) -> HttpResponse {
    let repository = data.book.clone();
    let books = handler::get_books(repository).await;
    let table = html::table(vec!["id", "name", "edit", "delete"], books, td);
    table.html()
}
