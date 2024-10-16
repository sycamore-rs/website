use sycamore::prelude::*;

#[component(inline_props)]
pub fn Post(id: String) -> View {
    view! {
        div {
            h1 { (id) }
            p { "This is a post!" }
        }
    }
}
