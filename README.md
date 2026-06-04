# Orbe

Open-source CRM for small-to-medium entrepreneurs in Colombia.

## Phase 1 — Inventory MVP

No auth, no billing, no CI. Local development only.

## Stack

| Layer | Tech |
|-------|------|
| Backend | Rust · Axum · SurrealDB 2 |
| Frontend | Vue 3 · Vite · UnoCSS · Pinia |

## Architecture

Backend: Clean Architecture — Core → Flow → Gate → Wire.
Frontend: JuanJo.Decimal file convention (`P/` replaces `src/`).

## Repos

- `OrbeBackend/` — API server (Axum + SurrealDB)
- `OrbeFrontend/` — PWA client (Vue + UnoCSS)

## Run Locally

```bash
# Backend
cd OrbeBackend
surreal start --user root --pass root &
cargo run

# Frontend
cd OrbeFrontend
pnpm install
pnpm dev
```

## Hosting

Primary: upd.dev  
Mirrors: GitHub  
Sync: jujutsu (jj) + git

## License

[To be determined]
