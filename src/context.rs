use std::sync::Arc;
use sqlx::SqlitePool;

use crate::features::book::infra::{on_memory, sqlite};
use crate::features::book::usecase::{BookUsecase, BookUsecaseImpl};

pub struct Context {
    pub book_usecase: Arc<dyn BookUsecase>,
}

impl Context {
    pub fn init() -> Self {
        let repository = Arc::new(on_memory::BookRepositoryOnMemory::default());
        let book_usecase = Arc::new(BookUsecaseImpl::new(repository.clone()));

        Self { book_usecase }
    }

    pub fn init_with_db(pool: SqlitePool) -> Self {
        let repository = Arc::new(sqlite::SqliteBookRepository::new(pool));
        let book_usecase = Arc::new(BookUsecaseImpl::new(repository.clone()));

        Self { book_usecase }
    }
}
