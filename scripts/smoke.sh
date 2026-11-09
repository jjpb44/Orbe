#!/usr/bin/env bash
# Orbe smoke suite — REST contract + RPC happy paths.
#
#   scripts/smoke.sh              # against http://localhost:8080
#   scripts/smoke.sh --spawn      # boot a private server on :8099, run, tear down
#   BASE=... scripts/smoke.sh     # custom target
#
# REST errors are asserted with real HTTP semantics (404/409/422).
# RPC (Dioxus `#[server]`) wraps every failure as 500 — only success paths are
# asserted there, because the status code carries no information.
set -euo pipefail

SPAWN=0
[[ "${1:-}" == "--spawn" ]] && SPAWN=1

PORT="${PORT:-8099}"
BASE="${BASE:-http://localhost:${PORT}}"
SERVER_PID=""

cleanup() { [[ -n "$SERVER_PID" ]] && kill "$SERVER_PID" 2>/dev/null || true; }
trap cleanup EXIT

PASS=0 FAIL=0
ok()   { printf '  \033[32m✓\033[0m %s\n' "$1"; PASS=$((PASS+1)); }
fail() { printf '  \033[31m✗\033[0m %s\n' "$1"; FAIL=$((FAIL+1)); }
jqp()  { python3 -c "import sys,json;d=json.load(sys.stdin);$1" 2>/dev/null; }
code() { curl -s -o /dev/null -w '%{http_code}' "$@"; }

if (( SPAWN )); then
    # The Dioxus SSR router needs the built web assets on disk.
    ROOT="$(cd "$(dirname "$0")/.." && pwd)"
    if [[ ! -d "$ROOT/target/dx/orbe-api/debug/web/public" ]]; then
        echo "  no web bundle yet — building (dx build --platform web) …"
        ( cd "$ROOT/crates/orbe-api" && dx build --platform web ) >/tmp/orbe-smoke-build.log 2>&1 \
            || { echo "  wasm build failed — see /tmp/orbe-smoke-build.log"; exit 1; }
    fi
    # `cargo run` resolves assets from target/<profile>/public; dx writes them under target/dx.
    mkdir -p "$ROOT/target/debug"
    rm -rf "$ROOT/target/debug/public"
    cp -r "$ROOT/target/dx/orbe-api/debug/web/public" "$ROOT/target/debug/public"
    echo "  booting server on $BASE …"
    mkdir -p "$ROOT/target/ci"
    ( cd "$ROOT" && PORT="$PORT" IP=127.0.0.1 exec cargo run -q >"$ROOT/target/ci/smoke-server.log" 2>&1 ) &
    SERVER_PID=$!
    for _ in $(seq 1 120); do
        [[ "$(code "$BASE/api/v1/inventory/products")" == "200" ]] && break
        sleep 1
    done
    [[ "$(code "$BASE/api/v1/inventory/products")" == "200" ]] || {
        echo "  server never came up — tail of target/ci/smoke-server.log:"
        tail -15 "$ROOT/target/ci/smoke-server.log" | sed 's/^/    /'
        exit 1
    }
fi

SKU="smk-$(date +%s)-$$"
REST="$BASE/api/v1/inventory/products"
RPC="$BASE/api/products"

# ── REST: create ─────────────────────────────────────────────────────────────
c=$(code -X POST "$REST" -H 'Content-Type: application/json' \
    -d '{"sku":"'"$SKU"'","name":"Smoke","price_cents":1250,"stock":10,"min_stock":2}')
[[ "$c" == "201" ]] && ok "REST POST /products → 201" || { fail "REST POST → $c"; exit 1; }

# ── REST: read back ──────────────────────────────────────────────────────────
curl -s "$REST/$SKU" | jqp "assert d['id']=='$SKU' and d['price_cents']==1250 and d['stock']==10" \
    && ok "REST GET /products/{id} → id + price_cents + stock" \
    || fail "REST GET /products/{id} mismatch"

curl -s "$REST" | jqp "assert any(p['id']=='$SKU' for p in d)" \
    && ok "REST GET /products → contains new product" || fail "REST list missing product"

# ── REST: V2 uniqueness → 409 ────────────────────────────────────────────────
c=$(code -X POST "$REST" -H 'Content-Type: application/json' \
    -d '{"sku":"'"$SKU"'","name":"Dup","price_cents":1,"stock":0,"min_stock":0}')
