use dioxus::prelude::*;

use crate::components::{console::AdminConsole, shell::AppShell};

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
pub fn App() -> Element {
    let _app_stylesheet = asset!(
        "/assets/app.css",
        AssetOptions::css().with_static_head(true)
    );
    let _font_assets = asset!("/assets/fonts", AssetOptions::folder());
    let _style_assets = asset!("/assets/styles", AssetOptions::folder());

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! { AdminConsole {} }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let route = format!("/{}", segments.join("/"));

    rsx! {
        AppShell {
            compact: false,
            article { class: "panel",
                div { class: "section-head",
                    div {
                        h2 { "页面未找到" }
                    }
                    span { class: "tag", "{route}" }
                }
            }
        }
    }
}
