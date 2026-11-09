# Orbe — Conventions

> One page. Pin to top of every PR. Updates require explicit `amend CONVENTIONS.md` + SPEC §C entry.

## 1. Repository layout (standard paths)

> **Status:** v0.3 — replaced JuanJo.Decimal Q/W/Z (friction > benefit at this size).
> **Apply when:** creating files, dirs, scaffolding, reviewing structure.

### Rules

- **No spaces in any path.** Non-negotiable.
- **Machine-read dirs are `lowercase-kebab-case`** — what a tool walks: `crates/`, `database/`, `scripts/`, `config/`, `docs/decks/`.
- **Human-read docs are `PascalCase`** — `docs/decisions/SurrealDB.md`, `README.md`, `SPEC.md`.
- **Files follow their language/tool convention.** Rust `snake_case.rs`, `mod.rs`, `Cargo.toml`, `Dioxus.toml`, `main.css`, `.surql`, `.toon`, `.typ`.
- **No numeric prefixes.** Ordering comes from the reader's alphabet, not from `3a.` — if order truly matters, the file is a sequence and belongs in one file.
- **Depth budget: 3 levels.** Anything deeper than `docs/decks/onboarding/` needs a reason.

### Tree

```
Orbe/
├── Cargo.toml                    # workspace root — members, profiles, single target/
├── crates/
│   └── orbe-api/                 # the only crate for MVP
│       ├── Cargo.toml            # features: server (default) | web | desktop | mobile
│       ├── Dioxus.toml           # default_platform = "web"
│       ├── public/               # served at root — main.css, manifest.json, icons
│       └── src/{core,flow,gate,wire,ui}/
├── database/
│   ├── schema/Inventory.surql
│   └── seeds/*.toon
├── config/Env.example
├── scripts/                      # operator + CI scripts (bash)
│   ├── smoke.sh
│   ├── check-bilingual-parity.sh
│   └── ci-report.sh
├── docs/
│   ├── decisions/                # ADRs — PascalCase
│   ├── decks/
│   │   ├── onboarding/{onboarding.en,onboarding.es}.md
│   │   └── client/{overview.typ,template.md,out/}
│   ├── archive/                  # superseded plans
│   └── PLAN.md
├── SPEC.md · README.md · CONVENTIONS.md · DESIGN.md
```

> `docker/`, `migrations/`, `benches/`, `examples/` may appear when the tool chain demands it. Framework names (`src/`, `public/`, `target/`) are never renamed.

## 2. `orbe-api` internal layout (Dioxus-native + Clean Arch)

```
crates/orbe-api/
├── Cargo.toml           # dioxus = { features = ["fullstack"] }; axum/tokio/surrealdb behind feature = "server"
├── Dioxus.toml          # default_platform = "web"
├── public/              # static assets served at root
│   ├── main.css         # design tokens as CSS vars
│   ├── manifest.json    # PWA manifest
│   └── icon-{192,512}.svg
└── src/
    ├── main.rs          # #[cfg(feature="server")] axum boot / else dioxus::launch(App)
    ├── core/            # ENTITIES + value objects + domain rules. Pure. No I/O.
    ├── flow/            # USE CASES. Orchestrate core + gate traits. No axum, no surrealdb types.
    ├── gate/
    │   ├── ports.rs     # TRAITS: ProductRepo, MovementRepo, CategoryRepo, SupplierRepo
    │   └── surreal/     # adapter — implements ports. Only place that imports `surrealdb::*`.
    ├── wire/            # axum router. Imports `gate/ports.rs` ONLY.
    └── ui/              # Dioxus app: App, pages, components, server_fns (#[server])
```

**Hard rule (V1, V7):**
```
core/  ←  flow/  ←  gate/ports.rs  ←  gate/surreal/
                                  ←  wire/
                                  ←  ui/ (via #[server] fns)
```
Reverse arrows are forbidden. Enforced by:
- `just layer-check` — greps for `use crate::gate::surreal` outside `gate/surreal/` and `main.rs`, and for `crate::core`/`crate::flow` inside `wire/`.
- review: same grep per PR.

## 3. Naming

