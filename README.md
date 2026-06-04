# Orbe

Open‑source CRM built for small‑medium entrepreneurs in Colombia.
Start with inventory. Grow from there.

## What

Orbe is a pragmatic CRM that respects your time. Phase 1 delivers an
inventory MVP — track products, categories, stock movements, nothing else.
No auth, no billing, no bloat. Just a fast, local-first tool that works.

Future phases layer in customer management, sales, and SaaS hosting.

## Stack

| Backend | Frontend |
|---------|----------|
| Rust · Axum 0.8 | Vue 3 · TypeScript |
| SurrealDB 2.x | Vite · UnoCSS |
| tokio · utoipa (OpenAPI) | Pinia · PWA |
| Clean Architecture | JuanJo.Decimal convention |

## Structure

```
Orbe/
├── Backend/                  # Rust workspace (Axum + SurrealDB)
│   ├── P/                    # Active crates (orbe-api, orbe-inv)
│   ├── R/                    # Resources (schema, config)
│   └── A/                    # Docs & ADRs
├── Frontend/                 # Vue 3 + Vite PWA
│   ├── P/                    # Source (JuanJo.Decimal convention)
│   ├── R/                    # Assets
│   └── A/                    # Docs & ADRs
├── SPEC.md                   # Project specification
├── DESIGN.md                 # Design system
└── README.md
```

## Quick Start

**Prerequisites:** Rust 1.80+, Node.js 22+, SurrealDB 2.x, jj (jujutsu)

```bash
# Clone
jj git clone https://upd.dev/jjpb/Orbe.git
cd Orbe

# Backend
cd Backend
cargo build
surreal start --user root --pass root

# Frontend (separate terminal)
cd ../Frontend
pnpm install
pnpm dev
```

## Architecture

Backend follows Clean Architecture, layered from domain to transport:

```
Core → Flow → Gate → Wire
```

| Layer | Dir | Role |
|-------|-----|------|
| Core | `core/` | Entities, value objects, domain rules |
| Flow | `flow/` | Use cases, application services |
| Gate | `gate/` | Repository traits (ports) + SurrealDB adapter |
| Wire | `wire/` | Axum handlers, routes, OpenAPI DTOs |

**Critical constraint:** `orbe-api` (Wire) must only import from `gate/ports.rs`
traits. It must never touch `core/` or `flow/` directly.

## Design

Dark-first palette inspired by Nequi, Nu, WhatsApp — immersive, fast, magical.

| Token | Role | Hex |
|-------|------|-----|
| Purple | Primary | `#6B21A8` |
| Black | Background | `#0A0A0A` |
| Teal | Accent | `#2DD4BF` |

Built with UnoCSS shortcuts (`o-btn`, `o-card`, `o-input`, `o-badge`).
Typography leans geometric display + mono accent. No light theme for MVP.

## Version Control

[jujutsu](https://github.com/jj-vcs/jj) (jj) with colocated git backend.
Primary remote: [Codeberg](https://codeberg.org). GitHub serves as mirror.

```bash
jj git clone https://upd.dev/jjpb/Orbe.git
```

## License

MIT — see [LICENSE](LICENSE).

## Status

**Phase 1 — Inventory MVP** (active development)

- [ ] Project specification (SPEC.md)
- [ ] Design system (DESIGN.md)
- [ ] Backend scaffold (Rust workspace + crates)
- [ ] SurrealDB schema
- [ ] Product CRUD endpoints
- [ ] Stock adjustments + movement log
- [ ] Vue 3 PWA client
- [ ] End-to-end local flow
