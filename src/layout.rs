use sycamore::prelude::*;

use crate::{CurrentRoute, Routes};

#[component(inline_props)]
fn Header(show_menu: ReadSignal<bool>, menu_open: Signal<bool>) -> View {
    let toggle_menu = move |_| menu_open.set(!menu_open.get());
    view! {
        header(class="fixed top-0 z-50 w-full border-b-2 border-gray-200 bg-gray-100") {
            nav(class="px-4") {
                div(class="flex flex-row justify-between items-center h-12") {
                    div(class="flex flex-row gap-4") {
                        (if show_menu.get() {
                            view! {
                                button(class="inline-block sm:hidden hover:text-gray-600", r#type="button", on:click=toggle_menu) {
                                    (if menu_open.get() {
                                        view! { i(class="bi bi-x-lg", aria-label="Close menu") }
                                    } else {
                                        view! { i(class="bi bi-list", aria-label="Open menu") }
                                    })
                                }
                            }
                        } else {
                            view! {}
                        })
                        a(class="flex flex-row items-center hover:underline font-semibold", href="/") {
                            img(src="/logo.svg", alt="Sycamore Logo", class="h-10 w-10 mr-2")
                            "Sycamore"
                        }
                    }
                    div(class="flex flex-row space-x-6 text-xl") {
                        a(href="/book/introduction") {
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
                            a(href="https://github.com/sycamore-rs/awesome-sycamore") { "Awesome" }
                        }
                    }
                }
                div {
                    p(class="font-semibold") { "Resources" }
                    ul {
                        li {
                            a(href="/book/introduction") { "Book" }
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
    let current_route = use_context::<CurrentRoute>();

    let menu_open = create_signal(false);
    // Show the menu only on book pages.
    let show_menu = create_selector(move || {
        matches!(
            current_route.0.get_clone(),
            Routes::BookSubsection(_, _) | Routes::BookSection(_)
        )
    });

    view! {
        div(class="flex flex-col min-h-screen") {
            Header(menu_open=menu_open, show_menu=show_menu)
            main(class="mt-12 flex-grow bg-gray-50") {
                div(class=if menu_open.get() { "transition-transform translate-x-44" } else { "transition-transform" }) {
                    (children)
                }
            }
            Footer {}
        }
    }
}
