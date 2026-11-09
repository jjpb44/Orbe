//! Stock movement entity. V5: qty ≠ 0.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::DomainError;

/// Why the stock changed. Display label is the kebab variant.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MovementKind {
    /// Goods received from a supplier (qty > 0).
    In,
    /// Sold or otherwise removed (qty < 0).
    Out,
    /// Manual correction (qty ≠ 0, signed).
    Adjust,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StockMovement {
    pub id: String,
    pub product_id: String,
    pub kind: MovementKind,
    /// Signed delta applied to product stock. Always non-zero.
    pub qty: i32,
    pub reason: String,
    pub at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewMovement {
    pub product_id: String,
    pub kind: MovementKind,
    pub qty: i32,
    pub reason: String,
}

impl NewMovement {
    /// V5: ∀ StockMovement → qty ≠ 0.
    /// Kind/qty coherence is also enforced: `In` → qty > 0, `Out` → qty < 0.
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.qty == 0 {
            return Err(DomainError::ZeroMovement);
        }
        match self.kind {
            MovementKind::In if self.qty <= 0 => Err(DomainError::ZeroMovement),
            MovementKind::Out if self.qty >= 0 => Err(DomainError::ZeroMovement),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_qty() {
        let m = NewMovement {
            product_id: "p".into(),
            kind: MovementKind::Adjust,
            qty: 0,
            reason: "x".into(),
        };
        assert!(matches!(m.validate(), Err(DomainError::ZeroMovement)));
    }

    #[test]
    fn rejects_in_with_negative_qty() {
        let m = NewMovement {
            product_id: "p".into(),
            kind: MovementKind::In,
            qty: -3,
            reason: "x".into(),
        };
        assert!(m.validate().is_err());
    }

    #[test]
    fn accepts_in_with_positive_qty() {
        let m = NewMovement {
            product_id: "p".into(),
            kind: MovementKind::In,
            qty: 10,
            reason: "received".into(),
        };
        m.validate().unwrap();
    }

    #[test]
    fn accepts_adjust_with_any_nonzero() {
        let m = NewMovement {
            product_id: "p".into(),
            kind: MovementKind::Adjust,
            qty: -2,
            reason: "correction".into(),
        };
        m.validate().unwrap();
    }
}
