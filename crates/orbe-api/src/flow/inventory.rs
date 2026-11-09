//! Inventory use cases.
//!
//! Every function takes the ports it needs as `&dyn` trait objects so the
//! caller (wire, server fn, test) controls which adapter to inject.

use crate::core::{DomainError, MovementKind, NewMovement, NewProduct, Product, StockMovement};
use crate::gate::{ProductRepo, RepoError};

/// Domain-level error spanning validation (core) and persistence (gate).
/// Wire/UI map each variant to the right HTTP status / ServerFnError.
#[derive(Debug, thiserror::Error)]
pub enum FlowError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repo(#[from] RepoError),
}

pub async fn create_product(repo: &dyn ProductRepo, new: NewProduct) -> Result<Product, FlowError> {
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
    // Missing product is a 404 (RepoError::NotFound), not a silent no-op.
    repo.get(id).await?;
    Ok(repo.delete(id).await?)
}

/// Adjust stock and persist the movement in one go.
/// V5 is checked here, before the adapter runs — the adapter's DB assertion is
/// the last line of defence, but a domain error must surface as 422, not 500.
pub async fn adjust_stock(
    repo: &dyn ProductRepo,
    product_id: &str,
    kind: MovementKind,
    qty: i32,
    reason: &str,
) -> Result<(Product, StockMovement), FlowError> {
    let m = NewMovement {
        product_id: product_id.into(),
        kind,
        qty,
        reason: reason.into(),
    };
    m.validate()?;

    let current = repo.get(product_id).await?;
    if current.stock + qty < 0 {
        return Err(DomainError::NegativeStock.into());
    }

    Ok(repo.adjust_stock(product_id, kind, qty, reason).await?)
}

#[cfg(test)]
mod tests {
    //! Flow-level invariants (V4/V5) against an in-memory fake port.
    use super::*;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeRepo {
        products: Mutex<Vec<Product>>,
        movements: Mutex<Vec<StockMovement>>,
    }

    fn product(id: &str, stock: i32) -> Product {
        Product {
            id: id.into(),
            sku: id.into(),
            name: "Widget".into(),
            price_cents: 1000,
            stock,
            min_stock: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    impl FakeRepo {
        fn with(products: Vec<Product>) -> Self {
            Self {
                products: Mutex::new(products),
                movements: Mutex::new(vec![]),
            }
        }
        fn movement_count(&self) -> usize {
            self.movements.lock().unwrap().len()
        }
    }

    #[async_trait]
    impl ProductRepo for FakeRepo {
        async fn create(&self, p: NewProduct) -> Result<Product, RepoError> {
            let mut out = product(&p.sku, p.stock);
            out.name = p.name;
            out.price_cents = p.price_cents;
            out.min_stock = p.min_stock;
            self.products.lock().unwrap().push(out.clone());
            Ok(out)
        }
        async fn list(&self) -> Result<Vec<Product>, RepoError> {
            Ok(self.products.lock().unwrap().clone())
        }
        async fn get(&self, id: &str) -> Result<Product, RepoError> {
            self.products
                .lock()
                .unwrap()
                .iter()
                .find(|p| p.id == id)
                .cloned()
                .ok_or_else(|| RepoError::NotFound(id.into()))
        }
        async fn update(&self, id: &str, patch: NewProduct) -> Result<Product, RepoError> {
            let mut list = self.products.lock().unwrap();
            let p = list
                .iter_mut()
                .find(|p| p.id == id)
                .ok_or_else(|| RepoError::NotFound(id.into()))?;
            p.name = patch.name;
            p.price_cents = patch.price_cents;
            p.min_stock = patch.min_stock;
            Ok(p.clone())
        }
        async fn delete(&self, id: &str) -> Result<(), RepoError> {
            let mut list = self.products.lock().unwrap();
            let before = list.len();
            list.retain(|p| p.id != id);
            if list.len() == before {
                return Err(RepoError::NotFound(id.into()));
            }
            Ok(())
        }
        async fn adjust_stock(
            &self,
            product_id: &str,
            kind: MovementKind,
            qty: i32,
            reason: &str,
        ) -> Result<(Product, StockMovement), RepoError> {
            let updated = {
                let mut list = self.products.lock().unwrap();
                let p = list
                    .iter_mut()
                    .find(|p| p.id == product_id)
                    .ok_or_else(|| RepoError::NotFound(product_id.into()))?;
                p.stock += qty;
                p.clone()
            };
            let m = StockMovement {
                id: format!("mv-{}", self.movement_count() + 1),
                product_id: product_id.into(),
                kind,
                qty,
                reason: reason.into(),
                at: Utc::now(),
            };
            self.movements.lock().unwrap().push(m.clone());
            Ok((updated, m))
        }
    }

    #[tokio::test]
    async fn adjust_increases_stock_and_writes_movement() {
        let repo = FakeRepo::with(vec![product("A1", 10)]);
        let (p, m) = adjust_stock(&repo, "A1", MovementKind::In, 5, "receive")
            .await
            .unwrap();
        assert_eq!(p.stock, 15);
        assert_eq!(m.qty, 5);
        assert_eq!(repo.movement_count(), 1, "V4: exactly one movement");
    }

    #[tokio::test]
    async fn adjust_below_zero_is_rejected_and_writes_nothing() {
        let repo = FakeRepo::with(vec![product("A1", 3)]);
        let err = adjust_stock(&repo, "A1", MovementKind::Out, -10, "oversell")
            .await
            .unwrap_err();
        assert!(matches!(err, FlowError::Domain(DomainError::NegativeStock)));
        assert_eq!(repo.movement_count(), 0, "V5: no movement on rejection");
        assert_eq!(repo.get("A1").await.unwrap().stock, 3, "stock untouched");
    }

    #[tokio::test]
    async fn adjust_to_exactly_zero_is_allowed() {
        let repo = FakeRepo::with(vec![product("A1", 4)]);
        let (p, _) = adjust_stock(&repo, "A1", MovementKind::Out, -4, "sell all")
            .await
            .unwrap();
        assert_eq!(p.stock, 0);
    }

    #[tokio::test]
    async fn zero_qty_movement_is_rejected() {
        let repo = FakeRepo::with(vec![product("A1", 4)]);
        let err = adjust_stock(&repo, "A1", MovementKind::Adjust, 0, "noop")
            .await
            .unwrap_err();
        assert!(matches!(err, FlowError::Domain(DomainError::ZeroMovement)));
    }

    #[tokio::test]
    async fn adjust_unknown_product_is_not_found() {
        let repo = FakeRepo::default();
        let err = adjust_stock(&repo, "nope", MovementKind::In, 1, "x")
            .await
            .unwrap_err();
        assert!(matches!(err, FlowError::Repo(RepoError::NotFound(_))));
    }

    #[tokio::test]
    async fn delete_unknown_product_is_not_found() {
        let repo = FakeRepo::default();
        let err = delete_product(&repo, "nope").await.unwrap_err();
        assert!(matches!(err, FlowError::Repo(RepoError::NotFound(_))));
    }
}
