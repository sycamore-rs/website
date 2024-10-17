use sycamore::prelude::*;

use crate::server_component::ServerOnly;

#[component(inline_props)]
pub fn Book(section: String, doc: Option<String>) -> View {
    let id = match &doc {
        Some(doc) => format!("Book_{section}_{doc}"),
        None => format!("Book_{section}"),
    };
    view! {
        ServerOnly(id=id, on_mount=|| { crate::utils::prism_highlight_all(); }) {
            BookBody(section=section, doc=doc)
        }
    }
}

#[cfg_not_ssr]
#[component(inline_props)]
fn BookBody(section: String, #[prop(!optional)] doc: Option<String>) -> View {
    let _ = section;
    let _ = doc;
    unreachable!()
}

#[cfg_ssr]
#[component(inline_props)]
fn BookBody(section: String, #[prop(!optional)] doc: Option<String>) -> View {
    let parsed = crate::content::DOCS
        .get(&(section.clone(), doc.clone()))
        .expect("doc not found")
        .clone();

    view! {
        div(class="flex flex-row gap-4 w-full justify-center") {
            BookSidebar {}
            div(class="grow-0 min-w-0 px-2 pt-0 pb-10 sm:pt-5 prose prose-gray") {
                mdsycx::MDSycX(body=parsed.body)
            }
            HeadingsOutline(headings=parsed.headings)
        }
    }
}

#[cfg_ssr]
#[component]
fn BookSidebar() -> View {
    let sidebar = crate::content::BOOK_SIDEBAR.clone();

    let view = sidebar
        .sections
        .into_iter()
        .map(|section| {
            let items = section
                .items
                .into_iter()
                .map(|item| {
                    view! {
                        li {
                            a(href=format!("/book/{}", item.href), class="hover:text-orange-700 transition-colors") { (item.name) }
                        }
                    }
                })
                .collect::<Vec<_>>();
            view! {
                div(class="pl-2") {
                    a(class="font-bold") { (section.title.clone()) }
                    ul(class="pl-2") {
                        (items)
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        div(
            class="flex-none w-44 pt-8 pb-5 pr-2 text-sm sticky top-12 max-h-[calc(100vh-3rem)] overflow-y-auto hidden sm:block"
        ) {
            (view)
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn HeadingsOutline(headings: Vec<mdsycx::OutlineHeading>) -> View {
    let outline = headings
        .into_iter()
        .map(|heading| {
            let href = format!("#{}", heading.id);
            let class = "hover:text-orange-700 transition-colors";
            match heading.level {
                1 => view! {},
                2 => view! {
                    li(class="mt-2") {
                        a(href=href, class=format!("{class} font-semibold")) { (heading.text) }
                    }
                },
                _n => view! {
                    li(class="mt-0.5 ml-2") {
                        a(href=href, class=format!("{class} pl-2")) { (heading.text) }
                    }
                },
            }
        })
        .collect::<Vec<_>>();
    view! {
        div(
            class="flex-none w-56 pt-8 pb-5 pr-2 text-sm sticky top-12 max-h-[calc(100vh-3rem)] overflow-y-auto hidden lg:block"
        ) {
            div {
                p(class="font-bold -mb-1") { "On this page" }
                ul(class="pl-2") {
                    (outline)
                }
            }
            div {
                p(class="font-bold mt-4") { "Previous versions" }
                ul(class="pl-2 font-semibold mt-1") {
                    li {
                        a(class="hover:text-orange-700", href="https://sycamore-rs.netlify.app/docs/v0.8/getting_started/installation") {
                            "v0.8"
                            i(class="bi bi-box-arrow-up-right ml-2")
                        }
                    }
                    li {
                        a(class="hover:text-orange-700", href="https://sycamore-rs.netlify.app/docs/v0.7/getting_started/installation") {
                            "v0.7"
                            i(class="bi bi-box-arrow-up-right ml-2")
                        }
                    }
                }
            }
        }
    }
}
