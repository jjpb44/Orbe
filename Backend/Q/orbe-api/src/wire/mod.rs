//! Orbe · Wire (REST over Axum).
//!
//! V1: wire never reaches into `gate::surreal` directly — it goes through
//! `flow::*` use cases. The flow module is the only thing wire imports from.

pub mod inventory;
