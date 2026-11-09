# Orbe

Open-source CRM built for small-medium entrepreneurs in Colombia.
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
| Fullstack | Dioxus `#[server]` fns over Axum (single crate, Cargo workspace) |
| Database | SurrealDB 3.2 (Rust SDK, kv-mem) |
| Architecture | Clean Arch — Core → Flow → Gate → Wire |
| Styling | Plain CSS + design tokens (CSS vars) |
| Build | `cargo build` (server) · `dx build --platform web` (WASM) |
| CI | `just ci` — local gate, `target/ci/report.json`, no hosted runner |
| VCS | jj (jujutsu) colocated with git |
| License | MIT |

No JS, no Node, no npm/bun/pnpm, no Vue/React/UnoCSS/Tailwind/Vite. Editor-agnostic.

## Structure

```
Orbe/
├── Cargo.toml                    # workspace root — members, profiles, single target/
├── crates/orbe-api/              # the only crate for the MVP
│   ├── src/{core,flow,gate,wire,ui}/
│   ├── public/                   # main.css, PWA manifest, icons
│   ├── Cargo.toml                # features = ["server"] (default) | web | desktop | mobile
│   └── Dioxus.toml               # default_platform = "web"
├── database/
│   ├── schema/Inventory.surql
│   └── seeds/                    # .toon seed data
├── config/Env.example
├── scripts/
│   ├── smoke.sh                  # REST + RPC e2e (spawns its own server)
│   ├── ci-report.sh              # the local CI gate
│   ├── commit.sh                 # `just commit`
│   ├── layer-check.sh            # V1/V7 boundary check
│   └── check-bilingual-parity.sh
├── docs/
│   ├── decisions/                # ADRs
│   ├── decks/                    # onboarding decks + client PDF template
│   ├── archive/
│   └── PLAN.md
├── DESIGN.md · SPEC.md · CONVENTIONS.md · README.md
```

## Quick Start

**Prerequisites:** Rust 1.96+, jj (jujutsu), `dx` CLI (`cargo install dioxus-cli --locked`), `just`, `typst`, `presenterm`.
Check yours with `just doctor`.

All commands run from project root via `just`:

```bash
just dev            # dx serve — fullstack hot reload (main dev loop)
just ci             # local gate: fmt, clippy, layers, tests, build, deck parity
just ci-full        # gate + smoke (spawns a server) + wasm build
just commit "msg"   # run the gate, then commit with jj (refuses on red)
just test           # unit + flow tests (nextest)
just smoke          # REST + RPC e2e against a private server
just build          # cargo build
just status         # jj log
just deck           # present the onboarding deck
just                # essentials; `just help` for the full list
```

Manual equivalents:

```bash
cargo run                                    # server + UI on http://127.0.0.1:8080
cargo nextest run                            # tests
cd crates/orbe-api && dx serve               # fullstack hot reload
dx build --platform web --release            # WASM bundle
dx bundle --platform web --pwa               # PWA assets
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
| Wire | `wire/` | Axum router (`/api/v1/inventory/*`) |
| UI | `ui/` | Dioxus components + server fns (`/api/*`) |

**Critical constraints (SPEC §V)** — enforced by `just layer-check`:

- V1: Wire imports `flow/` + `core/` DTOs only — never `gate/surreal`.
- V7: Dioxus UI ⊥ import from `gate/surreal/` — only via `#[server]` fns → flow → ports.
- V9: money is `i64` minor units (`*_cents`) — no floats, no decimals.

## Testing

| Suite | What | Command |
|---|---|---|
| Unit | domain invariants (V2/V3/V5) | `cargo nextest run` |
| Flow | use cases on a fake port: oversell → `Err` + no movement (V4/V5) | `cargo nextest run` |
| Smoke | 18 checks: REST contract (201/404/409/422/204) + RPC happy paths | `just smoke` |
| Layers | V1/V7 import boundaries | `just layer-check` |

## Design

Dark-first palette inspired by Nequi, Nu, WhatsApp — immersive, fast, magical.
Design tokens in `crates/orbe-api/public/main.css` as CSS vars (`--o-purple`, `--o-surface`, …).
See `DESIGN.md` for the full system.

## Version Control

[jujutsu](https://github.com/jj-vcs/jj) (jj), colocated with git.

| Remote | Role |
|---|---|
| `upd` — `upd-git@ssh.upd.dev:jjpb/Orbe.git` | private primary |
| `codeberg` — `ssh://git@codeberg.org/SethDev/Orbe.git` | public mirror |
| `github` | optional mirror (add when needed) |

```bash
jj git clone upd-git@ssh.upd.dev:jjpb/Orbe.git   # maintainers
git clone https://codeberg.org/SethDev/Orbe.git  # everyone else
```

## License

MIT — see [LICENSE](LICENSE).

## Status

**Phase 3 in progress.** 17 tests green (11 domain + 6 flow), 18 smoke checks green,
clippy clean, both targets compile.

- [x] SPEC, DESIGN, CONVENTIONS, PLAN
- [x] `orbe-api` crate — server + WASM compile from one workspace
- [x] SurrealDB schema — 4 tables (product, category, supplier, stock_movement)
- [x] Product CRUD — 7 REST routes + 7 server fns
- [x] Stock adjustments + movement log (V4/V5 enforced in flow)
- [x] PWA manifest + icons (192/512 SVG)
- [x] Local CI gate — `just ci` / `just commit`, JSON report, zero quota
- [x] Layer boundary enforcement (V1/V7) — `just layer-check`
- [x] Standard paths migration (Q/W/Z retired)
- [ ] Push to `upd` + `codeberg` (SPEC T15)
- [ ] PWA service worker bundled (T28)
- [ ] WASM size budget enforced (T29)
- [ ] Android smoke build (T30)
- [ ] Client PDF via typst (T31)
- [ ] `.toon` seed loader (T33)
