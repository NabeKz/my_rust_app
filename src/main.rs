use actix_web::{
    App, HttpResponse, HttpServer, Responder, http::header::ContentType, middleware, web,
};
use my_rust_app::router::web::hello;

const STYLE: &str = r"
<style>
    ul,li, form {margin:0;}
</style>
";

async fn render() -> impl Responder {
    HttpResponse::Ok().body(hello().html())
}

trait Html {
    fn html(self) -> String;
}

impl Html for String {
    fn html(self) -> String {
        STYLE.to_string() + &self
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
            .route("/", web::get().to(render))
    })
    .bind((url, port))?
    .run()
    .await
}
