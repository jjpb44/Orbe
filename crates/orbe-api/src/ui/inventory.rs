//! Inventory page · list + create + adjust.

use dioxus::prelude::*;

use crate::core::{MovementKind, NewProduct};
use crate::ui::server_fns::{
    adjust_stock as adjust_stock_fn, create_product as create_product_fn,
    list_products as list_products_fn,
};

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    // The shell (topbar + nav) wraps every route, so `Link` always has the
    // router context and the user can always reach the inventory.
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/inventory")]
    Inventory {},
    #[route("/inventory/new")]
    InventoryNew {},
}

/// App chrome — rendered around the matched route (Dioxus `#[layout]`).
#[component]
pub fn Shell() -> Element {
    rsx! {
        div { class: "o-shell",
            header { class: "o-topbar",
                h1 { class: "o-topbar__title", "Orbe" }
                nav { class: "o-nav",
                    // Plain anchors: a `#[layout]` component renders outside the
                    // router scope, where `Link` panics ("no parent router").
                    a { href: "/", class: "o-nav__link", "Inicio" }
                    a { href: "/inventory", class: "o-nav__link", "Inventario" }
                    a { href: "/inventory/new", class: "o-nav__link", "Nuevo" }
                }
            }
            main { class: "o-main",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx! { p { "Bienvenido a Orbe." } }
}

#[component]
pub fn Inventory() -> Element {
    let products = use_resource(|| async { list_products_fn().await });

    rsx! {
        section { class: "o-card",
            header { class: "o-card__head",
                h2 { class: "o-card__title", "Productos" }
                a { href: "/inventory/new", class: "o-btn o-btn--primary", "+ Nuevo" }
            }
            div { class: "o-card__body",
                match products.read().as_ref() {
                    Some(Ok(list)) if list.is_empty() => rsx! { p { "Sin productos." } },
                    Some(Ok(list)) => rsx! { ProductList { items: list.clone() } },
                    Some(Err(e)) => rsx! { p { class: "o-error", "Error: {e}" } },
                    None => rsx! { p { "Cargando…" } },
                }
            }
        }
    }
}

#[component]
fn ProductList(items: Vec<crate::core::Product>) -> Element {
    rsx! {
        ul { class: "o-list",
            for p in items {
                ProductCard { product: p }
            }
        }
    }
}

#[component]
fn ProductCard(product: crate::core::Product) -> Element {
    let mut kind = use_signal(|| MovementKind::In);
    let mut qty = use_signal(|| 1i32);
    let mut reason = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);

    let pid = product.id.clone();

    rsx! {
        li { class: "o-card o-card--row",
            div { class: "o-card__body",
                strong { "{product.sku}" }
                span { class: "o-card__meta", "{product.name}" }
                span { class: "o-card__meta", "stock={product.stock}" }
            }
            form {
                class: "o-form o-form--inline",
                onsubmit: move |e| {
                    e.prevent_default();
                    let k = *kind.read();
                    let q = *qty.read() as i32;
                    let r = reason.read().clone();
                    let pid2 = pid.clone();
                    async move {
                        match adjust_stock_fn(pid2, k, q, r).await {
                            Ok(_) => { reason.set(String::new()); error.set(None); }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                },
                select {
                    onchange: move |e| {
                        kind.set(match e.value().as_str() {
                            "out" => MovementKind::Out,
                            "adjust" => MovementKind::Adjust,
                            _ => MovementKind::In,
                        });
                    },
                    option { value: "in", "In" }
                    option { value: "out", "Out" }
                    option { value: "adjust", "Adjust" }
                }
                input {
                    r#type: "number",
                    value: "{qty}",
                    oninput: move |e| {
                        if let Ok(n) = e.value().parse::<i32>() { qty.set(n); }
                    },
                }
                input {
                    placeholder: "razón",
                    value: "{reason}",
                    oninput: move |e| reason.set(e.value()),
                }
                button { class: "o-btn", r#type: "submit", "Aplicar" }
            }
            if let Some(err) = error.read().as_ref() {
                p { class: "o-error", "{err}" }
            }
        }
    }
}

#[component]
pub fn InventoryNew() -> Element {
    let nav = use_navigator();
    let mut sku = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut price = use_signal(|| 0i64);
    let mut stock = use_signal(|| 0i32);
    let mut min_stock = use_signal(|| 0i32);
    let mut error = use_signal(|| Option::<String>::None);

    rsx! {
        section { class: "o-card",
            header { class: "o-card__head",
                h2 { class: "o-card__title", "Nuevo producto" }
            }
            div { class: "o-card__body",
                form {
                    onsubmit: move |e| {
                        e.prevent_default();
                        let new = NewProduct {
                            sku: sku.read().clone(),
                            name: name.read().clone(),
                            price_cents: *price.read(),
                            stock: *stock.read(),
                            min_stock: *min_stock.read(),
                        };
                        async move {
                            match create_product_fn(new).await {
                                Ok(_) => { nav.push(Route::Inventory {}); }
                                Err(e) => { error.set(Some(e.to_string())); }
                            }
                        }
                    },
                    class: "o-form",
                    label { "SKU" input { value: "{sku}", oninput: move |e| sku.set(e.value()) } }
                    label { "Nombre" input { value: "{name}", oninput: move |e| name.set(e.value()) } }
                    label { "Precio (centavos)" input {
                        r#type: "number",
                        value: "{price}",
                        oninput: move |e| { if let Ok(n) = e.value().parse::<i64>() { price.set(n); } }
                    } }
                    label { "Stock" input {
                        r#type: "number",
                        value: "{stock}",
                        oninput: move |e| { if let Ok(n) = e.value().parse::<i32>() { stock.set(n); } }
                    } }
                    label { "Stock mínimo" input {
                        r#type: "number",
                        value: "{min_stock}",
                        oninput: move |e| { if let Ok(n) = e.value().parse::<i32>() { min_stock.set(n); } }
                    } }
                    button { class: "o-btn o-btn--primary", r#type: "submit", "Crear" }
                    if let Some(err) = error.read().as_ref() {
                        p { class: "o-error", "{err}" }
                    }
                }
            }
        }
    }
}
