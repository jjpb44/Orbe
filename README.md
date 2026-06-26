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
| Database | SurrealDB 3.2 (Rust SDK, kv-mem) |
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
│   ├── Q/orbe-api/                # the only crate for MVP
│   │   ├── src/{core,flow,gate,wire,ui}/
│   │   ├── public/                # main.css, PWA manifest, icons (Dioxus default public_dir)
│   │   ├── Cargo.toml             # features = ["server"]
│   │   └── Dioxus.toml            # default_platform = "web"
│   ├── W/
│   │   ├── 1.Dat/1a.Sche/        # SurrealDB schemas
│   │   ├── 1.Dat/2.Sed/           # .toon seed data
│   │   ├── 2.Cfg/                 # Env.example
│   │   └── 3.Dec/                 # presenterm decks (onboarding + client PDFs)
│   └── Z/                         # ADRs
├── Frontend/                      # reserved — empty until a 2nd FE consumer
├── DESIGN.md                      # design system
├── SPEC.md                        # project specification
├── CONVENTIONS.md                 # how we name, layer, and test
├── PLAN.md                        # phased delivery plan
└── README.md
```

## Quick Start

**Prerequisites:** Rust 1.96+, jj (jujutsu), `dx` CLI (`cargo install dioxus-cli --locked`), `just` (`cargo install just` or system pkg).

All commands run from project root via `just`:

```bash
just dev            # dx serve — fullstack hot reload (main dev loop)
just build          # cargo build
just test           # cargo test --lib
just smoke          # curl smoke test (14 checks — requires `just run` first)
just status         # jj log -n 5
just deck           # present onboarding deck (4 slides)
just deck-client    # export client overview PDF (envsubst + presenterm)
just                # show all commands
```

Manual equivalents if you prefer:

```bash
cd Backend/Q/orbe-api
cargo run                                    # server only, http://127.0.0.1:8080
dx serve                                     # fullstack hot reload
cargo build --release                        # prod binary
dx build --platform web --release            # WASM bundle
dx bundle --platform web --pwa               # PWA-ready assets
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
Design tokens in `Backend/Q/orbe-api/public/main.css` as CSS vars (`--o-purple`, `--o-surface`, …).
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

**MVP complete** — all phases delivered. 14 curl smoke tests green, 11 unit tests green, 0 clippy warnings, PWA bundle ready.

- [x] Project specification (SPEC.md)
- [x] Design system (DESIGN.md)
- [x] Conventions (CONVENTIONS.md)
- [x] Phased delivery plan (PLAN.md)
- [x] Backend scaffold — `orbe-api` crate compiles (cargo build green)
- [x] SurrealDB schema — 4 tables (product, category, supplier, stock_movement)
- [x] Dioxus UI scaffold — `cargo run` boots Axum on :8080
- [x] Design tokens — CSS vars (`--o-purple`, `--o-surface`, etc.)
- [x] 3 ADRs — SurrealDB, LayerNaming, DioxusFullstack
- [x] Product CRUD — 7 REST routes + 7 server fns, V2/V3/V5 invariants enforced
- [x] Stock adjustments + movement log — V4 enforced, movements DESC ordered
- [x] PWA manifest + icons (192/512 SVG)
- [x] WASM compile — `dx build --platform web` green (server+WASM targets)
- [x] E2E verified — create→list→adjust→movements→delete via curl
- [x] Clean architecture — V1/V7/V8 invariants, no leaked imports
- [x] 0 clippy warnings, 0 unused imports
- [x] justfile — `just serve` for main dev loop
- [x] OrbeBackend/ OrbeFrontend/ stale dirs removed
