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
| C2 | Rust backend — Axum 0.8, Clean Architecture |
| C3 | Vue 3 + Vite + UnoCSS + Pinia — PWA |
| C4 | JuanJo.Decimal file convention (P/R/A top-level) |
| C5 | jj (jujutsu) VCS — Codeberg primary, GitHub mirror |
| C6 | No auth, no billing, no CI for Phase 1 |
| C7 | Dark-first UI — no light theme MVP |
| C8 | Single monorepo — `Backend/` + `Frontend/` root dirs |
| C9 | Everything ! compile & run locally before first commit |

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

## §T — Tasks

### Phase 1: Scaffold

| id | status | task | cites |
|----|--------|------|-------|
| T1 | . | jj git init --colocate | C5 |
| T2 | . | Create workspace Cargo.toml `members = ["Backend/P/orbe-api", "Backend/P/orbe-inv"]` | C2,C8 |
| T3 | . | Scaffold Backend Core/Flow/Gate/Wire layers — stub structs, ports.rs traits | C2,V1 |
| T4 | . | Scaffold Axum wire handlers — compile clean | C2,V1 |
| T5 | . | Write SurrealDB schema `Backend/R/1.Db/1a.Sch/Inventory.surql` | C1,I.surreal |
| T6 | . | Write `Backend/R/2.Cf/Env.example` — SURREAL_URL, SURREAL_USER, SURREAL_PASS | C1 |
| T7 | . | Generate ADRs — SurrealDB.md, LayerNaming.md | C1,C2 |
| T8 | . | `cargo build` — zero errors | C9 |
| T9 | . | Vite + Vue 3 scaffold — `Frontend/P/` structure | C3,C8 |
| T10 | . | UnoCSS config + design tokens | C3,C7 |
| T11 | . | Scaffold inventory components — stubs render clean | C3 |
| T12 | . | Router config — `/` → inventory list | C3 |
| T13 | . | PWA manifest + icons | C3 |
| T14 | . | `npm run dev` — zero errors | C9 |
| T15 | . | Push to upd.dev/jjpb/Orbe.git | C5 |

### Phase 2: Inventory CRUD

| id | status | task | cites |
|----|--------|------|-------|
| T16 | . | Implement `orbe-inv` core entities — Product, Category, Supplier, Movement | V1,V2,V3 |
| T17 | . | Implement flow use cases — create/update/delete/list/adjust | V4,V5 |
| T18 | . | Implement SurrealDB gate adapter — ports.rs impl | V1 |
| T19 | . | Wire Axum handlers → gate ports | V1,I.api |
| T20 | . | Frontend inventory store + API composable | I.api |
| T21 | . | ProductList + ProductCard + ProductForm components | C3 |
| T22 | . | Stock adjustment modal + badge | V4,V5 |
| T23 | . | E2E test: create product → list → adjust stock | V4,V5 |

## §B — Bugs

| id | date | cause | fix |
|----|------|-------|-----|
