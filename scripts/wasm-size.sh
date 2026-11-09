#!/usr/bin/env bash
# WASM size budget (SPEC C3 / T29).
#
#   scripts/wasm-size.sh            # check the newest built wasm
#   scripts/wasm-size.sh --release  # require a release build (what ships)
#
# Budget: 500 KB gzip for orbe-api_bg.wasm (PLAN risk table). Raw size is
# reported, not enforced — gzip is what users download.
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

BUDGET_KB=500
MODE="any"
[[ "${1:-}" == "--release" ]] && MODE="release"

case "$MODE" in
    release) GLOB="target/dx/orbe-api/release" ;;
    *)       # prefer what ships; fall back to a dev build with a warning
             if [[ -d target/dx/orbe-api/release ]]; then GLOB="target/dx/orbe-api/release";
             else GLOB="target/dx/orbe-api"; fi ;;
esac

wasm=$(fd -t f -g "*_bg*.wasm" "$GLOB" 2>/dev/null | head -1)
if [[ -z "$wasm" ]]; then
    if [[ "$MODE" == "release" ]]; then
        echo "  no release wasm — run: just pwa"
        exit 1
    fi
    echo "  no wasm artifact — run: just wasm"
    exit 1
fi

raw=$(stat -c %s "$wasm")
gz=$(gzip -c "$wasm" | wc -c)
raw_h=$(numfmt --to=iec "$raw")
gz_h=$(numfmt --to=iec "$gz")
gz_kb=$(( gz / 1024 ))

printf '  wasm   %s\n' "$wasm"
printf '  raw    %s\n' "$raw_h"
printf '  gzip   %s  (budget %d KB)\n' "$gz_h" "$BUDGET_KB"

if (( gz_kb > BUDGET_KB )); then
    printf '  \033[31mover budget\033[0m by %d KB\n' "$(( gz_kb - BUDGET_KB ))"
    exit 1
fi

printf '  \033[32mwithin budget\033[0m (%d KB of %d)\n' "$gz_kb" "$BUDGET_KB"
