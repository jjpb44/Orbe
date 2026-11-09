//! Dioxus `#[server]` functions.
//!
//! Sole FE→data path (V7). Each function takes the runtime by HTTP request
//! extension injected in `main.rs` via `DioxusRouterExt::serve_dioxus_application`.
//!
//! For Phase 2 we lazy-init an in-process SurrealDB handle on the server side
//! (via `tokio::sync::OnceCell`) so server fns can be called concurrently.
//! Phase 3 should switch to a request-scoped runtime passed via Axum `State`.

use dioxus::prelude::*;

use crate::core::{MovementKind, NewProduct, Product, StockMovement};
use crate::flow::{self, FlowError};
use crate::gate::MovementLog;

#[cfg(feature = "server")]
use crate::gate::Runtime;

#[cfg(feature = "server")]
static RUNTIME: tokio::sync::OnceCell<Runtime> = tokio::sync::OnceCell::const_new();

#[cfg(feature = "server")]
async fn runtime() -> Result<&'static Runtime, ServerFnError> {
    RUNTIME
        .get_or_try_init(|| async {
            Runtime::boot()
                .await
                .map_err(|e| ServerFnError::new(e.to_string()))
        })
        .await
}

fn map_err(e: FlowError) -> ServerFnError {
    ServerFnError::new(e.to_string())
}

// Each #[get/#[post]] here generates a server endpoint AND a client-side
// free function with the same name. Components call the free function
// directly (it returns Result<_, ServerFnError> and dispatches via fetch).

#[get("/api/products")]
pub async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    let rt = runtime().await?;
    flow::list_products(rt.products.as_ref())
        .await
        .map_err(map_err)
}

#[get("/api/products/{id}")]
pub async fn get_product(id: String) -> Result<Product, ServerFnError> {
    let rt = runtime().await?;
    flow::get_product(rt.products.as_ref(), &id)
        .await
        .map_err(map_err)
}

#[post("/api/products")]
pub async fn create_product(new: NewProduct) -> Result<Product, ServerFnError> {
    let rt = runtime().await?;
    flow::create_product(rt.products.as_ref(), new)
        .await
        .map_err(map_err)
}

#[put("/api/products/{id}")]
pub async fn update_product(id: String, patch: NewProduct) -> Result<Product, ServerFnError> {
    let rt = runtime().await?;
    flow::update_product(rt.products.as_ref(), &id, patch)
        .await
        .map_err(map_err)
}

#[delete("/api/products/{id}")]
pub async fn delete_product(id: String) -> Result<(), ServerFnError> {
    let rt = runtime().await?;
    flow::delete_product(rt.products.as_ref(), &id)
        .await
        .map_err(map_err)
}

#[post("/api/products/{id}/stock")]
pub async fn adjust_stock(
    id: String,
    kind: MovementKind,
    qty: i32,
    reason: String,
) -> Result<(Product, StockMovement), ServerFnError> {
    let rt = runtime().await?;
    flow::adjust_stock(rt.products.as_ref(), &id, kind, qty, &reason)
        .await
        .map_err(map_err)
}

#[get("/api/products/{id}/movements")]
pub async fn list_movements(id: String) -> Result<Vec<StockMovement>, ServerFnError> {
    let rt = runtime().await?;
    MovementLog::list_for_product(rt.movements.as_ref(), &id)
        .await
        .map_err(FlowError::from)
        .map_err(map_err)
}
