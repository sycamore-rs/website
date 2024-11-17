use sycamore::{prelude::*, web::cfg_ssr_item};

cfg_ssr_item!(
    pub mod content;
);
cfg_ssr_item! {
    pub mod api_stats;
}
pub mod layout;
pub mod pages;
pub mod server_component;
pub mod shell;
pub mod utils;

use self::shell::*;

#[cfg_ssr]
pub static DOCS_DIR: &str = match option_env!("DOCS_DIR") {
    Some(path) => path,
    None => "sycamore/docs/",
};

#[cfg_ssr]
#[tokio::main]
async fn main() {
    use std::{fs, path::PathBuf};

    static PUBLIC_PATH: &str = "dist/.stage";

    for (route, path) in content::get_static_paths() {
        let path = path.trim_start_matches('/');
        let path = PathBuf::from(PUBLIC_PATH).join(path);

        eprintln!("Rendering `{}`", path.display());

        let html = sycamore::render_to_string_await_suspense(|| {
            view! {
                Shell {
                    sycamore_router::StaticRouter(route=route, view=App)
                }
            }
        })
        .await;

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

    eprintln!("Generating sitemap.xml");
    let sitemap = content::generate_sitemap_xml().expect("failed to generate sitemap");
    fs::write(PathBuf::from(PUBLIC_PATH).join("sitemap.xml"), sitemap)
        .expect("failed to write sitemap.xml");
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
