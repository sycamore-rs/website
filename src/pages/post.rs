use std::fs;

use sycamore::prelude::*;

use crate::server_component::ServerOnly;

#[component(inline_props)]
pub fn Post(id: String) -> View {
    view! {
        ServerOnly(id=format!("Post_{id}")) {
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
    let file_path = format!("sycamore/docs/posts/{id}.md");
    let md = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("could not read post file `{id}.md`"));

    let parsed = mdsycx::parse::<()>(&md).unwrap();

    view! {
        div(class="container mx-auto px-2") {
            mdsycx::MDSycX(body=parsed.body)
        }
    }
}
