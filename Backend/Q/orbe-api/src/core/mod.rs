//! Orbe · Core domain types.
//!
//! Pure Rust, no I/O, no framework deps. These are the entities that flow
//! through every layer (gate, flow, wire, ui). `id` is always a non-empty
//! String — gate layer converts to/from SurrealDB `Thing`.
//!
//! V2: ∀ Product → sku non-empty, price ≥ 0, stock ≥ 0
//! V3: ∀ Category → slug non-empty, lowercase
//! V5: ∀ StockMovement → qty ≠ 0

pub mod category;
pub mod movement;
pub mod product;
pub mod supplier;

pub use category::{Category, NewCategory};
pub use movement::{MovementKind, NewMovement, StockMovement};
pub use product::{NewProduct, Product};
pub use supplier::{NewSupplier, Supplier};

/// Domain error returned by entity constructors when invariants fail.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("sku must not be empty")]
    EmptySku,
    #[error("name must not be empty")]
    EmptyName,
    #[error("slug must not be empty")]
    EmptySlug,
    #[error("slug must be lowercase: {0}")]
    SlugNotLowercase(String),
    #[error("price must be >= 0")]
    NegativePrice,
    #[error("stock must be >= 0")]
    NegativeStock,
    #[error("movement quantity must not be 0")]
    ZeroMovement,
}
