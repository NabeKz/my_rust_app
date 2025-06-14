use actix_web::{
    App, HttpServer,
    dev::Service,
    http::{Method, header::ContentType},
    middleware,
    web::Data,
};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use my_rust_app::{context::Context, router};

fn method_override(method: &Method, query: String) -> Method {
    let vec: Vec<&str> = query.split("_method=").collect();
    match (method, vec.as_slice()) {
        (&Method::POST, ["", "Delete"]) => Method::DELETE,
        (&Method::POST, ["", "Put"]) => Method::PUT,
        _ => method.clone(),
    }
}

async fn setup_database() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:./database.sqlite3";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url = "localhost";
    let port = 5000;
    println!("running on http://{}:{}", url, port);

    // Setup database
    let pool = setup_database().await.expect("Failed to setup database");
    let app_state = Context::init_with_db(pool);
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
            .configure(router::api_route)
    })
    .bind((url, port))?
    .run()
    .await
}
