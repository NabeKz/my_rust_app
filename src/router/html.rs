pub fn li(content: String) -> String {
    format!("<li>{}</li>", content)
}

enum Method {
    POST,
}

fn form(method: Method, action: &str, content: String) -> String {
    let req_method = match method {
        Method::POST => "POST",
    };

    format!(
        "
        <form action={} method={}>
            {}
            <div style=padding-top:8px;>
                <button type=submit> submit </button>
            </div>
        </form>
    ",
        action, req_method, content
    )
}
pub fn post_form(action: &str, content: String) -> String {
    form(Method::POST, action, content)
}

pub fn input(id: &str) -> String {
    format!(
        "<label for={}>
            {}
            <input id={} name={} />
        </label>
    ",
        id, id, id, id
    )
}
