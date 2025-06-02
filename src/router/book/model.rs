use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Clone)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
}

impl Book {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
        }
    }

    pub fn from_values(name: String) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
        }
    }
}

pub trait BookRepository: Sync + Send + 'static {
    fn list(&self) -> Vec<Book>;
    fn save(&self, book: Book) -> ();
    fn find(&self, id: Uuid) -> Result<Book, String>;
    fn update(&self, book: Book) -> Result<(), String>;
    fn delete(&self, id: Uuid) -> ();
}
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

    fn update(&self, book: Book) -> Result<(), String> {
        let mut items = self.items.lock().unwrap();
        if let Some(pos) = items.iter().position(|it| it.id == book.id) {
            items[pos] = book;
            Ok(())
        } else {
            Err("not found".to_string())
        }
    }
}
