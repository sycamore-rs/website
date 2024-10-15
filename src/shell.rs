use sycamore::prelude::*;
use sycamore_router::Route;

use crate::layout;
use crate::pages;

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
                    meta(charset="utf-8")
                    meta(name="viewport", content="width=device-width, initial-scale=1")

                    link(rel="preload", href="/sycamore-website.js", r#as="script", crossorigin="")
                    link(rel="preload", href="/sycamore-website_bg.wasm", r#as="fetch", crossorigin="")
                    script(r#type="module") {
                        r#"import init from "/sycamore-website.js"; init();"#
                    }

                    link(rel="stylesheet", href="/index.css")
                    link(rel="icon", href="/favicon.ico")

                    // Bootstrap Icons
                    link(rel="stylesheet", href="/icons/bootstrap-icons.min.css")
                    link(rel="preload", href="/icons/fonts/bootstrap-icons.woff2?dd67030699838ea613ee6dbda90effa6", r#as="font", r#type="font/woff2", crossorigin="anonymous")
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
        layout::Layout {
            (match route.get_clone() {
                Routes::Index | Routes::IndexHtml => pages::index::Index(),
                Routes::NotFound => view! {
                    h1 { "404 Not Found" }
                    p {
                        a(href="/") { "Return to the home page" }
                    }
                },
            })
        }
    }
}
