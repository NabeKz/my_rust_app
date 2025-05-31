pub mod book;
pub mod home;

pub mod web_router {
    pub use crate::router::book;
    pub use crate::router::home;
}
