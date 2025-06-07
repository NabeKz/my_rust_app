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
    fn save(&self, book: Book);
    fn find(&self, id: Uuid) -> Result<Book, String>;
    fn update(&self, book: Book) -> Result<(), Vec<String>>;
    fn delete(&self, id: Uuid);
}
