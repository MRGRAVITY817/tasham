use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    description: &'a str,
    name: &'a str,
}

pub async fn hello_page() -> Html<String> {
    let hello = HelloTemplate {
        title: "Hello page",
        description: "This is description",
        name: "Hoon Wee",
    };

    Html(hello.render().unwrap())
}
