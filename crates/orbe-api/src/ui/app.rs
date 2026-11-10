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
        // Root-relative paths, not `asset!()`: in SSR `asset!` leaks an absolute
        // filesystem path into the HTML (broken CSS + broken PWA manifest).
        // dx serves everything in `public/` from the web root, both dev and release.
        document::Title { "Orbe · Inventario" }
        document::Stylesheet { href: "/main.css" }
        document::Link { rel: "manifest", href: "/manifest.json" }
        document::Link { rel: "icon", href: "/icon-192.svg", r#type: "image/svg+xml" }
        meta { name: "theme-color", content: "#6B21A8" }
        meta { name: "description", content: "Orbe — CRM de inventario para pymes colombianas." }
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
