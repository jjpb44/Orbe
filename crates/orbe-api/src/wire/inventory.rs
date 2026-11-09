//! Inventory REST handlers.
//!
//! V1 ⊥: this module imports `crate::flow`, never `crate::gate::surreal`.
//! V6: error mapping happens here — `FlowError::Domain` → 400/422,
//! `FlowError::Repo(NotFound)` → 404, `UniqueViolation` → 409, other → 500.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::core::{MovementKind, NewProduct, Product, StockMovement};
use crate::flow::{self, FlowError};
use crate::gate::ports::MovementLog;
use crate::gate::Runtime;

/// Shared state for every inventory handler.
#[derive(Clone)]
pub struct InventoryState {
    pub runtime: Runtime,
}

pub fn router(state: InventoryState) -> Router {
    Router::new()
        .route("/products", get(list).post(create))
        .route("/products/{id}", get(get_one).put(update).delete(remove))
        .route("/products/{id}/stock", post(adjust))
        .route("/products/{id}/movements", get(movements))
        .with_state(state)
}

// ── Handlers ─────────────────────────────────────────────────────────────

async fn list(State(s): State<InventoryState>) -> Result<Json<Vec<Product>>, ApiError> {
    Ok(Json(
        flow::list_products(s.runtime.products.as_ref()).await?,
    ))
}

async fn get_one(
    State(s): State<InventoryState>,
    Path(id): Path<String>,
) -> Result<Json<Product>, ApiError> {
    Ok(Json(
        flow::get_product(s.runtime.products.as_ref(), &id).await?,
    ))
}

#[derive(Deserialize)]
struct CreateBody {
    sku: String,
    name: String,
    price_cents: i64,
    stock: i32,
    min_stock: i32,
}

async fn create(
    State(s): State<InventoryState>,
    Json(b): Json<CreateBody>,
) -> Result<Response, ApiError> {
    let new = NewProduct {
        sku: b.sku,
        name: b.name,
        price_cents: b.price_cents,
        stock: b.stock,
        min_stock: b.min_stock,
    };
    let product = flow::create_product(s.runtime.products.as_ref(), new).await?;
    Ok((StatusCode::CREATED, Json(product)).into_response())
}

async fn update(
    State(s): State<InventoryState>,
    Path(id): Path<String>,
    Json(b): Json<UpdateBody>,
) -> Result<Json<Product>, ApiError> {
    // PUT replaces editable fields (name, price, min_stock). Stock is managed
    // via the /stock endpoint, so preserve the existing stock value.
    let existing = flow::get_product(s.runtime.products.as_ref(), &id).await?;
    let patch = NewProduct {
        sku: id.clone(),
        name: b.name,
        price_cents: b.price_cents,
        stock: existing.stock,
        min_stock: b.min_stock,
    };
    Ok(Json(
        flow::update_product(s.runtime.products.as_ref(), &id, patch).await?,
    ))
}

#[derive(Deserialize)]
struct UpdateBody {
    name: String,
    price_cents: i64,
    min_stock: i32,
}

async fn remove(
    State(s): State<InventoryState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    flow::delete_product(s.runtime.products.as_ref(), &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct AdjustBody {
    kind: MovementKind,
    qty: i32,
    reason: String,
}

#[derive(serde::Serialize)]
struct AdjustResponse {
    product: Product,
    movement: StockMovement,
}

async fn adjust(
    State(s): State<InventoryState>,
    Path(id): Path<String>,
    Json(b): Json<AdjustBody>,
) -> Result<Json<AdjustResponse>, ApiError> {
    let (product, movement) =
        flow::adjust_stock(s.runtime.products.as_ref(), &id, b.kind, b.qty, &b.reason).await?;
    Ok(Json(AdjustResponse { product, movement }))
}

async fn movements(
    State(s): State<InventoryState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<StockMovement>>, ApiError> {
    Ok(Json(
        MovementLog::list_for_product(s.runtime.movements.as_ref(), &id)
            .await
            .map_err(FlowError::from)?,
    ))
}

// ── Error mapping (V6) ───────────────────────────────────────────────────

pub struct ApiError(FlowError);

impl From<FlowError> for ApiError {
    fn from(e: FlowError) -> Self {
        Self(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match &self.0 {
            FlowError::Domain(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.0.to_string()),
            FlowError::Repo(crate::gate::RepoError::NotFound(id)) => {
                (StatusCode::NOT_FOUND, format!("not found: {id}"))
            }
            FlowError::Repo(crate::gate::RepoError::UniqueViolation(msg)) => {
                (StatusCode::CONFLICT, format!("unique: {msg}"))
            }
            FlowError::Repo(crate::gate::RepoError::Backend(msg)) => {
                tracing::error!(error = %msg, "repo backend error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error".into())
            }
        };
        (status, body).into_response()
    }
}
