use actix_web::{web, HttpResponse};
use crate::presentation::{self, shared::html::HtmlResponse};

pub async fn home() -> HttpResponse {
    r"
    <div>
        <ul>
            <li>
                <a href=books >books</a>
            </li>
            <li>
                <a href=books/create >books create</a>
            </li>
            <li>
                <a href=books/delete >books delete</a>
            </li>
        </ul>
    </div>
    "
    .to_string()
    .html()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(home))
        .route(
            "/books",
            web::get().to(presentation::book::pages::list::query),
        )
        .route(
            "/books/create",
            web::get().to(presentation::book::pages::create::query),
        )
        .route(
            "/books/{id}",
            web::get().to(presentation::book::pages::update::query),
        )
        .route(
            "/books/{id}",
            web::put().to(presentation::book::pages::update::command),
        )
        .route(
            "/books/{id}",
            web::delete().to(presentation::book::pages::delete::command),
        )
        .route(
            "/books/create",
            web::post().to(presentation::book::pages::create::command),
        );
}