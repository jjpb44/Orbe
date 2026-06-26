# ADR-003 · Dioxus 0.7 fullstack over Axum

## Context
Need FE + BE in single repo, single language (Rust), crossplatform (web +
desktop + mobile). Team is one developer (no JS specialists). Existing Axum
0.8 + SurrealDB stack from ADR-001 must stay.

## Decision
**Dioxus 0.7** with `fullstack` feature. RSX renders to web (WASM) + desktop +
mobile. `#[server]` fns run on the existing Axum server.

## Rationale
- **Single toolchain**: `cargo` + `dx`. No Node, no bun, no Vite, no npm, no JS
  at any layer. Editor-agnostic — Neovim, Helix, Zed, IntelliJ all work.
- **Server functions = type-safe RPC.** Client gets a typed `async fn`. No
  hand-written HTTP clients, no DTO drift.
- **Crossplatform**: `dx build --platform web|desktop|ios|android` from one
  codebase. Phase 1 = web; Phase 3 = desktop + mobile.
- **Real DOM rendering on web** = full a11y, screen readers, keyboard nav, WCAG
  AA. No virtual DOM diff needed; signals trigger fine-grained updates.
- **~50 KB WASM hello world.** Tree-shakes unused components. Signals avoid
  vDOM work.
- **Hot reload** via `dx serve`: subsecond Rust patch + asset reload. Saves
  `cargo build` cycles during UI work.

## Rejected alternatives
| Option                       | Reject because                                                  |
| ---------------------------- | --------------------------------------------------------------- |
| Vue 3 + Vite                 | JS toolchain; not single-language; breaks "no JS" goal          |
| Yew / Leptos                 | Both are web-only — no first-class desktop/mobile path          |
| React + Tauri                | React violates "no JS"; Tauri + React = 2 toolchains            |
| Bevy                         | No mature text input / no a11y tree / no virtualization = WCAG blocker |
| vanilla-bean (JSX over DOM)  | Still requires a JS runtime; rejected per above                 |
| `ant` JS runtime             | ~8 MB binary, niche — Dioxus covers what we'd build with it     |

## Consequences
- **Dioxus is younger than React/Vue** → smaller ecosystem. Mitigated: we
  ship few components, and `lucide-dioxus` (or any Lucide icons via SVG)
  covers icon needs.
- **`#[server]` boundary** requires explicit `Result<T, ServerFnError>`.
  Errors must serialize. Phase 2 will define a typed error enum.
- **Mobile build needs Xcode/Android SDK** setup (Phase 3, deferred per
  PLAN.md).

## Cargo shape (locked in by this ADR)
```toml
dioxus = { version = "0.7", features = ["fullstack"] }
axum = { version = "0.8", optional = true }
tokio = { version = "1", features = ["full"], optional = true }
dioxus-cli-config = { version = "0.7", optional = true }

[features]
default = ["server"]
server = ["dioxus/server", "dep:axum", "dep:tokio", "dep:dioxus-cli-config"]
web = ["dioxus/web"]
```

## References
- https://dioxuslabs.com/learn/0.7/
- https://github.com/DioxusLabs/dioxus/tree/main/packages/fullstack-server
