# Orbe command runner. `just` (no args) lists the essentials; `just help` lists everything.
set shell := ["bash", "-euo", "pipefail", "-c"]
set positional-arguments

_crate := "crates/orbe-api"
_decks := "docs/decks"
_ci    := "scripts/ci-report.sh"

[private]
_h icon +name:
    @printf '\n  \033[1m\033[95m{{icon}}\033[0m {{name}}\n  \033[36m────────────────────────────────\033[0m\n\n'

default:
    @just essentials

# ── meta ─────────────────────────────────────────────────────────────────────
[group('meta')]
[doc('12 most-used commands')]
essentials:
    @printf '\n  \033[1m\033[35mOrbe\033[0m  \033[90m— essentials  (full list: just help)\033[0m\n\n'
    @printf '  \033[95m▶\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "dev" "dx serve (hot-reload)" "ci" "full local gate"
    @printf '  \033[95m◆\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "build" "cargo build" "test" "unit tests"
    @printf '  \033[95m◆\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "check" "cargo check" "smoke" "REST + RPC e2e"
    @printf '  \033[95m◆\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "run" "cargo run :8080" "lint" "clippy -D warnings"
    @printf '  \033[95m◆\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "status" "jj log" "fix" "fmt + clippy --fix"
    @printf '  \033[95m◆\033[0m %-10s \033[90m%-22s\033[0m  \033[95m■\033[0m %-10s \033[90m%s\033[0m\n' \
        "clean" "cargo clean" "commit" "ci + jj commit"
    @printf '  \033[95m◇\033[0m %-10s \033[90m%-22s\033[0m  \033[95m◇\033[0m %-10s \033[90m%s\033[0m\n\n' \
        "deck" "onboarding slides" "info" "repo stats"

[group('meta')]
[doc('full recipe reference')]
help:
    @just --list --list-heading $'\n  \033[1m\033[35mOrbe\033[0m \033[90m— all recipes\033[0m\n\n'

# ── dev ──────────────────────────────────────────────────────────────────────
[group('dev')]
[doc('dx serve fullstack (hot-reload)')]
dev:
    @just _h '▶' 'dev'
    @printf '  \033[96mhttp://localhost:8080\033[0m\n  \033[90mjust open · just smoke · Ctrl+C\033[0m\n\n'
    cd {{_crate}} && dx serve

# ── lifecycle ────────────────────────────────────────────────────────────────
[group('lifecycle')]
[doc('cargo build (server)')]
build:
    @just _h '◆' 'build'
    cargo build

[group('lifecycle')]
[doc('cargo check')]
check:
    @just _h '◆' 'check'
    cargo check

[group('lifecycle')]
[doc('cargo run — REST + UI on :8080')]
run:
    @just _h '◆' 'run'
    @printf '  \033[96mhttp://localhost:8080\033[0m\n\n'
    cargo run

[group('lifecycle')]
[doc('cargo clean (workspace target/)')]
clean:
    @just _h '◆' 'clean'
    cargo clean

[group('lifecycle')]
[doc('drop ship artifacts (release + dx platform dirs) — keeps the dev cache')]
clean-ship:
    @just _h '◆' 'clean-ship'
    @before=$(du -sh target 2>/dev/null | cut -f1); \
    rm -rf target/release target/dx target/server-release target/wasm-release \
           target/server-dev target/wasm-dev target/wasm32-unknown-unknown \
           target/x86_64-unknown-linux-gnu; \
    printf '  target %s → %s\n' "$before" "$(du -sh target 2>/dev/null | cut -f1)"