| What | Convention | Example |
|---|---|---|
| Crates | `orbe-<role>` (single role per crate) | `orbe-api` |
| Modules | `snake_case` | `gate::surreal` |
| Types | `PascalCase` | `Product`, `StockMovement` |
| Functions | `snake_case` | `list_products`, `adjust_stock` |
| Errors | `thiserror` enum per module, suffixed `Error` | `flow::Error` |
| Server fns | `snake_case`, named after the use case | `#[server] async fn list_products() -> Result<Vec<Product>>` |
| Surreal tables | `snake_case`, singular (matches SCHEMAFULL `record<table>`) | `product`, `stock_movement` |
| Surreal fields | `snake_case` | `price_cents`, `created_at` |
| Money | integer minor units, `i64`, suffixed `_cents` | `price_cents: 3500` = $35.00 COP |
| CSS vars | `--o-<role>` matching DESIGN.md tokens | `--o-purple`, `--o-surface` |

## 4. `#[server]` function — the FE→data contract

```rust
#[server]
async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    // 1. call flow use case — NEVER gate/surreal directly
    let repo = ctx_use_product_repo();
    flow::list_products(&repo).await.map_err(into_server_fn_error)
}
```

- Lives in `src/ui/server_fns.rs` (one module per page or feature).
- Returns `Result<T, ServerFnError>` — never panics across the wire.
- Re-uses flow use cases; wire handlers and server fns share the same flow.
- Wire-layer HTTP routes call the same flow fns directly (for non-Dioxus clients: curl, mobile native, future 3rd-party).

## 5. `.toon` for seed & fixture data

`.json` is banned from `database/seeds/`. Format:

```toon
products[3]{sku,name,price_cents,stock_qty}:
  A001,Coca-Cola 350ml,350000,48
  A002,Agua Cristal 600ml,250000,120
  A003,Papas Margarita 30g,180000,30
```

Hand-rolled loader in `gate/surreal/seed.rs` — 8-line parser, no extra dep. Cargo.toml/Dioxus.toml stay native TOML.

## 6. Errors

- Domain errors in `core/` → `thiserror` enum, no `String`.
- Flow errors wrap domain errors + add `NotFound`, `Conflict`, `Validation`.
- Wire maps `flow::Error` → HTTP status via one `match` in `wire/error.rs`.
- `#[server]` fns wrap into `ServerFnError` with a stable message string for the FE.

## 7. Tests

- Unit: domain invariants in `core/` (`stock_qty >= 0`, `sku != ""`, sku uniqueness rule encoded as a Set check).
- Flow: use case happy path + each invariant violation → expected `Err`. **Required before Phase 3 closes.**
- Wire: `scripts/smoke.sh` against a live server (REST + server-fn RPC).
- No test in `gate/surreal/` for MVP — covered by manual E2E + smoke.

## 8. Version control, CI and commits

- One commit per `T<n>` task minimum. `jj` colocated git.
- Commit prefix: `feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`, `perf:`.
- `B<n>` in SPEC §B references the bug → the `fix:` commit message includes `Fixes B<n>`.
- **`just commit "msg"`** is the only sanctioned commit path: it runs `just ci`, writes `target/ci/report.json`, and refuses to commit on red. `just commit --force "msg"` for emergencies (recorded in the report).
- No force-push, no skipped hooks.
- **Remotes:** `upd` (private primary) · `codeberg` (public mirror) · `github` (optional mirror).

## 9. What we DON'T do

- ❌ JS, TS, Node, npm, pnpm, bun, deno, vite, webpack, esbuild, swc, rollup, parsec.
- ❌ Vue, React, Svelte, Solid, vanilla-bean, ant runtime.
- ❌ Bevy (game engine; wrong tool for data apps).
- ❌ UnoCSS / Tailwind class soup — use semantic CSS classes from DESIGN.md tokens.
- ❌ `unsafe` outside `gate/surreal/` and wire adapter wiring.
- ❌ `.unwrap()` in flow/, wire/, ui/. Allowed in `main.rs` startup paths and tests.
- ❌ New top-level dirs without a SPEC §C entry — the tree in §1 is the whole list.
- ❌ JSON in `database/seeds/` (TOON only).
- ❌ Numeric-prefixed dirs (`3a.Onbo`) — retired with Q/W/Z.
- ❌ `float`/`decimal` for money — `i64` cents.
