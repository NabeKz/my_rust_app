pub fn li(content: String) -> String {
    format!("<li>{}</li>", content)
}

enum Method {
    GET,
    POST,
}

pub fn form(method: Method, action: String) {}
