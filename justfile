# Orbe command runner. Run `just` (no args) to list essentials.

set dotenv-load

_crate := "Backend/Q/orbe-api"
_decks := "Backend/W/3.Dec"

default:
    @just essentials

# ── essentials (default `just`) — 12 most-used ────────────────────────────────
[doc('12 most-used commands (full: just help)')]
[group('meta')]
essentials:
    @printf '\n  \033[1m\033[35mOrbe\033[0m  \033[90m— essentials  (full: just help)\033[0m\n\n'
    @printf '  \033[1m\033[95m▶\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m◆\033[0m %-11s \033[90m%s\033[0m\n' "dev" "Hot-reload fullstack" "build" "cargo build"
    @printf '  \033[1m\033[95m◆\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m◆\033[0m %-11s \033[90m%s\033[0m\n' "check" "cargo check" "run" "cargo run"
    @printf '  \033[1m\033[95m◆\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m▲\033[0m %-11s \033[90m%s\033[0m\n' "clean" "cargo clean" "ship" "interactive platforms"
    @printf '  \033[1m\033[95m■\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m■\033[0m %-11s \033[90m%s\033[0m\n' "test" "cargo test --lib" "smoke" "curl 14 checks"
    @printf '  \033[1m\033[95m■\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m■\033[0m %-11s \033[90m%s\033[0m\n' "lint" "cargo clippy" "fix" "fmt + clippy fix"
    @printf '  \033[1m\033[95m◆\033[0m %-11s \033[90m%-26s\033[0m  \033[1m\033[95m◇\033[0m %-11s \033[90m%s\033[0m\n' "status" "jj log" "deck" "onboarding slides"
    @printf '\n'

# ── help — full reference (7 groups) ──────────────────────────────────────────
[doc('concise DX reference')]
help:
    @printf '\n  \033[1m\033[35mOrbe\033[0m\n\n'
    @printf '  \033[1m\033[95m▶ dev\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\n' "dev" "dx serve (hot-reload)"
    @printf '\n'
    @printf '  \033[1m\033[95m◆ lifecycle\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "build" "cargo build" "check" "cargo check"
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "run" "cargo run" "clean" "cargo clean"
    @printf '  %-14s\033[90m %-24s\033[0m\n' "status" "jj log"
    @printf '\n'
    @printf '  \033[1m\033[95m▲ ship\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "ship" "interactive platforms" "wasm" "dx build --platform web"
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "android" "dx build --android" "desktop" "cargo build --feat desktop"
    @printf '  %-14s\033[90m %-24s\033[0m\n' "pwa" "dx bundle --web --pwa"
    @printf '\n'
    @printf '  \033[1m\033[95m■ test\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "test" "cargo test --lib" "test-all" "cargo test"
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "smoke" "curl 14 checks" "lint" "cargo clippy"
    @printf '  %-14s\033[90m %-24s\033[0m\n' "fix" "fmt + clippy fix"
    @printf '\n'
    @printf '  \033[1m\033[95m● db\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "schema" "view surql" "surreal" "start server"
    @printf '  %-14s\033[90m %-24s\033[0m\n' "seed" "load .toon data"
    @printf '\n'
    @printf '  \033[1m\033[95m◇ misc\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "info" "repo stats" "open" "browser"
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "run-desk" "dx run --desktop" "loop" "build+test+fix"
    @printf '\n'
    @printf '  \033[1m\033[95m◇ decks\033[0m\n'
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "deck" "interactive onboarding" "deck-pdf" "export onboarding PDF"
    @printf '  %-14s\033[90m %-24s\033[0m\t%-14s\033[90m %-24s\033[0m\n' "deck-client" "export client PDF" "deck-html" "export onboarding HTML"
    @printf '\n'

# · ▶ dev
[doc('dx serve fullstack (hot-reload) — main dev loop')]
[group('dev')]
dev:
    @printf '\n  \033[1m\033[35mOrbe dev\033[0m\n'
    @printf '  \033[36m────────────────────────\033[0m\n'
    @printf '  \033[96mhttp://localhost:8080\033[0m\n\n'
    @printf '  \033[90mjust open  → browser\033[0m\n'
    @printf '  \033[90mjust smoke → test endpoints\033[0m\n'
    @printf '  \033[90mCtrl+C     → stop\033[0m\n\n'
    cd {{_crate}} && dx serve

