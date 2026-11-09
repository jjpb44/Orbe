# ADR-001 · SurrealDB as data store

## Context
Orbe needs one store for documents + relations + graph queries (Phase 1: products,
categories, suppliers, stock movements). Single-language stack (Rust) on a single
monorepo. Colombian SMEs, low ops budget, embedded-friendly dev mode.

## Decision
**SurrealDB 2.x** as the single data store.

## Rationale
- **One engine** for relational + document + graph. No need to bolt on Neo4j or
  Postgres+JSONB; record links (`->product`) replace joins for graph traversal.
- **Schemafull mode** = strict types, defensive at the DB boundary. Lifts invariant
  guarantees that would otherwise live in app code.
- **Native `decimal` type** — Orbe stores prices as decimal (not float). No
  rounding errors in `price * qty`.
- **Rust SDK** (`surrealdb` crate) aligns with the no-JS stack decision (C3).
- **Embedded mode** for tests + dev; **standalone** for prod. Same SurrealQL
  either way.
- **WebSocket protocol** fits naturally into `axum` middleware.

## Consequences
- **New SQL dialect to learn** (SurrealQL). Mitigated: schema is small (4 tables),
  the team can grep the Surreal docs.
- **Smaller community than Postgres** → fewer SO answers, fewer ORMs. Mitigated:
  Rust SDK is stable, we don't use an ORM — we use `surrealdb::query()`.
- **Migration tooling less mature** than sqitch/flyway. Mitigated: schema is
  `DEFINE ... IF NOT EXISTS` — applied idempotently at startup by `gate::Surreal`.

## Alternatives considered
| Option       | Reject because                                             |
| ------------ | ---------------------------------------------------------- |
| Postgres     | Forces separate graph layer or PG extensions; bigger ops   |
| SQLite       | No graph, no live queries, no multi-writer                 |
| DynamoDB     | Vendor lock-in, eventual consistency hurts stock counters  |
| In-memory    | Doesn't survive restart; no shared state across processes  |

## References
- https://surrealdb.com/docs
- https://github.com/surrealdb/surrealdb/tree/main/crates/sdk
