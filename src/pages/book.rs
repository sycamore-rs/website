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
pub fn BookBody(section: String, #[prop(!optional)] doc: Option<String>) -> View {
    let _ = section;
    let _ = doc;
    unreachable!()
}

#[cfg_ssr]
#[component(inline_props)]
pub fn BookBody(section: String, #[prop(!optional)] doc: Option<String>) -> View {
    let parsed = crate::content::DOCS
        .get(&(section.clone(), doc.clone()))
        .expect("doc not found")
        .clone();

    view! {
        div(class="mx-auto px-2 sm:px-0 pt-0 sm:pt-5 prose prose-gray md:prose-lg") {
            mdsycx::MDSycX(body=parsed.body)
        }
    }
}
