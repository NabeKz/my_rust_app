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

pub fn index(controller: &BookListController) -> String {
    let form = |id: Uuid| -> String {
        let action = format!("/books/delete/{}?_method=DELETE", id.to_string());
        post_form(&action, "".to_string())
    };
    let items = controller
        .query()
        .iter()
        .map(|it| format!("<li>{}{}</li>", it.name, form(it.id)))
        .collect::<String>();
    format!("<ul>{}</ul>", items) + "<a href=/>back</a>"
}
