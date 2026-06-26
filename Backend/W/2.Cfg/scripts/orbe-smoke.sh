#!/usr/bin/env bash
# Orbe smoke test — 14-endpoint E2E suite
# Requires: server running at localhost:8080 (just dev)
set -e

BASE="http://localhost:8080"
PASS=0 FAIL=0

ok()  { echo "  ✓ $1"; PASS=$((PASS+1)); }
fail() { echo "  ✗ $1"; FAIL=$((FAIL+1)); }

# ── 1. List products (expect 200) ────────────────────────────────────────────
code=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/products")
[[ "$code" == "200" ]] && ok "GET /api/products → 200" || fail "GET /api/products → $code"

# ── 2. Create product ─────────────────────────────────────────────────────────
SKU="smk-$(date +%s)"
PRODUCT=$(curl -s -X POST "$BASE/api/products" \
    -H 'Content-Type: application/json' \
    -d '{"new":{"sku":"'"$SKU"'","name":"Smoke Test","price_cents":1250,"stock":10,"min_stock":2}}')

ID=$(echo "$PRODUCT" | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])" 2>/dev/null)
if [[ -n "$ID" && "$ID" == "$SKU" ]]; then
    ok "POST /api/products → 200, id=$ID"
else
    fail "POST /api/products — bad id: $ID | body: $PRODUCT"
    echo "  Smoke test cannot continue — fix create first"
    exit 1
fi

# ── 3. Get product by ID ──────────────────────────────────────────────────────
p=$(curl -s "$BASE/api/products/$ID")
echo "$p" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['id']=='$ID'" 2>/dev/null \
    && ok "GET /api/products/$ID → 200" \
    || fail "GET /api/products/$ID — mismatch: $p"

# ── 4. List all products (should contain new product) ─────────────────────────
list=$(curl -s "$BASE/api/products")
echo "$list" | python3 -c "import sys,json; d=json.load(sys.stdin); assert any(p['id']=='$ID' for p in d)" 2>/dev/null \
    && ok "GET /api/products → contains new product" \
    || fail "GET /api/products — product not in list"

# ── 5. Update product ─────────────────────────────────────────────────────────
updated=$(curl -s -X PUT "$BASE/api/products/$ID" \
    -H 'Content-Type: application/json' \
    -d '{"patch":{"sku":"'"$SKU"'","name":"Smoke Updated","price_cents":2000,"stock":20,"min_stock":5}}')
echo "$updated" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['name']=='Smoke Updated' and d['price_cents']==2000" 2>/dev/null \
    && ok "PUT /api/products/$ID → 200, updated" \
    || fail "PUT /api/products/$ID — failed: $updated"

# ── 6. Adjust stock in (+5) ───────────────────────────────────────────────────
adj=$(curl -s -X POST "$BASE/api/products/$ID/stock" \
    -H 'Content-Type: application/json' \
    -d '{"kind":"in","qty":5,"reason":"smoke test"}')
# Response is [Product, StockMovement]
echo "$adj" | python3 -c "import sys,json; d=json.load(sys.stdin); assert isinstance(d,list) and len(d)==2" 2>/dev/null \
    && ok "POST /api/products/$ID/stock (in +5) → 200" \
    || fail "POST /api/products/$ID/stock (in): $adj"

# ── 7. Adjust stock out (-3) ─────────────────────────────────────────────────
adj2=$(curl -s -X POST "$BASE/api/products/$ID/stock" \
    -H 'Content-Type: application/json' \
    -d '{"kind":"out","qty":-3,"reason":"smoke removal"}')
echo "$adj2" | python3 -c "import sys,json; d=json.load(sys.stdin); assert isinstance(d,list)" 2>/dev/null \
    && ok "POST /api/products/$ID/stock (out -3) → 200" \
    || fail "POST /api/products/$ID/stock (out): $adj2"

# ── 8. List movements ─────────────────────────────────────────────────────────
mov=$(curl -s "$BASE/api/products/$ID/movements")
echo "$mov" | python3 -c "import sys,json; d=json.load(sys.stdin); assert isinstance(d,list) and len(d)>=2" 2>/dev/null \
    && ok "GET /api/products/$ID/movements → 200, ≥2 movements" \
    || fail "GET /api/products/$ID/movements: $mov"

# ── 9. Delete product ─────────────────────────────────────────────────────────
code=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/products/$ID")
# Note: current impl returns 200 null, not 204 — tracked as bug
[[ "$code" == "200" ]] && ok "DELETE /api/products/$ID → 200" || fail "DELETE /api/products/$ID → $code (got $code, expected 200)"

# ── 10. GET deleted product → 500 (current impl returns 500, not 404) ─────────
code=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/api/products/$ID")
[[ "$code" == "500" ]] && ok "GET /api/products/$ID (deleted) → 500 (not found)" || fail "GET (deleted) → $code (expected 500)"

# ── 11. DELETE non-existent → 200 (current impl returns 200, not 404) ──────────
code=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$BASE/api/products/nonexistent")
[[ "$code" == "200" ]] && ok "DELETE /api/products/nonexistent → 200" || fail "DELETE nonexistent → $code"

# ── 12. POST with missing required field (sku) → 500 ─────────────────────────
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/products" \
    -H 'Content-Type: application/json' \
    -d '{"new":{"name":"NoSku","price_cents":0,"stock":0,"min_stock":0}}')
[[ "$code" == "500" ]] && ok "POST /api/products (missing sku) → 500" || fail "POST (missing sku) → $code"

# ── 13. POST with negative price → 500 ───────────────────────────────────────
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/products" \
    -H 'Content-Type: application/json' \
    -d '{"new":{"sku":"neg-test","name":"Neg","price_cents":-100,"stock":0,"min_stock":0}}')
[[ "$code" == "500" ]] && ok "POST /api/products (negative price) → 500" || fail "POST (negative price) → $code"

# ── 14. POST stock on non-existent → 500 ─────────────────────────────────────
code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE/api/products/nonexistent/stock" \
    -H 'Content-Type: application/json' \
    -d '{"kind":"in","qty":1,"reason":"test"}')
[[ "$code" == "500" ]] && ok "POST /api/products/nonexistent/stock → 500" || fail "POST stock (nonexistent) → $code"

# ── Summary ───────────────────────────────────────────────────────────────────
echo
echo "  Results: $PASS passed, $FAIL failed"
[[ "$FAIL" -eq 0 ]] && exit 0 || exit 1
