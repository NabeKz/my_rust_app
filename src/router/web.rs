pub use super::books;

pub fn home() -> String {
    r"
    <div>
        <ul>
            <li>
                <a href=books >books</a>
            </li>
            <li>
                <a href=books/create >books create</a>
            </li>
        </ul>
    </div>
    "
    .to_string()
}
