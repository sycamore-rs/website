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
pub fn ServerOnly(id: String, children: Children) -> View {
    is_ssr! {
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
        use sycamore::web::ViewHtmlNode;

        let _ = children;

        let container = create_node_ref();

        // Fetch the component HTML over HTTP if we are not hydrating.
        if !sycamore::web::is_hydrating() {
            let url = format!("/server_components/{id}.html");
            sycamore::futures::spawn_local_scoped(async move {
                let html = Request::get(&url).send().await.expect("could not send HTTP request").text().await.expect("could not get text from response");
                sycamore::web::DomNode::from_web_sys(container.get()).set_inner_html(html.into());
            });
        }

        view! {
            server-component(data-component=id, r#ref=container)
        }
    }
}
