use sycamore::prelude::*;

use crate::server_component::ServerOnly;

#[component(inline_props)]
pub fn Post(id: String) -> View {
    view! {
        ServerOnly(id=format!("Post_{id}"), on_load=|| { crate::utils::prism_highlight_all(); }) {
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
        div(class="flex flex-row gap-0 sm:gap-4 w-full justify-center") {
            div(class="flex-none w-40 hidden lg:block") // Empty block used for spacing
            div(class="grow-0 min-w-0 px-2 pt-5 pb-10 prose md:w-[80ch] dark:prose-invert") {
                span(class="text-sm") { (parsed.front_matter.date.to_string()) }
                mdsycx::MDSycX(body=parsed.body)
            }
            crate::utils::HeadingsOutline(headings=parsed.headings)
        }
    }
}
