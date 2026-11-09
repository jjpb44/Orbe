# Plan: Restructure Backend PRA → QWZ (by volatility)

## Objective
Rename `Backend/{P,R,A}/` → `Backend/{Q,W,Z}/` and update every reference. JuanJo.Decimal v0.1 shifts from PARA-type-based to **volatility-based** (change frequency) axis.

## Mapping

| Old | New | Axis shift |
|-----|-----|------------|
| `P/` Projects | `Q/` | frequent change (active source) |
| `R/` Resources+Areas | `W/` | low change (schemas, configs, decks) |
| `A/` Archive | `Z/` | archive (ADRs, old docs) — unchanged semantics |

Acronym: **PRA → QWZ**. Top-level carve-out rule (single-letter dirs) identical — only letters + rationale change.

## Scope — includes
- 3 directory renames
- 1 runtime-critical Rust `include_str!` path
- justfile config
- CONVENTIONS.md (the convention definition itself)
- SPEC.md C4/C12 constraints + §T citations
- README.md, PLAN.md, DESIGN.md, GUIDE.md, DECKS_PLAN.md
- Backend/W/3.Dec/{README.md, onboarding.md}

## Scope — excludes
- Area/Category labels (`1.Dat`, `3.Dec`, `3a.Onbo` — unchanged, only root letters move)
- `orbe-api` internal layout (core/flow/gate/wire/ui — untouched)
- `.jj`/git internals (jj tracks content; renames automatic)

## Critical (build-breaking)
`Backend/P/orbe-api/src/gate/mod.rs:32`:
```rust
// OLD
const SCHEMA: &str = include_str!("../../../../R/1.Dat/1a.Sche/Inventory.surql");
// NEW
const SCHEMA: &str = include_str!("../../../../W/1.Dat/1a.Sche/Inventory.surql");
```
`../` count stays 4 (depth unchanged — renaming within `Backend/`). Miss this → `cargo build` fails with file-not-found.

## Proposed changes — by file

### Dir renames (3)
- `Backend/P/` → `Backend/Q/`
- `Backend/R/` → `Backend/W/`
- `Backend/A/` → `Backend/Z/`

### Rust (1 file, 4 edits)
`Backend/Q/orbe-api/src/gate/mod.rs`:
- L24 doc: `Backend/R/1.Dat/...` → `Backend/W/1.Dat/...`
- L29 doc: `Backend/P/orbe-api` → `Backend/Q/orbe-api`
- L32 `include_str!`: `../../../../R/` → `../../../../W/` **[build-critical]**

`Backend/Q/orbe-api/src/gate/surreal/connect.rs`:
- L3 doc: `Backend/R/2.Cfg/...` → `Backend/W/2.Cfg/...`

### justfile (3 edits)
- L5: `_crate := "Backend/Q/orbe-api"`
- L6: `_decks := "Backend/W/3.Dec"`
- L183: `bat Backend/W/1.Dat/1a.Sche/Inventory.surql`

### CONVENTIONS.md (convention definition — heaviest)
- §1 status: "PARA-derived hierarchy (P/R/A)" → "QWZ volatility-based (Q/W/Z)"
- §Top-Level heading: "(PRA — PARA without Areas)" → "(QWZ — by change frequency)"
- Table: P/R/A rows → Q/W/Z rows with volatility purpose text
- "alongside P/R/A" → "alongside Q/W/Z"
- PascalCase §: "Top-level PRA: `P`, `R`, `A`" → "Top-level QWZ: `Q`, `W`, `Z`"
- Adaptation §: "PRA top-level" → "QWZ top-level"
- Orbe application tree: P/ R/ A/ → Q/ W/ Z/
- §2 layout: `Backend/P/orbe-api/` → `Backend/Q/orbe-api/`
- §5: `Backend/R/1.Dat/2.Sed/` → `Backend/W/1.Dat/2.Sed/`
- §9 two rules: "other than P/R/A" → "other than Q/W/Z"; `Backend/R/1.Dat/2.Sed/` → `Backend/W/...`

### SPEC.md
- C4: "PRA top-level (P/R/A)" → "QWZ top-level (Q/W/Z)"
- C12: `Backend/R/3.Dec/` → `Backend/W/3.Dec/`
- §T citations (T2,T5,T6,T7): update path strings to Q/ and W/ **[optional — these are historical log entries; updating keeps them searchable, leaving preserves audit trail. Recommend update.]**

### README.md
- Structure tree (lines ~37-47): P/→Q/, R/→W/, A/→Z/
- L76: `cd Backend/Q/orbe-api`
- L117: `Backend/Q/orbe-api/public/main.css`

### PLAN.md
- L74, L75, L172: `Backend/P/orbe-api/...` → `Backend/Q/...`

### DESIGN.md
- L66: `Backend/P/orbe-api/assets/main.css` → `Backend/Q/...`

### GUIDE.md
- L185: `Backend/P/orbe-api/src/` → `Backend/Q/...`

### DECKS_PLAN.md
- All `Backend/R/3.Dec/` → `Backend/W/3.Dec/` (~6 occurrences)
- L233: `cd Backend/P/orbe-api` → `cd Backend/Q/orbe-api`

### Backend/W/3.Dec/README.md
- L46: `Backend/R/3.Dec/3a.Onbo/...` → `Backend/W/3.Dec/3a.Onbo/...`

### Backend/W/3.Dec/3a.Onbo/onboarding.md
- L27-32: table `Backend/P/orbe-api/src/...` → `Backend/Q/orbe-api/src/...`

## Steps
1. Rename 3 dirs: `Backend/P→Q`, `R→W`, `A→Z` (jj mv or plain mv).
2. Edit `gate/mod.rs:32` FIRST (build-critical) + its doc comments.
3. Edit `connect.rs:3` doc.
4. Edit justfile (3 lines).
5. Edit CONVENTIONS.md (convention definition).
6. Edit SPEC.md C4/C12 (+ §T citations).
7. Edit remaining docs: README, PLAN, DESIGN, GUIDE, DECKS_PLAN, 3.Dec/README, onboarding.md.

## Verification
```bash
# 1. build catches the include_str! path
just build                        # cargo build — MUST be green
just check                        # cargo check

# 2. zero stale references
rg 'Backend/[PRA](/|\b)' --type-not binary   # expect: 0 matches
rg '\bPRA\b|P/R/A'                            # expect: 0 matches (unless intentional history)

# 3. justfile vars resolve
just schema                       # bat Backend/W/1.Dat/... — confirms _crate/_decks

# 4. convention self-consistency
just status                       # jj log — confirm renames tracked
```
Acceptance: `just build` green + `rg 'Backend/[PRA]/'` returns nothing.

## Risks + assumptions
- **Build break** if `gate/mod.rs:32` missed — mitigated: edit first, `just build` first verification gate.
- §T is a historical log — updating path strings rewrites "what was done" records. Assumption: user wants searchable paths over frozen audit trail. If not, skip §T edits (low cost either way).
- Mnemonics for Q/W/Z not user-defined beyond semantics (frequent/low/archive). Plan uses the volatility axis as rationale; letters taken as given.
- ADRs in `Backend/A/1.ADR/` reference nothing about PRA internally (verified: 3 ADR files are decision records, no path deps) — rename is transparent.
- Version bump: CONVENTIONS.md §1 says "Draft v0.1". This is a convention change → recommend bump to v0.2 + SPEC §C entry per the "Updates require explicit amend CONVENTIONS.md + SPEC §C entry" rule at top of CONVENTIONS.md.
