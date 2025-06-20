use crate::presentation::{self, shared::html::HtmlResponse};
use actix_web::{HttpResponse, web};

pub async fn home() -> HttpResponse {
    r"
    <div>
        <ul>
            <li>
                <a href=books >books</a>
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
            web::get().to(presentation::book::pages::query::list),
        )
        .route(
            "/books/create",
            web::get().to(presentation::book::pages::query::create),
        )
        .route(
            "/books/{id}",
            web::get().to(presentation::book::pages::query::edit),
        )
        .route(
            "/books/create",
            web::post().to(presentation::book::pages::command::create),
        )
        .route(
            "/books/{id}",
            web::put().to(presentation::book::pages::command::update),
        )
        .route(
            "/books/{id}",
            web::delete().to(presentation::book::pages::command::delete),
        );
}

pub fn api_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
            .route(
                "/books",
                web::get().to(presentation::book::http::response::get),
            )
            .route(
                "/books",
                web::post().to(presentation::book::http::response::post),
            )
            .route(
                "/books/{id}",
                web::put().to(presentation::book::http::response::put),
            )
            .route(
                "/books/{id}",
                web::delete().to(presentation::book::http::response::delete),
            ),
    );
}
