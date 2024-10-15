use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        h1(class="bg-red-400") { "Sycamore!" }
    }
}

fn main() {
    console_error_panic_hook::set_once();

    sycamore::render(App);
}
