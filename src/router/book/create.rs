use std::sync::Arc;

use crate::router::html::{input, post_form};

use super::model::{Book, BookRepository};

pub struct BookCreateController {
    repository: Arc<dyn BookRepository>,
}

impl BookCreateController {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
    pub fn query(&self) -> Vec<Book> {
        self.repository.list()
    }
}

pub fn index() -> String {
    let content = input("name");
    post_form("/books/create", content).to_string()
}
