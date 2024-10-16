pub mod layout;
pub mod pages;
pub mod server_component;
pub mod shell;

use self::shell::*;

use sycamore::prelude::*;

pub fn get_posts() -> Vec<String> {
    use std::fs;

    fs::read_dir("sycamore/docs/posts/")
        .expect("failed to read posts directory")
        .map(|entry| {
            entry
                .expect("failed to read post entry")
                .file_name()
                .into_string()
                .expect("failed to convert post entry to string")
                .trim_end_matches(".md")
                .to_string()
        })
        .collect()
}

pub fn get_static_paths() -> Vec<String> {
    let mut paths = vec![];

    paths.push("/index.html".to_string());
    paths.push("/404.html".to_string());

    let posts = get_posts();
    for post in posts {
        paths.push(format!("/post/{}.html", post));
    }

    paths
}

#[cfg_ssr]
fn main() {
    use std::fs;
    use std::path::PathBuf;
    use sycamore_router::Route;

    static PUBLIC_PATH: &str = "dist/.stage";

    let routes = get_static_paths();
    for route in routes {
        let path = PathBuf::from(PUBLIC_PATH).join(route.trim_start_matches('/'));

        eprintln!("Rendering `{route}` to `{}`", path.display());

        let html = sycamore::render_to_string(|| {
            view! {
                Shell {
                    sycamore_router::StaticRouter(route=Routes::default().match_path(&route), view=App)
                }
            }
        });

        let dir = path.parent().expect("failed to get parent dir");
        fs::create_dir_all(dir).expect("failed to create parent dir");
        fs::write(path, format!("<!DOCTYPE html>{html}")).expect("failed to write html file");
    }

    let mut server_components = server_component::SERVER_COMPONENTS.lock().unwrap();
    for (id, html) in server_components.drain() {
        let path = PathBuf::from(PUBLIC_PATH)
            .join("server_components")
            .join(format!("{}.html", id));

        eprintln!("Rendering server component `{id}` to `{}`", path.display());

        let dir = path.parent().expect("failed to get parent dir");
        fs::create_dir_all(dir).expect("failed to create parent dir");
        fs::write(path, html).expect("failed to write html file");
    }
}

#[cfg_not_ssr]
fn main() {
    console_error_panic_hook::set_once();

    sycamore::hydrate_to(
        || {
            view! {
                Shell {
                    sycamore_router::Router(integration=sycamore_router::HistoryIntegration::new(), view=App)
                }
            }
        },
        &document(),
    );
}
