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

                    (if cfg!(debug_assertions) {
                        view! {
                            script(dangerously_set_inner_html=AUTORELOAD)
                        }
                    } else {
                        view! {}
                    })
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

/// Trunk autoreload script. We need to inject this manually into our HTML file since we are
/// generating the HTML file by hand.
static AUTORELOAD: &str = r##""use strict";
(function () {
    const address = 'localhost:8000/';
    const base = '';
    let protocol = '';
    protocol =
        protocol
            ? protocol
            : window.location.protocol === 'https:'
                ? 'wss'
                : 'ws';
    const url = protocol + '://' + address + base + '.well-known/trunk/ws';

    class Overlay {
        constructor() {
            // create an overlay
            this._overlay = document.createElement("div");
            const style = this._overlay.style;
            style.height = "100vh";
            style.width = "100vw";
            style.position = "fixed";
            style.top = "0";
            style.left = "0";
            style.backgroundColor = "rgba(222, 222, 222, 0.5)";
            style.fontFamily = "sans-serif";
            // not sure that's the right approach
            style.zIndex = "1000000";
            style.backdropFilter = "blur(1rem)";

            const container = document.createElement("div");
            // center it
            container.style.position = "absolute";
            container.style.top = "30%";
            container.style.left = "15%";
            container.style.maxWidth = "85%";

            this._title = document.createElement("div");
            this._title.innerText = "Build failure";
            this._title.style.paddingBottom = "2rem";
            this._title.style.fontSize = "2.5rem";

            this._message = document.createElement("div");
            this._message.style.whiteSpace = "pre-wrap";

            const icon= document.createElement("div");
            icon.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" fill="#dc3545" viewBox="0 0 16 16"><path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/></svg>';
            this._title.prepend(icon);

            container.append(this._title, this._message);
            this._overlay.append(container);

            this._inject();
            window.setInterval(() => {
                this._inject();
            }, 250);
        }

        set reason(reason) {
            this._message.textContent = reason;
        }

        _inject() {
            if (!this._overlay.isConnected) {
                // prepend it
                document.body?.prepend(this._overlay);
            }
        }

    }

    class Client {
        constructor(url) {
            this.url = url;
            this.poll_interval = 5000;
            this._overlay = null;
        }

        start() {
            const ws = new WebSocket(this.url);
            ws.onmessage = (ev) => {
                const msg = JSON.parse(ev.data);
                switch (msg.type) {
                    case "reload":
                        this.reload();
                        break;
                    case "buildFailure":
                        this.buildFailure(msg.data)
                        break;
                }
            };
            ws.onclose = () => this.onclose();
        }

        onclose() {
            window.setTimeout(
                () => {
                    // when we successfully reconnect, we'll force a
                    // reload (since we presumably lost connection to
                    // trunk due to it being killed, so it will have
                    // rebuilt on restart)
                    const ws = new WebSocket(this.url);
                    ws.onopen = () => window.location.reload();
                    ws.onclose = () => this.onclose();
                },
                this.poll_interval);
        }

        reload() {
            window.location.reload();
        }

        buildFailure({reason}) {
            // also log the console
            console.error("Build failed:", reason);

            console.debug("Overlay", this._overlay);

            if (!this._overlay) {
                this._overlay = new Overlay();
            }
            this._overlay.reason = reason;
        }
    }

    new Client(url).start();

})()"##;
