# Orbe — Conventions

> One page. Pin to top of every PR. Updates require explicit `amend CONVENTIONS.md` + SPEC §C entry.

## 1. Repo layout (JuanJo.Decimal, top-level only)

```
Orbe/
├── Backend/         # ALL Rust — workspace lives here
│   ├── P/           # crates (production code)
│   │   └── orbe-api/
│   ├── R/           # resources (schema, config, seeds)
│   │   ├── 1.Db/    # 1a.Sch/ schemas · 2.Sd/ seeds
│   │   └── 2.Cf/    # Env.example, runtime config
│   └── A/           # ADRs (records of decision)
├── Frontend/        # RESERVED — empty until a second consumer needs a separate FE stack
├── DESIGN.md
├── README.md
├── SPEC.md
├── CONVENTIONS.md   # this file
└── PLAN.md
```

**Rule:** new top-level dir under `Backend/` → must be `P/`, `R/`, or `A/`. Same for `Backend/R/` subdirs.

## 2. `orbe-api` internal layout (Dioxus-native + Clean Arch)

```
Backend/P/orbe-api/
├── Cargo.toml           # features = ["server"] optional; dioxus = { features = ["fullstack"] }
├── Dioxus.toml          # default_platform = "web"
├── assets/              # static assets served at root
│   ├── main.css         # design tokens as CSS vars
│   └── manifest.json    # PWA manifest
└── src/
    ├── main.rs          # #[cfg(feature="server")] axum boot / else dioxus::launch(App)
    ├── core/            # ENTITIES + value objects + domain rules. Pure. No I/O.
    ├── flow/            # USE CASES. Orchestrate core + gate traits. No axum, no surrealdb types.
    ├── gate/
    │   ├── ports.rs     # TRAITS: ProductRepo, MovementRepo, CategoryRepo, SupplierRepo
    │   └── surreal/     # adapter — implements ports. Only place that imports `surrealdb::*`.
    ├── wire/            # axum router + utoipa OpenAPI. Imports `gate/ports.rs` ONLY.
    └── ui/              # Dioxus app: App, pages, components, server_fns (#[server])
```

**Hard rule (V1, V7):**
```
core/  ←  flow/  ←  gate/ports.rs  ←  gate/surreal/
                                  ←  wire/
                                  ←  ui/ (via #[server] fns)
```
Reverse arrows are forbidden. Enforced by:
- review: every PR grep'd for `use crate::gate::surreal` outside `gate/surreal/` and `wire/main.rs` adapter wiring
- future: `cargo-deny` dep rules + `cargo-modules` lint in CI (Phase 3)

## 3. Naming

| What | Convention | Example |
|---|---|---|
| Crates | `orbe-<role>` (single role per crate) | `orbe-api` |
| Modules | `snake_case` | `gate::surreal` |
| Types | `PascalCase` | `Product`, `StockMovement` |
| Functions | `snake_case` | `list_products`, `adjust_stock` |
| Errors | `thiserror` enum per module, suffixed `Error` | `flow::Error` |
| Server fns | `snake_case`, named after the use case | `#[server] async fn list_products() -> Result<Vec<Product>>` |
| Surreal tables | `snake_case`, plural preferred | `product`, `stock_movement` |
| Surreal fields | `snake_case` | `stock_qty`, `created_at` |
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

`.json` is banned from `Backend/R/1.Db/2.Sd/`. Format:

```toon
products[3]{sku,name,price,stock_qty}:
  A001,Coca-Cola 350ml,3500,48
  A002,Agua Cristal 600ml,2500,120
  A003,Papas Margarita 30g,1800,30
```

Hand-rolled loader in `gate/surreal/seed.rs` — 8-line parser, no extra dep. Cargo.toml/Dioxus.toml stay native TOML.

## 6. Errors

- Domain errors in `core/` → `thiserror` enum, no `String`.
- Flow errors wrap domain errors + add `NotFound`, `Conflict`, `Validation`.
- Wire maps `flow::Error` → HTTP status via one `match` in `wire/error.rs`.
- `#[server]` fns wrap into `ServerFnError` with a stable message string for the FE.

## 7. Tests

- Unit: domain invariants in `core/` (`stock_qty >= 0`, `sku != ""`, sku uniqueness rule encoded as a Set check).
- Flow: use case happy path + each invariant violation → expected `Err`.
- Wire: only smoke tests on the OpenAPI doc generation, no full HTTP integration in MVP (Phase 3).
- No test in `gate/surreal/` for MVP — covered by Phase 2 manual E2E + Phase 3 docker-compose test.

## 8. Git hygiene

- One commit per `T<n>` task minimum. Squash before push to keep `main@origin` linear.
- Commit prefix: `feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`.
- `B<n>` in SPEC §B references the bug → the `fix:` commit message includes `Fixes B<n>`.
- No force-push, no `--no-verify`, no skipped hooks. Empty commits only via `jj commit --allow-empty` for WIP checkpoints.

## 9. What we DON'T do

- ❌ JS, TS, Node, npm, pnpm, bun, deno, vite, webpack, esbuild, swc, rollup, parsec.
- ❌ Vue, React, Svelte, Solid, vanilla-bean, ant runtime.
- ❌ Bevy (game engine; wrong tool for data apps).
- ❌ UnoCSS / Tailwind class soup — use semantic CSS classes from DESIGN.md tokens.
- ❌ `unsafe` outside `gate/surreal/` and wire adapter wiring.
- ❌ `.unwrap()` in flow/, wire/, ui/. Allowed in `main.rs` startup paths and tests.
- ❌ New top-level dir under `Backend/` other than `P/`, `R/`, `A/`.
- ❌ JSON in `Backend/R/1.Db/2.Sd/` (TOON only).