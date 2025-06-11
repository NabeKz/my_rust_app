use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, Form},
};
use actix_session::Session;

use crate::{
    context::Context,
    features::book::usecase::CreateDto,
    presentation::shared::html::{HtmlResponse, input, post_form},
    flash_message::{FlashMessage, get_flash_message, set_flash_message},
};

pub async fn query(session: Session) -> HttpResponse {
    let flash_message = get_flash_message(&session);
    let flash_html = match flash_message {
        Some(msg) => format!(
            r#"<div class="{}">{}</div>"#,
            msg.message_type.css_class(),
            msg.message
        ),
        None => String::new(),
    };
    
    let content = format!("{}{}", flash_html, input("name", ""));
    post_form("/books/create", content).to_string().html()
}

fn success() -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books"))
        .finish()
}

fn failure(session: Session, err: String) -> HttpResponse {
    // FlashMessageにエラーメッセージを保存
    let flash_message = FlashMessage::error(err);
    let _ = set_flash_message(&session, flash_message);
    
    HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/books/create"))
        .finish()
}

pub async fn command(data: Data<Context>, form: Form<CreateDto>, session: Session) -> HttpResponse {
    let result = data.book_usecase.create_book(form.into_inner()).await;

    match result {
        Result::Ok(_) => {
            // 成功メッセージも追加
            let success_message = FlashMessage::success("Book created successfully!".to_string()); 
            let _ = set_flash_message(&session, success_message);
            success()
        },
        Result::Err(err) => failure(session, err),
    }
}
