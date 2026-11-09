use sycamore::prelude::*;

use crate::{
    docs::{DocPage, DocsSource},
    server_component::ServerOnly,
};

#[component(inline_props)]
pub fn Book(source: DocsSource, page: DocPage) -> View {
    let id = source.component_id(&page);
    let create_headings_outline = crate::utils::use_headings_outline_callback();
    view! {
        sycamore::web::Suspense {
            ServerOnly(id=id, on_load=move |element| {
                crate::utils::prism_highlight_all();
                create_headings_outline(element);
            }) {
                BookBody(source=source, page=page)
            }
        }
    }
}

#[cfg_not_ssr]
#[component(inline_props)]
fn BookBody(source: DocsSource, page: DocPage) -> View {
    let _ = (source, page);
    unreachable!()
}

#[cfg_ssr]
#[component(inline_props)]
fn BookBody(source: DocsSource, page: DocPage) -> View {
    use crate::server_component::ServerTitle;

    let book = crate::content::book(source);
    let Some(parsed) = book.docs.get(&page).cloned() else {
        return crate::pages::not_found::NotFound();
    };
    let github_edit_link = source.edit_url(&page);
    let title = format!("{} — {}", parsed.front_matter.title, source.label());
    let versions = VersionLinks(source, &page);
    let latest_url = crate::content::book(DocsSource::latest()).switch_url(&page);

    view! {
        ServerTitle(title=title)
        div(class="flex flex-row w-full") {
            div(class="flex-none self-start w-60 pt-8 pb-5 px-6 space-y-2 text-sm sticky top-14 max-h-[calc(100vh-3.5rem)] overflow-y-auto block -ml-60 sm:ml-0") {
                BookIndex(source=source, page=page)
            }
            div(class="flex flex-1 min-w-0 items-start justify-center gap-0 sm:gap-4") {
                div(class="grow-0 min-w-0 px-2 pt-5 pb-10 prose md:w-[80ch] dark:prose-invert") {
                    // Keep version navigation visible on mobile as well as desktop.
                    nav(aria-label="Documentation versions", class="not-prose mb-6 border-b border-neutral-300 dark:border-neutral-700 pb-3 text-sm") {
                        p(class="font-semibold mb-2") { "Documentation: " (source.label()) }
                        ul(class="flex flex-wrap gap-x-4 gap-y-2") { (versions) }
                    }
                    (if !source.is_latest() {
                        let latest_url = latest_url.clone();
                        view! {
                            p(class="not-prose mb-5 rounded bg-amber-100 dark:bg-amber-950 p-3 text-sm") {
                                "You are reading release documentation. "
                                a(href=latest_url.clone(), class="underline") { "Read the latest development docs." }
                            }
                        }
                    } else { view! {} })
                    mdsycx::MDSycX(body=parsed.body)
                    div(class="mt-6 mr-2 text-right") {
                        a(class="text-sm", href=github_edit_link) {
                            i(class="bi bi-pencil mr-2")
                            "Edit this page on GitHub"
                        }
                    }
                }
                crate::utils::HeadingsOutline(headings=parsed.headings) {}
            }
        }
    }
}

#[cfg_ssr]
#[allow(non_snake_case)]
fn VersionLinks(source: DocsSource, page: &DocPage) -> View {
    let hosted = crate::content::BOOKS.iter().map(|book| {
        let href = book.switch_url(page);
        let label = book.source.label();
        let current = book.source == source;
        view! {
            li {
                a(href=href, aria-current=if current { "page" } else { "false" },
                    class=if current { "font-semibold text-orange-700 dark:text-orange-500" } else { "hover:text-orange-700 dark:hover:text-orange-500" }) {
                    (label)
                }
            }
        }
    }).collect::<Vec<_>>();
    view! {
        (hosted)
        li {
            a(class="hover:text-orange-700 dark:hover:text-orange-500", href="https://sycamore-rs.netlify.app/docs/v0.8/getting_started/installation") {
                "v0.8" i(class="bi bi-box-arrow-up-right ml-2")
            }
        }
        li {
            a(class="hover:text-orange-700 dark:hover:text-orange-500", href="https://sycamore-rs.netlify.app/docs/v0.7/getting_started/installation") {
                "v0.7" i(class="bi bi-box-arrow-up-right ml-2")
            }
        }
    }
}

#[cfg_ssr]
#[component(inline_props)]
fn BookIndex(source: DocsSource, page: DocPage) -> View {
    let current_href = source.url(&page);
    crate::content::book(source)
        .index
        .sections
        .iter()
        .map(|section| {
            let items = section
                .subsections
                .iter()
                .map(|item| {
                    let href = source.url(&item.path);
                    let class = if href == current_href {
                        "text-orange-700 dark:text-orange-500"
                    } else {
                        "hover:text-orange-700 dark:hover:text-orange-500 transition-colors"
                    };
                    let class = format!("block rounded-md -mx-2 px-2 py-0.5 hover:bg-orange-500/[0.08] dark:hover:bg-orange-500/[0.12] transition-colors {class}");
                    let title = item.title.clone();
                    view! { li(class="mt-0.5") { a(href=href, class=class) { (title) } } }
                })
                .collect::<Vec<_>>();
            let href = source.url(&section.path);
            let class = if href == current_href {
                "font-semibold text-orange-700 dark:text-orange-500"
            } else {
                "font-semibold hover:text-orange-700 dark:hover:text-orange-500 transition-colors"
            };
            let class = format!("block rounded-md -mx-2 px-2 py-0.5 hover:bg-orange-500/[0.08] dark:hover:bg-orange-500/[0.12] transition-colors {class}");
            let title = section.title.clone();
            view! {
                div {
                    a(class=class, href=href) { (title) }
                    ul(class="ml-4") { (items) }
                }
            }
        })
        .collect::<Vec<_>>()
        .into()
}
