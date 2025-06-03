use actix_web::{HttpResponse, Result, web::Data};

use super::handler;
use crate::features::shared::Html;
use crate::{domain::book::Book, handler::Context};

fn td(book: &Book) -> String {
    format!(
        "
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        ",
        book.id, book.name, "edit", "delete"
    )
}

fn index(books: Vec<Book>) -> String {
    let items = books
        .iter()
        .map(|it| format!("<tr>{}</tr>", td(&it)))
        .collect::<String>();
    format!(
        "
    <table>
        <thead>
            <tr>
                <th>id</th>
                <th>name</th>
                <th>edit</th>
                <th>delete</th>
            </tr>
        </thead>
        <tbody>
        {}
        </tbody>
    </table>
    ",
        items
    )
}

pub async fn get_books(data: Data<Context>) -> Result<HttpResponse> {
    let repository = data.book.clone();
    let books = handler::get_books(repository).await;
    Result::Ok(index(books).html())
}
