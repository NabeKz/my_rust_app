use std::sync::Arc;

use actix_web::web;

use crate::router::book::{
    create::BookCreateController, delete::BookDeleteController, list::BookListController,
    model::BookRepositoryOnMemory,
};

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
    pub book: Arc<BookListController>,
    pub book_create: Arc<BookCreateController>,
    pub book_delete: Arc<BookDeleteController>,
}

impl Context {
    pub fn init() -> Self {
        let repository = Arc::new(BookRepositoryOnMemory::new());
        Self {
            book: Arc::new(BookListController::new(repository.clone())),
            book_create: Arc::new(BookCreateController::new(repository.clone())),
            book_delete: Arc::new(BookDeleteController::new(repository.clone())),
        }
    }
}
