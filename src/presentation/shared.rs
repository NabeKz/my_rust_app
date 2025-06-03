use actix_web::HttpResponse;

const STYLE: &str = r"
    <style>
        ul,li, form { margin:0; }
        label { display: grid; width: fit-content; }
        label + div:has(button) { padding-top: 8px; }
        .flex { display: flex; }
        .grid { display: grid; }
    </style>
    ";

pub trait Html {
    fn html(self) -> HttpResponse;
}

impl Html for String {
    fn html(self) -> HttpResponse {
        HttpResponse::Ok().body(STYLE.to_string() + &self)
    }
}

pub trait Validator<T> {
    fn required(&self) -> Result<(), T>;
}

impl Validator<String> for String {
    fn required(&self) -> Result<(), String> {
        match self.is_empty() {
            true => Err("required".to_string()),
            false => Ok(()),
        }
    }
}
