use std::{str::FromStr, sync::Arc};

use serde::Deserialize;
use uuid::Uuid;

use crate::router::html;

use super::model::{Book, BookRepository};

#[derive(Deserialize)]
pub struct FormData {
    name: String,
}

#[derive(Clone)]
pub struct BookGetController {
    repository: Arc<dyn BookRepository>,
}

impl BookGetController {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
    pub fn invoke(&self, id: Uuid) -> Result<Book, String> {
        self.repository.find(id)
    }
    pub fn update(&self, book: Book) -> Result<(), String> {
        self.repository.update(book)
    }
}

pub fn find_success(item: Book) -> String {
    let action = format!("/books/update/{}", item.id);
    html::put_form(&action, html::input("name", &item.name.to_string()))
}

pub fn find(controller: &BookGetController, id: String) -> String {
    let uuid = Uuid::from_str(&id).unwrap();
    let item = controller.invoke(uuid);
    match item {
        Result::Ok(item) => find_success(item),
        Result::Err(_) => "ng".to_string(),
    }
}

pub fn update(controller: &BookGetController, id: String, form: &FormData) -> Result<(), String> {
    let uuid = Uuid::from_str(&id).unwrap();
    let item = controller.invoke(uuid);
    if item.is_err() {
        return Err("not found".to_string());
    }
    let mut item = item.unwrap();
    item.name = form.name.clone();

    let result = controller.update(item);
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
