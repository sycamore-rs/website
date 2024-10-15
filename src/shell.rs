use sycamore::prelude::*;
use sycamore_router::Route;

#[derive(Debug, Clone, Route)]
pub enum Routes {
    #[to("/")]
    Index,
    #[to("/index.html")]
    IndexHtml,
    #[to("/404.html")]
    #[not_found]
    NotFound,
}

#[component(inline_props)]
pub fn Shell(children: Children) -> View {
    let children = children.call();
    view! {
        html {
            sycamore::web::NoHydrate {
                head {
                    link(rel="preload", href="/sycamore-website.js", r#as="script", crossorigin="")
                    link(rel="preload", href="/sycamore-website_bg.wasm", r#as="fetch", crossorigin="")
                    script(r#type="module") {
                        r#"import init from "/sycamore-website.js"; init();"#
                    }
                    link(rel="stylesheet", href="/index.css")
                    link(rel="icon", href="/favicon.ico")
                }
            }
            body {
                (children)
            }
        }
    }
}

#[component]
pub fn App(route: ReadSignal<Routes>) -> View {
    view! {
        (match route.get_clone() {
            Routes::Index | Routes::IndexHtml => view! {
                h1 { "Index" }
            },
            Routes::NotFound => view! {
                h1 { "404 Not Found" }
                p {
                    a(href="/") { "Return to the home page" }
                }
            },
        })
    }
}
