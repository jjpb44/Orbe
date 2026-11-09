//! Product entity. V2 invariants enforced at construction.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::DomainError;

/// A product in the inventory.
///
/// `id` is the SurrealDB record key (the part after `product:`).
/// Empty on creation; gate assigns.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub name: String,
    /// Stock keeping unit price in COP cents. Integer to avoid float drift.
    pub price_cents: i64,
    pub stock: i32,
    /// Optional alert threshold for low-stock UI badge.
    pub min_stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub sku: String,
    pub name: String,
    pub price_cents: i64,
    pub stock: i32,
    pub min_stock: i32,
}

impl NewProduct {
    /// V2: ∀ Product → sku ∉ ∅, price ≥ 0, stock ≥ 0
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.sku.trim().is_empty() {
            return Err(DomainError::EmptySku);
        }
        if self.name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        if self.price_cents < 0 {
            return Err(DomainError::NegativePrice);
        }
        if self.stock < 0 {
            return Err(DomainError::NegativeStock);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_sku() {
        let p = NewProduct {
            sku: " ".into(),
            name: "x".into(),
            price_cents: 0,
            stock: 0,
            min_stock: 0,
        };
        assert!(matches!(p.validate(), Err(DomainError::EmptySku)));
    }

    #[test]
    fn rejects_negative_price() {
        let p = NewProduct {
            sku: "A".into(),
            name: "x".into(),
            price_cents: -1,
            stock: 0,
            min_stock: 0,
        };
        assert!(matches!(p.validate(), Err(DomainError::NegativePrice)));
    }

    #[test]
    fn rejects_negative_stock() {
        let p = NewProduct {
            sku: "A".into(),
            name: "x".into(),
            price_cents: 0,
            stock: -1,
            min_stock: 0,
        };
        assert!(matches!(p.validate(), Err(DomainError::NegativeStock)));
    }

    #[test]
    fn accepts_zero_price_and_stock() {
        let p = NewProduct {
            sku: "A".into(),
            name: "x".into(),
            price_cents: 0,
            stock: 0,
            min_stock: 0,
        };
        p.validate().unwrap();
    }
}
