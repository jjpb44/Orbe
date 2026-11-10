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
| C1 | SurrealDB 3.x (Rust SDK kv-mem) — single binary dev, cluster prod. Transitive `surrealdb-core` rides a beta line: `Cargo.lock` is committed and authoritative (see B2) |
| C2 | Rust backend — Axum 0.8, Clean Architecture (Core/Flow/Gate/Wire) |
| C3 | Dioxus 0.7 + RSX + signals — fullstack over Axum, WASM (web/desktop/mobile) |
| C4 | Standard paths — `crates/orbe-api/`, `database/{schema,seeds}/`, `config/`, `scripts/`, `docs/{decisions,decks,archive}/`. No numeric-prefixed dirs. Framework names (`src/`, `public/`) never renamed. See CONVENTIONS §1 |
| C5 | jj (jujutsu) VCS — remotes: `upd` (private primary, `upd-git@ssh.upd.dev:jjpb/Orbe.git`) · `codeberg` (public mirror, `SethDev/Orbe`) · `github` (public mirror, `jjpb44/Orbe`). All three track `main`; `just push` fans out |
| C6 | No auth, no billing, no hosted CI for Phase 1 — local gate only (`just ci`) |
| C7 | Dark-first UI — no light theme MVP |
| C8 | Single monorepo, single crate `orbe-api` under a root Cargo workspace. A second crate only when a second consumer exists |
| C9 | Everything must compile & run locally before commit — enforced by `just commit` |
| C10 | `cargo build` (server) + `dx build --platform web` (WASM); no JS toolchain |
| C11 | `.toon` for seed/fixture data in `database/seeds/`; Cargo.toml/Dioxus.toml stay native |
| C12 | presenterm for interactive terminal decks only — `docs/decks/onboarding/onboarding.{en,es}.md` (bilingual, parity enforced by `scripts/check-bilingual-parity.sh`); built-in `catppuccin-mocha` via `-t`. Client-facing PDFs are **typst**, not presenterm (no TTY hack, no `script -qc`) |
| C13 | `just ci` is the CI. Writes `target/ci/report.json` (input hash + per-check result). Self-attested, local, zero quota. No hosted CI runners |
| C14 | Money is `i64` minor units (`*_cents`). No `float`, no `decimal` on the wire or in Rust |

### Change log
- 2026-07-13 — Top-level P/R/A → Q/W/Z (C4, C12, T2, T5, T6 paths updated).
- 2026-11-09 — Q/W/Z → standard paths (C4, all §T paths). Root Cargo workspace added (C8). Local CI gate `just ci` (C6, C13). Money pinned to `i64` cents (C14) — §I reconciled with code. Remotes split primary/mirror (C5). presenterm narrowed to interactive decks; client PDFs → typst (C12).

## §I — Interfaces

### Backend API (tag: `inventory`)

REST (non-Dioxus clients), mounted at `/api/v1/inventory`:

```
api: GET    /api/v1/inventory/products             → 200 [{id,sku,name,price_cents,stock,min_stock,category?,supplier?}]
api: POST   /api/v1/inventory/products             → 200 {id,sku,...}
api: GET    /api/v1/inventory/products/{id}        → 200 {id,...}
api: PUT    /api/v1/inventory/products/{id}        → 200 {id,...}
api: DELETE /api/v1/inventory/products/{id}        → 204
api: POST   /api/v1/inventory/products/{id}/stock  → 200 {product,movement}
api: GET    /api/v1/inventory/products/{id}/movements → 200 [{product,delta,kind,reason,at}]
```

Server functions (Dioxus RPC), mounted at `/api` — same flow, `new:` envelope:
`list_products` · `create_product` · `get_product` · `update_product` · `delete_product` · `adjust_stock` · `list_movements`.

Every product carries `id == sku` (SKU is the natural key and the record id).

### SurrealDB Schema

Source of truth: `database/schema/Inventory.surql` (idempotent, applied at boot by `gate::apply_schema`).

