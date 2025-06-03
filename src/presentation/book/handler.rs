use std::sync::Arc;

use serde::Deserialize;

use crate::{
    features::book::model::{Book, BookRepository},
    presentation::shared::Validator,
};

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

pub async fn get_book() -> Book {
    todo!()
}

pub async fn get_books(repository: Arc<dyn BookRepository>) -> Vec<Book> {
    repository.list()
}

pub fn create_book<T>(repository: T, form: &FormData) -> Result<(), Vec<String>>
where
    T: BookRepository,
{
    let book = Book::from_values(form.name.clone());
    let _ = repository.save(book);
    Ok(())
}
