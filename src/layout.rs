use sycamore::prelude::*;

use crate::{CurrentRoute, Routes};

#[component(inline_props)]
fn Header(show_menu: ReadSignal<bool>, menu_open: Signal<bool>) -> View {
    let toggle_menu = move |_| menu_open.set(!menu_open.get());
    view! {
        header(class="fixed top-0 z-50 w-full border-b border-gray-200 bg-gray-100 dark:bg-gray-950 dark:border-gray-800") {
            nav(class="px-4") {
                div(class="flex flex-row justify-between items-center h-14") {
                    div(class="flex flex-row gap-4") {
                        if show_menu.get() {
                            button(class="inline-block sm:hidden hover:text-gray-600", r#type="button", on:click=toggle_menu) {
                                (if menu_open.get() {
                                    view! { i(class="bi bi-x-lg", aria-label="Close menu") }
                                } else {
                                    view! { i(class="bi bi-list", aria-label="Open menu") }
                                })
                            }
                        }
                        a(class="flex flex-row items-center hover:underline font-semibold", href="/") {
                            img(src="/logo.svg", alt="Sycamore Logo", class="h-10 w-10 mr-2")
                            "Sycamore"
                        }
                    }
                    div(class="flex flex-row items-center gap-1 sm:gap-2") {
                        a(href="/book/introduction", aria-label="Book", class="flex items-center gap-2 rounded-md px-2 lg:px-3 py-2 hover:text-gray-600 dark:hover:text-gray-300 transition-colors") {
                            i(class="bi bi-book-half text-xl", aria-hidden="true")
                            span(class="hidden lg:inline text-sm") { "Book" }
                        }
                        a(href="https://github.com/sycamore-rs/sycamore", target="_blank", rel="noopener", aria-label="GitHub", class="flex items-center gap-2 rounded-md px-2 lg:px-3 py-2 hover:text-gray-600 dark:hover:text-gray-300 transition-colors") {
                            i(class="bi bi-github text-xl", aria-hidden="true")
                            span(class="hidden lg:inline text-sm") {
                                crate::server_component::ServerOnly(id="HeaderGitHubStars".to_string()) {
                                    GitHubStars {}
                                }
                            }
                        }
                        a(href="https://discord.gg/vDwFUmm6mU", target="_blank", rel="noopener", aria-label="Discord", class="flex items-center gap-2 rounded-md px-2 lg:px-3 py-2 hover:text-gray-600 dark:hover:text-gray-300 transition-colors") {
                            i(class="bi bi-discord text-xl", aria-hidden="true")
                        }
                        DarkModeToggle {}
                    }
                }
            }
        }
    }
}

#[cfg_ssr]
#[component]
fn GitHubStars() -> View {
    let stars = crate::api_stats::get_repo_stats().stargazers_count;
    let hundreds = (stars as f64 / 100.0).round() as u32;
    view! { (format!("{}.{}k", hundreds / 10, hundreds % 10)) }
}

#[cfg_not_ssr]
#[component]
fn GitHubStars() -> View {
    unreachable!()
}

#[derive(Clone, Copy)]
pub struct DarkMode(pub Signal<bool>);

#[component]
fn DarkModeToggle() -> View {
    let DarkMode(dark_mode) = use_context::<DarkMode>();
    let toggle = move |_| dark_mode.set(!dark_mode.get());

    view! {
        button(title="Toggle dark mode", aria-label="Toggle dark mode", class="flex items-center gap-2 rounded-md px-2 lg:px-3 py-2 cursor-pointer hover:text-gray-600 dark:hover:text-gray-300 transition-colors", on:click=toggle) {
            if dark_mode.get() {
                i(class="bi bi-sun text-xl", aria-hidden="true")
            } else {
                i(class="bi bi-moon text-xl", aria-hidden="true")
            }
        }
    }
}

#[component]
fn Footer() -> View {
    view! {
        footer(class="text-sm px-4 md:px-22 pt-6 pb-10 border-t-2 border-gray-200 bg-gray-100 dark:bg-gray-900 dark:border-gray-800") {
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
                    p(class="font-semibold my-2") { "Community" }
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
                    p(class="font-semibold my-2") { "Resources" }
                    ul {
                        li {
                            a(href="/book/introduction") { "Book" }
                        }
                        li {
                            a(href="https://docs.rs/sycamore") { "docs.rs" }
                        }
                        li {
                            a(href="https://play.sycamore.dev") { "Playground" }
                        }
                    }
                }
            }
            p(class="mt-6 text-gray-700 dark:text-gray-300 text-xs") { "© 2026 Sycamore" }
        }
    }
}

#[component(inline_props)]
pub fn Layout(children: Children) -> View {
    let current_route = use_context::<CurrentRoute>();

    let menu_open = create_signal(false);
    // Show the menu only on book pages.
    let is_book_page =
        create_selector(move || matches!(current_route.0.get_clone(), Routes::Book(_)));

    view! {
        div(class="flex flex-col min-h-screen dark:text-gray-100") {
            Header(menu_open=menu_open, show_menu=is_book_page)
            main(class="mt-14 flex-grow bg-gray-50 dark:bg-gray-950") {
                div(class=if menu_open.get() { "transition-transform translate-x-60" } else { "transition-transform" }) {
                    (children)
                }
            }
            // Don't show the footer on book pages since they have a different layout.
            if !is_book_page.get() {
                Footer {}
            }
        }
    }
}
