# Orbe Guide — Rust + Dioxus for new devs

Covers only patterns used in this project. Open this alongside the code.

## 1. Dev loop

```sh
just dev        # dx serve → hot reload on localhost:8080
just build      # cargo build (check compilation)
just test       # cargo test --lib (11 unit tests)
just smoke      # 14 curl tests against running server
```

- Edit any `.rs` file → save → browser updates (no refresh)
- Edit `public/main.css` → instant CSS update
- Add a new `#[server]` fn → restart needed (macro generates route)

## 2. Rust you'll see daily

### Struct = data bag
```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub price_cents: i64,   // COP cents, not float
    pub stock: i32,
}
```
- `pub` = field visible outside module (everything is pub in core)
- `#[derive(...)]` = auto-implement traits. `Clone` to copy, `Debug` to `{:#?}` print, `Serialize/Deserialize` for JSON/SurrealDB

### Enum = one-of
```rust
pub enum MovementKind {
    In,
    Out,
    Adjust,
}
```
- `match` on it exhaustively (compiler checks you handled all variants)

### Result = success or error
```rust
pub fn validate(&self) -> Result<(), DomainError> {
    if self.sku.trim().is_empty() {
        return Err(DomainError::EmptySku);
    }
    Ok(())
}
```
- `Ok(())` = success with nothing to return
- `Err(...)` = failure. Caller uses `?` to propagate: `new.validate()?;`
- `Result<T, E>` → `?` unwraps Ok(T) or returns Err(E) from current fn

### Async everywhere
```rust
pub async fn list_products(repo: &dyn ProductRepo) -> Result<Vec<Product>, FlowError> {
    repo.list().await   // .await = wait for I/O
}
```
- `async fn` returns a future. Call with `.await`
- All I/O (DB queries, HTTP) is async

### match = switch on steroids
```rust
match products.read().as_ref() {
    Some(Ok(list)) if list.is_empty() => rsx! { p { "Sin productos." } },
    Some(Ok(list)) => rsx! { ProductList { items: list.clone() } },
    Some(Err(e)) => rsx! { p { "Error: {e}" } },
    None => rsx! { p { "Cargando…" } },
}
```
- `Some(Ok(list))` matches nested `Option<Result<...>>`
- `if list.is_empty()` = guard condition
- Compiler checks exhaustiveness

### Phantom types
```rust
pub struct NewProduct { pub sku: String, pub name: String, ... }
pub struct  Product { pub id: String, pub sku: String, ... /* + timestamps */ }
```
- `NewProduct` = what you send (no id, no timestamps)
- `Product` = what the DB returns (has id + timestamps)
- Two types, no confusion. Compiler enforces.

## 3. Dioxus patterns

### Component = fn returning Element
```rust
#[component]
pub fn Inventory() -> Element {   // <Inventory /> in rsx
    rsx! { ... }
}
```
- `#[component]` marks it as a component
- Props: `fn ProductCard(product: Product) -> Element` — pass as `<ProductCard product={x} />`

### rsx! = HTML in Rust
```rust
rsx! {
    section { class: "o-card",
        h2 { "Título" }
        p { "Texto con {variable}" }    // {brace} = expression
        button { class: "o-btn", r#type: "submit", "Enviar" }   // r#type = reserved word escape
    }
}
```
- Tags are functions: `div { ... }`, `p { ... }`, `button { ... }`
- Attributes: `class: "value"`, `onclick: move |_| ...`
- Text: `"literal {expr}"`
- Nesting: `parent { child { "text" } }`

### Signals = reactive state
```rust
let mut name = use_signal(|| String::new());   // create
name.set("Nuevo".to_string());                  // write
let v = name.read().clone();                    // read
```
- `.read()` returns an RAII guard (auto-drops). Calling `.read()` inside rsx! automatically subscribes the component to changes
- `.set(val)` updates + re-renders
- `use_signal` replaces React's `useState`

