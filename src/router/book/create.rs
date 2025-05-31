use std::sync::Arc;

use serde::Deserialize;

use super::model::{Book, BookRepository};
use crate::router::html::{input, post_form};
use Result::{Err, Ok};

pub struct BookCreateController {
    repository: Arc<dyn BookRepository>,
}

trait Validator<T> {
    fn required(&self) -> Result<(), String>;
}

impl Validator<String> for String {
    fn required(&self) -> Result<(), String> {
        match self.is_empty() {
            true => Err("required".to_string()),
            false => Ok(()),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct FormData {
    pub name: String,
}

impl FormData {
    fn validate(self) -> Result<Self, Vec<String>> {
        let result = vec![self.name.required()]
            .into_iter()
            .filter_map(|res| res.err())
            .collect::<Vec<String>>();
        match result[..] {
            [] => Ok(self),
            _ => Err(result),
        }
    }
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

pub fn create(controller: &BookCreateController, form: &FormData) -> Result<(), Vec<String>> {
    let book = Book::from_values(form.name.clone());
    let _ = controller.repository.save(book);
    Ok(())
}
