use Result::Ok;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

use super::model::BookRepository;

pub struct BookDeleteController {
    repository: Arc<dyn BookRepository>,
}
impl BookDeleteController {
    pub fn new(repository: Arc<dyn BookRepository>) -> Self {
        Self { repository }
    }
}

pub fn delete(controller: &BookDeleteController, id: String) -> Result<(), Vec<String>> {
    // TODO: handle error
    let uuid = Uuid::from_str(&id).unwrap();
    let _ = controller.repository.delete(uuid);
    Ok(())
}
