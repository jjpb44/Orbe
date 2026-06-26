//! Orbe · Flow use cases.
//!
//! Pure orchestration: validate input, call ports, return output. No I/O,
//! no framework. Wire (REST) and UI (`#[server]`) both call into here.
//!
//! V6: error mapping lives in `wire::inventory` (REST) and `ui::server_fns`
//! (Dioxus), not here.

pub mod inventory;

pub use inventory::{
    adjust_stock, adjust_stock_with_clock, create_product, delete_product, get_product,
    list_products, update_product, FlowError,
};