# · ◆ lifecycle
[doc('cargo build (server binary)')]
[group('lifecycle')]
build:
    @printf '\n  \033[1m\033[95m◆\033[0m build\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo build

[doc('cargo check (fast lint)')]
[group('lifecycle')]
check:
    @printf '\n  \033[1m\033[95m◆\033[0m check\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo check

[doc('cargo run (server binary)')]
[group('lifecycle')]
run:
    @printf '\n  \033[1m\033[95m◆\033[0m run\n'
    @printf '  \033[36m────────────────────────\033[0m\n'
    @printf '  \033[96mhttp://localhost:8080\033[0m\n\n'
    @printf '  \033[90mjust open  → browser\033[0m\n'
    @printf '  \033[90mjust smoke → test endpoints\033[0m\n'
    @printf '  \033[90mjust dev   → hot-reload\033[0m\n\n'
    cd {{_crate}} && cargo run

[doc('cargo clean')]
[group('lifecycle')]
clean:
    @cd {{_crate}} && cargo clean

# · ▲ ship
[doc('build to selected platforms (interactive)')]
[group('ship')]
ship:
    @printf '\n  \033[1m\033[95m▲\033[0m ship\n  \033[36m────────────────────────\033[0m\n\n'
    @printf "  WASM (web)? [y/N] " && read ans && [ "$$ans" = "y" -o "$$ans" = "Y" ] && cd {{_crate}} && dx build --platform web || true
    @printf "  Android APK? [y/N] " && read ans && [ "$$ans" = "y" -o "$$ans" = "Y" ] && cd {{_crate}} && dx build --platform android || true
    @printf "  Desktop? [y/N] " && read ans && [ "$$ans" = "y" -o "$$ans" = "Y" ] && cd {{_crate}} && cargo build --features desktop || true
    @printf "  PWA bundle? [y/N] " && read ans && [ "$$ans" = "y" -o "$$ans" = "Y" ] && cd {{_crate}} && dx bundle --platform web --pwa || true

[doc('jj log — recent commits')]
[group('lifecycle')]
status:
    @printf '\n  \033[1m\033[95m◆\033[0m status\n  \033[36m────────────────────────\033[0m\n\n'
    @jj log --no-graph -n 5

# · ▲ ship
[doc('build desktop binary (native webview)')]
[group('ship')]
desktop:
    @printf '\n  \033[1m\033[95m◇\033[0m desktop\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo build --features desktop

[doc('run desktop app (hot-reload webview)')]
[group('misc')]
run-desk:
    @printf '\n  \033[1m\033[95m◇\033[0m run-desk\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && dx run --desktop

[doc('build WASM bundle')]
[group('ship')]
wasm:
    @printf '\n  \033[1m\033[95m◇\033[0m wasm\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && dx build --platform web

[doc('build android APK')]
[group('ship')]
android:
    @printf '\n  \033[1m\033[95m◇\033[0m android\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && dx build --platform android

[doc('bundle PWA (WASM + manifest + icons)')]
[group('ship')]
pwa:
    @printf '\n  \033[1m\033[95m◇\033[0m pwa\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && dx bundle --platform web --pwa

# · ■ test
[doc('run unit tests')]
[group('test')]
test:
    @printf '\n  \033[1m\033[95m■\033[0m test\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo test --lib

[doc('run all tests (unit + integration)')]
[group('test')]
test-all:
    @printf '\n  \033[1m\033[95m■\033[0m test-all\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo test

[doc('curl 14-endpoint smoke test suite')]
[group('test')]
smoke:
    @printf '\n  \033[1m\033[95m■\033[0m smoke\n  \033[36m────────────────────────\033[0m\n\n'
    @bash Backend/W/2.Cfg/scripts/orbe-smoke.sh

[doc('cargo clippy — D warnings')]
[group('test')]
lint:
    @printf '\n  \033[1m\033[95m■\033[0m lint\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo clippy -- -D warnings

[doc('fmt + clippy fix + check')]
[group('test')]
fix:
    @printf '\n  \033[1m\033[95m■\033[0m fix\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo fmt && cargo clippy --fix --allow-dirty -- -D warnings && cargo check

# · ● db
[doc('view SurrealQL schema')]
[group('db')]
schema:
    @printf '\n  \033[1m\033[95m●\033[0m schema\n  \033[36m────────────────────────\033[0m\n\n'
    @bat Backend/W/1.Dat/1a.Sche/Inventory.surql

