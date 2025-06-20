pub mod html {
    use actix_web::{HttpResponse, http::header};

    const STYLE: &str = r"
    <style>
        ul,li, form { margin:0; }
        label { display: grid; width: fit-content; }
        label + div:has(button) { padding-top: 8px; }
        .flex { display: flex; }
        .grid { display: grid; }
    </style>
    ";

    pub trait HtmlResponse {
        fn html(self) -> HttpResponse;
        fn flush(self, name: &str) -> HttpResponse;
    }

    impl HtmlResponse for String {
        fn html(self) -> HttpResponse {
            HttpResponse::Ok().body(STYLE.to_string() + &self)
        }
        fn flush(self, name: &str) -> HttpResponse {
            HttpResponse::Ok()
                .append_header((header::SET_COOKIE, format!("{}=''; Max-Age=0", name)))
                .body(STYLE.to_string() + &self)
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
        Post,
        Put,
        Delete,
    }

    fn form(method: Method, action: &str, content: String) -> String {
        let action = match method {
            Method::Put => action.to_owned() + "?_method=Put",
            Method::Delete => action.to_owned() + "?_method=Delete",
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
        form(Method::Post, action, content)
    }
    pub fn put_form<S: Into<String>>(action: S, content: String) -> String {
        form(Method::Put, &action.into(), content)
    }
    pub fn delete_form(action: String, content: &str) -> String {
        form(Method::Delete, &action, content.to_string())
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

    pub fn t_head(headers: Vec<&str>) -> String {
        headers
            .iter()
            .map(|header| format!("<th>{}</th>", header))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn t_data<T>(bodies: Vec<T>, f: fn(&T) -> String) -> String {
        bodies
            .iter()
            .map(|body| format!("<tr>{}</tr>", f(body)))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn table<T>(headers: Vec<&str>, bodies: Vec<T>, f: fn(&T) -> String) -> String {
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

    pub fn redirect(to: &str) -> HttpResponse {
        HttpResponse::SeeOther()
            .append_header((header::LOCATION, to))
            .finish()
    }
    pub fn redirect_with_error<S: Into<String>>(to: &str, error: S) -> HttpResponse {
        HttpResponse::SeeOther()
            .append_header((header::LOCATION, to))
            .append_header((header::SET_COOKIE, String::from("error=") + &error.into()))
            .finish()
    }
}
