use std::sync::Arc;

use actix_web::{
    App, HttpResponse, HttpServer,
    http::header::ContentType,
    middleware,
    web::{self, Data},
};

use my_rust_app::web_router::{
    book::{self, list::BookRepositoryOnMemory},
    home,
};

const STYLE: &str = r"
<style>
    ul,li, form {margin:0;}
    .flex { display: flex; }
    .grid { display: grid; }
</style>
";

struct Context {
    book: book::list::BookListController,
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
        App::new()
            .app_data(Data::new(Context {
                book: book::list::BookListController::new(Arc::new(BookRepositoryOnMemory::new())),
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
    })
    .bind((url, port))?
    .run()
    .await
}
