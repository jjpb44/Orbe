//! Orbe — Phase 1 scaffold entry point.
//! Single Rust crate serves Dioxus UI (WASM) + REST API (Axum) from one process.
//! V7 invariant: FE reaches data ONLY via `#[server]` fns in this file.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use {axum::Router, dioxus::cli_config::fullstack_address_or_localhost};

#[cfg(feature = "server")]
use dioxus_server::{DioxusRouterExt, ServeConfig};

// ── Core (Phase 2 will move this into src/core/) ──────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub name: String,
}

// ── Flow (Phase 2 will move this into src/flow/) ──────────────────────────

#[server]
async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    // Phase 1: stub returns empty. Phase 2: hit gate adapter.
    Ok(vec![])
}

// ── UI ────────────────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let products = use_resource(|| async { list_products().await });

    rsx! {
        document::Stylesheet { href: asset!("/public/main.css") }
        div { class: "o-root",
            h1 { class: "o-title", "Orbe" }
            p { class: "o-muted", "Phase 1 scaffold — Dioxus fullstack on Axum." }
            div { class: "o-card",
                h2 { class: "o-card-title", "Inventario" }
                match &*products.read() {
                    Some(Ok(items)) => rsx! {
                        p { "Productos cargados: {items.len()}" }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "o-danger", "Error: {e}" }
                    },
                    None => rsx! {
                        p { class: "o-muted", "Cargando…" }
                    },
                }
            }
        }
    }
}

// ── Entry points ──────────────────────────────────────────────────────────

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let address = fullstack_address_or_localhost();
    let router = Router::new().serve_dioxus_application(ServeConfig::new(), App);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("orbe-api listening on http://{address}");
    axum::serve(listener, router).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
