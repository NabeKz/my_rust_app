use std::sync::Arc;

#[derive(Clone)]
pub struct Book {
    name: String,
}

impl Book {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

pub trait BookRepository {
    fn list(&self) -> Vec<Book>;
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
}

#[derive(Clone)]
pub struct BookListController {
    repository: Arc<dyn BookRepository>,
}

impl BookListController {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
    pub fn query(&self) -> Vec<Book> {
        self.repository.list()
    }
}

pub fn index(controller: &BookListController) -> String {
    let items = controller
        .query()
        .iter()
        .map(|it| format!("<li>{}</li>", it.name))
        .collect::<String>();

    format!("<ul>{}</ul>", items)
}
