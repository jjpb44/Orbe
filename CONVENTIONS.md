# Orbe — Conventions

> One page. Pin to top of every PR. Updates require explicit `amend CONVENTIONS.md` + SPEC §C entry.

## 1. JuanJo.Decimal convention

> **Status:** Draft v0.2 — under active development. Rules may evolve. Flexibility is a feature.
> **Apply when:** creating directories, naming files, scaffolding, or reviewing file structure. Enforces PascalCase, no spaces, and a volatility-based hierarchy (Q/W/Z) with Areas, Categories, and optional IDs. Adapts to framework and language constraints while preserving the spirit of the convention.

### Philosophy

- No spaces. Ever.
- Numeric prefixes for ordering and identification, not for meaning.
- Simple enough to apply by hand. Rigid enough to be consistent.
- Framework conventions always win when there is a genuine conflict — adapt the spirit, not the letter.

### Scope: Dirs vs Files

**JuanJo.Decimal numbering (Areas, Categories, IDs) applies to DIRECTORY names only.**

Files follow a separate, lighter rule:
- **PascalCase preferred** on file names — but not mandatory.
- No numbering scheme enforced on files.
- Framework or language convention takes precedence (e.g. `mod.rs`, `main.ts`, `vite.config.ts`).

```
2a.Prd/                ← dir: full JD (mandatory)
  ProductList.vue      ← file: PascalCase preferred, no JD prefix required
  product_list.rs      ← file: snake_case ok if language demands it
  useProducts.ts       ← file: camelCase tolerated if team prefers it
```

### Top-Level Structure (QWZ — by change frequency)

Every workspace or repo starts with three root dirs. Areas are unified into R.

| Dir | Volatility | Purpose |
|-----|------------|---------|
| `Q/` | High | Active development, current work, things that change frequently |
| `W/` | Low | References, configs, schemas, assets, knowledge |
| `Z/` | Archive | Slow, rarely-changing content. Docs, ADRs, old specs |

> In software repos, framework-mandated dirs (`src/`, `public/`, `crates/`) coexist at root alongside Q/W/Z. Do not rename them — map them mentally: `src/` ≈ Q, config files ≈ W.

### Areas

Format: `{n}.{Xxx}/`

- `{n}` — single digit (1–9), unique within the parent container
- `{Xxx}` — exactly 3 letters, PascalCase (first capitalized, next two lowercase)
- Dot separates number from label

Examples:
```
1.Cor/   ← Core
2.Inv/   ← Inventory
3.Shr/   ← Shared
9.Out/   ← Output
```

### Categories

Format: `{n}{a}.{Xxxx}/`

- `{n}` — parent area number (same digit)
- `{a}` — single lowercase letter, unique within the area (a, b, c…)
- `{Xxxx}` — exactly 4 letters, PascalCase (first capitalized, next three lowercase)
- Dot separates the combined prefix from the label

Examples:
```
1a.Shel/   ← area 1, category a, Shell
1b.Desi/   ← area 1, category b, Design
2a.Prod/   ← area 2, category a, Products
2b.Stck/   ← area 2, category b, Stock
5a.Game/   ← area 5, category a, Games
8a.Inbx/   ← area 8, category a, Inbox
```

### IDs (Optional)

Format: `{na}{nn}.{Name}`

- `{na}` — parent category prefix (e.g. `2a`)
- `{nn}` — two-digit zero-padded sequence number (01–99)
- `{Name}` — arbitrary PascalCase name, any length, no spaces
- Dot separates prefix+sequence from the name
- **This level is optional.** A category without IDs is valid.

Use IDs when:
- Files are created automatically or on the fly
- Content within a category needs explicit ordering
- Archive or deep development dirs need stable references

Examples:
```
2a01.ProductList.vue
2a02.ProductCard.vue
1a01.SurrealDB.md        ← ADR #1
5e12.Linux               ← notes on Linux, area 5, cat e, ID 12
8a11.Inbox               ← fast-landing files
```

### PascalCase Rules

**Dirs** — apply to the label segment of every JD dir:
- Area label: `1.Cor` ✓, `1.cor` ✗, `1.Co` ✗ (must be 3 letters)
- Category label: `2a.Prod` ✓, `2a.prod` ✗, `2a.Prd` ✗ (must be 4 letters)
- Top-level QWZ: `Q`, `W`, `Z` ✓

**Files** — PascalCase preferred, not enforced:
- `ProductList.vue` ✓ preferred
- `product_list.rs` ✓ acceptable (Rust convention)
- `useProducts.ts` ✓ tolerated
- `vite.config.ts`, `Cargo.toml`, `mod.rs` — keep framework names as-is

### Project Adaptation Rules

JuanJo.Decimal bends when a framework demands it. Priority order:

1. **No spaces** — non-negotiable
2. **QWZ top-level** — apply where possible; coexist with framework root dirs
3. **PascalCase** — apply unless language convention explicitly forbids it
4. **Area/Category structure** — apply inside `P/` or `src/`; adapt prefix when identifiers must be valid (e.g. drop dots for module names)
5. **IDs** — always optional; skip freely

### Rust adaptation

Dots and leading digits are invalid in Rust identifiers. Adaptation:
- Use meaningful short names as modules: `core/`, `flow/`, `gate/`, `wire/`
- Annotate with JD reference in `mod.rs` header comment: `// 1.Co — Core domain`
- PascalCase applies to types, structs, enums, traits (idiomatic Rust)

### Anti-Patterns

```
✗ 2a.products/     ← label not PascalCase
✗ 2 a.Prod/        ← space
✗ 2A.Prod/         ← category letter should be lowercase
✗ 2aa.Prod/        ← two letters in category position
✗ 2a.Prd/          ← category label too short (must be 4 letters)
✗ 2a.Produ/        ← category label too long (must be exactly 4 letters)
✗ 1.Co/            ← area label too short (must be 3 letters)
✗ 1.Core/          ← area label too long (must be exactly 3 letters)
```

### Orbe application

```
Orbe/
├── Backend/            # ALL Rust — workspace lives here
│   ├── Q/              # crates (production code)
│   │   └── orbe-api/
│   ├── W/              # resources (schema, config, seeds, decks)
│   │   ├── 1.Dat/      # schemas + seeds
│   │   ├── 2.Cfg/      # env config
│   │   └── 3.Dec/      # presenterm decks — onboarding (bilingual EN/ES) + client PDF template
│   ├── Z/              # ADRs (records of decision)
├── Frontend/           # RESERVED — empty until a second consumer needs a separate FE stack
├── DESIGN.md
├── README.md
├── SPEC.md
├── CONVENTIONS.md      # this file
└── PLAN.md
```

## 2. `orbe-api` internal layout (Dioxus-native + Clean Arch)

```
Backend/Q/orbe-api/
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

`.json` is banned from `Backend/W/1.Dat/2.Sed/`. Format:

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
- ❌ New top-level dir under `Backend/` other than `Q/`, `W/`, `Z/`.
- ❌ JSON in `Backend/W/1.Dat/2.Sed/` (TOON only).