use actix_web::HttpRequest;

struct Book {
    name: String,
}

impl Book {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

trait BookRepository {
    fn list(self) -> Vec<Book>;
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
    fn list(self) -> Vec<Book> {
        self.items
    }
}

#[derive(Clone)]
pub struct BookListController {}

impl BookListController {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn index(controller: &BookListController) -> String {
    format!(
        "
        <ul>
            <li>book</li>
        </ul>
        "
    )
}
