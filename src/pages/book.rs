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
    use crate::server_component::ServerTitle;

    let parsed = crate::content::DOCS
        .get(&(section.clone(), doc.clone()))
        .expect("doc not found")
        .clone();

    let title = match parsed.front_matter.title {
        Some(title) => title,
        None => parsed
            .headings
            .first()
            .map(|heading| heading.text.clone())
            .unwrap_or("Title Missing".to_string()),
    };

    let github_edit_link = match &doc {
        Some(doc) => format!("https://github.com/sycamore-rs/sycamore/edit/main/docs/next/{section}/{doc}.md"),
        None => format!("https://github.com/sycamore-rs/sycamore/edit/main/docs/next/{section}.md"),
    };

    view! {
        ServerTitle(title=title)
        div(class="flex flex-row gap-0 sm:gap-4 w-full justify-center") {
            div(class="flex-none w-44 pt-8 pb-5 px-2 space-y-2 text-sm sticky top-12 max-h-[calc(100vh-3rem)] overflow-y-auto block -ml-44 sm:ml-0") {
                BookSidebar(section=section, doc=doc)
            }
            div(class="grow-0 min-w-0 px-2 pt-5 pb-10 prose md:w-[80ch]") {
                mdsycx::MDSycX(body=parsed.body)

                div(class="mt-6 mr-2 text-right") {
                    a(class="text-sm", href=github_edit_link) {
                        i(class="bi bi-pencil mr-2")
                        "Edit this page on GitHub"
                    }
                }
            }
            HeadingsOutline(headings=parsed.headings)
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn BookSidebar(section: String, #[prop(!optional)] doc: Option<String>) -> View {
    let sidebar = crate::content::BOOK_SIDEBAR.clone();
    
    let current_href = match &doc {
        Some(doc) => format!("/book/{}/{}", section, doc),
        None => format!("/book/{}", section),
    };

    sidebar
        .sections
        .into_iter()
        .map(|section| {
            let items = section
                .items
                .into_iter()
                .map(|item| {
                    let href = format!("/book/{}", item.href);
                    let class = if href == current_href {
                        "text-orange-700"
                    } else {
                        "hover:text-orange-700 transition-colors"
                    };
                    view! {
                        li(class="mt-0.5") {
                            a(href=href, class=class) { (item.name) }
                        }
                    }
                })
                .collect::<Vec<_>>();
            view! {
                div {
                    a(class="font-semibold") { (section.title.clone()) }
                    ul(class="ml-4") {
                        (items)
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .into()
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
                    li(class="mt-2 font-semibold") {
                        a(href=href, class=class) { (heading.text) }
                    }
                },
                _n => view! {
                    li(class="mt-0.5 ml-4") {
                        a(href=href, class=class) { (heading.text) }
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
                p(class="-mb-1 uppercase text-xs") { "On this page" }
                ul {
                    (outline)
                }
            }
            div(class="mt-4") {
                p(class="uppercase text-xs") { "Versions" }
                ul(class="font-semibold mt-1") {
                    li {
                        "v0.9" span(class="font-normal") { " (current)" }
                    }
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
