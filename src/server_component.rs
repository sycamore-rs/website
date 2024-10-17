//! Only run a component at build-time/during SSR.
//!
//! Eventually consider upstreaming this into Sycamore or into a seperate crate.

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use sycamore::prelude::*;

type ServerComponentMap = HashMap<String, String>;

/// A map from server component ids to the rendered content.
pub static SERVER_COMPONENTS: LazyLock<Mutex<ServerComponentMap>> = LazyLock::new(Default::default);

/// Only run the component at build-time/during SSR. On the client side, if not hydrating, this
/// will fetch the component HTML over HTTP.
#[component(inline_props)]
pub fn ServerOnly(
    id: String,
    children: Children,
    #[prop(default, setter(transform = |f: impl Fn() + 'static| Some(Box::new(f) as Box<dyn Fn()>)))]
    on_mount: Option<Box<dyn Fn()>>,
) -> View {
    is_ssr! {
        let _ = on_mount;
        // Render the children, as well as adding it to SERVER_COMPONENTS.
        let mut children = Some(children);
        let view = view! {
            sycamore::web::NoHydrate {
                (children.take().unwrap().call())
            }
        };
        let html = sycamore::render_to_string_in_scope(|| view);
        SERVER_COMPONENTS.lock().unwrap().insert(id.clone(), html.clone());

        view! {
            server-component(data-component=id, dangerously_set_inner_html=html)
        }
    }
    is_not_ssr! {
        use gloo_net::http::Request;
        use sycamore::web::{AsHtmlNode, ViewHtmlNode, wasm_bindgen::JsCast};

        let _ = children;

        let container = create_node_ref();

        // Fetch the component HTML over HTTP if we are not hydrating.
        if !sycamore::web::is_hydrating() {
            let url = format!("/server_components/{id}.html");
            sycamore::futures::create_suspense_task(async move {
                let html = Request::get(&url).send().await.expect("could not send HTTP request").text().await.expect("could not get text from response");
                sycamore::web::DomNode::from_web_sys(container.get()).set_inner_html(html.into());

                // Recreate all the script tags so that they run.
                let scripts = container.get().unchecked_into::<web_sys::Element>().query_selector_all("script").unwrap();
                let n = scripts.length();
                for i in 0..n {
                    let script = scripts.get(i).unwrap().unchecked_into::<web_sys::Element>();
                    let mut new_script = sycamore::web::tags::script().dangerously_set_inner_html(script.inner_html());
                    document().body().unwrap().append_child(&new_script.as_html_node().as_web_sys()).unwrap();
                }

                if let Some(on_mount) = on_mount {
                    on_mount();
                }
            });
        }

        view! {
            server-component(data-component=id, r#ref=container)
        }
    }
}

/// Allows setting the document title from a server component.
#[component(inline_props)]
pub fn ServerTitle<S: Into<String>>(title: S) -> View {
    if is_ssr!() {
        let title = title.into();
        let js = format!("document.title = '{title}'");
        crate::shell::set_title(title);
        view! {
            script(dangerously_set_inner_html=js)
        }
    } else {
        view! {}
    }
}
