use sycamore::prelude::*;
use sycamore_router::Route;

use crate::{
    layout::{self, DarkMode},
    pages,
};

#[derive(Debug, Clone, PartialEq, Route)]
pub enum Routes {
    #[to("/")]
    Index,
    #[to("/post/<id>")]
    Post(String),
    #[to("/book/<path..>")]
    Book(Vec<String>),
    #[not_found]
    NotFound,
}

impl Routes {
    fn published_path(&self) -> Option<String> {
        match self {
            Self::Index => Some("/".to_owned()),
            Self::Post(id) => Some(format!("/post/{id}")),
            Self::Book(path) => {
                crate::docs::resolve_book(path).map(|(source, page)| source.url(&page))
            }
            Self::NotFound => None,
        }
    }

    pub fn exists(&self) -> bool {
        self.published_path().is_some_and(|path| {
            crate::docs::PAGE_PATHS
                .binary_search(&path.as_str())
                .is_ok()
        })
    }

    pub fn validated(self) -> Self {
        if self.exists() {
            self
        } else {
            Self::NotFound
        }
    }

    pub fn is_indexable(&self) -> bool {
        if !self.exists() {
            return false;
        }
        match self {
            Self::NotFound => false,
            Self::Book(path) => {
                crate::docs::resolve_book(path).is_some_and(|(source, _)| source.is_latest())
            }
            _ => true,
        }
    }

