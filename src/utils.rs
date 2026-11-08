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

/// Observe a reactive set of elements and dispose the observer and JS closure
/// when targets change or the owning component is disposed.
#[cfg_not_ssr]
pub fn use_intersection_observer(
    targets: ReadSignal<Vec<web_sys::Element>>,
    options: web_sys::IntersectionObserverInit,
    callback: impl Fn(Vec<web_sys::IntersectionObserverEntry>) + 'static,
) {
    use std::rc::Rc;
    use sycamore::web::wasm_bindgen::closure::Closure;

    let callback = Rc::new(callback);
    create_effect(move || {
        let targets = targets.get_clone();
        if targets.is_empty() {
            return;
        }
        let callback = callback.clone();
        let closure = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
            callback(entries.iter().map(|entry| entry.unchecked_into()).collect());
        });
        // Scroll tracking still works if the browser cannot create an observer.
        if let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
            closure.as_ref().unchecked_ref(),
            &options,
        ) {
            for target in targets {
                observer.observe(&target);
            }
            on_cleanup(move || {
                observer.disconnect();
                drop(closure);
            });
        }
    });
}

/// Return the last heading above the activation line. Before the first heading,
/// nothing is active. At the bottom of a scrollable page, select the last one.
#[cfg(any(target_arch = "wasm32", test))]
fn active_heading_index(tops: &[f64], activation_line: f64, at_bottom: bool) -> Option<usize> {
    if at_bottom {
        tops.len().checked_sub(1)
    } else {
        tops.iter().rposition(|top| *top <= activation_line)
    }
}

/// Track headings independently from the outline's styling. IntersectionObserver
/// handles visibility/layout changes; a frame-throttled scroll listener also
/// covers large jumps, long sections, and short final sections.
#[cfg_not_ssr]
pub fn use_active_heading(
    headings: ReadSignal<Vec<web_sys::Element>>,
) -> ReadSignal<Option<String>> {
    use std::{cell::Cell, rc::Rc};
    use sycamore::web::wasm_bindgen::closure::Closure;

    let active = create_signal(None);
    let update = Rc::new(move || {
        let headings = headings.get_clone_untracked();
        let tops = headings
            .iter()
            .map(|heading| heading.get_bounding_client_rect().top())
            .collect::<Vec<_>>();
        let activation_line = document()
            .query_selector("header")
            .unwrap()
            .map_or(64.0, |header| {
                header.get_bounding_client_rect().bottom() + 16.0
            });
        let root = document().document_element().unwrap();
        let viewport = window().inner_height().unwrap().as_f64().unwrap();
        let height = f64::from(root.scroll_height());
        let at_bottom =
            height > viewport + 2.0 && window().scroll_y().unwrap() + viewport >= height - 2.0;
        let next = active_heading_index(&tops, activation_line, at_bottom)
            .map(|index| headings[index].id());
        if active.get_clone_untracked() != next {
            active.set(next);
        }
    });

    let options = web_sys::IntersectionObserverInit::new();
    options.set_root_margin("-64px 0px 0px 0px");
    let on_intersection = update.clone();
    use_intersection_observer(headings, options, move |_| on_intersection());

    create_effect(move || {
        let targets = headings.get_clone();
        if targets.is_empty() {
            active.set(None);
            return;
        }
        update();
        let pending = Rc::new(Cell::new(None));
        let frame_pending = pending.clone();
        let frame_update = update.clone();
        let frame = Rc::new(Closure::<dyn FnMut(f64)>::new(move |_| {
            frame_pending.set(None);
            frame_update();
        }));
        let event_pending = pending.clone();
        let event_frame = frame.clone();
        let event = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            if event_pending.get().is_none() {
                let id = window()
                    .request_animation_frame(event_frame.as_ref().as_ref().unchecked_ref())
                    .unwrap();
                event_pending.set(Some(id));
            }
        });
        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(true);
        for name in ["scroll", "resize", "hashchange"] {
            window()
                .add_event_listener_with_callback_and_add_event_listener_options(
                    name,
                    event.as_ref().unchecked_ref(),
                    &options,
                )
                .unwrap();
        }
        // Also sample the next frame, after initial hash scrolling/layout.
        pending.set(Some(
            window()
                .request_animation_frame(frame.as_ref().as_ref().unchecked_ref())
                .unwrap(),
        ));
        on_cleanup(move || {
            for name in ["scroll", "resize", "hashchange"] {
                window()
                    .remove_event_listener_with_callback(name, event.as_ref().unchecked_ref())
                    .unwrap();
            }
            if let Some(id) = pending.take() {
                window().cancel_animation_frame(id).unwrap();
            }
            drop(event);
            drop(frame);
        });
    });
    *active
}

