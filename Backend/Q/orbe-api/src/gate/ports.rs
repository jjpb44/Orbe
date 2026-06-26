//! Gate · Ports (hexagonal traits).
//!
//! Flow use cases depend on these traits, not on SurrealDB. The `surreal`
//! submodule provides the concrete adapter.
//!
//! V1 invariant: Wire (REST) and Dioxus UI never reach into gate directly
//! for entity storage — they call flow use cases that hold a `&dyn ProductRepo`.

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::core::{MovementKind, NewMovement, NewProduct, Product, StockMovement};

/// Domain error from a port call. Surreal-specific errors map to one of these.
#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unique constraint violated: {0}")]
    UniqueViolation(String),
    #[error("backend error: {0}")]
    Backend(String),
}

/// Persistence port for products.
#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn create(&self, p: NewProduct) -> Result<Product, RepoError>;
    async fn list(&self) -> Result<Vec<Product>, RepoError>;
    async fn get(&self, id: &str) -> Result<Product, RepoError>;
    async fn update(&self, id: &str, patch: NewProduct) -> Result<Product, RepoError>;
    async fn delete(&self, id: &str) -> Result<(), RepoError>;
    /// Atomically adjust stock by `delta` (signed). Returns the new product.
    /// Implementation MUST persist a `StockMovement` row alongside the update.
    async fn adjust_stock(
        &self,
        product_id: &str,
        kind: MovementKind,
        qty: i32,
        reason: &str,
    ) -> Result<(Product, StockMovement), RepoError>;
}

/// Persistence port for stock movements.
#[async_trait]
pub trait MovementLog: Send + Sync {
    async fn append(&self, m: NewMovement) -> Result<StockMovement, RepoError>;
    async fn list_for_product(&self, product_id: &str) -> Result<Vec<StockMovement>, RepoError>;
}

/// Wall-clock abstraction so use cases stay deterministic in tests.
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
