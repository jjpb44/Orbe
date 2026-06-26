//! SurrealDB-backed `MovementLog` implementation.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::types::{RecordId, SurrealValue};

use super::super::ports::{MovementLog, RepoError};
use super::repo::id_key;
use crate::core::{MovementKind, NewMovement, StockMovement};

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
struct MovementRow {
    id: RecordId,
    product: RecordId,
    delta: i32,
    #[serde(default)]
    kind_str: Option<String>,
    reason: String,
    #[serde(default)]
    at: DateTime<Utc>,
}

impl MovementRow {
    fn into_core(self, product_id_fallback: &str, kind_fallback: MovementKind) -> StockMovement {
        use crate::core::movement::MovementKind as K;
        let kind = match self.kind_str.as_deref() {
            Some("in") => K::In,
            Some("out") => K::Out,
            Some("adjust") => K::Adjust,
            _ => match self.delta.signum() {
                1 => K::In,
                -1 => K::Out,
                _ => kind_fallback,
            },
        };
        let pid = id_key(&self.product);
        let pid = if pid.is_empty() { product_id_fallback.to_string() } else { pid };
        StockMovement {
            id: id_key(&self.id),
            product_id: pid,
            kind,
            qty: self.delta,
            reason: self.reason,
            at: self.at,
        }
    }
}

#[derive(Clone)]
pub struct SurrealMovementLog {
    db: Arc<Surreal<Db>>,
}

impl SurrealMovementLog {
    pub fn new(db: Arc<Surreal<Db>>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl MovementLog for SurrealMovementLog {
    async fn append(&self, m: NewMovement) -> Result<StockMovement, RepoError> {
        let kind_str = match m.kind {
            MovementKind::In => "in",
            MovementKind::Out => "out",
            MovementKind::Adjust => "adjust",
        };
        // Query (not .content()): construct Thing server-side via type::thing()
        // — serde_json tuple `("product", id)` does NOT coerce to record<product>
        // in SCHEMAFULL mode. Use plain `CREATE stock_movement` so SurrealDB
        // generates a random record id.
        let row: Option<MovementRow> = self
            .db
            .query(
                "CREATE stock_movement SET \
                 product = type::record('product', $pid), \
                 delta = $delta, kind_str = $kind, reason = $reason RETURN *",
            )
            .bind(("pid", m.product_id.clone()))
            .bind(("delta", m.qty))
            .bind(("kind", kind_str))
            .bind(("reason", m.reason.clone()))
            .await
            .map_err(map_db_err)?
            .take(0)
            .map_err(map_db_err)?;
        row.map(|r| r.into_core(&m.product_id, m.kind))
            .ok_or_else(|| RepoError::Backend("movement insert returned nothing".into()))
    }

    async fn list_for_product(&self, product_id: &str) -> Result<Vec<StockMovement>, RepoError> {
        let pid = product_id.to_string();
        // Construct Thing server-side via type::thing() so the comparison matches
        // the stored record link exactly (serde tuple binding doesn't coerce to
        // record<product> reliably).
        let rows: Vec<MovementRow> = self
            .db
            .query(
                "SELECT * FROM stock_movement \
                 WHERE product = type::record('product', $pid) ORDER BY at DESC",
            )
            .bind(("pid", pid.clone()))
            .await
            .map_err(map_db_err)?
            .take(0)
            .map_err(map_db_err)?;
        Ok(rows
            .into_iter()
            .map(|r| r.into_core(&pid, MovementKind::Adjust))
            .collect())
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
