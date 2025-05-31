use std::sync::Arc;

use anyhow::Ok;
use serde::Deserialize;

use crate::router::html::{input, post_form};

use super::model::{Book, BookRepository};

pub struct BookCreateController {
    repository: Arc<dyn BookRepository>,
}
#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
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

pub fn create(controller: &BookCreateController, form: &FormData) -> Result<(), String> {
    if form.name == "" {
        Result::Err("ng".to_string())
    } else {
        Result::Ok(())
    }
}
