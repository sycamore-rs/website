use sycamore::prelude::*;

use crate::{server_component::ServerOnly, set_title};

#[component]
pub fn Index() -> View {
    set_title("Sycamore");
    view! {
        ServerOnly(id="IndexBody".to_string(), on_load=move |_| { crate::utils::prism_highlight_all(); }) {
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
        div(class="flex flex-col container px-2 md:px-20 mx-auto pb-10") {
            div(class="mt-10 md:mt-20 flex flex-col md:flex-row gap-10 items-center justify-between") {
                div(class="max-w-[530px]") {
                    h1(class="text-4xl sm:text-5xl pb-5 font-bold bg-gradient-to-br from-orange-800 from-20% to-orange-800 to-80% via-orange-950 dark:from-orange-200 dark:to-orange-500 dark:via-orange-400 text-transparent bg-clip-text") {
                        "Reactive Apps with"
                        br {}
                        "Effortless Performance."
                    }
                    p(class="text-2xl") {
                        span(class="font-bold text-orange-900 dark:text-orange-400") { "Sycamore" } " is a next gen Rust web UI library powered by fine-grained reactivity."
                    }

                    div(class="flex flex-row flex-wrap gap-4 font-semibold mt-10") {
                        a(class="block px-5 py-1.5 min-w-40 text-center bg-orange-400 dark:bg-orange-500 rounded-full hover:bg-orange-500 dark:hover:bg-orange-400 transition-colors", href="/book/introduction", data-umami-event="Read book") {
                            "Read the Book"
                        }
                        a(class="block px-5 py-1.5 min-w-40 text-center text-white bg-gray-800 rounded-full hover:bg-gray-900 transition-colors", href="https://play.sycamore.dev", data-umami-event="Playground") {
                            "Try in Browser"
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
        button(on_click=move |_| value += 1) {
            "Count: {value}"
        }
    }
}"#
                        }
                    }
                }
            }

            SectionHeading(content="Features")
            p(class="text-center text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto mb-4") {
                "Less framework. More building. See what a little Rust can do."
            }
            div(class="divide-y divide-gray-200 dark:divide-gray-800") {
                FeatureSection(
                    number="01",
                    label="Fine-grained reactivity",
                    title="Update only what changes.",
                    description="Built on Rust and WebAssembly, Sycamore pairs native control with fine-grained updates. Signals track their dependencies, so changing a value updates the UI that uses it—not your entire component.",
                    href="/book/introduction/adding-state",
                    link_text="Explore reactivity",
                    filename="reactivity.rs",
                    example=r#"let mut count = create_signal(0);
let doubled = create_memo(move || count.get() * 2);

view! {
    button(on_click=move |_| count += 1) {
        "Count: {count}"
    }
    // Only this text updates when doubled changes.
    p { "Doubled: {doubled}" }
}"#,
                )
                FeatureSection(
                    number="02",
                    label="Type-checked UI",
                    title="Write views. Keep Rust's guarantees.",
                    description="Build components with a familiar, HTML-like syntax and the full power of Rust. Typed props and event handlers help catch mistakes at compile time, before they reach your users.",
                    href="/book/introduction/your-first-app",
                    link_text="Build your first component",
                    filename="components.rs",
                    reverse=true,
                    example=r#"#[component]
fn Greeting(#[prop(into)] name: String) -> View {
    view! {
        h2 { "Hello, {name}!" }
    }
}

view! {
    Greeting(name="Sycamore")
}"#,
                )
                FeatureSection(
                    number="03",
                    label="Server-side rendering",
                    title="Start on the server. Come alive in the browser.",
                    description="Render your components to HTML on the server, then hydrate them in the browser to add interactivity. Prefer a client-only app? The same component model works there, too.",
                    href="/book/server-side-rendering",
                    link_text="Learn about server rendering",
                    filename="server.rs",
                    example=r#"#[component]
fn App() -> View {
    view! {
        h1 { "Hello from the server." }
    }
}

let html = sycamore::render_to_string(App);"#,
                )
                FeatureSection(
                    number="04",
                    label="Async and Suspense",
                    title="Give loading states a place to live.",
                    description="Use async Rust directly in your components. Suspense shows a fallback while async work finishes, so your loading state stays right beside the content it belongs to.",
                    href="/book/guide/resources-and-suspense",
                    link_text="Meet resources and Suspense",
                    filename="async.rs",
                    reverse=true,
                    example=r#"#[component]
async fn Welcome() -> View {
    // Your async data-loading function.
    let name = fetch_name().await;
    view! { p { "Welcome, {name}" } }
}

view! {
    Suspense(fallback=|| view! { "Loading…" }) {
        Welcome {}
    }
}"#,
                )
                FeatureSection(
                    number="05",
                    label="Built-in routing",
                    title="A home for every page.",
                    description="Define your routes with a Rust enum, including typed URL parameters and a not-found fallback. Sycamore's router supports client-side navigation and server-side rendering as your app grows.",
                    href="/book/router",
                    link_text="Find your way around the router",
                    filename="routes.rs",
                    example=r#"use sycamore_router::Route;

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/hello/<name>")]
    Hello { name: String },
    #[not_found]
    NotFound,
}"#,
                )
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
fn FeatureSection(
    number: &'static str,
    label: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
    link_text: &'static str,
    filename: &'static str,
    example: &'static str,
    #[prop(default)] reverse: bool,
) -> View {
    let layout = if reverse {
        "flex flex-col md:flex-row-reverse items-center gap-8 lg:gap-16 py-12 lg:py-16"
    } else {
        "flex flex-col md:flex-row items-center gap-8 lg:gap-16 py-12 lg:py-16"
    };
    view! {
        section(class=layout) {
            div(class="w-full md:w-1/2 min-w-0") {
                p(class="flex items-center gap-3 text-sm font-semibold tracking-wider uppercase text-orange-800 dark:text-orange-400 mb-4") {
                    span(class="font-mono text-orange-800/50 dark:text-orange-400/50", aria-hidden="true") { (number) }
                    (label)
                }
                h3(class="text-3xl lg:text-4xl font-bold tracking-tight mb-5") { (title) }
                p(class="text-lg leading-relaxed text-gray-600 dark:text-gray-400") { (description) }
                a(href=href, class="inline-flex items-center gap-2 mt-6 font-semibold text-orange-800 dark:text-orange-400 underline decoration-orange-400/40 underline-offset-4 hover:decoration-current focus-visible:outline-2 focus-visible:outline-offset-4 focus-visible:outline-orange-500 rounded-sm") {
                    (link_text)
                    span(aria-hidden="true") { "→" }
                }
            }
            div(class="w-full md:w-1/2 min-w-0 overflow-hidden rounded-xl border border-gray-700 shadow-lg") {
                div(class="flex items-center gap-2 bg-gray-800 px-5 py-3 border-b border-gray-700 text-gray-400 text-xs font-mono") {
                    span(class="h-2 w-2 rounded-full bg-orange-400", aria-hidden="true") {}
                    (filename)
                    span(class="ml-auto text-gray-400") { "Rust" }
                }
                pre(class="language-rust !m-0 !rounded-none text-xs sm:text-sm !overflow-x-auto", tabindex="0", aria-label=format!("{label} code example")) {
                    code(class="language-rust") { (example) }
                }
            }
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
