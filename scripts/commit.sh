#!/usr/bin/env bash
# `just commit "msg"` — run the local gate, then commit with jj.
#
# Refuses to commit when the gate is red or when the tree changed while CI ran
# (the report's digest would no longer describe the tree being committed).
# Bypass: ORBE_SKIP_CI=1 (recorded in the report and printed loudly).
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

MSG="${1:-}"
[[ -n "$MSG" ]] || { echo "usage: just commit \"message\""; exit 2; }

REPORT="target/ci/report.json"

if [[ "${ORBE_SKIP_CI:-0}" == "1" ]]; then
    printf '\n  \033[33m!! CI skipped (ORBE_SKIP_CI=1)\033[0m\n'
else
    bash scripts/ci-report.sh || {
        printf '\n  \033[31mcommit blocked\033[0m — fix the failing checks, or ORBE_SKIP_CI=1 to override\n\n'
        exit 1
    }
fi

# The report must describe the tree we are about to commit.
if [[ "${ORBE_SKIP_CI:-0}" != "1" ]]; then
    reported=$(python3 -c "import json;print(json.load(open('$REPORT'))['inputs_digest'])" 2>/dev/null)
    current=$(bash -c '
        { jj log -r @ --no-graph -T "change_id.short() ++ commit_id.short()" 2>/dev/null || echo no-jj
          sha256sum Cargo.lock 2>/dev/null
          sha256sum database/schema/Inventory.surql 2>/dev/null
          rustc --version 2>/dev/null
          cargo --version 2>/dev/null
          echo "mode=fast"
        } | sha256sum | cut -d" " -f1')
    if [[ "$reported" != "$current" ]]; then
        printf '\n  \033[31mtree changed during CI\033[0m — rerun `just commit`\n\n'
        exit 1
    fi
fi

printf '\n  \033[32mcommitting\033[0m → %s\n\n' "$MSG"
jj describe -m "$MSG"
jj new
jj log --no-graph -n 3 -T 'change_id.short() ++ " " ++ description.first_line() ++ "\n"'
