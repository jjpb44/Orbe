# Orbe — OpenCode Bootstrap v2
# Model: minimax-m3
# Skills: /use spec /use cove /use caveman /use caveman-commit
# Run from parent workspace dir containing both repos.
# ─────────────────────────────────────────────────────────────────────────────

/use spec
/use cove
/use caveman

---

## Project Identity

| Key | Value |
|-----|-------|
| Name | Orbe |
| Type | Open-source CRM · SaaS hosting tier |
| Phase 1 | Inventory MVP — no auth, no billing |
| Target | SMEs in Colombia |
| UX inspiration | Nequi, Nu, WhatsApp — fast, bold, minimal, tactile |
| Palette | Purple `#6B21A8` · Black `#0A0A0A` · Teal `#2DD4BF` |
| Typography | Geometric display + mono accent. Avoid Inter/Roboto. |

---

## Repos

Two separate repos, both colocated jj+git (Codeberg primary):

```bash
mkdir OrbeBackend && cd OrbeBackend && jj git init --colocate && cd ..
mkdir OrbeFrontend && cd OrbeFrontend && jj git init --colocate && cd ..
```

---

## File Convention: JuanJo.Decimal

All structure follows JuanJo.Decimal (see `juanjodecimal` skill).
**Rules summary:**
- No spaces. PascalCase on all name segments.
- Top-level: `P/` (active dev) · `R/` (resources/config) · `A/` (docs/archive)
- Area: `{n}.{Xx}/` → e.g. `1.Co/`, `2.In/`
- Category: `{na}.{Xxx}/` → e.g. `1a.She/`, `2a.Prd/`
- ID (optional): `{na}{nn}.{Name}/` → dir only, e.g. `2a01.Legacy/`
- **JD numbering = DIRECTORY names only.** Files: PascalCase preferred, not enforced.

## OrbeBackend Structure

### Clean Architecture layers (Core/Flow/Gate/Wire)

| Dir | Layer | First2 | Responsibility |
|-----|-------|--------|----------------|
| `core/` | Core | Co | Entities, value objects, domain rules |
| `flow/` | Flow | Fl | Use cases, application services |
| `gate/` | Gate | Ga | SurrealDB adapter, port traits |
| `wire/` | Wire | Wi | Axum handlers, routes, DTOs |

> JD adaptation: layer names ARE the area identifiers for Rust source.
> Comment in each `mod.rs`: `// JD: 1.Co — Core domain`

### Workspace layout

```
OrbeBackend/
├── Cargo.toml                   ← workspace root
├── P/                           ← active crates
│   ├── orbe-api/                ← bin: server entry, Wire layer
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── wire/
│   │       │   ├── mod.rs
│   │       │   ├── inventory.rs ← Axum router + handlers
│   │       │   └── dto.rs       ← Request/Response + utoipa schemas
│   │       └── main.rs
│   └── orbe-inv/                ← lib: Inventory domain
│       ├── Cargo.toml
│       └── src/
│           ├── core/            ← JD 1.Co
│           │   ├── mod.rs
│           │   ├── product.rs
│           │   ├── category.rs
│           │   ├── movement.rs
│           │   └── supplier.rs
│           ├── flow/            ← JD 2.Fl
│           │   ├── mod.rs
│           │   ├── create_product.rs
│           │   ├── update_product.rs
│           │   ├── delete_product.rs
│           │   ├── list_products.rs
│           │   └── adjust_stock.rs
│           ├── gate/            ← JD 3.Ga
│           │   ├── mod.rs
│           │   ├── ports.rs     ← repository traits (the only contract orbe-api touches)
│           │   └── surreal_product.rs
│           └── lib.rs
├── R/                           ← resources
│   ├── 1.Db/
│   │   └── 1a.Sch/
│   │       └── Inventory.surql
│   └── 2.Cf/
│       └── Env.example
└── A/                           ← docs
    └── 1.Ad/
        └── 1a.Arc/
            ├── SurrealDB.md
            └── LayerNaming.md

# Cargo.toml workspace members:
# members = ["P/orbe-api", "P/orbe-inv"]
```

### Key dependencies

```toml
# P/orbe-inv/Cargo.toml
[dependencies]
surrealdb = "2"
serde = { version = "1", features = ["derive"] }
thiserror = "2"
uuid = { version = "1", features = ["v7", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tokio = { version = "1", features = ["full"] }

# P/orbe-api/Cargo.toml
[dependencies]
orbe-inv = { path = "../orbe-inv" }
axum = { version = "0.8", features = ["macros"] }
tower-http = { version = "0.6", features = ["cors", "trace"] }
utoipa = { version = "5", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "8", features = ["axum"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
dotenvy = "0.15"
```

