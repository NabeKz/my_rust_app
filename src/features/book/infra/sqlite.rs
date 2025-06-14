use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::features::book::model::{Book, BookId, BookName, BookRepository, DomainResult, DomainError};

pub struct SqliteBookRepository {
    pool: SqlitePool,
}

impl SqliteBookRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BookRepository for SqliteBookRepository {
    async fn find(&self, _id: &BookId) -> DomainResult<Book> {
        todo!()
    }

    async fn list(&self) -> Vec<Book> {
        let rows = match sqlx::query!("SELECT id, name FROM books ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
        {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("Database error in list: {}", err);
                return Vec::new();
            }
        };

        rows.into_iter()
            .filter_map(|row| {
                let uuid = Uuid::parse_str(&row.id).ok()?;
                let book_id = BookId::from_uuid(uuid);
                let book_name = BookName::new(row.name).ok()?;

                Some(Book::from_parts(book_id, book_name))
            })
            .collect()
    }

    async fn save(&self, _book: Book) -> DomainResult<()> {
        todo!()
    }

    async fn update(&self, _book: Book) -> DomainResult<()> {
        todo!()
    }

    async fn delete(&self, _id: &BookId) -> DomainResult<()> {
        todo!()
    }
}
