//! Open an in-memory SurrealDB and apply the inventory schema.
//!
//! NS=orbe DB=inventory — same convention as `config/Env.example`.

use std::sync::Arc;

use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

use super::super::apply_schema;

const NS: &str = "orbe";
const DB: &str = "inventory";

/// Open a fresh, in-memory SurrealDB with the inventory schema applied.
/// The handle is cheap to clone (`Arc` internally).
pub async fn open() -> Result<Arc<Surreal<Db>>, surrealdb::Error> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns(NS).use_db(DB).await?;
    apply_schema(&db).await?;
    Ok(Arc::new(db))
}