[group('lifecycle')]
[doc('disk usage breakdown of target/')]
target-size:
    @just _h '◆' 'target-size'
    @du -sh target/* 2>/dev/null | sort -h

[group('lifecycle')]
[doc('jj log — recent changes')]
status:
    @just _h '◆' 'status'
    jj log --no-graph -n 6

# ── ship ─────────────────────────────────────────────────────────────────────
[group('ship')]
[doc('build WASM bundle')]
wasm:
    @just _h '▲' 'wasm'
    cd {{_crate}} && dx build --platform web

[group('ship')]
[doc('wasm bundle size vs 500 KB gzip budget (add --release to check what ships)')]
wasm-size:
    @just _h '▲' 'wasm-size'
    bash scripts/wasm-size.sh

[group('ship')]
[doc('bundle release web app + PWA assets (manifest, icons, sw.js from public/)')]
pwa:
    @just _h '▲' 'pwa'
    cd {{_crate}} && dx bundle --platform web --release

[group('ship')]
[doc('build android APK')]
android:
    @just _h '▲' 'android'
    cd {{_crate}} && dx build --platform android

[group('ship')]
[doc('build native desktop binary')]
desktop:
    @just _h '▲' 'desktop'
    cargo build --features desktop

[group('ship')]
[doc('run desktop app')]
run-desktop:
    @just _h '▲' 'run-desktop'
    cd {{_crate}} && dx run --desktop

[group('ship')]
[doc('wasm + pwa, no prompts')]
ship: wasm pwa
    @just _h '▲' 'ship'
    @bash -c 'f=$$(fd -t f orbe-api_bg.wasm target/dx -1); [ -n "$$f" ] && printf "  wasm %s raw · %s gzip\n" "$$(du -h $$f | cut -f1)" "$$(gzip -c $$f | wc -c | numfmt --to=iec)" || true'

# ── test ─────────────────────────────────────────────────────────────────────
[group('test')]
[doc('unit tests (nextest when available)')]
test:
    @just _h '■' 'test'
    @if command -v cargo-nextest >/dev/null; then cargo nextest run; else cargo test; fi

[group('test')]
[doc('all tests incl. integration targets')]
test-all:
    @just _h '■' 'test-all'
    cargo test --workspace

[group('test')]
[doc('clippy -D warnings (all targets)')]
lint:
    @just _h '■' 'lint'
    cargo clippy --all-targets -- -D warnings

[group('test')]
[doc('layer boundaries V1/V7')]
layer-check:
    @just _h '■' 'layer-check'
    bash scripts/layer-check.sh

[group('test')]
[doc('REST + RPC end-to-end (spawns its own server)')]
smoke:
    @just _h '■' 'smoke'
    bash scripts/smoke.sh --spawn

[group('test')]
[doc('fmt + clippy --fix + check')]
fix:
    @just _h '■' 'fix'
    cargo fmt --all
    cargo clippy --fix --allow-dirty --allow-staged --all-targets -- -D warnings
    cargo check

# ── ci ───────────────────────────────────────────────────────────────────────
[group('ci')]
[doc('local CI gate — writes target/ci/report.json')]
ci:
    @bash {{_ci}}

[group('ci')]
[doc('full gate: ci + smoke + wasm')]
ci-full:
    @bash {{_ci}} --full

[group('ci')]
[doc('run gate then commit — args: MESSAGE')]
commit message:
    @bash scripts/commit.sh "{{message}}"

# ── db ───────────────────────────────────────────────────────────────────────
[group('db')]
[doc('view SurrealQL schema')]
schema:
    @just _h '●' 'schema'
    bat database/schema/Inventory.surql

[group('db')]
[doc('start SurrealDB (memory, trace)')]
surreal:
    @just _h '●' 'surreal'
    surreal start --log trace --user root --pass root memory

[group('db')]
[doc('boot with database/seeds/inventory.toon loaded (ORBE_SEED=1)')]
seed:
    @just _h '●' 'seed'
    @printf '  \033[96mhttp://localhost:8080\033[0m  \033[90mseeded · idempotent · Ctrl+C to stop\033[0m\n\n'
    ORBE_SEED=1 cargo run

# ── decks ────────────────────────────────────────────────────────────────────
[group('decks')]
[doc('present onboarding deck (EN)')]
deck: deck-en

[group('decks')]
[doc('present onboarding deck EN — interactive terminal')]
deck-en:
    @just _h '◇' 'deck-en'
    presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.en.md -X

[group('decks')]
[doc('present onboarding deck ES')]
deck-es:
    @just _h '◇' 'deck-es'
    presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.es.md -X

[group('decks')]
[doc('export onboarding decks as PDF (needs a TTY — that is presenterm, not us)')]
deck-pdf:
    @just _h '◇' 'deck-pdf'
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.en.md --export-pdf -o {{_decks}}/onboarding/onboarding.en.pdf" /dev/null
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.es.md --export-pdf -o {{_decks}}/onboarding/onboarding.es.pdf" /dev/null

[group('decks')]
[doc('export onboarding decks as HTML')]
deck-html:
    @just _h '◇' 'deck-html'
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.en.md --export-html -o {{_decks}}/onboarding/onboarding.en.html" /dev/null
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/onboarding/onboarding.es.md --export-html -o {{_decks}}/onboarding/onboarding.es.html" /dev/null

[group('decks')]
[doc('EN/ES structural parity check')]
deck-parity:
    @just _h '◇' 'deck-parity'
    bash scripts/check-bilingual-parity.sh

[group('decks')]
[doc('client PDF via typst — positional: CLIENT DATE SUMMARY NEXT CONTACT')]
deck-client client date='today' summary='Update in progress.' next='TBD' contact='Orbe team':
    @just _h '◇' 'deck-client'
    mkdir -p {{_decks}}/client/out
    typst compile {{_decks}}/client/overview.typ \
        --input "client=$1" --input "date=$2" --input "summary=$3" \
        --input "next=$4" --input "contact=$5" \
        {{_decks}}/client/out/orbe-overview.pdf
    @printf '  \033[96m→ {{_decks}}/client/out/orbe-overview.pdf\033[0m\n'

# ── misc ─────────────────────────────────────────────────────────────────────
[group('misc')]
[doc('repo stats — files, size, target size')]
info:
    @just _h '◇' 'info'
    @printf 'files   %s\n' "$(jj file list 2>/dev/null | wc -l)"
    @printf 'source  %s\n' "$(du -sh --exclude=target --exclude=.git --exclude=.jj . | cut -f1)"
    @printf 'target  %s\n' "$(du -sh target 2>/dev/null | cut -f1)"

[group('misc')]
[doc('open browser at :8080')]
open:
    @xdg-open http://localhost:8080 2>/dev/null || echo "→ http://localhost:8080"

[group('misc')]
[doc('fmt → clippy fix → build → test')]
loop:
    @just fix
    @just build
    @just test

[group('misc')]
[doc('check required toolchain')]
doctor:
    @just _h '◇' 'doctor'
    @for t in cargo rustc jj just dx typst presenterm cargo-nextest; do \
        if command -v "$t" >/dev/null; then printf '  \033[32m✓\033[0m %s\n' "$t"; \
        else printf '  \033[31m✗\033[0m %s \033[90m(missing)\033[0m\n' "$t"; fi; \
    done