```sql
DEFINE TABLE IF NOT EXISTS product SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS sku        ON TABLE product TYPE string ASSERT $value != NONE AND string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS name       ON TABLE product TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS category   ON TABLE product TYPE option<record<category>>;
DEFINE FIELD IF NOT EXISTS supplier   ON TABLE product TYPE option<record<supplier>>;
DEFINE FIELD IF NOT EXISTS price      ON TABLE product TYPE int DEFAULT 0 ASSERT $value >= 0;   -- minor units (cents)
DEFINE FIELD IF NOT EXISTS stock      ON TABLE product TYPE int DEFAULT 0 ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS min_stock  ON TABLE product TYPE int DEFAULT 0 ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE product TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE product TYPE datetime DEFAULT time::now();
DEFINE INDEX IF NOT EXISTS product_sku ON TABLE product COLUMNS sku UNIQUE;

DEFINE TABLE IF NOT EXISTS category SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS slug ON TABLE category TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS name ON TABLE category TYPE string ASSERT $value != NONE;
DEFINE INDEX IF NOT EXISTS category_slug ON TABLE category COLUMNS slug UNIQUE;

DEFINE TABLE IF NOT EXISTS supplier SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS nit  ON TABLE supplier TYPE option<string>;
DEFINE FIELD IF NOT EXISTS name ON TABLE supplier TYPE string ASSERT $value != NONE;
DEFINE INDEX IF NOT EXISTS supplier_nit ON TABLE supplier COLUMNS nit UNIQUE;

DEFINE TABLE IF NOT EXISTS stock_movement SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS product  ON TABLE stock_movement TYPE record<product> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS delta    ON TABLE stock_movement TYPE int ASSERT $value != 0;
DEFINE FIELD IF NOT EXISTS kind_str ON TABLE stock_movement TYPE string ASSERT $value IN ["in", "out", "adjust"];
DEFINE FIELD IF NOT EXISTS reason   ON TABLE stock_movement TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS at       ON TABLE stock_movement TYPE datetime DEFAULT time::now();
```

## §V — Invariants

| id | invariant |
|----|-----------|
| V1 | Wire layer imports `flow/` + `core/` DTOs only — ⊥ import `gate/surreal` adapters (⊥ `surrealdb::*`) |
| V2 | ∀ product → `sku` UNIQUE |
| V3 | ∀ category → `slug` UNIQUE |
| V4 | ∀ stock adjustment → `stock_movement` record created |
| V5 | `stock_qty` ≥ 0 ∀ product |
| V6 | `created_at` READONLY — never mutable after insert |
| V7 | `#[server]` fns = sole FE→data path — Dioxus UI ⊥ import from `gate/surreal/` (only `gate/ports.rs` traits) |
| V8 | SurrealDB writes datetime/record fields via parametric query w/ server-side `time::now()` + `type::thing('table',$id)` — ⊥ `.content()`/`.merge()` w/ serde `DateTime` ISO string \| tuple-as-record (SCHEMAFULL rejects both) |
| V9 | Money = `i64` minor units end-to-end (`*_cents` in Rust, `int` in SurrealDB) — ⊥ `float`/`decimal`/string money |
| V10 | `Cargo.lock` committed — never deleted or regenerated without a green `just ci` + deliberate bump (B2) |

## §T — Tasks

### Phase 1: Scaffold

| id | status | task | cites |
|----|--------|------|-------|
| T1 | x | jj git init --colocate | C5 |
| T2 | x | `crates/orbe-api/Cargo.toml` — single crate, `features = ["server"]` | C8,C10 |
| T3 | x | Scaffold `src/{core,flow,gate,wire,ui}/` modules — stub structs, ports.rs traits | C2,V1,V7 |
| T4 | x | Wire: server fn returns empty `[]`; OpenAPI deferred | C2,V1,V7 |
| T5 | x | SurrealDB schema `database/schema/Inventory.surql` | C1,I.surreal |
| T6 | x | `config/Env.example` — SURREAL_URL, SURREAL_USER, SURREAL_PASS, ORBE_PORT | C1 |
| T7 | x | ADRs — `docs/decisions/{SurrealDB,LayerNaming,DioxusFullstack}.md` | C1,C2,C10 |
| T8 | x | `cargo build` — zero errors | C9 |
| T9 | x | `Dioxus.toml` — `default_platform = "web"`, hot reload watch | C3,C10 |
| T10 | x | `src/ui/` — App component, routes, `#[server] fn list_products()` | C3,V7 |
| T11 | x | Inventory page stub — renders empty list via `use_resource` | C3 |
| T12 | x | CSS vars from DESIGN.md in `public/main.css` | C7,C11 |
| T13 | x | PWA manifest + icons (192/512 SVG) in `public/` | C3 |
| T14 | x | `cargo run` boots Axum on :8080; `dx serve` fullstack hot reload verified | C9,C10 |
| T15 | x | Pushed: `upd` (private primary) + `codeberg` + `github` mirrors — all at `main` 0c20e44a | C5 |

