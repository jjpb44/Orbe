//! Gate · SurrealDB adapter.
//!
//! Wraps the in-process `surrealdb::Surreal<Db>` engine for Phase 2.
//! Phase 3 may swap to a remote `Ws` engine without touching flow/wire/ui.

pub mod connect;
pub mod log;
pub mod repo;

pub use connect::open;
pub use log::SurrealMovementLog;
pub use repo::SurrealProductRepo;
