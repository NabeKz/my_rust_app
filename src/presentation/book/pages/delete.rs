use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Path},
};

use crate::{context::Context, features::book::usecase};

fn success() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}

pub async fn command(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let _ = usecase::delete_book(data.book.clone(), id).await;

    success()
}
