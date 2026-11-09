//! Runtime · owns the SurrealDB handle and exposes the adapters.
//!
//! Single instance per process. Cloning is cheap (Arc internally).

use std::sync::Arc;

use surrealdb::engine::local::Db;
use surrealdb::Surreal;

use super::surreal::{open, SurrealMovementLog, SurrealProductRepo};
use crate::core::StockMovement;

/// Concrete adapters backed by one shared SurrealDB handle.
#[derive(Clone)]
pub struct Runtime {
    pub products: Arc<SurrealProductRepo>,
    pub movements: Arc<SurrealMovementLog>,
    pub db: Arc<Surreal<Db>>,
}

impl Runtime {
    /// Open an in-memory SurrealDB, apply schema, return ready-to-use adapters.
    ///
    /// With `ORBE_SEED=1` the committed `database/seeds/inventory.toon` fixture is
    /// loaded too (`just seed`). Seeding is idempotent — records that already exist
    /// are left alone.
    pub async fn boot() -> Result<Self, surrealdb::Error> {
        let db = open().await?;
        if std::env::var("ORBE_SEED").is_ok_and(|v| v == "1") {
            const SEED: &str = include_str!("../../../../database/seeds/inventory.toon");
            match super::surreal::apply_seed_file(&db, SEED).await {
                Ok(n) => tracing::info!(rows = n, "seed loaded"),
                Err(e) => tracing::warn!(error = %e, "seed failed"),
            }
        }
        Ok(Self::from_handle(db))
    }

    /// Wrap an existing handle (useful in tests after applying fixtures).
    pub fn from_handle(db: Arc<Surreal<Db>>) -> Self {
        let movements = Arc::new(SurrealMovementLog::new(db.clone()));
        let products = Arc::new(SurrealProductRepo::new(db.clone(), movements.clone()));
        Self {
            products,
            movements,
            db,
        }
    }
}

// Re-export `apply_schema` so the smoke-test binary can call it directly.
pub use super::apply_schema as reapply_schema;

// Re-export the result types so flow/wire don't need to import deeper.
pub type ProductRepoHandle = Arc<SurrealProductRepo>;
pub type MovementLogHandle = Arc<SurrealMovementLog>;

// Helper: ergonomic append from a NewMovement value.
impl SurrealMovementLog {
    pub async fn append_from(
        &self,
        m: crate::core::NewMovement,
    ) -> Result<StockMovement, super::ports::RepoError> {
        use super::ports::MovementLog;
        MovementLog::append(self, m).await
    }
}
