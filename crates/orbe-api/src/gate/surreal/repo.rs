//! SurrealDB-backed `ProductRepo` implementation.
//!
//! Maps between core `Product` (id = record key string) and SurrealDB
//! `product` table. Field names align 1:1 with `Inventory.surql`.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::types::{RecordId, RecordIdKey, SurrealValue};
use surrealdb::Surreal;

use super::super::ports::{MovementLog, ProductRepo, RepoError};
use crate::core::{MovementKind, NewMovement, NewProduct, Product, StockMovement};

/// Row shape persisted to SurrealDB. The `id` field is the record's RecordId.
#[derive(Debug, Serialize, Deserialize, SurrealValue)]
struct ProductRow {
    id: RecordId,
    sku: String,
    name: String,
    price: i64,
    stock: i32,
    #[serde(default)]
    min_stock: i32,
    #[serde(default = "default_now")]
    created_at: DateTime<Utc>,
    #[serde(default = "default_now")]
    updated_at: DateTime<Utc>,
}

fn default_now() -> DateTime<Utc> {
    Utc::now()
}

impl ProductRow {
    fn into_core(self) -> Product {
        let key = id_key(&self.id);
        Product {
            id: key,
            sku: self.sku,
            name: self.name,
            price_cents: self.price,
            stock: self.stock,
            min_stock: self.min_stock,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Extract just the key portion of a RecordId (e.g. "BEAN-1" from `product:BEAN-1`).
/// For our SKUs the key is always a String variant; any other variant falls back
/// to a debug-format of the key.
pub(super) fn id_key(rid: &RecordId) -> String {
    match &rid.key {
        RecordIdKey::String(s) => s.clone(),
        other => format!("{:?}", other),
    }
}

fn rid(sku: &str) -> RecordId {
    RecordId::new("product", sku)
}

#[derive(Clone)]
pub struct SurrealProductRepo {
    db: Arc<Surreal<Db>>,
    log: Arc<super::SurrealMovementLog>,
}

impl SurrealProductRepo {
    pub fn new(db: Arc<Surreal<Db>>, log: Arc<super::SurrealMovementLog>) -> Self {
        Self { db, log }
    }
}

#[async_trait]
impl ProductRepo for SurrealProductRepo {
    async fn create(&self, p: NewProduct) -> Result<Product, RepoError> {
        let row: Option<ProductRow> = self
            .db
            .create(rid(&p.sku))
            .content(serde_json::json!({
                "sku": p.sku,
                "name": p.name,
                "price": p.price_cents,
                "stock": p.stock,
                "min_stock": p.min_stock,
            }))
            .await
            .map_err(map_db_err)?;
        match row {
            Some(r) => Ok(r.into_core()),
            None => self.get(&p.sku).await,
        }
    }

    async fn list(&self) -> Result<Vec<Product>, RepoError> {
        let rows: Vec<ProductRow> = self.db.select("product").await.map_err(map_db_err)?;
        Ok(rows.into_iter().map(ProductRow::into_core).collect())
    }

    async fn get(&self, id: &str) -> Result<Product, RepoError> {
        let row: Option<ProductRow> = self.db.select(rid(id)).await.map_err(map_db_err)?;
        row.map(ProductRow::into_core)
            .ok_or_else(|| RepoError::NotFound(format!("product:{id}")))
    }

    async fn update(&self, id: &str, patch: NewProduct) -> Result<Product, RepoError> {
        // Use a query (not .content()) so SurrealDB computes `time::now()` itself
        // — serde_json serializes chrono DateTime as ISO string, which SCHEMAFULL
        // datetime fields reject.
        let row: Option<ProductRow> = self
            .db
            .query(
                "UPDATE type::record('product', $id) SET \
                 name = $name, price = $price, stock = $stock, min_stock = $min_stock, \
                 updated_at = time::now() RETURN *",
            )
            .bind(("id", id.to_string()))
            .bind(("name", patch.name))
            .bind(("price", patch.price_cents))
            .bind(("stock", patch.stock))
            .bind(("min_stock", patch.min_stock))
            .await
            .map_err(map_db_err)?
            .take(0)
            .map_err(map_db_err)?;
        row.map(ProductRow::into_core)
            .ok_or_else(|| RepoError::NotFound(format!("product:{id}")))
    }

    async fn delete(&self, id: &str) -> Result<(), RepoError> {
        let _: Option<ProductRow> = self.db.delete(rid(id)).await.map_err(map_db_err)?;
        Ok(())
    }

    async fn adjust_stock(
        &self,
        product_id: &str,
        kind: MovementKind,
        qty: i32,
        reason: &str,
    ) -> Result<(Product, StockMovement), RepoError> {
        // Two-step: read → write + append. Acceptable for MVP single-writer.
        // Phase 3 should wrap in `db.query(... BEGIN/COMMIT)` for atomicity.
        let row: Option<ProductRow> = self.db.select(rid(product_id)).await.map_err(map_db_err)?;
        let mut row = row.ok_or_else(|| RepoError::NotFound(format!("product:{product_id}")))?;
        let new_stock = row
            .stock
            .checked_add(qty)
            .ok_or_else(|| RepoError::Backend("stock overflow".into()))?;
        if new_stock < 0 {
            return Err(RepoError::Backend(format!(
                "stock would go negative (current={}, delta={})",
                row.stock, qty
            )));
        }
        row.stock = new_stock;

        // Query (not .merge()) so SurrealDB computes `time::now()` itself —
        // serde_json DateTime → ISO string is rejected by SCHEMAFULL datetime.
        let _updated: Option<ProductRow> = self
            .db
            .query(
                "UPDATE type::record('product', $id) SET stock = $stock, \
                 updated_at = time::now() RETURN *",
            )
            .bind(("id", product_id.to_string()))
            .bind(("stock", new_stock))
            .await
            .map_err(map_db_err)?
            .take(0)
            .map_err(map_db_err)?;

        let mv = MovementLog::append(
            self.log.as_ref(),
            NewMovement {
                product_id: product_id.into(),
                kind,
                qty,
                reason: reason.into(),
            },
        )
        .await?;
        Ok((row.into_core(), mv))
    }
}

fn map_db_err(e: surrealdb::Error) -> RepoError {
    let msg = e.to_string();
    if msg.contains("already exists") || msg.to_lowercase().contains("unique") {
        RepoError::UniqueViolation(msg)
    } else {
        RepoError::Backend(msg)
    }
}
