enum Method {
    POST,
    DELETE,
}

fn form(method: Method, action: &str, content: String) -> String {
    let req_method = match method {
        Method::POST => "POST",
        Method::DELETE => "DELETE",
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
pub fn delete_form(action: &str, content: String) -> String {
    form(Method::DELETE, action, content)
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
