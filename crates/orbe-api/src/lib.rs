//! Orbe API · lib root.
//!
//! Layered: `core` ⊥ `flow` → `gate` (ports + adapter) → `wire` + `ui` (entry).
//! V1: wire and ui may only import `flow` and `core`. Never `gate::surreal`.

pub mod core;
pub mod flow;
pub mod gate;
#[cfg(feature = "server")]
pub mod wire;

pub mod ui;
