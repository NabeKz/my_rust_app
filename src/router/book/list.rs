use std::sync::Arc;

use uuid::Uuid;

use crate::router::html::post_form;

use super::model::{Book, BookRepository};

#[derive(Clone)]
pub struct BookListController {
    repository: Arc<dyn BookRepository>,
}

impl BookListController {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
    pub fn query(&self) -> Vec<Book> {
        self.repository.list()
    }
}

fn link(id: Uuid) -> String {
    format!("<a href=/books/update/{}>{}</a>", id, "link")
}

fn form(id: Uuid) -> String {
    let action = format!("/books/delete/{}?_method=DELETE", id.to_string());
    post_form(&action, "".to_string())
}

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
        link(book.id),
        form(book.id)
    )
}

pub fn index(controller: &BookListController) -> String {
    let items = controller
        .query()
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
