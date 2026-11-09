//! Orbe · Dioxus UI.
//!
//! V7: every data access goes through a `#[server]` function, which itself
//! calls into `crate::flow::*`. The FE never imports `crate::gate`.

pub mod app;
pub mod inventory;
pub mod server_fns;

pub use app::App;
