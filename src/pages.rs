pub mod index;

use maud::{html, Markup, DOCTYPE};

fn body(content: Markup) -> Markup {
    html! {
        body {
            main {
              (content)
            }
            script src="https://cdn.jsdelivr.net/npm/@unocss/runtime" {}
            script src="/reload/reload.js" {}
        }
    }
}

fn head(title: &str, desc: &str, url: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (title) };
            meta name="description" content=(desc);
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta property="og:title" content=(title);
            meta property="og:type" content="website";
            meta property="og:url" content=(url);
            meta property="og:image" content="";
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="manifest" href="site.webmanifest";
            meta name="theme-color" content="#fafafa";
            // styles
            link rel="stylesheet" href="https://rsms.me/inter/inter.css";
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@unocss/reset/tailwind.min.css";
        }
    }
}

pub(crate) fn page(host: &str, title: &str, desc: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head(title, desc, host));
            (body(content));
        }
    }
}
