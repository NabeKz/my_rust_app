use std::{str::FromStr, sync::Arc};

use uuid::Uuid;

use crate::router::html::post_form;

use super::model::{Book, BookRepository};

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
}

pub fn find(controller: &BookGetController, id: String) -> String {
    let uuid = Uuid::from_str(&id).unwrap();
    let item = controller.invoke(uuid);
    let result = match item {
        Result::Ok(_) => "ok",
        Result::Err(_) => "ng",
    };
    result.to_string()
}
