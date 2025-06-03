pub mod book;
pub mod shared {
    const STYLE: &str = r"
    <style>
        ul,li, form { margin:0; }
        label { display: grid; width: fit-content; }
        label + div:has(button) { padding-top: 8px; }
        .flex { display: flex; }
        .grid { display: grid; }
    </style>
    ";

    use actix_web::HttpResponse;

    pub trait Html {
        fn html(self) -> HttpResponse;
    }

    impl Html for String {
        fn html(self) -> HttpResponse {
            HttpResponse::Ok().body(STYLE.to_string() + &self)
        }
    }
}
