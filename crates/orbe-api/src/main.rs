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
        .serve_dioxus_application(ServeConfig::new(), App)
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
