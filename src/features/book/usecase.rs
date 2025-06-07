use std::{str::FromStr, sync::Arc};

use serde::Deserialize;
use uuid::Uuid;

use crate::features::book::model::{Book, BookRepository};

#[derive(Deserialize)]
pub struct CreateDto {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateDto {
    pub name: String,
}

pub async fn get_books(repository: &dyn BookRepository) -> Vec<Book> {
    repository.list()
}

pub async fn get_book(repo: &dyn BookRepository, id: String) -> Result<Book, String> {
    let uuid = Uuid::from_str(&id).unwrap();
    repo.find(uuid)
}

pub async fn create_book(repo: &dyn BookRepository, dto: CreateDto) -> Result<(), Vec<String>> {
    let book = Book::from_values(dto.name);
    repo.save(book);
    Ok(())
}

pub async fn update_book(
    repo: &dyn BookRepository,
    id: String,
    dto: UpdateDto,
) -> Result<(), Vec<String>> {
    let uuid = Uuid::from_str(&id).unwrap();
    let item = repo.find(uuid);
    if item.is_err() {
        return Result::Err(vec!["ng".to_string()]);
    }
    let mut item = item.unwrap();
    item.name = dto.name;

    repo.update(item)
}

pub async fn delete_book(repo: Arc<dyn BookRepository>, id: String) -> Result<(), Vec<String>> {
    let uuid = Uuid::from_str(&id).unwrap();
    repo.delete(uuid);
    Ok(())
}
