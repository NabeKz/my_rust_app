use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::features::book::model::{Book, BookRepository};

pub struct BookRepositoryOnMemory {
    items: Arc<Mutex<Vec<Book>>>,
}

impl BookRepositoryOnMemory {
    pub fn new() -> Self {
        let items = vec![Book::new("hoge"), Book::new("fuga")];
        Self {
            items: Arc::new(Mutex::new(items)),
        }
    }
}

impl BookRepository for BookRepositoryOnMemory {
    fn list(&self) -> Vec<Book> {
        self.items.lock().unwrap().clone()
    }
    fn save(&self, item: Book) -> () {
        let _ = self.items.lock().unwrap().push(item);
    }
    fn delete(&self, id: Uuid) -> () {
        let mut items = self.items.lock().unwrap();
        // TODO: handle not found
        items.retain(|it| it.id != id);
    }

    fn find(&self, id: Uuid) -> Result<Book, String> {
        let items = self.items.lock().unwrap();
        let item = items.iter().find(|it| it.id == id);
        match item {
            Some(book) => Result::Ok(book.clone()),
            None => Result::Err("not found".to_string()),
        }
    }

    fn update(&self, book: Book) -> Result<(), Vec<String>> {
        let mut items = self.items.lock().unwrap();
        if let Some(pos) = items.iter().position(|it| it.id == book.id) {
            items[pos] = book;
            Ok(())
        } else {
            Err(vec!["not found".to_string()])
        }
    }
}
