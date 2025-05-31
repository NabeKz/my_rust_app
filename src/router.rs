pub mod book;
pub mod home;
pub mod html;

pub mod web_router {
    pub use crate::router::book;
    pub use crate::router::home;
}
