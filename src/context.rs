use std::sync::Arc;

use crate::features::book::model::BookRepository;

use crate::features::book::infra::on_memory;

pub struct Context {
    pub book: Arc<dyn BookRepository>,
}

impl Context {
    pub fn init() -> Self {
        let repository = Arc::new(on_memory::BookRepositoryOnMemory::default());
        Self {
            book: repository.clone(),
        }
    }
}
