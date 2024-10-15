pub mod pages;
pub mod shell;

use self::shell::*;

use sycamore::prelude::*;
use sycamore_router::Route;

static ROUTES: &[&str] = &["/index.html", "/404.html"];
static PUBLIC_PATH: &str = "dist/.stage";

#[cfg_ssr]
#[tokio::main]
async fn main() {
    use std::path::PathBuf;
    use tokio::fs;

    for route in ROUTES {
        let path = PathBuf::from(PUBLIC_PATH).join(route.trim_start_matches('/'));

        eprintln!("Rendering `{route}` to `{}`", path.display());

        let html = sycamore::render_to_string_await_suspense(|| {
            view! {
                Shell {
                    sycamore_router::StaticRouter(route=Routes::default().match_path(route), view=App)
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
