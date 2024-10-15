use sycamore::prelude::*;

#[component]
pub fn Index() -> View {
    view! {
        div(class="flex flex-col container mx-2 md:mx-auto") {
            div(class="mt-20 flex flex-col md:flex-row gap-10 items-center justify-between") {
                div {
                    h1(class="text-5xl pb-5 font-bold bg-gradient-to-br from-orange-800 from-20% to-orange-800 to-80% via-orange-950 text-transparent bg-clip-text") {
                        "Reactive Apps with"
                        br {}
                        "Effortless Performance."
                    }
                    p(class="text-2xl") {
                        "Sycamore is a next generation Rust UI library."
                    }

                    div(class="flex flex-row flex-wrap gap-4 font-semibold mt-10") {
                        a(class="block px-5 py-1.5 min-w-40 text-center bg-orange-400 rounded-full hover:bg-orange-500 transition-colors", href="/book") {
                            "Read the Book"
                        }
                        a(class="block px-5 py-1.5 min-w-40 text-center text-white bg-gray-800 rounded-full hover:bg-gray-900 transition-colors", href="https://discord.gg/vDwFUmm6mU") {
                            "Join the Discord"
                        }
                    }
                }
                // Code example
                div(class="flex-grow w-full md:w-auto") {
                    pre(class="bg-gray-800 rounded-lg mx-auto p-5 text-white text-xs sm:text-sm md:text-base overflow-x-hidden w-full md:max-w-[550px]") {
                        code {
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
            h2(class="text-4xl font-bold mt-20") {
                "Features"
            }
        }
    }
}