### Phase 2: Inventory CRUD

| id | status | task | cites |
|----|--------|------|-------|
| T16 | x | Core entities — Product, Category, Supplier, Movement (+11 unit tests, V2/V3/V5 green) | V2,V3,V5 |
| T17 | x | Flow use cases — create/update/delete/list/adjust (6 fns + FlowError) | V4,V5 |
| T18 | x | SurrealDB gate adapter — SurrealProductRepo + SurrealMovementLog (V8 queries) | V1,V8 |
| T19 | x | Wire Axum handlers → gate ports (7 routes, ApiError→status) | V1,I.api |
| T20 | x | Dioxus signals + `#[server]` flow wrappers (7 server fns) | I.api,V7 |
| T21 | x | ProductList + ProductCard + InventoryPage components | C3 |
| T22 | x | Stock adjustment — inline form in ProductCard, `#[server] adjust_stock` | V4,V5 |
| T23 | x | Smoke suite — `scripts/smoke.sh` (REST + RPC) | V4,V5 |

### Phase 3: Modernize & package

| id | status | task | cites |
|----|--------|------|-------|
| T24 | x | Paths migration Q/W/Z → standard tree + root Cargo workspace | C4,C8 |
| T25 | x | Workspace profiles — deps debuginfo off, single `target/`; `just clean-ship` / `just target-size` for disk control | C9 |
| T26 | x | `just ci` + `just commit` + `scripts/ci-report.sh` — hash + JSON report, no quota | C6,C13 |
| T27 | x | justfile rewrite — banner helper, non-interactive, `layer-check` | C9 |
| T28 | x | PWA service worker — `public/sw.js` (shell cache, API bypass) served at `/sw.js`, registered from `App` | C3 |
| T29 | x | WASM budget enforced — release 415 KB gzip vs 500 KB, `scripts/wasm-size.sh` in `just ci-full` | C3 |
| T30 | . | Android smoke build — BLOCKED: no ANDROID_HOME/NDK on this machine (targets installed) | C3 |
| T31 | x | Client PDF → `docs/decks/client/overview.typ` (typst `--input`), presenterm kept for onboarding only | C12 |
| T32 | x | Flow-layer tests — 7 tests on a fake port: adjust-below-zero → Err + zero movements (V4/V5), 404s | V4,V5 |
| T33 | x | Seed loader — `gate/surreal/seed.rs` (hand-rolled TOON parser + idempotent CREATE), `ORBE_SEED=1` / `just seed` | C11 |

## §B — Bugs

| id | date | cause | fix |
|----|------|-------|-----|
| B1 | 2026-06-26 | SCHEMAFULL rejects serde `DateTime` ISO string + serde tuple `("table",id)` as `record<table>` on `.content()`/`.merge()` — 3 symptoms (update 500, adjust 500, movement insert fail) | V8 |
| B2 | 2026-11-09 | Deleting `Cargo.lock` re-resolved `surrealdb 3.2.0-beta.2`'s beta-line transitive deps to `surrealdb-core 3.3.5` → 34 compile errors (`kvs::LockType`, `rpc::DbResultStats` gone). Beta crate version does not pin its core. | V10 (lock committed); fix = restore lock, bump only after green CI |
| B3 | 2026-11-09 | Oversell (`adjust_stock` below 0) surfaced as 500: V5 was only enforced by the SurrealDB ASSERT, so the failure came back as `RepoError::Backend` | V5 checked in `flow::adjust_stock` before the adapter → 422 + no movement |
| B4 | 2026-11-09 | `DELETE` on a missing product returned 204 — `flow::delete_product` never read the record, so a typo looked like success | `repo.get()` first → `RepoError::NotFound` → 404 |
