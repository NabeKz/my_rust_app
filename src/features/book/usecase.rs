use serde::Deserialize;

use crate::features::book::model::{Book, BookRepository};

#[derive(Deserialize)]
pub struct CreateDto {
    pub name: String,
}

pub async fn get_users(repository: &dyn BookRepository) -> Vec<Book> {
    repository.list()
}

pub async fn create_user(repo: &dyn BookRepository, dto: CreateDto) -> Result<(), Vec<String>> {
    let book = Book::from_values(dto.name);
    let _ = repo.save(book);
    Ok(())
}
