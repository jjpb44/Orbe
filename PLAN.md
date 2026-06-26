# Orbe MVP — Phased Plan (Full‑Rust, Dioxus 0.7)

> Status: **proposed** · awaiting user OK before SPEC/CONVENTIONS edits land.
> Target: runnable Phase‑1 Inventory MVP end‑to‑end on a single `dx serve`.

## 0. North star

One Rust binary. One source tree. Two compile targets:

| Target | Output | Used by |
|---|---|---|
| `cargo build` (server feature) | native binary | local `dx serve` / prod |
| `dx build --platform web` | WASM bundle | browser / PWA / iOS‑Android web shell |

Dioxus 0.7 + Axum 0.8 deliver this. **`#[server]` functions are the only data path** — the FE never touches SurrealDB directly. That is the project's structural invariant.

---

## 1. Repo layout (target state)

```
Orbe/
├── Backend/                         # JuanJo.Decimal — Rust crates
│   ├── P/
│   │   └── orbe-api/                # the ONLY crate for MVP
│   │       ├── src/
│   │       │   ├── core/            # entities, value objects, domain rules
│   │       │   ├── flow/            # use cases (create / list / adjust …)
│   │       │   ├── gate/            # ports.rs (traits) + SurrealDB adapter
│   │       │   ├── wire/            # axum router + utoipa OpenAPI
│   │       │   ├── ui/              # Dioxus components + server functions
│   │       │   └── main.rs          # one binary, two cfg axes
│   │       └── Cargo.toml           # features: server | web (mutually compatible via fullstack)
│   ├── R/
│   │   ├── 1.Db/1a.Sch/Inventory.surql
│   │   └── 2.Cf/Env.example
│   └── A/                           # docs (this file, ADRs)
├── Frontend/                        # empty until Phase 3 — see §6
├── DESIGN.md
├── README.md
├── SPEC.md
├── CONVENTIONS.md
└── PLAN.md
```

Rationale:
- One crate is the cheapest path that still keeps Clean Architecture (Core/Flow/Gate/Wire) intact as internal modules.
- The `ui/` module is the Dioxus app; it lives inside the same crate so `#[server]` functions can be `pub(crate)` and reuse the `gate/` ports without crossing a crate boundary.
- A future second crate (`orbe-inv`) is unnecessary at MVP scale; promote when a real second consumer (CLI, mobile native) needs it.

---

## 2. Phase breakdown

### Phase 0 — Decisions & docs (this session)

Goal: lock the stack change into the spec, no code yet.

| Task | Done |
|---|---|
| Amend SPEC.md — drop C3 (Vue/Vite/UnoCSS/Pinia), add C10 (Dioxus fullstack over Axum), rework T9–T14 + T20, add V7 (server functions = sole data path) | ☐ |
| Update README.md — stack table, structure tree, Quick Start | ☐ |
| Update DESIGN.md — styling path → CSS vars / Tailwind‑native, drop UnoCSS shortcuts | ☐ |
| Create CONVENTIONS.md — Decimal for crates, Dioxus `src/` inside the FE module, `.toon` for seeds/fixtures, server‑function boundary | ☐ |

Exit criterion: `jj diff` shows coherent doc changes; user approves.

### Phase 1 — Scaffold (runnable hello world)

Goal: `dx serve` boots, browser loads, `/` renders a Dioxus page, `/api/v1/inventory/products` returns `[]`.

| Task | Notes |
|---|---|
| `Backend/P/orbe-api/Cargo.toml` | `dioxus = { features = ["fullstack"] }`, optional `axum`/`tokio` behind `feature = "server"` |
| `Backend/P/orbe-api/src/main.rs` | `#[cfg(feature="server")]` → axum + `serve_dioxus_application`; else `dioxus::launch(App)` |
| `src/core/` | stub entities (`Product`, `Category`, `Supplier`, `StockMovement`) |
| `src/flow/` | stub use cases returning empty vectors |
| `src/gate/ports.rs` | `trait ProductRepo`, `trait MovementRepo` |
| `src/gate/surreal/` | stub adapter returning `Ok(vec![])` |
| `src/wire/` | axum router with `/api/v1/inventory/products` returning `[]`, OpenAPI doc |
| `src/ui/` | Dioxus `App` component, `routes![home]`, one server fn `list_products()` calling the wire |
| `Backend/R/1.Db/1a.Sch/Inventory.surql` | from SPEC §I (verbatim) |
| `Backend/R/2.Cf/Env.example` | `SURREAL_URL`, `SURREAL_USER`, `SURREAL_PASS`, `ORBE_PORT` |
| `Dioxus.toml` | `default_platform = "web"`, hot reload flags on |
| `dx serve` | zero errors, browser renders "Orbe" + empty inventory table |

Exit criterion: `cargo build` + `dx serve` both green; can open `http://localhost:8080` and see the app.

### Phase 2 — Inventory CRUD (vertical slice)

