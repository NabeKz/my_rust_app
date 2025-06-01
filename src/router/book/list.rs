use std::sync::Arc;

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
    let items = controller
        .query()
        .iter()
        .map(|it| format!("<li>{}</li>", it.name))
        .collect::<String>();

    format!("<ul>{}</ul>", items) + "<a href=/>back</a>"
}
