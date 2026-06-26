//! Gate · hexagonal ports + adapters.
//!
//! V1: Wire (REST) and UI (`#[server]`) reach data ONLY through flow
//! use cases. Flow holds a `&dyn ProductRepo` / `&dyn MovementLog`.

pub mod ports;

#[cfg(feature = "server")]
pub mod surreal;

#[cfg(feature = "server")]
pub mod runtime;

pub use ports::{Clock, MovementLog, ProductRepo, RepoError, SystemClock};

#[cfg(feature = "server")]
pub use runtime::Runtime;

#[cfg(feature = "server")]
use surrealdb::Surreal;
#[cfg(feature = "server")]
use surrealdb::engine::local::Db;

/// Idempotent schema bootstrap. Loads `Backend/W/1.Dat/1a.Sche/Inventory.surql`
/// at compile time and runs the whole script as one query (SurrealDB parses
/// multiple statements + line comments itself). Splitting on `;` ourselves
/// breaks when a `--` comment contains a `;`.
///
/// Path resolved relative to crate root (`Backend/Q/orbe-api`).
#[cfg(feature = "server")]
pub async fn apply_schema(db: &Surreal<Db>) -> Result<(), surrealdb::Error> {
    const SCHEMA: &str = include_str!("../../../../W/1.Dat/1a.Sche/Inventory.surql");
    let _ = db.query(SCHEMA).await?;
    Ok(())
}
