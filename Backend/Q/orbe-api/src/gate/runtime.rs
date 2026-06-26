//! Runtime · owns the SurrealDB handle and exposes the adapters.
//!
//! Single instance per process. Cloning is cheap (Arc internally).

use std::sync::Arc;

use surrealdb::Surreal;
use surrealdb::engine::local::Db;

use crate::core::StockMovement;
use super::surreal::{SurrealMovementLog, SurrealProductRepo, open};

/// Concrete adapters backed by one shared SurrealDB handle.
#[derive(Clone)]
pub struct Runtime {
    pub products: Arc<SurrealProductRepo>,
    pub movements: Arc<SurrealMovementLog>,
    pub db: Arc<Surreal<Db>>,
}

impl Runtime {
    /// Open an in-memory SurrealDB, apply schema, return ready-to-use adapters.
    pub async fn boot() -> Result<Self, surrealdb::Error> {
        let db = open().await?;
        Ok(Self::from_handle(db))
    }

    /// Wrap an existing handle (useful in tests after applying fixtures).
    pub fn from_handle(db: Arc<Surreal<Db>>) -> Self {
        let movements = Arc::new(SurrealMovementLog::new(db.clone()));
        let products = Arc::new(SurrealProductRepo::new(db.clone(), movements.clone()));
        Self { products, movements, db }
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
