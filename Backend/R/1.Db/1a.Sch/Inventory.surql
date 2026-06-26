-- Orbe · Inventory schema (Phase 1 stub)
-- Loaded via: surreal import --ns orbe --db inventory < Inventory.surql
-- Phase 2 wires the gate adapter (`gate::Surreal`) which applies this at startup
-- (or migrations apply it manually; idempotent because of IF NOT EXISTS semantics on DEFINE).

-- ─── products ─────────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS product SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS sku        ON TABLE product TYPE string     ASSERT $value != NONE AND string::len($value) > 0;
DEFINE FIELD IF NOT EXISTS name       ON TABLE product TYPE string     ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS category   ON TABLE product TYPE option<record<category>>;
DEFINE FIELD IF NOT EXISTS supplier   ON TABLE product TYPE option<record<supplier>>;
DEFINE FIELD IF NOT EXISTS price      ON TABLE product TYPE option<decimal>;
DEFINE FIELD IF NOT EXISTS stock      ON TABLE product TYPE int         DEFAULT 0 ASSERT $value >= 0;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE product TYPE datetime   DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE product TYPE datetime   DEFAULT time::now();
DEFINE INDEX IF NOT EXISTS product_sku ON TABLE product COLUMNS sku UNIQUE;

-- ─── categories ───────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS category SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS slug ON TABLE category TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS name ON TABLE category TYPE string ASSERT $value != NONE;
DEFINE INDEX IF NOT EXISTS category_slug ON TABLE category COLUMNS slug UNIQUE;

-- ─── suppliers ────────────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS supplier SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS nit  ON TABLE supplier TYPE option<string>;
DEFINE FIELD IF NOT EXISTS name ON TABLE supplier TYPE string ASSERT $value != NONE;
DEFINE INDEX IF NOT EXISTS supplier_nit ON TABLE supplier COLUMNS nit UNIQUE;

-- ─── stock movements ──────────────────────────────────────────────────────
DEFINE TABLE IF NOT EXISTS stock_movement SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS product ON TABLE stock_movement TYPE record<product> ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS delta   ON TABLE stock_movement TYPE int        ASSERT $value != 0;
DEFINE FIELD IF NOT EXISTS reason  ON TABLE stock_movement TYPE string     ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS at      ON TABLE stock_movement TYPE datetime   DEFAULT time::now();
