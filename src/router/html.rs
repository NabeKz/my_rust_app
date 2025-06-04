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
pub fn delete_form<S: Into<String>>(action: S, content: String) -> String {
    form(Method::DELETE, &action.into(), content.into())
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
        .map(|header| format!("<th>{}</td>", header))
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
