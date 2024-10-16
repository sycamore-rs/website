use sycamore::prelude::*;

use crate::server_component::ServerOnly;

#[component(inline_props)]
pub fn Post(id: String) -> View {
    view! {
        ServerOnly(id=format!("Post_{id}"), on_mount=|| { crate::utils::prism_highlight_all(); }) {
            PostBody(id=id)
        }
    }
}

#[cfg_not_ssr]
#[component(inline_props)]
pub fn PostBody(id: String) -> View {
    let _ = id;
    unreachable!()
}

#[cfg_ssr]
#[component(inline_props)]
pub fn PostBody(id: String) -> View {
    let parsed = crate::content::POSTS
        .get(&id)
        .expect("post not found")
        .clone();

    view! {
        div(class="mx-auto px-2 sm:px-0 prose prose-gray md:prose-lg") {
            mdsycx::MDSycX(body=parsed.body)
        }
    }
}
