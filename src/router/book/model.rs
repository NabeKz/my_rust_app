#[derive(Clone)]
pub struct Book {
    pub name: String,
}

impl Book {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

pub trait BookRepository {
    fn list(&self) -> Vec<Book>;
    fn save(&mut self, book: Book) -> ();
}
pub struct BookRepositoryOnMemory {
    items: Vec<Book>,
}

impl BookRepositoryOnMemory {
    pub fn new() -> Self {
        let items = vec![Book::new("hoge"), Book::new("fuga")];
        Self { items }
    }
}
impl BookRepository for BookRepositoryOnMemory {
    fn list(&self) -> Vec<Book> {
        self.items.clone()
    }
    fn save(&mut self, item: Book) -> () {
        self.items.push(item);
    }
}
