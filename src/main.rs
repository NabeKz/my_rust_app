use actix_web::{App, HttpResponse, HttpServer, http::header::ContentType, middleware, web};
use my_rust_app::router::web::home;

const STYLE: &str = r"
<style>
    ul,li, form {margin:0;}
    .flex { display: flex; }
    .grid { display: grid; }
</style>
";

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
            .wrap(middleware::DefaultHeaders::new().add(ContentType::html()))
            .route(
                "/",
                web::get().to(async || -> HttpResponse { String::from("hello").html() }),
            )
            .route(
                "/books",
                web::get().to(async || -> HttpResponse { home().html() }),
            )
    })
    .bind((url, port))?
    .run()
    .await
}
