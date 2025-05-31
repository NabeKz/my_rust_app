use std::sync::Arc;

use actix_web::{
    App, HttpResponse, HttpServer,
    http::header::ContentType,
    middleware,
    web::{self, Data},
};

use my_rust_app::web_router::{
    book::{
        self, create::BookCreateController, list::BookListController, model::BookRepositoryOnMemory,
    },
    home,
};

const STYLE: &str = r"
<style>
    ul,li, form {margin:0;}
    label { display: grid; width: fit-content; }
    .flex { display: flex; }
    .grid { display: grid; }
</style>
";

struct Context {
    book: book::list::BookListController,
    book_create: book::create::BookCreateController,
}
trait Html {
    fn html(self) -> HttpResponse;
}

impl Html for String {
    fn html(self) -> HttpResponse {
        HttpResponse::Ok().body(STYLE.to_string() + &self)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "localhost";
    let port = 5000;
    println!("running on http://{}:{}", url, port);

    HttpServer::new(|| {
        let book_repository = Arc::new(BookRepositoryOnMemory::new());
        App::new()
            .app_data(Data::new(Context {
                book: BookListController::new(book_repository.clone()),
                book_create: BookCreateController::new(book_repository.clone()),
            }))
            .wrap(middleware::DefaultHeaders::new().add(ContentType::html()))
            .route(
                "/",
                web::get().to(async || -> HttpResponse { home::index().html() }),
            )
            .route(
                "/books",
                web::get().to(async |data: Data<Context>| book::list::index(&data.book).html()),
            )
            .route(
                "/books/create",
                web::get().to(async |_data: Data<Context>| book::create::index().html()),
            )
    })
    .bind((url, port))?
    .run()
    .await
}
