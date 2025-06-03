use std::sync::Arc;

use actix_web::web;

use crate::router::book::{
    create::BookCreateController, delete::BookDeleteController, model::BookRepositoryOnMemory,
    update::BookGetController,
};

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
    pub book: Arc<on_memory::BookRepositoryOnMemory>,
    pub book_create: Arc<BookCreateController>,
    pub book_update: Arc<BookGetController>,
    pub book_delete: Arc<BookDeleteController>,
}

impl Context {
    pub fn init() -> Self {
        let repository = Arc::new(BookRepositoryOnMemory::new());
        Self {
            book: Arc::new(on_memory::BookRepositoryOnMemory::new()),
            book_create: Arc::new(BookCreateController::new(repository.clone())),
            book_update: Arc::new(BookGetController::new(repository.clone())),
            book_delete: Arc::new(BookDeleteController::new(repository.clone())),
        }
    }
}
