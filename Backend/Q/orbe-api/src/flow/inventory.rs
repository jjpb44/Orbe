//! Inventory use cases.
//!
//! Every function takes the ports it needs as `&dyn` trait objects so the
//! caller (wire, server fn, test) controls which adapter to inject.

use crate::core::{DomainError, MovementKind, NewProduct, Product, StockMovement};
use crate::gate::{Clock, MovementLog, ProductRepo, RepoError};

/// Domain-level error spanning validation (core) and persistence (gate).
/// Wire/UI map each variant to the right HTTP status / ServerFnError.
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repo(#[from] RepoError),
}

pub async fn create_product(
    repo: &dyn ProductRepo,
    new: NewProduct,
) -> Result<Product, FlowError> {
    new.validate()?;
    Ok(repo.create(new).await?)
}

pub async fn list_products(repo: &dyn ProductRepo) -> Result<Vec<Product>, FlowError> {
    Ok(repo.list().await?)
}

pub async fn get_product(repo: &dyn ProductRepo, id: &str) -> Result<Product, FlowError> {
    Ok(repo.get(id).await?)
}

pub async fn update_product(
    repo: &dyn ProductRepo,
    id: &str,
    patch: NewProduct,
) -> Result<Product, FlowError> {
    patch.validate()?;
    Ok(repo.update(id, patch).await?)
}

pub async fn delete_product(repo: &dyn ProductRepo, id: &str) -> Result<(), FlowError> {
    Ok(repo.delete(id).await?)
}

/// Adjust stock and persist the movement in one go.
/// `kind` must be coherent with the sign of `qty` (V5 enforced in core).
pub async fn adjust_stock(
    repo: &dyn ProductRepo,
    product_id: &str,
    kind: MovementKind,
    qty: i32,
    reason: &str,
) -> Result<(Product, StockMovement), FlowError> {
    use crate::core::NewMovement;
    let m = NewMovement { product_id: product_id.into(), kind, qty, reason: reason.into() };
    m.validate()?;
    Ok(repo.adjust_stock(product_id, kind, qty, reason).await?)
}

/// Convenience for server fns that need to stamp movement `at` from a clock.
#[allow(dead_code)]
pub async fn adjust_stock_with_clock(
    repo: &dyn ProductRepo,
    log: &dyn MovementLog,
    _clock: &dyn Clock,
    product_id: &str,
    kind: MovementKind,
    qty: i32,
    reason: &str,
) -> Result<(Product, StockMovement), FlowError> {
    let m = crate::core::NewMovement {
        product_id: product_id.into(),
        kind,
        qty,
        reason: reason.into(),
    };
    m.validate()?;
    let (product, movement) = repo.adjust_stock(product_id, kind, qty, reason).await?;
    // Movement's `at` is set by the adapter; we just ensure log is touched so
    // the dual-port dependency is explicit (future: derive log here for tests).
    let _ = log;
    Ok((product, movement))
}
