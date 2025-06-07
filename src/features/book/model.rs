use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct BookName(String);

#[derive(Debug, Clone, PartialEq)]
pub struct BookId(Uuid);

#[derive(Debug)]
pub enum DomainError {
    InvalidBookName(String),
    BookNotFound(BookId),
    ValidationError(Vec<String>),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidBookName(msg) => write!(f, "Invalid book name: {}", msg),
            DomainError::BookNotFound(id) => write!(f, "Book not found: {:?}", id),
            DomainError::ValidationError(errors) => write!(f, "Validation errors: {:?}", errors),
        }
    }
}

pub type DomainResult<T> = Result<T, DomainError>;

impl BookName {
    pub fn new<S: Into<String>>(name: S) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::InvalidBookName(
                "Name cannot be empty".to_string(),
            ));
        }
        if name.len() > 255 {
            return Err(DomainError::InvalidBookName(
                "Name cannot exceed 255 characters".to_string(),
            ));
        }
        Ok(BookName(name.trim().to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl BookId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        BookId(uuid)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl Default for BookId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Book {
    id: BookId,
    name: BookName,
}

impl Book {
    pub fn new(name: BookName) -> Self {
        Self {
            id: BookId::new(),
            name,
        }
    }

    pub fn from_parts(id: BookId, name: BookName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &BookId {
        &self.id
    }

    pub fn name(&self) -> &BookName {
        &self.name
    }

    pub fn update_name(&self, new_name: BookName) -> Self {
        Self {
            id: self.id.clone(),
            name: new_name,
        }
    }
}

pub trait BookRepository: Sync + Send + 'static {
    fn list(&self) -> Vec<Book>;
    fn save(&self, book: Book) -> DomainResult<()>;
    fn find(&self, id: &BookId) -> DomainResult<Book>;
    fn update(&self, book: Book) -> DomainResult<()>;
    fn delete(&self, id: &BookId) -> DomainResult<()>;
}
