use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Path},
};

use std::str::FromStr;
use uuid::Uuid;

use crate::handler::Context;

pub async fn command(data: Data<Context>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let uuid = Uuid::from_str(&id).unwrap();
    let _ = data.book_create.delete(uuid);

    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}
