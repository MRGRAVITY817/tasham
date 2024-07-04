use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

async fn hello_world() -> Markup {
    let terms = vec![
        "You agree to the terms and conditions",
        "You agree to the privacy policy",
        "You agree to the cookie policy",
    ];

    html! {
    (DOCTYPE)
    html {
        head {
            title { "Sign Up" }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/milligram/1.4.1/milligram.min.css";
            script src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" defer = "true" {}
        }
        body {
            div class="container" {
                h1 { "Sign Up" }
                form action="/submit-sign-up" method="post" {
                    fieldset {
                        label for="username" { "Username" }
                        input type="text" name="username" id="username" required;

                        label for="email" { "Email" }
                        input type="email" name="email" id="email" required;

                        label for="password" { "Password" }
                        input type="password" name="password" id="password" required;

                        label for="confirm_password" { "Confirm Password" }
                        input type="password" name="confirm_password" id="confirm_password" required;

                        button type="submit" { "Sign Up" }
                    }
                }
                div x-data="{ open: false }" {
                    button "@click"="open = !open" { "Toggle Terms and Conditions" }
                    div x-show="open" {
                        h2 { "Terms and Conditions" }
                        ul {
                            @for term in terms {
                                li { (term) }
                            }
                        }
                    }
                }
            }
        }
    } }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4400").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
