#!/usr/bin/env bash
# Orbe local CI — the only gate. No hosted runner, no quota.
#
#   scripts/ci-report.sh            # fast gate: fmt, clippy, layers, tests, build
#   scripts/ci-report.sh --full     # + smoke (spawns server) + wasm build
#
# Writes target/ci/report.json and prints a summary.
# Exit 0 only if every executed check passed.
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT" || exit 1
OUT="$ROOT/target/ci"
REPORT="$OUT/report.json"
mkdir -p "$OUT"

MODE="fast"
[[ "${1:-}" == "--full" ]] && MODE="full"

CHECKS="$OUT/checks.tsv"
: > "$CHECKS"

# ── input digest ─────────────────────────────────────────────────────────────
# What the result is a statement about: the revision, the lockfile, the toolchain.
digest() {
    {
        jj log -r @ --no-graph -T 'change_id.short() ++ commit_id.short()' 2>/dev/null || echo "no-jj"
        sha256sum Cargo.lock 2>/dev/null
        sha256sum database/schema/Inventory.surql 2>/dev/null
        rustc --version 2>/dev/null
        cargo --version 2>/dev/null
        echo "mode=$MODE"
    } | sha256sum | cut -d' ' -f1
}
INPUTS=$(digest)

# ── runner ───────────────────────────────────────────────────────────────────
declare -a NAMES=()
FAILED=0

run() {
    local id="$1" label="$2" t0 t1 rc status
    shift 2
    t0=$(date +%s%3N)
    if command -v "${1%% *}" >/dev/null 2>&1; then
        "$@" > "$OUT/$id.log" 2>&1
        rc=$?
    else
        rc=127
        echo "tool not installed: $1" > "$OUT/$id.log"
    fi
    t1=$(date +%s%3N)
    case "$rc" in
        0)     status="pass" ;;
        127)   status="skip" ;;
        *)     status="fail"; FAILED=$((FAILED+1)) ;;
    esac
    printf '%s\t%s\t%s\t%s\n' "$id" "$status" "$((t1-t0))" "$label" >> "$CHECKS"
    NAMES+=("$id")
    if [[ "$status" == "fail" ]]; then
        printf '  \033[31m✗\033[0m %-14s %s  \033[90m(tail below)\033[0m\n' "$id" "$label"
        tail -15 "$OUT/$id.log" | sed 's/^/      /'
    elif [[ "$status" == "skip" ]]; then
        printf '  \033[33m•\033[0m %-14s %s  \033[90m(skipped: tool missing)\033[0m\n' "$id" "$label"
    else
        printf '  \033[32m✓\033[0m %-14s %s  \033[90m%d ms\033[0m\n' "$id" "$label" "$((t1-t0))"
    fi
}

echo
echo "  Orbe · local CI ($MODE)"
echo "  ────────────────────────────────────────────────────────"

run fmt     "cargo fmt --check"        cargo fmt --all --check
run clippy  "clippy -D warnings"       cargo clippy --all-targets -- -D warnings
run layer   "layer boundaries (V1/V7)" bash scripts/layer-check.sh
run test    "tests"                    cargo nextest run --no-fail-fast
run build   "server build"             cargo build
run parity  "deck bilingual parity"    bash scripts/check-bilingual-parity.sh
run typos   "spelling"                 typos --config .typos.toml

if [[ "$MODE" == "full" ]]; then
    run smoke "REST+RPC smoke"         bash scripts/smoke.sh --spawn
    run wasm  "release wasm bundle"    dx bundle --platform web --release
    run size  "wasm size budget"       bash scripts/wasm-size.sh --release
fi

# ── report ───────────────────────────────────────────────────────────────────
VERDICT=$([[ "$FAILED" -eq 0 ]] && echo pass || echo fail)

python3 - "$CHECKS" "$REPORT" "$INPUTS" "$MODE" "$VERDICT" <<'PY'
import json, sys, datetime, pathlib

checks_path, report_path, inputs, mode, verdict = sys.argv[1:6]
checks = []
for line in pathlib.Path(checks_path).read_text().splitlines():
    if not line.strip():
        continue
    cid, status, ms, label = (line.split("\t") + ["", "", "", ""])[:4]
    checks.append({"id": cid, "status": status, "ms": int(ms or 0), "label": label})

report = {
    "schema": 1,
    "tool": "orbe-ci",
    "at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
    "mode": mode,
    "verdict": verdict,
    "inputs_digest": inputs,
    "checks": checks,
    "note": "self-attested local run — proves which inputs were tested, not that the tree still matches",
}
pathlib.Path(report_path).write_text(json.dumps(report, indent=2) + "\n")
print(f"  report: {report_path}")
print(f"  digest: {inputs[:16]}…")
PY

echo "  ────────────────────────────────────────────────────────"
if [[ "$VERDICT" == "pass" ]]; then
    printf '  \033[32mCI green\033[0m\n\n'
    exit 0
fi
printf '  \033[31mCI red\033[0m — %d check(s) failed. Logs: target/ci/*.log\n\n' "$FAILED"
exit 1
