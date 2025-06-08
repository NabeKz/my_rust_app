use std::sync::Arc;

use crate::features::book::infra::on_memory;
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
}
