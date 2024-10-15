use sycamore::prelude::*;

#[component]
fn Header() -> View {
    view! {
        header(class="fixed top-0 z-50 w-full border-b-2 border-gray-200 bg-gray-100") {
            nav(class="px-4") {
                div(class="flex flex-row justify-between items-center h-10") {
                    a(href="/") {
                        img(src="/logo.svg", alt="Sycamore Logo", class="h-10 w-10")
                    }
                    div(class="flex flex-row space-x-4") {
                        a(href="https://github.com/sycamore-rs/sycamore") {
                            i(class="bi bi-github text-3xl hover:text-gray-600", aria-label="GitHub")
                        }
                        a(href="https://discord.gg/vDwFUmm6mU") {
                            i(class="bi bi-discord text-3xl hover:text-gray-600", aria-label="Discord")
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> View {
    view! {
        footer(class="text-sm px-4 py-2 border-t-2 border-gray-200 bg-gray-100") {
            div {
                "© 2024 Sycamore"
            }
        }
    }
}

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    let children = children.call();
    view! {
        div(class="flex flex-col min-h-screen") {
            Header {}
            main(class="mt-10 flex-grow") {
                (children)
            }
            Footer {}
        }
    }
}
