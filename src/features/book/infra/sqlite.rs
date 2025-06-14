use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::features::book::model::{
    Book, BookId, BookName, BookRepository, DomainError, DomainResult,
};

#[derive(FromRow, Debug)]
struct BookRow {
    id: Option<String>,
    name: Option<String>,
    created_at: Option<NaiveDateTime>,
}

impl BookRow {
    fn get_id(&self) -> DomainResult<BookId> {
        let id_str = self
            .id
            .as_ref()
            .ok_or_else(|| DomainError::DatabaseError("Missing id".into()))?;
        let uuid = Uuid::parse_str(id_str)
            .map_err(|_| DomainError::DatabaseError("Invalid UUID format".into()))?;
        Ok(BookId::from_uuid(uuid))
    }

    fn get_name(&self) -> DomainResult<BookName> {
        let name_str = self
            .name
            .as_ref()
            .ok_or_else(|| DomainError::DatabaseError("Missing name".into()))?;
        BookName::new(name_str.clone())
    }
    fn get_created_at(&self) -> DomainResult<NaiveDateTime> {
        let created_at = self
            .created_at
            .ok_or_else(|| DomainError::DatabaseError("Missing name".into()))?;
        Ok(created_at)
    }
}

impl TryFrom<BookRow> for Book {
    type Error = DomainError;

    fn try_from(row: BookRow) -> Result<Self, Self::Error> {
        Ok(Book::from_parts(
            row.get_id()?,
            row.get_name()?,
            row.get_created_at()?,
        ))
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
            "SELECT id, name, created_at FROM books ORDER BY created_at DESC"
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

    async fn save(&self, book: Book) -> DomainResult<()> {
        let id = book.id().value().to_string();
        let name = book.name().value();
        let created_at = book.created_at();
        let updated_at = book.created_at();

        sqlx::query!(
            "INSERT INTO books (id, name, created_at, updated_at) VALUES ($1, $2, $3, $4)",
            id,
            name,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|err| DomainError::DatabaseError(format!("Failed to save book: {}", err)))?;

        Ok(())
    }

    async fn update(&self, book: Book) -> DomainResult<()> {
        let id = book.id().value().to_string();
        let name = book.name().value().to_string();

        sqlx::query!("UPDATE books SET name = $1 WHERE id = $2", name, id)
            .execute(&self.pool)
            .await
            .map_err(|err| DomainError::DatabaseError(format!("Failed to update book: {}", err)))?;

        Ok(())
    }

    async fn delete(&self, id: &BookId) -> DomainResult<()> {
        let id = id.value().to_string();
        sqlx::query!("DELETE FROM books WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|err| DomainError::DatabaseError(format!("Failed to delete book: {}", err)))?;

        Ok(())
    }
}
