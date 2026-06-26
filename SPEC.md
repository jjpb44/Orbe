# Orbe — SPEC

## §G — Goal

Open-source CRM for Colombian SME entrepreneurs.
Phase 1: Inventory MVP — no auth, no billing.
UX: Nequi, Nu, WhatsApp — fast, bold, minimal, tactile.
Feel: immersive, magical, instant.

## §C — Constraints

| id | constraint |
|----|------------|
| C1 | SurrealDB 2.x — single binary dev, cluster prod |
| C2 | Rust backend — Axum 0.8, Clean Architecture (Core/Flow/Gate/Wire) |
| C3 | Dioxus 0.7 + RSX + signals — fullstack over Axum, WASM (web/desktop/mobile) |
| C4 | JuanJo.Decimal file convention (P/R/A top-level) |
| C5 | jj (jujutsu) VCS — upd.dev/Codeberg primary, GitHub mirror |
| C6 | No auth, no billing, no CI for Phase 1 |
| C7 | Dark-first UI — no light theme MVP |
| C8 | Single monorepo — `Backend/` (Rust); `Frontend/` reserved for future FE stack split |
| C9 | Everything ! compile & run locally before first commit |
| C10 | Single Rust crate `orbe-api` — `cargo build` (server) + `dx build --platform web` (WASM); no JS toolchain |
| C11 | `.toon` for seed/fixture data; Cargo.toml/Dioxus.toml stay native |

## §I — Interfaces

### Backend API (tag: `inventory`)

```
api: GET    /api/v1/inventory/products          → 200 [{id,sku,name,price,stock_qty,category?,supplier?}]
api: POST   /api/v1/inventory/products          → 201 {id,sku,name,...}
api: GET    /api/v1/inventory/products/{id}     → 200 {id,...}
api: PUT    /api/v1/inventory/products/{id}     → 200 {id,...}
api: DELETE /api/v1/inventory/products/{id}     → 204
api: POST   /api/v1/inventory/products/{id}/stock → 200 {delta,reason,new_qty}
api: GET    /api/v1/inventory/movements         → 200 [{id,product,delta,reason,created_at}]
```

### SurrealDB Schema

```sql
DEFINE NAMESPACE orbe;
USE NS orbe DB inventory;

DEFINE TABLE product SCHEMAFULL;
DEFINE FIELD id          ON product TYPE record<product>;
DEFINE FIELD sku         ON product TYPE string  ASSERT $value != NONE;
DEFINE FIELD name        ON product TYPE string  ASSERT $value != NONE;
DEFINE FIELD description ON product TYPE option<string>;
DEFINE FIELD price       ON product TYPE decimal ASSERT $value >= 0;
DEFINE FIELD stock_qty   ON product TYPE int     ASSERT $value >= 0;
DEFINE FIELD category    ON product TYPE option<record<category>>;
DEFINE FIELD supplier    ON product TYPE option<record<supplier>>;
DEFINE FIELD created_at  ON product TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD updated_at  ON product TYPE datetime VALUE time::now();

DEFINE TABLE category SCHEMAFULL;
DEFINE FIELD id   ON category TYPE record<category>;
DEFINE FIELD name ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON category TYPE string ASSERT $value != NONE;

DEFINE TABLE supplier SCHEMAFULL;
DEFINE FIELD id    ON supplier TYPE record<supplier>;
DEFINE FIELD name  ON supplier TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON supplier TYPE option<string>;
DEFINE FIELD phone ON supplier TYPE option<string>;

DEFINE TABLE stock_movement SCHEMAFULL;
DEFINE FIELD id         ON stock_movement TYPE record<stock_movement>;
DEFINE FIELD product    ON stock_movement TYPE record<product>;
DEFINE FIELD delta      ON stock_movement TYPE int;
DEFINE FIELD reason     ON stock_movement TYPE string;
DEFINE FIELD created_at ON stock_movement TYPE datetime VALUE time::now() READONLY;

DEFINE INDEX idx_product_sku   ON product  COLUMNS sku  UNIQUE;
DEFINE INDEX idx_category_slug ON category COLUMNS slug UNIQUE;
```

## §V — Invariants

| id | invariant |
|----|-----------|
| V1 | Wire layer (orbe-api) ⊥ import from `core/` \| `flow/` — ONLY from `gate/ports.rs` |
| V2 | ∀ product → `sku` UNIQUE |
| V3 | ∀ category → `slug` UNIQUE |
| V4 | ∀ stock adjustment → `stock_movement` record created |
| V5 | `stock_qty` ≥ 0 ∀ product |
| V6 | `created_at` READONLY — never mutable after insert |
| V7 | `#[server]` fns = sole FE→data path — Dioxus UI ⊥ import from `gate/surreal/` (only `gate/ports.rs` traits) |

## §T — Tasks

### Phase 1: Scaffold

| id | status | task | cites |
|----|--------|------|-------|
| T1 | x | jj git init --colocate | C5 |
| T2 | x | Create `Backend/P/orbe-api/Cargo.toml` — single crate, `features = ["server"]` | C10 |
| T3 | x | Scaffold `src/{core,flow,gate,wire,ui}/` modules — stub structs, ports.rs traits | C2,V1,V7 |
| T4 | ~ | Wire: server fn returns empty `[]`; utoipa OpenAPI deferred to P2 | C2,V1,V7 |
| T5 | x | Write SurrealDB schema `Backend/R/1.Db/1a.Sch/Inventory.surql` | C1,I.surreal |
| T6 | x | Write `Backend/R/2.Cf/Env.example` — SURREAL_URL, SURREAL_USER, SURREAL_PASS, ORBE_PORT | C1 |
| T7 | x | Generate ADRs — SurrealDB.md, LayerNaming.md, DioxusFullstack.md | C1,C2,C10 |
| T8 | x | `cargo build` — zero errors (290 crates, 55s cold; ~0.3s incremental) | C9 |
| T9 | x | `Dioxus.toml` — `default_platform = "web"`, hot reload watch | C3,C10 |
| T10 | x | `src/ui/` — App component, routes, `#[server] fn list_products()` | C3,V7 |
| T11 | x | Inventory page stub — renders empty list via `use_resource` | C3 |
| T12 | x | CSS vars from DESIGN.md in `public/main.css` (Dioxus default `public_dir`) | C7,C11 |
| T13 | . | PWA manifest + icons — deferred to P3 (not MVP-critical) | C3 |
| T14 | ~ | `cargo run` boots Axum on :8080, browser shows "Phase 1 scaffold"; `dx` CLI install + `dx serve` deferred to P2 | C9,C10 |
| T15 | . | Push to upd.dev/jjpb/Orbe.git — blocked on SSH egress (user pushes manually) | C5 |

### Phase 2: Inventory CRUD

| id | status | task | cites |
|----|--------|------|-------|
| T16 | . | Implement core entities — Product, Category, Supplier, Movement | V2,V3,V5 |
| T17 | . | Implement flow use cases — create/update/delete/list/adjust | V4,V5 |
| T18 | . | Implement SurrealDB gate adapter — ports.rs impl | V1 |
| T19 | . | Wire Axum handlers → gate ports | V1,I.api |
| T20 | . | Dioxus signals + `#[server]` flow wrappers for inventory page | I.api,V7 |
| T21 | . | ProductList + ProductCard + ProductForm components | C3 |
| T22 | . | Stock adjustment modal + badge | V4,V5 |
| T23 | . | E2E test: create product → list → adjust stock | V4,V5 |

## §B — Bugs

| id | date | cause | fix |
|----|------|-------|-----|
