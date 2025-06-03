use std::sync::Arc;

use actix_web::{Result, web::Data};

use super::handler;
use crate::domain::book::{Book, BookRepository};

fn td(book: &Book) -> String {
    format!(
        "
        <td>{}</td>
        <td>{}</td>
        ",
        book.id, book.name,
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

pub async fn get_books<T>(data: Data<Arc<T>>) -> Result<String>
where
    T: BookRepository,
{
    let books = handler::get_books(data.get_ref().clone()).await;
    Result::Ok(index(books))
}