[[ "$c" == "409" ]] && ok "REST POST duplicate sku → 409 (V2)" || fail "REST dup sku → $c"

# ── REST: update keeps stock ─────────────────────────────────────────────────
curl -s -X PUT "$REST/$SKU" -H 'Content-Type: application/json' \
    -d '{"name":"Smoke Updated","price_cents":2000,"min_stock":5}' \
    | jqp "assert d['name']=='Smoke Updated' and d['price_cents']==2000 and d['stock']==10" \
    && ok "REST PUT /products/{id} → updates fields, preserves stock" \
    || fail "REST PUT failed"

# ── REST: stock in / out (V4) ────────────────────────────────────────────────
curl -s -X POST "$REST/$SKU/stock" -H 'Content-Type: application/json' \
    -d '{"kind":"in","qty":5,"reason":"smoke in"}' \
    | jqp "assert d['product']['stock']==15 and d['movement']['qty']==5" \
    && ok "REST POST /stock in +5 → stock 15, movement +5" || fail "REST stock in failed"

curl -s -X POST "$REST/$SKU/stock" -H 'Content-Type: application/json' \
    -d '{"kind":"out","qty":-3,"reason":"smoke out"}' \
    | jqp "assert d['product']['stock']==12" \
    && ok "REST POST /stock out -3 → stock 12" || fail "REST stock out failed"

# ── REST: V5 oversell → 422, no movement written ─────────────────────────────
c=$(code -X POST "$REST/$SKU/stock" -H 'Content-Type: application/json' \
    -d '{"kind":"out","qty":-9999,"reason":"oversell"}')
[[ "$c" == "422" ]] && ok "REST POST /stock oversell → 422 (V5)" || fail "REST oversell → $c"

curl -s "$REST/$SKU/movements" | jqp "assert len(d)==2" \
    && ok "REST GET /movements → exactly 2 (oversell wrote none, V4/V5)" \
    || fail "REST movements count wrong"

# ── REST: 422 on negative price ──────────────────────────────────────────────
c=$(code -X POST "$REST" -H 'Content-Type: application/json' \
    -d '{"sku":"neg-'"$SKU"'","name":"Neg","price_cents":-100,"stock":0,"min_stock":0}')
[[ "$c" == "422" ]] && ok "REST POST negative price → 422" || fail "REST negative price → $c"

# ── REST: 404s ───────────────────────────────────────────────────────────────
c=$(code "$REST/nope-$SKU");        [[ "$c" == "404" ]] && ok "REST GET unknown → 404" || fail "REST GET unknown → $c"
c=$(code -X DELETE "$REST/nope-$SKU"); [[ "$c" == "404" ]] && ok "REST DELETE unknown → 404" || fail "REST DELETE unknown → $c"

# ── REST: delete ─────────────────────────────────────────────────────────────
c=$(code -X DELETE "$REST/$SKU");   [[ "$c" == "204" ]] && ok "REST DELETE → 204" || fail "REST DELETE → $c"
c=$(code "$REST/$SKU");             [[ "$c" == "404" ]] && ok "REST GET after delete → 404" || fail "REST GET-after-delete → $c"

# ── RPC: happy paths only (Dioxus wraps failures as 500) ─────────────────────
RSKU="rpc-$(date +%s)-$$"
curl -s -X POST "$RPC" -H 'Content-Type: application/json' \
    -d '{"new":{"sku":"'"$RSKU"'","name":"Rpc","price_cents":900,"stock":3,"min_stock":1}}' \
    | jqp "assert d['id']=='$RSKU'" && ok "RPC create_product" || fail "RPC create_product"

curl -s -X POST "$RPC/$RSKU/stock" -H 'Content-Type: application/json' \
    -d '{"kind":"in","qty":2,"reason":"rpc"}' \
    | jqp "assert isinstance(d,list) and len(d)==2" && ok "RPC adjust_stock → [Product, Movement]" || fail "RPC adjust_stock"

curl -s "$RPC/$RSKU/movements" | jqp "assert isinstance(d,list) and len(d)>=1" \
    && ok "RPC list_movements" || fail "RPC list_movements"

code -X DELETE "$RPC/$RSKU" >/dev/null && ok "RPC delete_product" || fail "RPC delete_product"

# ── Summary ──────────────────────────────────────────────────────────────────
echo
printf '  Results: %d passed, %d failed\n' "$PASS" "$FAIL"
[[ "$FAIL" -eq 0 ]] || exit 1
