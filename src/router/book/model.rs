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
    fn save(&self, item: Book) -> () {
        // self.items.push(item);
        ()
    }
}
