use serde::Deserialize;

use crate::features::book::model::{Book, BookRepository};

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
}

pub async fn get_users(repository: &dyn BookRepository) -> Vec<Book> {
    repository.list()
}
