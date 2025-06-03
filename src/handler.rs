use std::sync::Arc;

use actix_web::web;

use crate::features::book::model::BookRepository;

use crate::features::book::infra::on_memory;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/");
    // .service(health_checker_handler)
    // .service(todos_list_handler)
    // .service(create_todo_handler)
    // .service(get_todo_handler)
    // .service(edit_todo_handler)
    // .service(delete_todo_handler);

    conf.service(scope);
}

pub struct Context {
    pub book: Arc<dyn BookRepository>,
    pub book_create: Arc<dyn BookRepository>,
    pub book_update: Arc<dyn BookRepository>,
    pub book_delete: Arc<dyn BookRepository>,
}

impl Context {
    pub fn init() -> Self {
        let repository = Arc::new(on_memory::BookRepositoryOnMemory::new());
        Self {
            book: repository.clone(),
            book_create: repository.clone(),
            book_update: repository.clone(),
            book_delete: repository.clone(),
        }
    }
}
