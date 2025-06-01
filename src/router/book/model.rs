use std::sync::Mutex;

#[derive(Clone)]
pub struct Book {
    pub name: String,
}

impl Book {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    pub fn from_values(name: String) -> Self {
        Self { name }
    }
}

pub trait BookRepository: 'static {
    fn list(&self) -> Vec<Book>;
    fn save(&self, book: Book) -> ();
}
pub struct BookRepositoryOnMemory {
    items: Mutex<Vec<Book>>,
}

impl BookRepositoryOnMemory {
    pub fn new() -> Self {
        let items = vec![Book::new("hoge"), Book::new("fuga")];
        Self {
            items: Mutex::new(items),
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
}
