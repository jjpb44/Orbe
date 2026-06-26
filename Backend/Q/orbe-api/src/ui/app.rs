//! Dioxus App component. Router + global styles.

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/public/main.css") }
        document::Link { rel: "manifest", href: asset!("/public/manifest.json") }
        meta { name: "theme-color", content: "#6B21A8" }
        div { class: "o-shell",
            header { class: "o-topbar",
                h1 { class: "o-topbar__title", "Orbe" }
                span { class: "o-topbar__sub", "Inventario" }
            }
            main { class: "o-main",
                Router::<crate::ui::inventory::Route> {}
            }
        }
    }
}
