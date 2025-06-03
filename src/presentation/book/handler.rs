use std::sync::Arc;

use crate::domain::book::{Book, BookRepository};

pub async fn get_book() -> Book {
    todo!()
}

pub async fn get_books<T>(repository: Arc<T>) -> Vec<Book>
where
    T: BookRepository,
{
    repository.list()
}
