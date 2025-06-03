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

enum Method {
    POST,
    PUT,
    DELETE,
}

fn form(method: Method, action: &str, content: String) -> String {
    let action = match method {
        Method::PUT => action.to_owned() + "?_method=PUT",
        Method::DELETE => action.to_owned() + "?_method=DELETE",
        _ => action.to_owned(),
    };

    format!(
        "
        <form action={} method=POST>
            {}
            <div>
                <button type=submit> submit </button>
            </div>
        </form>
    ",
        action, content
    )
}
pub fn post_form(action: &str, content: String) -> String {
    form(Method::POST, action, content)
}
pub fn put_form(action: &str, content: String) -> String {
    form(Method::PUT, action, content)
}
pub fn delete_form(action: &str, content: String) -> String {
    form(Method::DELETE, action, content)
}

pub fn input(id: &str, value: &str) -> String {
    format!(
        "<label for={}>
            {}
            <input id={} name={} value='{}' />
        </label>
    ",
        id, id, id, id, value
    )
}

pub fn link(href: String, text: String) -> String {
    format!("<a href={} >{}</a>", href, text)
}

pub fn t_head(headers: Vec<String>) -> String {
    headers
        .iter()
        .map(|header| format!("<th>{}</td>", header))
        .collect::<Vec<String>>()
        .join("")
}

pub fn t_data(bodies: Vec<Vec<String>>, f: fn(&Vec<String>) -> String) -> String {
    bodies
        .iter()
        .map(|body| format!("<tr>{}</tr>", f(body)))
        .collect::<Vec<String>>()
        .join("")
}

pub fn table(
    headers: Vec<String>,
    bodies: Vec<Vec<String>>,
    f: fn(&Vec<String>) -> String,
) -> String {
    format!(
        "
    <table>
        <thead>
            <tr>{}</tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>
    ",
        t_head(headers),
        t_data(bodies, f)
    )
}
