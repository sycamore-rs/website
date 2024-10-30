use sycamore::{
    prelude::*,
    web::{js_sys, wasm_bindgen::JsCast},
};

/// Call `Prism.highlightAll()` to highlight all code blocks.
pub fn prism_highlight_all() -> Option<()> {
    let prism = js_sys::Reflect::get(&js_sys::global(), &"Prism".into()).ok()?;
    let highlight_all = js_sys::Reflect::get(&prism, &"highlightAll".into())
        .ok()?
        .unchecked_into::<js_sys::Function>();
    highlight_all.call0(&js_sys::global()).ok()?;
    Some(())
}

#[cfg_ssr]
#[component(inline_props)]
pub fn HeadingsOutline(headings: Vec<mdsycx::OutlineHeading>, children: Children) -> View {
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
            (children)
        }
    }
}
