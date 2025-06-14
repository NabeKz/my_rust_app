use async_trait::async_trait;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::features::book::model::{Book, BookId, BookName, BookRepository, DomainResult, DomainError};

#[derive(FromRow)]
struct BookRow {
    id: String,
    name: String,
}

impl TryFrom<BookRow> for Book {
    type Error = DomainError;

    fn try_from(row: BookRow) -> Result<Self, Self::Error> {
        let uuid = Uuid::parse_str(&row.id)
            .map_err(|_| DomainError::DatabaseError("Invalid UUID format".into()))?;
        let book_id = BookId::from_uuid(uuid);
        let book_name = BookName::new(row.name)?;
        Ok(Book::from_parts(book_id, book_name))
    }
}

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
        let rows = match sqlx::query_as!(
            BookRow,
            "SELECT id, name FROM books ORDER BY created_at DESC"
        )
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
            .filter_map(|row| Book::try_from(row).ok())
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
