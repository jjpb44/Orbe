# Orbe — SPEC

## §G — Goal

Open-source CRM for Colombian SME entrepreneurs.
Phase 1: Inventory MVP — no auth, no billing.
UX: Nequi, Nu, WhatsApp — fast, bold, minimal, tactile.
Feel: immersive, magical, instant.
Principles: reduce cognitive-load always · transmit the mood explicitly, not as decoration.

## §C — Constraints

| id | constraint |
|----|------------|
| C1 | SurrealDB 3.x (Rust SDK kv-mem) — single binary dev, cluster prod |
| C2 | Rust backend — Axum 0.8, Clean Architecture (Core/Flow/Gate/Wire) |
| C3 | Dioxus 0.7 + RSX + signals — fullstack over Axum, WASM (web/desktop/mobile) |
| C4 | JuanJo.Decimal file convention v0.2 — QWZ top-level (Q/W/Z); Areas=3 letters, Categories=4 letters, IDs optional; framework wins on conflict |
| C5 | jj (jujutsu) VCS — upd.dev/Codeberg primary, GitHub mirror |
| C6 | No auth, no billing, no CI for Phase 1 |
| C7 | Dark-first UI — no light theme MVP |
| C8 | Single monorepo — `Backend/` (Rust); `Frontend/` reserved for future FE stack split |
| C9 | Everything ! compile & run locally before first commit |
| C10 | Single Rust crate `orbe-api` — `cargo build` (server) + `dx build --platform web` (WASM); no JS toolchain |
| C11 | `.toon` for seed/fixture data; Cargo.toml/Dioxus.toml stay native |
| C12 | presenterm for decks — `Backend/W/3.Dec/` with `3a.Onbo/onboarding.{en,es}.md` (bilingual, parity enforced) + `3b.Clnt/template.md` (client, lang-neutral placeholders); built-in `catppuccin-mocha` via `-t` flag (no custom theme, no config file); non-TTY wrapper `script -qc "..."`; client PDFs NEVER execute snippets |

### Change log
- 2026-07-13 — Top-level P/R/A → Q/W/Z (C4, C12, T2, T5, T6 paths updated). Volatility-based hierarchy: Q=active code, W=resources/config/decks, Z=archive/ADRs. See CONVENTIONS.md v0.2 §1.

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
| V8 | SurrealDB writes datetime/record fields via parametric query w/ server-side `time::now()` + `type::thing('table',$id)` — ⊥ `.content()`/`.merge()` w/ serde `DateTime` ISO string \| tuple-as-record (SCHEMAFULL rejects both) |

## §T — Tasks

### Phase 1: Scaffold

| id | status | task | cites |
|----|--------|------|-------|
| T1 | x | jj git init --colocate | C5 |
| T2 | x | Create `Backend/Q/orbe-api/Cargo.toml` — single crate, `features = ["server"]` | C10 |
| T3 | x | Scaffold `src/{core,flow,gate,wire,ui}/` modules — stub structs, ports.rs traits | C2,V1,V7 |
| T4 | x | Wire: server fn returns empty `[]`; utoipa OpenAPI deferred to P2 | C2,V1,V7 |
| T5 | x | Write SurrealDB schema `Backend/W/1.Dat/1a.Sche/Inventory.surql` | C1,I.surreal |
| T6 | x | Write `Backend/W/2.Cfg/Env.example` — SURREAL_URL, SURREAL_USER, SURREAL_PASS, ORBE_PORT | C1 |
| T7 | x | Generate ADRs — SurrealDB.md, LayerNaming.md, DioxusFullstack.md | C1,C2,C10 |
| T8 | x | `cargo build` — zero errors | C9 |
| T9 | x | `Dioxus.toml` — `default_platform = "web"`, hot reload watch | C3,C10 |
| T10 | x | `src/ui/` — App component, routes, `#[server] fn list_products()` | C3,V7 |
| T11 | x | Inventory page stub — renders empty list via `use_resource` | C3 |
| T12 | x | CSS vars from DESIGN.md in `public/main.css` (Dioxus default `public_dir`) | C7,C11 |
| T13 | x | PWA manifest + icons (192/512 SVG) in `public/` | C3 |
| T14 | x | `cargo run` boots Axum on :8080; `dx serve` fullstack hot reload verified | C9,C10 |
| T15 | . | Push to upd.dev/jjpb/Orbe.git — blocked on SSH egress (user pushes manually) | C5 |

### Phase 2: Inventory CRUD

| id | status | task | cites |
|----|--------|------|-------|
| T16 | x | Implement core entities — Product, Category, Supplier, Movement (+11 unit tests, V2/V3/V5 green) | V2,V3,V5 |
| T17 | x | Implement flow use cases — create/update/delete/list/adjust (6 fns + FlowError) | V4,V5 |
| T18 | x | Implement SurrealDB gate adapter — SurrealProductRepo + SurrealMovementLog (V8 queries) | V1,V8 |
| T19 | x | Wire Axum handlers → gate ports (7 routes, ApiError→status) | V1,I.api |
| T20 | x | Dioxus signals + `#[server]` flow wrappers (7 server fns, auto-registered) | I.api,V7 |
| T21 | x | ProductList + ProductCard + InventoryPage components (rsx + Router) | C3 |
| T22 | x | Stock adjustment — inline form in ProductCard, `#[server] adjust_stock` | V4,V5 |
| T23 | x | E2E smoke test — 14 curl tests green (create/list/get/update/delete/adjust/movements) | V4,V5 |

## §B — Bugs

| id | date | cause | fix |
|----|------|-------|-----|
| B1 | 2026-06-26 | SCHEMAFULL rejects serde `DateTime` ISO string + serde tuple `("table",id)` as `record<table>` on `.content()`/`.merge()` — 3 symptoms (update 500, adjust 500, movement insert fail) | V8 |
