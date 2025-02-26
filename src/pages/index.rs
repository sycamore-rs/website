use sycamore::prelude::*;

use crate::{server_component::ServerOnly, set_title};

#[component]
pub fn Index() -> View {
    set_title("Sycamore");
    view! {
        ServerOnly(id="IndexBody".to_string(), on_load=move || { crate::utils::prism_highlight_all(); }) {
            IndexBody {}
        }
    }
}

#[cfg_not_ssr]
#[component]
fn IndexBody() -> View {
    unreachable!()
}

#[cfg_ssr]
#[component]
fn IndexBody() -> View {
    let latest_release = crate::api_stats::get_latest_release();
    view! {
        div(class="flex flex-col container px-2 mx-auto pb-10") {
            div(class="mt-10 md:mt-20 flex flex-col md:flex-row gap-10 items-center justify-between") {
                div(class="max-w-[530px]") {
                    h1(class="text-4xl sm:text-5xl pb-5 font-bold bg-gradient-to-br from-orange-800 from-20% to-orange-800 to-80% via-orange-950 dark:from-orange-200 dark:to-orange-500 dark:via-orange-400 text-transparent bg-clip-text") {
                        "Reactive Apps with"
                        br {}
                        "Effortless Performance."
                    }
                    p(class="text-2xl") {
                        span(class="font-bold text-orange-900 dark:text-orange-400") { "Sycamore" } " is a next generation Rust UI library powered by fine-grained reactivity."
                    }

                    div(class="flex flex-row flex-wrap gap-4 font-semibold mt-10") {
                        a(class="block px-5 py-1.5 min-w-40 text-center bg-orange-400 dark:bg-orange-500 rounded-full hover:bg-orange-500 dark:hover:bg-orange-400 transition-colors", href="/book/introduction", data-umami-event="Read book") {
                            "Read the Book"
                        }
                        a(class="block px-5 py-1.5 min-w-40 text-center text-white bg-gray-800 rounded-full hover:bg-gray-900 transition-colors", href="https://discord.gg/vDwFUmm6mU", data-umami-event="Join discord") {
                            "Join the Discord"
                        }
                    }

                    p(class="text-sm mt-4 text-gray-800 dark:text-gray-400") {
                        "Latest Release: " a(href=latest_release.html_url, class="underline") { (latest_release.name) }
                    }
                }
                // Code example
                div(class="flex-grow w-full md:w-auto") {
                    pre(class="bg-gray-800 rounded-lg text-white text-xs sm:text-sm md:text-base overflow-x-hidden w-full md:max-w-[550px] shadow-lg !mx-auto language-rust") {
                        code(class="language-rust") {
                            r#"use sycamore::prelude::*;

#[component]
fn Counter(initial: i32) -> View {
    let mut value = create_signal(initial);

    view! {
        button(on:click=move |_| value += 1) {
            "Count: " (value)
        }
    }
}"#
                        }
                    }
                }
            }

            SectionHeading(content="Features")
            div(class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4") {
                FeatureCard(
                    icon="rocket-takeoff-fill",
                    title="Effortless Performance",
                ) {
                    p {
                        "Sycamore is built on top of "
                        a(href="https://www.rust-lang.org", class="underline") { "Rust" } " and "
                        a(href="https://webassembly.org/", class="underline") { "WebAssembly" }
                        ", giving you full control over performance."
                    }
                }
                FeatureCard(
                    icon="arrow-left-right",
                    title="Fine-Grained Reactivity",
                ) {
                    p {
                        "Sycamore's reactivity system is fine-grained, meaning that only the parts of your app that need to be updated will be."
                    }
                }
                FeatureCard(
                    icon="body-text",
                    title="Type-checked UI",
                ) {
                    p {
                        "Whether you use our custom DSL or the builder API, Sycamore type-checks your code to catch errors at compile-time."
                    }
                }
                FeatureCard(
                    icon="database-fill",
                    title="Server Side Rendering (SSR)",
                ) {
                    p {
                        "Sycamore supports Server Side Rendering out of the box. If you don't need it, however, SPA mode works just as well."
                    }
                }
                FeatureCard(
                    icon="arrow-clockwise",
                    title="Async and Suspense"
                ) {
                    p {
                        "Easily load and display asynchronous data using the resources and suspense API with first-class async/await support."
                    }
                }
                FeatureCard(
                    icon="grid-3x3",
                    title="Built-in Routing",
                ) {
                    p {
                        "Sycamore comes with a built-in router that supports both client-side navigation and server side rendering."
                    }
                }
            }

            SectionHeading(content="Community")
            CommunitySection {}

            SectionHeading(content="News", id="news")
            NewsList {}
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn SectionHeading(
    content: &'static str,
    #[prop(attributes(html, h2))] attributes: Attributes,
) -> View {
    view! {
        h2(class="text-4xl text-center font-bold mt-20 mb-5", ..attributes) {
            (content)
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn FeatureIcon(icon: &'static str) -> View {
    view! {
        div(class="h-12 w-12 bg-orange-400 text-white text-2xl rounded-lg flex items-center justify-center mx-auto mb-3") {
            i(class=format!("bi bi-{icon}"))
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn FeatureCard(icon: &'static str, title: &'static str, children: Children) -> View {
    view! {
        div(class="hover:shadow-lg hover:bg-white dark:hover:bg-gray-900 rounded-lg py-4 px-5 transition mx-auto max-w-[500px]") {
            FeatureIcon(icon=icon)
            h3(class="text-xl font-semibold text-center") {
                (title)
            }
            (children)
        }
    }
}

#[cfg_ssr]
#[component]
fn CommunitySection() -> View {
    let repo_stats = crate::api_stats::get_repo_stats();
    let stars_hundreds = (repo_stats.stargazers_count as f64 / 100.0).round() as u32;
    let stars_text = format!("{}.{}k", stars_hundreds / 10, stars_hundreds % 10);

    let contributors = crate::api_stats::get_contributors();
    let contributors_len = contributors.len();

    let crates_io_downloads = crate::api_stats::get_crate_io_stats()._crate.downloads;

    view! {
        div(class="grid grid-rows-3 sm:grid-rows-1 sm:grid-cols-3 text-2xl font-bold text-center divide-gray-200 divide-y-2 sm:divide-y-0 sm:divide-x-2 divide-solid rounded-lg max-w-[1000px] mx-auto") {
            div(class="px-4") {
                (format!("{stars_text} Stars"))
                p(class="text-sm font-normal") { "on GitHub" }
            }
            div(class="px-4") {
                (format!("{} Contributors", contributors_len))
                p(class="text-sm font-normal") { "on GitHub" }
            }
            div(class="px-4") {
                (format!("{}k Downloads", crates_io_downloads / 1000))
                p(class="text-sm font-normal") { "on crates.io" }
            }
        }
        div(class="mt-5 text-center") {
            p { "Sycamore is made possible by all our " a(class="underline", href="https://github.com/sycamore-rs/sycamore/graphs/contributors") { "community contributors" } ". Thank you!" }

            div(class="mx-auto my-2 sm:max-w-[800px] flex flex-wrap justify-center gap-2") {
                Indexed(
                    list=contributors,
                    view=|contributor| view! {
                        a(href=contributor.html_url) {
                            img(src=contributor.avatar_url, title=contributor.login, class="rounded-full w-12 h-12 hover:shadow-lg transition", loading="lazy")
                        }
                    }
                )
            }

            p { "Interested in contributing as well? Check out our " a(class="underline", href="https://github.com/sycamore-rs/sycamore/blob/main/CONTRIBUTING.md") { "contribution guide" } "." }
        }
    }
}

#[cfg_ssr]
#[component]
fn NewsList() -> View {
    // Sort posts by date.
    let mut posts = crate::content::POSTS
        .iter()
        .map(|(id, post)| (id.clone(), post.clone()))
        .collect::<Vec<_>>();
    posts.sort_by_key(|(_, post)| post.front_matter.date);

    posts
        .into_iter()
        .rev()
        .map(|(id, post)| {
            view! {
                a(href=format!("/post/{id}"), class="mt-5") {
                    p(class="text-xs") {
                        (post.front_matter.date.to_string())
                    }
                    p(class="text-2xl font-semibold") {
                        (post.front_matter.title.clone())
                    }
                    p(class="text-gray-800 dark:text-gray-400") {
                        (post.front_matter.description.clone())
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .into()
}
