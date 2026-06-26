# ADR-002 · Clean Architecture layering for `orbe-api`

## Context
Single Rust crate (`orbe-api`). Strict layer boundaries keep test + compile
times low and make it impossible for the wire layer to bypass an invariant by
calling a gate implementation directly.

## Decision
Four layers in `src/`, top-down:

```
core/   — domain entities, value objects, errors.            NO IO
flow/   — use cases. Orchestrate core + ports.               NO IO
gate/   — adapters: SurrealDB impl of ports + in-memory impl for tests
wire/   — Axum routes, OpenAPI schemas, request/response DTOs
ui/     — Dioxus components + `#[server]` fns
```

**Port traits live in `gate/ports.rs`.** Flow imports the trait, not the impl.
This is the only place a swap (in-memory ↔ SurrealDB) happens.

## Invariants (carried from SPEC.md)
- **V1** `wire/` ⊥ `core/` | `flow/`. Wire only imports `gate/ports.rs` + DTOs.
- **V7** FE reaches data ONLY via `#[server]` fns (in `ui/`). Those fns delegate
  to `flow/`, which uses `gate/ports.rs`.

## Rationale
- **Test pyramid**: `core` + `flow` are unit-testable without a DB. `gate`
  is integration-tested against in-memory + real Surreal. `wire` is contract-
  tested against `flow` with mock ports.
- **Wire swap (HTTP → gRPC)** = one file change.
- **Gate swap (Surreal → in-memory for tests)** = one feature flag, zero prod change.

## Consequences
- **Upfront boilerplate**: every use case needs a port trait + DTO. We accept
  this for the compile-time guarantee.
- **"Where does X go?"** is answered by the V1/V7 check, not judgment.

## Module map (Phase 2 target)
```
src/
├── main.rs          # cfg(server) entry; ui mounted on Axum
├── core/
│   ├── mod.rs
│   ├── product.rs
│   ├── category.rs
│   ├── supplier.rs
│   ├── movement.rs
│   └── error.rs
├── flow/
│   ├── mod.rs
│   ├── list_products.rs
│   ├── create_product.rs
│   ├── adjust_stock.rs
│   └── ...
├── gate/
│   ├── mod.rs
│   ├── ports.rs
│   ├── memory.rs    # in-memory impl, used by tests
│   └── surreal.rs   # Phase 2 wire-up
├── wire/
│   ├── mod.rs       # Router + OpenAPI
│   └── dto.rs       # request/response types
└── ui/
    ├── mod.rs       # App root + router
    ├── inventory.rs # list, create, adjust pages
    └── components/  # ProductCard, StockBadge, etc.
```

## References
- Clean Architecture — Robert C. Martin (2017)
- Hexagonal Architecture — Alistair Cockburn (2005)
