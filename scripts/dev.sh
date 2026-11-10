#!/usr/bin/env bash
# Build the web bundle and serve it — the deterministic way to run Orbe locally.
#
#   scripts/dev.sh                 # debug bundle → http://localhost:8080
#   scripts/dev.sh --release       # release bundle (no dev overlay, slower build)
#   ORBE_SEED=1 scripts/dev.sh     # with the demo catalogue
#   PORT=8090 scripts/dev.sh       # different port
#
# Why not `dx serve`: in this workspace layout the CLI only builds the *server*
# and never emits the client bundle, so the browser gets HTML without the WASM
# entry point and the UI stays on "Cargando…". Building the bundle ourselves
# gives the server a complete `public/` next to the binary — SSR, hydration and
# assets all work.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROFILE=debug
[[ "${1:-}" == "--release" ]] && PROFILE=release

cd "$ROOT/crates/orbe-api"

echo "  building web bundle ($PROFILE) …"
dx build --platform web $([[ "$PROFILE" == "release" ]] && echo --release)

OUT="$ROOT/target/dx/orbe-api/$PROFILE/web"
BIN=$(ls -t "$OUT"/server-* 2>/dev/null | head -1)
[[ -x "$BIN" ]] || { echo "  server binary not found in $OUT"; exit 1; }

echo
echo "  Orbe → http://localhost:${PORT:-8080}"
echo "  Ctrl+C to stop"
echo

cd "$OUT"
exec "$BIN" "$@"
