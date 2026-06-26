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

**Prerequisites:** Rust 1.80+, `dx` (Dioxus CLI), SurrealDB 2.x, jj (jujutsu)

```bash
# Clone
jj git clone https://upd.dev/jjpb/Orbe.git
cd Orbe

# Backend (one terminal)
cd Backend/P/orbe-api
cargo install dioxus-cli --locked   # one-time
dx serve                            # builds server + WASM, opens browser
```

`dx serve` boots the Axum server (REST + Dioxus SSR + server fns) on
`http://localhost:8080` and live-reloads the WASM bundle on save.

For prod-style builds:
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
Design tokens in `Backend/P/orbe-api/assets/main.css` as CSS vars (`--o-purple`, `--o-surface`, …).
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
- [ ] Backend scaffold (orbe-api crate compiles)
- [ ] SurrealDB schema
- [ ] Dioxus UI scaffold (browser loads empty inventory)
- [ ] Product CRUD via #[server] fns
- [ ] Stock adjustments + movement log
- [ ] PWA manifest + icons
- [ ] End-to-end local flow