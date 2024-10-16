use sycamore::web::{js_sys, wasm_bindgen::JsCast};

/// Call `Prism.highlightAll()` to highlight all code blocks.
pub fn prism_highlight_all() -> Option<()> {
    let prism = js_sys::Reflect::get(&js_sys::global(), &"Prism".into()).ok()?;
    let highlight_all = js_sys::Reflect::get(&prism, &"highlightAll".into())
        .ok()?
        .unchecked_into::<js_sys::Function>();
    highlight_all.call0(&js_sys::global()).ok()?;
    Some(())
}
