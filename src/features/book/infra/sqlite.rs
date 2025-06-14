use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::features::book::model::{Book, BookId, BookName, BookRepository, DomainResult};

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
        let rows = sqlx::query("SELECT id, name FROM books ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();

        rows.into_iter()
            .filter_map(|row| {
                let id_str: String = row.get("id");
                let name_str: String = row.get("name");

                let uuid = Uuid::parse_str(&id_str).ok()?;
                let book_id = BookId::from_uuid(uuid);
                let book_name = BookName::new(name_str).ok()?;

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