### SurrealDB schema (R/1.Db/1a.Sch/Inventory.surql)

```sql
DEFINE NAMESPACE orbe;
USE NS orbe DB inventory;

DEFINE TABLE product SCHEMAFULL;
DEFINE FIELD id          ON product TYPE record<product>;
DEFINE FIELD sku         ON product TYPE string  ASSERT $value != NONE;
DEFINE FIELD name        ON product TYPE string  ASSERT $value != NONE;
DEFINE FIELD description ON product TYPE option<string>;
DEFINE FIELD price       ON product TYPE decimal ASSERT $value >= 0;
DEFINE FIELD stock_qty   ON product TYPE int     ASSERT $value >= 0;
DEFINE FIELD category    ON product TYPE option<record<category>>;
DEFINE FIELD supplier    ON product TYPE option<record<supplier>>;
DEFINE FIELD created_at  ON product TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD updated_at  ON product TYPE datetime VALUE time::now();

DEFINE TABLE category SCHEMAFULL;
DEFINE FIELD id   ON category TYPE record<category>;
DEFINE FIELD name ON category TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON category TYPE string ASSERT $value != NONE;

DEFINE TABLE supplier SCHEMAFULL;
DEFINE FIELD id    ON supplier TYPE record<supplier>;
DEFINE FIELD name  ON supplier TYPE string ASSERT $value != NONE;
DEFINE FIELD email ON supplier TYPE option<string>;
DEFINE FIELD phone ON supplier TYPE option<string>;

DEFINE TABLE stock_movement SCHEMAFULL;
DEFINE FIELD id         ON stock_movement TYPE record<stock_movement>;
DEFINE FIELD product    ON stock_movement TYPE record<product>;
DEFINE FIELD delta      ON stock_movement TYPE int;   -- positive=in, negative=out
DEFINE FIELD reason     ON stock_movement TYPE string;
DEFINE FIELD created_at ON stock_movement TYPE datetime VALUE time::now() READONLY;

DEFINE INDEX idx_product_sku   ON product  COLUMNS sku  UNIQUE;
DEFINE INDEX idx_category_slug ON category COLUMNS slug UNIQUE;
```

### OpenAPI routes (utoipa, tag: `inventory`)

```
GET    /api/v1/inventory/products
POST   /api/v1/inventory/products
GET    /api/v1/inventory/products/{id}
PUT    /api/v1/inventory/products/{id}
DELETE /api/v1/inventory/products/{id}
POST   /api/v1/inventory/products/{id}/stock
GET    /api/v1/inventory/movements
```

---

## OrbeFrontend Structure

Vite root stays at repo root. `P/` replaces `src/`.
Configure `vite.config.ts`: `resolve.alias['@'] = './P'`

```
OrbeFrontend/
├── P/                           ← active source (≈ src/)
│   ├── 1.Co/                    ← Core area
│   │   ├── 1a.She/              ← Shell
│   │   │   ├── App.vue
│   │   │   └── RouterLayout.vue
│   │   ├── 1b.Des/              ← Design tokens
│   │   │   └── Tokens.ts
│   │   └── 1c.Rou/              ← Router
│   │       └── Index.ts
│   ├── 2.In/                    ← Inventory module
│   │   ├── 2a.Prd/              ← Products
│   │   │   ├── ProductList.vue
│   │   │   ├── ProductCard.vue
│   │   │   ├── ProductForm.vue
│   │   │   └── UseProducts.ts
│   │   ├── 2b.Stk/              ← Stock
│   │   │   ├── StockBadge.vue
│   │   │   └── StockAdjustModal.vue
│   │   ├── 2c.Cat/              ← Categories
│   │   │   └── CategorySelect.vue
│   │   └── 2d.Sto/              ← Pinia store
│   │       └── InventoryStore.ts
│   ├── 3.Sh/                    ← Shared
│   │   ├── 3a.Cmp/              ← Components
│   │   │   ├── OButton.vue
│   │   │   ├── OInput.vue
│   │   │   ├── OModal.vue
│   │   │   └── OTable.vue
│   │   ├── 3b.Use/              ← Composables
│   │   │   └── UseApi.ts
│   │   └── 3c.Utl/              ← Utils
│   │       └── Format.ts
│   └── main.ts
├── R/                           ← resources
│   └── 1.As/
│       └── 1a.Ico/              ← PWA icons (512, 192, 96)
├── A/                           ← docs
│   └── 1.Ad/
│       └── 1a.Arc/
│           ├── SurrealDB.md
│           └── LayerNaming.md
├── public/                      ← Vite static (framework convention)
├── index.html
├── vite.config.ts
├── uno.config.ts
├── tsconfig.json
└── package.json
```