Goal: create a product, list it, adjust stock, see the movement — all from the browser, all via `#[server]` fns.

| Task | Notes |
|---|---|
| `src/core/` — flesh out `Product`, `Category`, `StockMovement` with validation | V2 sku unique, V3 slug unique, V5 stock ≥ 0 |
| `src/flow/` — `CreateProduct`, `ListProducts`, `UpdateProduct`, `DeleteProduct`, `AdjustStock` | V4: every adjust writes a movement |
| `src/gate/surreal/` — real adapter over `surrealdb` crate | Use Rust SDK, not HTTP — fewer moving parts |
| `src/wire/` — utoipa handlers bound to flow, error → HTTP mapping | |
| `src/ui/server_fns.rs` — `#[server]` wrappers around flow use cases | V7 enforcement point |
| `src/ui/pages/inventory.rs` — list, product card, create form | Optimistic UI for instant feedback |
| `src/ui/components/StockBadge.rs` — pulse animation from DESIGN §5 | |
| Seed data | `Backend/R/1.Db/2.Sd/inventory.toon` (TOON format, not JSON) — see §4 |
| `cargo test` — unit tests on flow (stock ≥ 0 invariant, movement creation) | |
| Manual E2E | create product → list → adjust stock → reload → see updated qty + movement |

Exit criterion: full CRUD flow works end‑to‑end in `dx serve`; tests green.

### Phase 3 — Polish & package (MVP release)

Goal: shippable, installable, presentable.

| Task | Notes |
|---|---|
| PWA manifest + service worker | Dioxus `dx bundle --platform web --pwa` |
| Icons (192/512) | Purple‑on‑black O mark |
| `dx build --platform android` / `--platform ios` smoke build | prove crossplatform claim; not submitted to stores yet |
| README "Status" checklist → all ticked | |
| Demo .gif of one full CRUD round | optional, 30s |

Exit criterion: documented, runnable, demonstrably crossplatform.

---

## 3. Toolchain (what runs on the dev's machine)

| Tool | Version | Source |
|---|---|---|
| `rustc` / `cargo` | 1.92 (already installed) | `rustup` |
| `dx` | 0.7+ | `cargo install dioxus-cli` |
| `surreal` | 2.x (already installed) | official installer |
| `jj` | 0.41 (already installed) | system |
| Node / npm / pnpm / bun | **none** | — |
| Vite / UnoCSS / Vue / Pinia | **none** | — |

Editor: any. Recommended: Helix, Zed, or Neovim (no LSP plugins required beyond `rust-analyzer`).

---

## 4. `.toon` for seed data — concrete example

`Backend/R/1.Db/2.Sd/inventory.toon`:

```toon
categories[2]{slug,name}:
  beverages,Bebidas
  snacks,Snacks

products[3]{sku,name,price,stock_qty,category_slug}:
  A001,Coca-Cola 350ml,3500,48,beverages
  A002,Agua Cristal 600ml,2500,120,beverages
  A003,Papas Margarita 30g,1800,30,snacks
```

Loader uses `toon-rs` or hand‑rolled 8‑line parser. JSON stays banned from version‑controlled fixtures.

---

## 5. Risks & mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| Dioxus 0.7 ecosystem immaturity (some docs assume 0.6) | M | Pin version, read `/dioxuslabs/dioxus` Context7 docs before each scaffold step |
| SurrealDB Rust SDK breaking between 2.x minors | L | Pin to 2.2.x, hold minor upgrades to Phase 3 |
| WASM bundle size creeps past "fast" pillar | M | `dx bundle --release` report checked in Phase 3; budget < 500KB gzip |
| Single‑crate scaling wall (long compile) | L | `cargo nextest` + `--features server` only on CI; split crate when build > 30s |
| Server functions leaking into FE bundle | M | V7 invariant + `cargo tree` check in Phase 2 review |

---

## 6. Why no `Frontend/` dir for MVP

The Dioxus app **lives inside** `Backend/P/orbe-api/src/ui/` because there is one Rust toolchain producing two targets. A separate `Frontend/` dir would only make sense if:

1. We adopt a second FE language (rejected — full Rust), or
2. We publish the UI crate independently (rejected — premature).

When Phase 3 ships a mobile `.ipa`/`.apk` and we want a separate `Frontend/` workspace, the move is a `jj mv` with no API changes.

---

## 7. What I need from you

1. **Approve plan** — say "go" and I update SPEC/README/DESIGN/CONVENTIONS in one batch.
2. **Pick a Phase 0 → Phase 1 boundary**: do you want Phase 0 doc edits reviewed separately, or rolled straight into Phase 1 scaffold?
3. **Confirm `.toon` for seeds** (vs `.json` despite your "if possible" hedge). One line.

Once approved, the dispatch order is:
```
Phase 0 (docs) → user review → Phase 1 scaffold (sequential tasks) → verify → Phase 2 …
```