### use_resource = async data
```rust
let products = use_resource(|| async { list_products_fn().await });
// products: Resource<Result<Vec<Product>, ServerFnError>>
match products.read().as_ref() {
    Some(Ok(list)) => ...,
    Some(Err(e)) => ...,
    None => ...,  // loading
}
```
- Fetches on mount, re-fetches on signal change (not auto-poll)

### #[server] = RPC
```rust
#[post("/api/products")]
pub async fn create_product(new: NewProduct) -> Result<Product, ServerFnError> {
    // This runs on SERVER, called from CLIENT
    flow::create_product(rt.products.as_ref(), new).await.map_err(map_err)
}
```
- Macro generates TWO things: server endpoint + client free fn
- Client calls `create_product_fn(new).await` (the `_fn` suffix is auto-generated)
- V7 invariant: `#[server]` fns = sole FE→data path
- Note: `#[server]` macros register early in boot. Adding a new one requires restart.

### Router
```rust
#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/inventory")]
    Inventory {},
}

// Use:
Link { to: Route::Inventory {}, "Ir a inventario" }
let nav = use_navigator();
nav.push(Route::Inventory {});
```

### Forms in Dioxus
```rust
let mut name = use_signal(|| String::new());
form {
    onsubmit: move |e| {
        e.prevent_default();
        let n = name.read().clone();
        async move { create_product_fn(n).await; }
    },
    input { value: "{name}", oninput: move |e| name.set(e.value()) }
    button { r#type: "submit", "Guardar" }
}
```
- Every input has its own signal
- `onsubmit` captures signals by `move |e|`
- Async handlers inside `onsubmit`: wrap in `async move { ... }`

## 4. Codebase map

```
Backend/Q/orbe-api/src/
├── main.rs          # Entry point. Boots Axum, wires Dioxus + REST
├── lib.rs           # Re-exports modules
├── core/            # Pure domain. No framework deps.
│   ├── product.rs   #   Product, NewProduct + validation
│   ├── movement.rs  #   MovementKind, StockMovement
│   └── mod.rs       #   DomainError enum
├── flow/            # Use cases. Business logic.
│   └── inventory.rs #   create_product, list_products, adjust_stock…
├── gate/            # Database adapter (ports pattern)
│   ├── ports.rs     #   ProductRepo trait, MovementLog trait
│   └── surreal/     #   SurrealDB implementation
├── wire/            # REST API (Axum handlers)
│   └── inventory.rs #   GET/POST/PUT/DELETE /api/v1/inventory/products
└── ui/              # Dioxus app
    ├── app.rs       #   App shell (topbar + router)
    ├── inventory.rs #   Pages + components
    └── server_fns.rs #  #[server] functions
```

### Data flow
```
Browser → [Dioxus WASM] → #[server] fn → flow → gate → SurrealDB
                        → REST API    → flow → gate → SurrealDB
```

## 5. Common tasks

### Add a field to Product
1. `src/core/product.rs`: add field to `Product` + `NewProduct`
2. `src/gate/surreal/repo.rs`: field must exist in UPDATE query SET clause
3. `Backend/W/1.Dat/1a.Sche/Inventory.surql`: add `DEFINE FIELD` if SCHEMAFULL

### Add a new REST endpoint
1. Define async fn in `src/flow/inventory.rs`
2. Add handler in `src/wire/inventory.rs` (route in `pub fn router()`)
3. Add server fn in `src/ui/server_fns.rs` (if FE needs it)

### Add a page
1. Add variant to `Route` enum in `src/ui/inventory.rs` with `#[route("/path")]`
2. Create `#[component] fn MyPage() -> Element { ... }`
3. Link: `<Link to: Route::MyPage {}>`

## 6. Quick gotchas

- `#[server]` macros: server compiles route at startup. Adding new = restart
- `SurrealDB 3.x` uses `type::record('table', $id)` in queries (not `type::thing`)
- Prices in `i64` cents (COP). Never `f32`/`f64` — float drift kills money
- `rsx!` dynamic children: never wrap in closures. Direct `match signal.read()...` in the body
- `async move { ... }` inside `onsubmit`: the closure captures, inner block awaits
