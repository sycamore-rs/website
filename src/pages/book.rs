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
        div(class="flex flex-row gap-4 w-full") {
            BookSidebar {}
            div(class="flex-1 px-2 pt-0 pb-10 sm:pt-5 prose prose-gray") {
                mdsycx::MDSycX(body=parsed.body)
            }
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
                    a(class="font-semibold") { (section.title.clone()) }
                    ul(class="pl-2") {
                        (items)
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        div(
            class="flex-none w-44 pt-8 pb-5 pr-2 text-sm sticky top-12 max-h-[calc(100vh-3rem)] overflow-y-auto
                hidden sm:block"
        ) {
            (view)
        }
    }
}
