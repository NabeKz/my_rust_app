use std::{str::FromStr, sync::Arc};

use serde::Deserialize;
use uuid::Uuid;

use crate::features::book::model::{
    Book, BookId, BookName, BookRepository, DomainError, DomainResult,
};

#[derive(Deserialize)]
pub struct CreateDto {
    pub name: String,
}
#[derive(Deserialize)]
pub struct UpdateDto {
    pub name: String,
}

pub trait BookUsecase: Sync + Send + 'static {
    fn get_book(&self, id: String) -> DomainResult<Book>;
    fn get_books(&self) -> Vec<Book>;
    fn create_book(&self, dto: CreateDto) -> DomainResult<()>;
    fn update_book(&self, id: String, dto: UpdateDto) -> DomainResult<()>;
    fn delete_book(&self, id: String) -> DomainResult<()>;
}

#[derive(Clone)]
pub struct BookUsecaseImpl {
    repository: Arc<dyn BookRepository>,
}

impl BookUsecaseImpl {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
}

impl BookUsecase for BookUsecaseImpl {
    fn get_books(&self) -> Vec<Book> {
        self.repository.list()
    }
    fn get_book(&self, id: String) -> DomainResult<Book> {
        let uuid = Uuid::from_str(&id)
            .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
        let book_id = BookId::from_uuid(uuid);
        self.repository.find(&book_id)
    }
    fn create_book(&self, dto: CreateDto) -> DomainResult<()> {
        let book_name = BookName::new(dto.name)?;
        let book = Book::new(book_name);
        self.repository.save(book)
    }
    fn update_book(&self, id: String, dto: UpdateDto) -> DomainResult<()> {
        let uuid = Uuid::from_str(&id)
            .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
        let book_id = BookId::from_uuid(uuid);
        let existing_book = self.repository.find(&book_id)?;
        let new_name = BookName::new(dto.name)?;
        let updated_book = existing_book.update_name(new_name);
        self.repository.update(updated_book)
    }
    fn delete_book(&self, id: String) -> DomainResult<()> {
        let uuid = Uuid::from_str(&id)
            .map_err(|_| DomainError::ValidationError(vec!["Invalid UUID format".to_string()]))?;
        let book_id = BookId::from_uuid(uuid);
        self.repository.delete(&book_id)
    }
}
