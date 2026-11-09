//! Dioxus App component. Router + global styles + service-worker registration.

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    // PWA: register the shell cache once the app is running in a browser.
    // `document::eval` is a no-op on the server (SSR), so no cfg gymnastics here.
    use_effect(|| {
        spawn(async {
            let _ = document::eval(
                r#"
                if ("serviceWorker" in navigator && location.protocol.startsWith("http")) {
                    navigator.serviceWorker.register("/sw.js").catch(() => {});
                }
                "#,
            )
            .await;
        });
    });

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
