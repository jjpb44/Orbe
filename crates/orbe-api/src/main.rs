//! Orbe — single Rust crate serving Dioxus UI (WASM) + REST API (Axum).
//!
//! Phase 2 entry point. Both surfaces share one Axum process:
//!   - `/`                    → Dioxus SSR + hydration
//!   - `/main.css`, `/sw.js`, `/manifest.json` → public/ assets (served verbatim by Dioxus)
//!   - `/api/v1/inventory/*`  → REST handlers (crate::wire::inventory)
//!   - `/api/products[/...]`  → `#[server]` fns (crate::ui::server_fns)
//!
//! V7: server fns are the only path the FE uses to reach data.

#[cfg(feature = "server")]
use dioxus::prelude::*;

#[cfg(feature = "server")]
use {
    axum::Router,
    dioxus::cli_config::fullstack_address_or_localhost,
    dioxus_server::{DioxusRouterExt, ServeConfig},
    tracing_subscriber::EnvFilter,
};

#[cfg(feature = "server")]
use orbe_api::{gate::Runtime, ui::App, wire};

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,orbe_api=debug")),
        )
        .init();

    let runtime = Runtime::boot().await.expect("runtime boot");

    let inventory_state = wire::inventory::InventoryState {
        runtime: runtime.clone(),
    };
    let address = fullstack_address_or_localhost();

    let router = Router::new()
        .serve_dioxus_application(serve_config(), App)
        .nest(
            "/api/v1/inventory",
            wire::inventory::router(inventory_state),
        )
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(address).await.expect("bind");
    tracing::info!(%address, "orbe-api listening");
    axum::serve(listener, router).await.expect("serve");
}

#[cfg(not(feature = "server"))]
fn main() {
    use dioxus::launch;
    use orbe_api::ui::App;
    launch(App);
}

/// Build the SSR index.
///
/// `ServeConfig::new()` falls back to a script-less template when it cannot find
/// an `index.html` next to the binary — the page renders, but the WASM client
/// never loads, so `use_resource` stays on "Cargando…" forever. We therefore
/// make sure an index exists before building the config:
///
/// 1. release bundles ship an `index.html` (hashed asset names) → used as-is;
/// 2. `dx serve` has none next to the binary → we write one pointing at the dev
///    client bundle (`/wasm/<package>.js`), served by the CLI dev server.
#[cfg(feature = "server")]
fn serve_config() -> ServeConfig {
    let public = public_dir();
    let index = public.join("index.html");

    if !index.exists() {
        if let Some(parent) = public.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match std::fs::write(&index, dev_index_html()) {
            Ok(()) => tracing::info!(path = %index.display(), "dev index.html written"),
            Err(e) => tracing::warn!(error = %e, "could not write dev index.html"),
        }
    }

    ServeConfig::new()
}

/// Directory the server serves static assets from: the CLI's public dir, or
/// `public/` next to the binary.
#[cfg(feature = "server")]
fn public_dir() -> std::path::PathBuf {
    std::env::var("DIOXUS_PUBLIC_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|dir| dir.join("public")))
                .unwrap_or_else(|| std::path::PathBuf::from("public"))
        })
}

/// Minimal HTML with the dev client bundle — the shape `dx serve` expects.
#[cfg(feature = "server")]
fn dev_index_html() -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="es">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Orbe</title>
  </head>
  <body>
    <div id="main"></div>
    <script type="module" async src="/wasm/{}.js"></script>
  </body>
</html>"#,
        env!("CARGO_PKG_NAME")
    )
}