[doc('start SurrealDB server (memory, trace)')]
[group('db')]
surreal:
    @printf '\n  \033[1m\033[95m●\033[0m surreal\n  \033[36m────────────────────────\033[0m\n\n'
    @surreal start --log trace --user root --pass root memory

[doc('load .toon seed data into DB')]
[group('db')]
seed:
    @printf '\n  \033[1m\033[95m●\033[0m seed\n  \033[36m────────────────────────\033[0m\n\n'
    @echo "TODO: wire .toon seed loader"

# · ◇ misc
[doc('repo stats — files + size')]
[group('misc')]
info:
    @printf '\n  \033[1m\033[95m▲\033[0m info\n  \033[36m────────────────────────\033[0m\n\n'
    @printf "files  "; jj file list | wc -l
    @printf "size   "; du -sh --exclude=target --exclude=.git --exclude=.jj .

[doc('open browser at localhost:8080')]
[group('misc')]
open:
    @printf '\n  \033[1m\033[95m▲\033[0m open\n  \033[36m────────────────────────\033[0m\n\n'
    @xdg-open http://localhost:8080 2>/dev/null || open http://localhost:8080 2>/dev/null || echo "→ http://localhost:8080"

[doc('loop: fmt → clippy fix → build → test')]
[group('misc')]
loop:
    @printf '\n  \033[1m\033[95m▲\033[0m loop\n  \033[36m────────────────────────\033[0m\n\n'
    @cd {{_crate}} && cargo fmt && cargo clippy --fix --allow-dirty -- -D warnings && cargo build && cargo test --lib && echo "✅ all green"

# · ◇ decks
[doc('present onboarding deck EN (terminal, interactive, runs +exec_replace)')]
[group('decks')]
deck:
    @just deck-en

[doc('present onboarding deck EN (alias of `deck`)')]
[group('decks')]
deck-en:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-en\n  \033[36m────────────────────────\033[0m\n\n'
    presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.en.md -X

[doc('present onboarding deck ES')]
[group('decks')]
deck-es:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-es\n  \033[36m────────────────────────\033[0m\n\n'
    presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.es.md -X

[doc('export both onboarding decks as PDF (static; +exec_replace rendered as code)')]
[group('decks')]
deck-pdf:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-pdf\n  \033[36m────────────────────────\033[0m\n\n'
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.en.md --export-pdf -o {{_decks}}/3a.Onbo/onboarding.en.pdf" /dev/null
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.es.md --export-pdf -o {{_decks}}/3a.Onbo/onboarding.es.pdf" /dev/null
    @printf '  \033[96m→ {{_decks}}/3a.Onbo/onboarding.en.pdf + onboarding.es.pdf\033[0m\n'

[doc('export both onboarding decks as HTML')]
[group('decks')]
deck-html:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-html\n  \033[36m────────────────────────\033[0m\n\n'
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.en.md --export-html -o {{_decks}}/3a.Onbo/onboarding.en.html" /dev/null
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.es.md --export-html -o {{_decks}}/3a.Onbo/onboarding.es.html" /dev/null

[doc('check structural parity between onboarding.en.md and onboarding.es.md')]
[group('decks')]
deck-parity:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-parity\n  \033[36m────────────────────────\033[0m\n\n'
    bash {{_decks}}/scripts/check-bilingual-parity.sh {{_decks}}/3a.Onbo/onboarding.en.md {{_decks}}/3a.Onbo/onboarding.es.md

[doc('export client overview PDF — args: CLIENT DATE SUMMARY NEXT CONTACT (positional)')]
[group('decks')]
deck-client CLIENT DATE='today' SUMMARY='Update in progress.' NEXT='TBD' CONTACT='Orbe team':
    @printf '\n  \033[1m\033[95m◇\033[0m deck-client\n  \033[36m────────────────────────\033[0m\n\n'
    @mkdir -p {{_decks}}/3b.Clnt/out
    env CLIENT_NAME="{{CLIENT}}" DATE="{{DATE}}" STATUS_SUMMARY="{{SUMMARY}}" NEXT_MILESTONES="{{NEXT}}" CONTACT="{{CONTACT}}" \
        envsubst < {{_decks}}/3b.Clnt/template.md > /tmp/orbe-client.md
    script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha /tmp/orbe-client.md --export-pdf -o {{_decks}}/3b.Clnt/out/orbe-overview.pdf" /dev/null
    @printf '  \033[96m→ {{_decks}}/3b.Clnt/out/orbe-overview.pdf\033[0m\n'
