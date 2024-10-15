pub mod pages;
pub mod shell;

use self::shell::*;

use sycamore::prelude::*;

pub async fn get_static_paths() -> Vec<String> {
    let mut paths = vec![];

    paths.push("/index.html".to_string());
    paths.push("/404.html".to_string());

    paths
}

#[cfg_ssr]
#[tokio::main]
async fn main() {
    use std::path::PathBuf;
    use sycamore_router::Route;
    use tokio::fs;

    static PUBLIC_PATH: &str = "dist/.stage";

    let routes = get_static_paths().await;
    for route in routes {
        let path = PathBuf::from(PUBLIC_PATH).join(route.trim_start_matches('/'));

        eprintln!("Rendering `{route}` to `{}`", path.display());

        let html = sycamore::render_to_string_await_suspense(|| {
            view! {
                Shell {
                    sycamore_router::StaticRouter(route=Routes::default().match_path(&route), view=App)
                }
            }
        })
        .await;

        fs::create_dir_all(PUBLIC_PATH)
            .await
            .expect("failed to create public dir");
        fs::write(path, html)
            .await
            .expect("failed to write html file");
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
