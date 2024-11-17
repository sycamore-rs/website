use sycamore::prelude::*;
use sycamore_router::Route;

use crate::{layout, pages};

#[derive(Debug, Clone, PartialEq, Route)]
pub enum Routes {
    #[to("/")]
    Index,
    #[to("/post/<id>")]
    Post(String),
    #[to("/book/<section>")]
    BookSection(String),
    #[to("/book/<section>/<doc>")]
    BookSubsection(String, String),
    #[not_found]
    NotFound,
}

/// Context value for setting the document title.
/// Also renders a `<title>` tag in SSR.
#[derive(Debug, Clone, Copy)]
struct Title(Signal<String>);

/// Set the document title.
pub fn set_title(title: impl Into<String>) {
    use_context::<Title>().0.set(title.into());
}

#[component(inline_props)]
pub fn Shell(children: Children) -> View {
    let title = Title(create_signal(String::new()));
    provide_context(title);

    if is_not_ssr!() {
        create_effect(move || {
            let title = title.0.get_clone();
            if !title.is_empty() {
                document().set_title(&title);
            }
        });
    }

    let title_static = title.0.get_clone();

    view! {
        html(lang="en") {
            sycamore::web::NoHydrate {
                head {
                    meta(charset="utf-8")
                    meta(name="viewport", content="width=device-width, initial-scale=1")

                    meta(name="description", content="Sycamore is a next generation Rust UI library powered by fine-grained reactivity. Create reactive apps with effortless performance")

                    title { (title_static) }

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

                    // PrismJS
                    script(src="/prism/prism.js")
                    link(rel="stylesheet", href="/prism/prism-gruvbox-dark.css")

                    // Analytics
                    script(defer=true, src="https://sycamore-analytics.netlify.app/script.js", data-website-id="e539dd59-791b-44a2-9df4-18db88d9cb80")

                    sycamore::web::HydrationScript {}
                }
            }
            body {
                (children)
            }
        }
    }
}

/// A context value for storing the current route.
#[derive(Debug, Clone, Copy)]
pub struct CurrentRoute(pub ReadSignal<Routes>);

#[component]
pub fn App(route: ReadSignal<Routes>) -> View {
    provide_context(CurrentRoute(route));
    view! {
        layout::Layout {
            (match route.get_clone() {
                Routes::Index => pages::index::Index(),
                Routes::Post(id) => view! { pages::post::Post(id=id) },
                Routes::BookSection(section) => view! { pages::book::Book(section=section) },
                Routes::BookSubsection(section, doc) => view! { pages::book::Book(section=section, doc=doc) },
                Routes::NotFound =>  { pages::not_found::NotFound() },
            })
        }
    }
}
