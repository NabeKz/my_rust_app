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
    web_router::book::{self},
};

const STYLE: &str = r"
<style>
    ul,li, form { margin:0; }
    label { display: grid; width: fit-content; }
    label + div:has(button) { padding-top: 8px; }
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

pub fn home() -> String {
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
            .route(
                "/",
                web::get().to(async || -> HttpResponse { home().html() }),
            )
            .route(
                "/books",
                web::get().to(async |data: Data<Context>| book::list::index(&data.book).html()),
            )
            .route(
                "/books/create",
                web::get().to(async |_data: Data<Context>| book::create::index().html()),
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
                web::delete().to(async |data: Data<Context>, path: Path<String>| {
                    let id = path.into_inner();
                    let result = book::delete::delete(&data.book_delete, id);
                    match result {
                        Ok(()) => HttpResponse::SeeOther()
                            .append_header((header::LOCATION, "/books"))
                            .finish(),
                        Err(_) => HttpResponse::SeeOther()
                            .append_header((header::LOCATION, "/books"))
                            .finish(),
                    }
                }),
            )
            .route(
                "/books/create",
                web::post().to(
                    async |data: Data<Context>, form: Form<book::create::FormData>| {
                        let result = book::create::create(&data.book_create, &form);
                        match result {
                            Ok(()) => HttpResponse::SeeOther()
                                .append_header((header::LOCATION, "/books"))
                                .finish(),
                            Err(err) => err.join("<br />").html(),
                        }
                    },
                ),
            )
    })
    .bind((url, port))?
    .run()
    .await
}
