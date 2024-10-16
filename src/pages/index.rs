use sycamore::prelude::*;

#[component]
pub fn Index() -> View {
    view! {
        div(class="flex flex-col container mx-2 md:mx-auto") {
            div(class="mt-10 md:mt-20 flex flex-col md:flex-row gap-10 items-center justify-between") {
                div(class="max-w-[530px]") {
                    h1(class="text-5xl pb-5 font-bold bg-gradient-to-br from-orange-800 from-20% to-orange-800 to-80% via-orange-950 text-transparent bg-clip-text") {
                        "Reactive Apps with"
                        br {}
                        "Effortless Performance."
                    }
                    p(class="text-2xl") {
                        span(class="font-bold text-orange-900") { "Sycamore" } " is a next generation Rust UI library powered by fine-grained reactivity."
                    }

                    div(class="flex flex-row flex-wrap gap-4 font-semibold mt-10") {
                        a(class="block px-5 py-1.5 min-w-40 text-center bg-orange-400 rounded-full hover:bg-orange-500 transition-colors", href="/book") {
                            "Read the Book"
                        }
                        a(class="block px-5 py-1.5 min-w-40 text-center text-white bg-gray-800 rounded-full hover:bg-gray-900 transition-colors", href="https://discord.gg/vDwFUmm6mU") {
                            "Join the Discord"
                        }
                    }

                    p(class="text-sm mt-4 text-gray-800") {
                        "Latest Release: " a(href="https://crates.io/crates/sycamore", class="underline") { "v0.9.0-beta.4" }
                    }
                }
                // Code example
                div(class="flex-grow w-full md:w-auto") {
                    pre(class="bg-gray-800 rounded-lg mx-auto text-white text-xs sm:text-sm md:text-base overflow-x-hidden w-full md:max-w-[550px] shadow-lg language-rust") {
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

            // Feature descriptions
            div {
                h2(class="text-4xl text-center font-bold mt-20 mb-5") {
                    "Features"
                }
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
            }

            // News
            h2(class="text-4xl text-center font-bold mt-20") {
                "News"
            }
        }
    }
}

#[component(inline_props)]
fn FeatureIcon(icon: &'static str) -> View {
    view! {
        div(class="h-12 w-12 bg-orange-400 text-white text-2xl rounded-lg flex items-center justify-center mx-auto mb-3") {
            i(class=format!("bi bi-{icon}"))
        }
    }
}

#[component(inline_props)]
fn FeatureCard(icon: &'static str, title: &'static str, children: Children) -> View {
    view! {
        div(class="hover:shadow-lg hover:bg-white rounded-lg py-4 px-5 transition mx-auto max-w-[500px]") {
            FeatureIcon(icon=icon)
            h3(class="text-xl font-semibold text-center") {
                (title)
            }
            (children)
        }
    }
}
