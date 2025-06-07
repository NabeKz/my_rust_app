use actix_web::{HttpResponse, web::Data};

use crate::context::Context;
use crate::features::book::model::Book;
use crate::features::book::usecase;
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
    let repository = data.book.as_ref();
    let books = usecase::get_books(repository).await;
    let table = html::table(vec!["id", "name", "edit", "delete"], books, td);
    table.html()
}
