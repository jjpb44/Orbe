#!/usr/bin/env bash
# Layer boundary check — enforces SPEC §V V1 and V7 over the source tree.
#
#   V1: wire/ speaks flow/ + core/ DTOs only — ⊥ gate/surreal
#   V7: ui/   ⊥ import gate/surreal ...
#
# Exits 1 with the offending lines when a boundary is crossed.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
SRC="crates/orbe-api/src"
rc=0

report() { printf '  %s\n' "$*"; rc=1; }

# V1 — wire must not reach the SurrealDB adapter (it talks to ports + flow only).
hits=$(rg -n --no-heading 'use crate::gate::surreal' "$SRC/wire" 2>/dev/null || true)
if [[ -n "$hits" ]]; then
    report "V1 violated — wire/ imports gate/surreal:"
    printf '%s\n' "$hits" | sed 's/^/      /'
fi

hits=$(rg -n --no-heading 'surrealdb::' "$SRC/wire" 2>/dev/null || true)
if [[ -n "$hits" ]]; then
    report "V1 violated — wire/ imports surrealdb:"
    printf '%s\n' "$hits" | sed 's/^/      /'
fi

# V7 — UI must not import the SurrealDB adapter, only ports through flow.
hits=$(rg -n --no-heading 'use crate::gate::surreal' "$SRC/ui" 2>/dev/null || true)
if [[ -n "$hits" ]]; then
    report "V7 violated — ui/ imports gate/surreal:"
    printf '%s\n' "$hits" | sed 's/^/      /'
fi

# V7 — UI must not reach the DB crate at all.
hits=$(rg -n --no-heading 'surrealdb::' "$SRC/ui" 2>/dev/null || true)
if [[ -n "$hits" ]]; then
    report "V7 violated — ui/ imports surrealdb:"
    printf '%s\n' "$hits" | sed 's/^/      /'
fi

# Flow must stay framework-free (no axum, no surrealdb, no dioxus).
hits=$(rg -n --no-heading 'use (axum|surrealdb|dioxus)\b' "$SRC/flow" 2>/dev/null || true)
if [[ -n "$hits" ]]; then
    report "C2 violated — flow/ imports a framework:"
    printf '%s\n' "$hits" | sed 's/^/      /'
fi

[[ "$rc" -eq 0 ]] && echo "  layers clean"
exit "$rc"
