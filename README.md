# Orbe

Open‑source CRM built for small‑medium entrepreneurs in Colombia.
Start with inventory. Grow from there.

## What

Orbe is a pragmatic CRM that respects your time. Phase 1 delivers an
inventory MVP — track products, categories, stock movements, nothing else.
No auth, no billing, no bloat. Just a fast, local-first tool that works.

Future phases layer in customer management, sales, and SaaS hosting.

## Stack

Full Rust. One toolchain (`cargo` + `dx`).

| Layer | Choice |
|---|---|
| Backend | Rust · Axum 0.8 |
| Frontend | Dioxus 0.7 · RSX · signals |
| Fullstack | Dioxus `#[server]` fns over Axum (single crate) |
| Database | SurrealDB 2.x (Rust SDK) |
| Architecture | Clean Arch — Core → Flow → Gate → Wire |
| Styling | Plain CSS + design tokens (CSS vars) |
| Build | `cargo build` (server) · `dx build --platform web` (WASM) |
| VCS | jj (jujutsu) with colocated git |
| License | MIT |

No JS, no Node, no npm/bun/pnpm, no Vue/React/UnoCSS/Tailwind/Vite. Editor-agnostic.

## Structure

```
Orbe/
├── Backend/                       # ALL Rust — workspace + crates
│   ├── P/orbe-api/                # the only crate for MVP
│   │   ├── src/{core,flow,gate,wire,ui}/
│   │   ├── assets/                # main.css, PWA manifest, icons
│   │   ├── Cargo.toml             # features = ["server"]
│   │   └── Dioxus.toml            # default_platform = "web"
│   ├── R/
│   │   ├── 1.Db/1a.Sch/           # SurrealDB schemas
│   │   ├── 1.Db/2.Sd/             # .toon seed data
│   │   └── 2.Cf/                  # Env.example
│   └── A/                         # ADRs
├── Frontend/                      # reserved — empty until a 2nd FE consumer
├── DESIGN.md                      # design system
├── SPEC.md                        # project specification
├── CONVENTIONS.md                 # how we name, layer, and test
├── PLAN.md                        # phased delivery plan
└── README.md
```

## Quick Start

**Prerequisites:** Rust 1.80+, jj (jujutsu). SurrealDB 2.x + `dx` CLI come in Phase 2.

```bash
# Clone
jj git clone https://upd.dev/jjpb/Orbe.git
cd Orbe

# Phase 1: server runs from cargo, no JS toolchain needed
cd Backend/P/orbe-api
cargo run                              # → http://127.0.0.1:8080
```

The page renders the empty inventory stub via Dioxus SSR + `#[server]` fn.
For WASM hot reload (Phase 2 onward):
```bash
cargo install dioxus-cli --locked      # one-time
dx serve                               # builds server + WASM, opens browser
```

Prod builds:
```bash
cargo build --release --features server       # native server binary
dx build --platform web --release             # WASM bundle
dx bundle --platform web --pwa                # PWA-ready assets
```

## Architecture

Backend follows Clean Architecture, layered from domain to transport:

```
Core → Flow → Gate (ports.rs) → Wire (axum)
                       ↑
                       └── Gate (surreal/) — adapter implementing the ports
```

Plus the Dioxus UI sits on top of the same Flow use cases via `#[server]` fns:

```
UI (Dioxus) → #[server] → Flow → Gate ports → SurrealDB adapter
                          ↑
Wire (axum) →————————————┘  (same flow fns, no duplication)
```

| Layer | Dir | Role |
|---|---|---|
| Core | `core/` | Entities, value objects, domain rules |
| Flow | `flow/` | Use cases — orchestrate core + gate traits |
| Gate | `gate/` | Ports (`ports.rs`) + SurrealDB adapter (`surreal/`) |
| Wire | `wire/` | Axum router + utoipa OpenAPI |
| UI | `ui/` | Dioxus components + server fns |

**Critical constraints (SPEC §V):**
- V1: Wire ⊥ import from `core/` | `flow/` — only from `gate/ports.rs`.
- V7: Dioxus UI ⊥ import from `gate/surreal/` — only via `#[server]` fns → flow → ports.

## Design

Dark-first palette inspired by Nequi, Nu, WhatsApp — immersive, fast, magical.
Design tokens in `Backend/P/orbe-api/public/main.css` as CSS vars (`--o-purple`, `--o-surface`, …).
See `DESIGN.md` for the full system.

## Version Control

[jujutsu](https://github.com/jj-vcs/jj) (jj) with colocated git backend.
Primary remote: upd.dev / Codeberg. GitHub serves as mirror.

```bash
jj git clone https://upd.dev/jjpb/Orbe.git
```

## License

MIT — see [LICENSE](LICENSE).

## Status

**Phase 1 — Inventory MVP** (active development)

- [x] Project specification (SPEC.md)
- [x] Design system (DESIGN.md)
- [x] Conventions (CONVENTIONS.md)
- [x] Phased delivery plan (PLAN.md)
- [x] Backend scaffold — `orbe-api` crate compiles (`cargo build` green, 290 crates)
- [x] SurrealDB schema — `Backend/R/1.Db/1a.Sch/Inventory.surql` (4 tables, indexes, asserts)
- [x] Dioxus UI scaffold — `cargo run` boots Axum on :8080, browser shows empty inventory
- [x] Design tokens — `public/main.css` exposes `--o-purple`, `--o-surface`, etc.
- [x] 3 ADRs — SurrealDB, LayerNaming, DioxusFullstack (`Backend/A/1.ADR/`)
- [ ] Product CRUD via #[server] fns *(Phase 2)*
- [ ] Stock adjustments + movement log *(Phase 2)*
- [ ] PWA manifest + icons *(Phase 3, deferred)*
- [ ] End-to-end local flow *(Phase 2)*