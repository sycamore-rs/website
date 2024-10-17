use sycamore::prelude::*;

use crate::set_title;

#[component]
pub fn NotFound() -> View {
    set_title("404 Not Found");
    view! {
        div(class="mt-8 mx-auto px-2 sm:px-0 prose prose-lg") {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
            p { a(href="/") { "Go back to the home page." } }
        }
    }
}