/// Server rendering emits an unselected outline; browser tracking starts on load.
#[cfg_ssr]
pub fn use_headings_outline_callback() -> impl Fn(web_sys::Element) {
    |_| {}
}

/// Create an on_load callback for ServerOnly. It discovers outlined headings
/// inside that container and bridges the active signal to the SSR-only links.
#[cfg_not_ssr]
pub fn use_headings_outline_callback() -> impl Fn(web_sys::Element) {
    let content = create_signal(None::<web_sys::Element>);
    let pairs = create_memo(move || {
        let Some(root) = content.get_clone() else {
            return Vec::new();
        };
        let links = root.query_selector_all("[data-outline-heading]").unwrap();
        let headings = root
            .query_selector_all(
                ".prose h2[id], .prose h3[id], .prose h4[id], .prose h5[id], .prose h6[id]",
            )
            .unwrap();
        (0..headings.length())
            .filter_map(|i| {
                let heading = headings.item(i)?.unchecked_into::<web_sys::Element>();
                let link = (0..links.length()).find_map(|j| {
                    let link = links.item(j)?.unchecked_into::<web_sys::Element>();
                    (link.get_attribute("data-outline-heading").as_deref()
                        == Some(heading.id().as_str()))
                    .then_some(link)
                })?;
                Some((heading, link))
            })
            .collect::<Vec<_>>()
    });
    let headings = create_memo(move || {
        pairs
            .get_clone()
            .into_iter()
            .map(|(heading, _)| heading)
            .collect()
    });
    let active = use_active_heading(headings);
    create_effect(move || {
        let active = active.get_clone();
        for (heading, link) in pairs.get_clone() {
            let selected = active.as_deref() == Some(heading.id().as_str());
            for class in ["text-orange-700", "dark:text-orange-500"] {
                link.class_list()
                    .toggle_with_force(class, selected)
                    .unwrap();
            }
            if selected {
                link.set_attribute("aria-current", "location").unwrap();
            } else {
                link.remove_attribute("aria-current").unwrap();
            }
        }
    });
    move |root| content.set(Some(root))
}

#[cfg(test)]
mod tests {
    use super::active_heading_index;

    #[test]
    fn no_active_entry_before_the_first_heading_or_for_an_empty_outline() {
        assert_eq!(active_heading_index(&[], 64.0, false), None);
        assert_eq!(active_heading_index(&[], 64.0, true), None);
        assert_eq!(active_heading_index(&[100.0, 500.0], 64.0, false), None);
    }

    #[test]
    fn selects_the_last_heading_above_the_header_in_both_directions() {
        assert_eq!(active_heading_index(&[64.0, 500.0], 64.0, false), Some(0));
        // Stay on the preceding heading through a long section.
        assert_eq!(active_heading_index(&[-900.0, 500.0], 64.0, false), Some(0));
        assert_eq!(active_heading_index(&[-900.0, 64.0], 64.0, false), Some(1));
        // Moving back above the second heading reactivates the first.
        assert_eq!(active_heading_index(&[-500.0, 200.0], 64.0, false), Some(0));
    }

    #[test]
    fn handles_large_jumps_and_short_final_sections() {
        assert_eq!(
            active_heading_index(&[-1000.0, -500.0, -100.0, 900.0], 64.0, false),
            Some(2)
        );
        assert_eq!(
            active_heading_index(&[-1000.0, 200.0, 500.0], 64.0, true),
            Some(2)
        );
    }
}

#[cfg_ssr]
#[component(inline_props)]
pub fn HeadingsOutline(headings: Vec<mdsycx::OutlineHeading>, children: Children) -> View {
    let outline = headings
        .into_iter()
        .map(|heading| {
            let href = format!("#{}", heading.id);
            let class = "hover:text-orange-700 dark:hover:text-orange-500 transition-colors";
            match heading.level {
                1 => view! {},
                2 => view! {
                    li(class="mt-2 font-semibold") {
                        a(href=href, class=class, data-outline-heading=heading.id) { (heading.text) }
                    }
                },
                _n => view! {
                    li(class="mt-0.5 ml-4") {
                        a(href=href, class=class, data-outline-heading=heading.id) { (heading.text) }
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
