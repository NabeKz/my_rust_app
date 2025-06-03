use actix_web::{
    App, HttpResponse, HttpServer,
    dev::Service,
    http::{
        Method,
        header::{self, ContentType},
    },
    middleware,
    web::{self, Data, Form, Path},
};

use my_rust_app::{
    handler::Context,
    presentation,
    presentation::shared::Html,
    web_router::book::{self},
};

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

fn method_override(method: &Method, query: String) -> Method {
    let vec: Vec<&str> = query.split("_method=").collect();
    match (method, vec.as_slice()) {
        (&Method::POST, ["", "DELETE"]) => Method::DELETE,
        (&Method::POST, ["", "PUT"]) => Method::PUT,
        _ => method.clone(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "localhost";
    let port = 5000;
    println!("running on http://{}:{}", url, port);

    let app_state = Context::init();
    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::DefaultHeaders::new().add(ContentType::html()))
            .wrap_fn(|mut req, srv| {
                let method = req.method().clone();
                let query = req.query_string().to_string();
                req.head_mut().method = method_override(&method, query);
                srv.call(req)
            })
            .route("/", web::get().to(home))
            .route("/books", web::get().to(presentation::book::get_books))
            .route(
                "/books/create",
                web::get().to(presentation::book::pages::create::index),
            )
            .route(
                "/books/update/{id}",
                web::get().to(async |data: Data<Context>, path: Path<String>| {
                    let id = path.into_inner();
                    book::update::find(&data.book_update, id).html()
                }),
            )
            .route(
                "/books/update/{id}",
                web::put().to(
                    async |data: Data<Context>,
                           path: Path<String>,
                           form: Form<book::update::FormData>| {
                        let id = path.into_inner();
                        let result = book::update::update(&data.book_update, id.clone(), &form);
                        match result {
                            Ok(()) => HttpResponse::SeeOther()
                                .append_header((header::LOCATION, "/books"))
                                .finish(),
                            Err(_) => HttpResponse::SeeOther()
                                .append_header((
                                    header::LOCATION,
                                    format!("/books/update/{}", id.clone()),
                                ))
                                .finish(),
                        }
                    },
                ),
            )
            .route(
                "/books/delete/{id}",
                web::delete().to(presentation::book::pages::delete::command),
            )
            .route(
                "/books/create",
                web::post().to(presentation::book::pages::create::command),
            )
    })
    .bind((url, port))?
    .run()
    .await
}
