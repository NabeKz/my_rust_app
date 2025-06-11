use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::features::book::model::{
    Book, BookId, BookName, BookRepository, DomainError, DomainResult,
};

pub struct BookRepositoryOnMemory {
    items: Arc<Mutex<Vec<Book>>>,
}

impl Default for BookRepositoryOnMemory {
    fn default() -> Self {
        let items = vec![
            Book::new(BookName::new("Sample Book 1").unwrap()),
            Book::new(BookName::new("Sample Book 2").unwrap()),
        ];
        Self {
            items: Arc::new(Mutex::new(items)),
        }
    }
}

#[async_trait]
impl BookRepository for BookRepositoryOnMemory {
    async fn list(&self) -> Vec<Book> {
        self.items.lock().unwrap().clone()
    }
    fn save(&self, item: Book) -> DomainResult<()> {
        self.items.lock().unwrap().push(item);
        Ok(())
    }

    fn find(&self, id: &BookId) -> DomainResult<Book> {
        let items = self.items.lock().unwrap();
        let item = items.iter().find(|it| it.id() == id);
        match item {
            Some(book) => Ok(book.clone()),
            None => Err(DomainError::BookNotFound(id.clone())),
        }
    }

    fn update(&self, book: Book) -> DomainResult<()> {
        let mut items = self.items.lock().unwrap();
        if let Some(pos) = items.iter().position(|it| it.id() == book.id()) {
            items[pos] = book;
            Ok(())
        } else {
            Err(DomainError::BookNotFound(book.id().clone()))
        }
    }

    fn delete(&self, id: &BookId) -> DomainResult<()> {
        let mut items = self.items.lock().unwrap();
        let initial_len = items.len();
        items.retain(|it| it.id() != id);
        if items.len() < initial_len {
            Ok(())
        } else {
            Err(DomainError::BookNotFound(id.clone()))
        }
    }
}
