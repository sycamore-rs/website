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
    use crate::server_component::ServerTitle;

    let parsed = crate::content::POSTS
        .get(&id)
        .expect("post not found")
        .clone();

    view! {
        ServerTitle(title=parsed.front_matter.title.clone())
        div(class="mx-auto px-2 pt-0 pb-10 sm:pt-5 prose prose-gray") {
            span(class="text-sm") { (parsed.front_matter.date.to_string()) }
            mdsycx::MDSycX(body=parsed.body)
        }
    }
}