    pub fn canonical_url(&self) -> Option<String> {
        if !self.is_indexable() {
            return None;
        }
        let path = match self {
            Self::Index => "/".to_owned(),
            Self::Post(id) => format!("/post/{id}"),
            Self::Book(path) => {
                let (source, page) = crate::docs::resolve_book(path)?;
                source.url(&page)
            }
            Self::NotFound => return None,
        };
        Some(format!("https://sycamore.dev{path}"))
    }
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
pub fn Shell(children: Children, initial_route: Option<Routes>) -> View {
    let initial_route = initial_route.unwrap_or(Routes::NotFound);
    let indexable = initial_route.is_indexable();
    let canonical = initial_route.canonical_url();
    let title = Title(create_signal(String::new()));
    provide_context(title);

    let dark_mode = if is_ssr!() {
        false
    } else {
        let local_storage = window().local_storage().unwrap();
        // Get dark mode from media query.
        let dark_mode_mq = window()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap()
            .matches();
        if let Some(local_storage) = &local_storage {
            let dark_mode_ls = local_storage.get_item("dark_mode").unwrap();
            dark_mode_ls.as_deref() == Some("true") || (dark_mode_ls.is_none() && dark_mode_mq)
        } else {
            dark_mode_mq
        }
    };
    let dark_mode = DarkMode(create_signal(dark_mode));
    if is_not_ssr!() {
        let local_storage = window().local_storage().unwrap();
        create_effect(move || {
            if let Some(local_storage) = &local_storage {
                local_storage
                    .set_item("dark_mode", &dark_mode.0.get().to_string())
                    .unwrap();
            }
        });
        // Force a re-render so that UI state is consistent.
        on_mount(move || dark_mode.0.set(dark_mode.0.get()));
    }

    provide_context(dark_mode);

    if is_not_ssr!() {
        create_effect(move || {
            let title = title.0.get_clone();
            if !title.is_empty() {
                document().set_title(&title);
            }
        });
    }

    // Call the children now so that we can get the title before we render the head.
    let children = children.call();
    let title_static = title.0.get_clone();

    view! {
        html(lang="en", "data-color-scheme"=if dark_mode.0.get() { "dark" } else { "light" }) {
            sycamore::web::NoHydrate {
                head {
                    meta(charset="utf-8")
                    meta(name="viewport", content="width=device-width, initial-scale=1")

                    meta(name="description", content="Sycamore is a next generation Rust UI library powered by fine-grained reactivity. Create reactive apps with effortless performance")

                    title { (title_static) }
                    if !indexable {
                        meta(name="robots", content="noindex")
                    }
                    if let Some(canonical) = canonical {
                        link(rel="canonical", href=canonical)
                    }

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

                    if option_env!("TRUNK_AUTO_RELOAD").is_some() {
                        script(dangerously_set_inner_html=AUTORELOAD)
                    }
                    // Synchronously check if we are in dark mode or not.
                    // If so, set the `data-color-scheme` attribute to "dark" and add the dark class to the body.
                    script(dangerously_set_inner_html=
                        r#"const darkModeLs = localStorage.getItem("dark_mode");
                        const darkMode = darkModeLs === "true" || (darkModeLs === null && window.matchMedia("(prefers-color-scheme: dark)").matches);
                        document.documentElement.setAttribute("data-color-scheme", darkMode ? "dark" : "light");"#)
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
    // Resolve unpublished URLs identically during SSR, hydration, and navigation.
    // In particular, a 404 must not acquire book chrome or an indexable canonical.
    let route = create_memo(move || route.get_clone().validated());
    provide_context(CurrentRoute(route));
    // NoHydrate owns the initial head; maintain SEO metadata on client navigation.
    is_not_ssr! {
        create_effect(move || {
            let current = route.get_clone();
            for selector in ["meta[name=robots]", "link[rel=canonical]"] {
                if let Some(element) = document().query_selector(selector).unwrap() {
                    element.remove();
                }
            }
            let head = document().head().unwrap();
            if !current.is_indexable() {
                let meta = document().create_element("meta").unwrap();
                meta.set_attribute("name", "robots").unwrap();
                meta.set_attribute("content", "noindex").unwrap();
                head.append_child(&meta).unwrap();
            }
            if let Some(url) = current.canonical_url() {
                let link = document().create_element("link").unwrap();
                link.set_attribute("rel", "canonical").unwrap();
                link.set_attribute("href", &url).unwrap();
                head.append_child(&link).unwrap();
            }
        });
    }
    view! {
        layout::Layout {
            match route.get_clone() {
                Routes::Index => pages::index::Index(),
                Routes::Post(id) => pages::post::Post(id=id),
                Routes::Book(path) => match crate::docs::resolve_book(&path) {
                    Some((source, page)) => pages::book::Book(source=source, page=page),
                    None => pages::not_found::NotFound(),
                },
                Routes::NotFound =>  { pages::not_found::NotFound() },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_urls_resolve_to_non_indexable_not_found() {
        for url in [
            "/book/does-not-exist",
            "/book/introduction/does-not-exist",
            "/post/does-not-exist",
        ] {
            let route = Routes::default().match_path(url);
            assert!(!route.exists(), "{url}");
            assert!(!route.is_indexable(), "{url}");
            assert_eq!(route.canonical_url(), None, "{url}");
            assert_eq!(route.validated(), Routes::NotFound, "{url}");
        }
        for source in crate::docs::DOCS_SOURCES.iter().filter(|s| !s.is_latest()) {
            let url = format!("/book/{}/does-not-exist", source.slug);
            assert_eq!(
                Routes::default().match_path(&url).validated(),
                Routes::NotFound
            );
        }
    }

    #[test]
    fn published_manifest_matches_static_generation_exactly() {
        let mut paths = crate::content::get_static_paths()
            .into_iter()
            .filter(|(route, _)| *route != Routes::NotFound)
            .map(|(route, _)| {
                assert!(route.exists());
                assert_eq!(route.clone().validated(), route);
                route.published_path().unwrap()
            })
            .collect::<Vec<_>>();
        paths.sort();
        assert_eq!(paths, crate::docs::PAGE_PATHS);
    }

    #[tokio::test]
    async fn unknown_book_and_post_render_the_404_layout_and_head() {
        for url in ["/book/does-not-exist", "/post/does-not-exist"] {
            let route = Routes::default().match_path(url);
            let html = sycamore::render_to_string_await_suspense(|| {
                view! {
                    Shell(initial_route=route.clone()) {
                        sycamore_router::StaticRouter(route=route, view=App)
                    }
                }
            })
            .await;
            let head = html.split("</head>").next().unwrap();
            assert!(head.contains("content=\"noindex\""), "{url}");
            assert!(!head.contains("rel=\"canonical\""), "{url}");
            assert!(html.contains("404 Not Found"), "{url}");
            assert!(!html.contains("<server-component"), "{url}");
            assert!(html.contains("<footer"), "{url}");
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
