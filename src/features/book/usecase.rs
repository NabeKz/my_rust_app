use std::{str::FromStr, sync::Arc};

use serde::Deserialize;
use uuid::Uuid;

use crate::features::book::model::{Book, BookId, BookName, BookRepository, DomainResult, DomainError};

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

pub async fn get_book(repo: &dyn BookRepository, id: String) -> DomainResult<Book> {
    let uuid = Uuid::from_str(&id)
        .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
    let book_id = BookId::from_uuid(uuid);
    repo.find(&book_id)
}

pub async fn create_book(repo: &dyn BookRepository, dto: CreateDto) -> DomainResult<()> {
    let book_name = BookName::new(dto.name)?;
    let book = Book::new(book_name);
    repo.save(book)
}

pub async fn update_book(
    repo: &dyn BookRepository,
    id: String,
    dto: UpdateDto,
) -> DomainResult<()> {
    let uuid = Uuid::from_str(&id)
        .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
    let book_id = BookId::from_uuid(uuid);
    let existing_book = repo.find(&book_id)?;
    let new_name = BookName::new(dto.name)?;
    let updated_book = existing_book.update_name(new_name);
    repo.update(updated_book)
}

pub async fn delete_book(repo: Arc<dyn BookRepository>, id: String) -> DomainResult<()> {
    let uuid = Uuid::from_str(&id)
        .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
    let book_id = BookId::from_uuid(uuid);
    repo.delete(&book_id)
}
