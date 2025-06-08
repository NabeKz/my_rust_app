use actix_web::{
    App, HttpServer,
    dev::Service,
    http::{Method, header::ContentType},
    middleware,
    web::Data,
};

use my_rust_app::{context::Context, router};

fn method_override(method: &Method, query: String) -> Method {
    let vec: Vec<&str> = query.split("_method=").collect();
    match (method, vec.as_slice()) {
        (&Method::POST, ["", "Delete"]) => Method::DELETE,
        (&Method::POST, ["", "Put"]) => Method::PUT,
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
                let method = &req.method();
                let query = req.query_string().to_string();
                req.head_mut().method = method_override(method, query);
                srv.call(req)
            })
            .configure(router::configure)
    })
    .bind((url, port))?
    .run()
    .await
}
