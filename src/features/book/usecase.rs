use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use std::result::Result::Ok;
use uuid::Uuid;

use crate::features::book::model::{
    Book, BookId, BookName, BookRepository, DomainError, DomainResult,
};

pub struct CreateBookInput {
    pub name: String,
}

pub struct UpdateBookInput {
    pub name: String,
}

#[async_trait]
pub trait BookUsecase: Sync + Send + 'static {
    async fn get_book(&self, id: String) -> DomainResult<Book>;
    async fn get_books(&self) -> Vec<Book>;
    async fn create_book(&self, input: CreateBookInput) -> DomainResult<()>;
    async fn update_book(&self, id: String, input: UpdateBookInput) -> DomainResult<()>;
    async fn delete_book(&self, id: String) -> DomainResult<()>;
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

#[async_trait]
impl BookUsecase for BookUsecaseImpl {
    async fn get_books(&self) -> Vec<Book> {
        self.repository.list().await
    }
    async fn get_book(&self, id: String) -> DomainResult<Book> {
        let uuid = Uuid::from_str(&id).map_err(|_| DomainError::ValidationError {
            errors: vec!["Invalid UUID format".to_string()],
        })?;
        let book_id = BookId::from_uuid(uuid);
        self.repository.find(&book_id).await
    }
    async fn create_book(&self, input: CreateBookInput) -> DomainResult<()> {
        let book_name = BookName::new(input.name)?;

        let book = Book::new(book_name);
        let _ = self.repository.save(book).await;
        Ok(())
    }
    async fn update_book(&self, id: String, input: UpdateBookInput) -> DomainResult<()> {
        let uuid = Uuid::from_str(&id).map_err(|_| DomainError::ValidationError {
            errors: vec!["Invalid UUID format".to_string()],
        })?;
        let book_id = BookId::from_uuid(uuid);
        let existing_book = self.repository.find(&book_id).await?;
        let new_name = BookName::new(input.name)?;
        let updated_book = existing_book.update_name(new_name);
        self.repository.update(updated_book).await
    }
    async fn delete_book(&self, id: String) -> DomainResult<()> {
        let uuid = Uuid::from_str(&id).map_err(|_| DomainError::ValidationError {
            errors: vec!["Invalid UUID format".to_string()],
        })?;
        let book_id = BookId::from_uuid(uuid);
        self.repository.delete(&book_id).await
    }
}
