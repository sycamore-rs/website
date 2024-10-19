use sycamore::prelude::*;

#[component]
fn Header() -> View {
    view! {
        header(class="fixed top-0 z-50 w-full border-b-2 border-gray-200 bg-gray-100") {
            nav(class="px-4") {
                div(class="flex flex-row justify-between items-center h-12") {
                    a(class="flex flex-row items-center hover:underline font-semibold", href="/") {
                        img(src="/logo.svg", alt="Sycamore Logo", class="h-10 w-10 mr-2")
                        "Sycamore"
                    }
                    div(class="flex flex-row space-x-4 md:space-x-6 text-xl") {
                        a(href="/book/getting_started/installation") {
                            i(class="bi bi-book-half hover:text-gray-600", aria-label="Book")
                        }
                        a(href="https://github.com/sycamore-rs/sycamore") {
                            i(class="bi bi-github hover:text-gray-600", aria-label="GitHub")
                        }
                        a(href="https://discord.gg/vDwFUmm6mU") {
                            i(class="bi bi-discord hover:text-gray-600", aria-label="Discord")
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
        footer(class="text-sm px-4 pt-4 pb-2 border-t-2 border-gray-200 bg-gray-100") {
            div(class="flex flex-col sm:flex-row gap-10 md:gap-20 lg:gap-40") {
                div {
                    div(class="flex flex-row items-center gap-4") {
                        img(src="/logo.svg", alt="Sycamore Logo", class="h-10 w-10")
                        span(class="font-semibold text-base") { "Sycamore" }
                    }
                    p(class="text-xs") {
                        "This website is also built with Sycamore."
                        br {}
                        "Check out the " a(class="underline", href="https://github.com/sycamore-rs/website") { "source" } "!"
                    }
                }
                div {
                    p(class="font-semibold") { "Community" }
                    ul {
                        li {
                            a(href="https://github.com/sycamore-rs/sycamore") { "GitHub" }
                        }
                        li {
                            a(href="https://discord.gg/vDwFUmm6mU") { "Discord" }
                        }
                        li {
                            a(href="https://github.com/sycamore-rs/sycamore-awesome") { "Awesome" }
                        }
                    }
                }
                div {
                    p(class="font-semibold") { "Resources" }
                    ul {
                        li {
                            a(href="/book/getting_started/installation") { "Book" }
                        }
                        li {
                            a(href="https://docs.rs/sycamore") { "docs.rs" }
                        }
                    }
                }
            }
            p(class="mt-6 text-gray-700 text-xs") { "© 2024 Sycamore" }
        }
    }
}

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    let children = children.call();
    view! {
        div(class="flex flex-col min-h-screen") {
            Header {}
            main(class="mt-12 flex-grow bg-gray-50") {
                (children)
            }
            Footer {}
        }
    }
}
