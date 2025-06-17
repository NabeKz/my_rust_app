use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct BookName(String);

#[derive(Debug, Clone, PartialEq)]
pub struct BookId(Uuid);

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid book name: {reason}")]
    InvalidBookName { reason: String },
    
    #[error("Book not found with ID: {id}")]
    BookNotFound { id: String },
    
    #[error("Validation failed: {errors:?}")]
    ValidationError { errors: Vec<String> },
    
    #[error("Data conversion failed: {message}")]
    DataConversionError { message: String },
    
    #[error("Repository operation failed")]
    RepositoryError(#[source] anyhow::Error),
}

pub type DomainResult<T> = Result<T, DomainError>;

impl BookName {
    pub fn new<S: Into<String>>(name: S) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::InvalidBookName {
                reason: "Name cannot be empty".to_string(),
            });
        }
        if name.len() > 255 {
            return Err(DomainError::InvalidBookName {
                reason: "Name cannot exceed 255 characters".to_string(),
            });
        }
        Ok(BookName(name.trim().to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl BookId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        BookId(uuid)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for BookId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Book {
    id: BookId,
    name: BookName,
    created_at: NaiveDateTime,
}

impl Book {
    pub fn new(name: BookName) -> Self {
        Self {
            id: BookId::new(),
            name,
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn from_parts(id: BookId, name: BookName, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            name,
            created_at,
        }
    }

    pub fn id(&self) -> &BookId {
        &self.id
    }

    pub fn name(&self) -> &BookName {
        &self.name
    }
    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn update_name(&self, new_name: BookName) -> Self {
        Self {
            name: new_name,
            ..self.clone()
        }
    }
}

#[async_trait]
pub trait BookRepository: Sync + Send + 'static {
    async fn find(&self, id: &BookId) -> DomainResult<Book>;
    async fn list(&self) -> Vec<Book>;
    async fn save(&self, book: Book) -> DomainResult<()>;
    async fn update(&self, book: Book) -> DomainResult<()>;
    async fn delete(&self, id: &BookId) -> DomainResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_book_name_valid_creation(name in "[a-zA-Z0-9 ]{1,255}") {
            let result = BookName::new(&name);
            prop_assert!(result.is_ok());
            if let Ok(book_name) = result {
                prop_assert_eq!(book_name.value(), name.trim());
            }
        }

        #[test]
        fn prop_book_name_empty_fails(whitespace in "[ \t\n\r]*") {
            let result = BookName::new(&whitespace);
            prop_assert!(result.is_err());
        }

        #[test]
        fn prop_book_name_too_long_fails(name in "[a-zA-Z0-9]{256,1000}") {
            let result = BookName::new(&name);
            prop_assert!(result.is_err());
        }

        #[test]
        fn prop_book_update_preserves_id_and_created_at(
            original_name in "[a-zA-Z0-9 ]{1,255}",
            new_name in "[a-zA-Z0-9 ]{1,255}"
        ) {
            let original_book_name = BookName::new(&original_name).unwrap();
            let new_book_name = BookName::new(&new_name).unwrap();
            
            let book = Book::new(original_book_name);
            let original_id = book.id().value();
            let original_created_at = *book.created_at();
            
            let updated_book = book.update_name(new_book_name.clone());
            
            prop_assert_eq!(updated_book.id().value(), original_id);
            prop_assert_eq!(updated_book.created_at(), &original_created_at);
            prop_assert_eq!(updated_book.name().value(), new_book_name.value());
        }
    }
}
