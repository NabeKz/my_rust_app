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