### Design tokens (P/1.Co/1b.Des/Tokens.ts)

```ts
export const OrbePalette = {
  purple:      '#6B21A8',
  purpleLight: '#9333EA',
  purpleDim:   '#3B0764',
  teal:        '#2DD4BF',
  tealDim:     '#0F766E',
  black:       '#0A0A0A',
  surface:     '#111111',
  surfaceAlt:  '#1A1A1A',
  text:        '#F5F5F5',
  textMuted:   '#9CA3AF',
} as const;
```

### UnoCSS (uno.config.ts)

- Extend with Orbe tokens
- Shortcuts: `o-btn`, `o-card`, `o-input`, `o-badge`
- Dark-first. No light theme for MVP.
- Font: pair a geometric display (e.g. DM Sans, Space Grotesk not allowed) with mono accent

### PWA manifest

```ts
VitePWA({
  manifest: {
    name: 'Orbe',
    short_name: 'Orbe',
    theme_color: '#6B21A8',
    background_color: '#0A0A0A',
    display: 'standalone',
  },
  workbox: { globPatterns: ['**/*.{js,css,html,ico,png,svg}'] }
})
```

---

## ADR Templates (generate in A/1.Ad/1a.Arc/ of each repo)

### SurrealDB.md
- **Status**: Accepted
- **Context**: Need concurrent multi-model DB with graph edges for CRM relations; single binary in dev, cluster in prod; native Rust client
- **Decision**: SurrealDB 2.x
- **Consequences**: No Postgres ecosystem; live queries native; revisit at 10k+ sessions

### LayerNaming.md
- **Status**: Accepted
- **Context**: Clean architecture layer names must be short, distinct first-2-letters, modern
- **Decision**: Core / Flow / Gate / Wire (Co/Fl/Ga/Wi)
- **Consequences**: Deviates from textbook DDD; more approachable for new contributors

---

## Scaffold Tasks (execute in order)

### OrbeBackend
1. `jj git init --colocate` in `OrbeBackend/`
2. Create workspace `Cargo.toml` with `members = ["P/orbe-api", "P/orbe-inv"]`
3. Scaffold `P/orbe-inv/src/` — all 4 layers, stub structs, `ports.rs` traits
4. Scaffold `P/orbe-api/src/wire/` — Axum stubs compile clean
5. Write `R/1.Db/1a.Sch/Inventory.surql`
6. Write `R/2.Cf/Env.example` — `SURREAL_URL`, `SURREAL_USER`, `SURREAL_PASS`
7. Generate ADRs in `A/1.Ad/1a.Arc/`
8. `cargo build` — zero errors required before commit
9. `/use caveman-commit` → `feat(scaffold): workspace, Core/Flow/Gate/Wire, SurrealDB schema`

### OrbeFrontend
1. `jj git init --colocate` in `OrbeFrontend/`
2. `npm create vite@latest . -- --template vue-ts`
3. Move `src/` contents into `P/`; update `vite.config.ts` alias `@` → `./P`
4. Install: `@vite-pwa/vite-plugin unocss @unocss/vite pinia @tanstack/vue-query`
5. Write `uno.config.ts` with Orbe tokens
6. Write `P/1.Co/1b.Des/Tokens.ts`
7. Scaffold all dirs and stub components (must render without errors)
8. Configure router: `/` → inventory list stub
9. Generate ADRs in `A/1.Ad/1a.Arc/`
10. `npm run dev` — zero errors required before commit
11. `/use caveman-commit` → `feat(scaffold): vue3 pwa, JuanJo.Decimal structure, design tokens, inventory stubs`

---

## Critical Architecture Constraint

`orbe-api` (Wire layer) must ONLY import from `gate/ports.rs` traits.
It must NEVER import directly from `core/` or `flow/` of `orbe-inv`.
If `orbe-api` touches domain types directly → flag with `/use caveman-review`.

---

## Session Constraints

- No auth
- No CI
- No billing module
- No external APIs
- Everything must compile and run locally before first commit
- SurrealDB running locally: `surreal start --user root --pass root`
