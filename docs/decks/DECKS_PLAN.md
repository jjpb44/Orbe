# Plan — presenterm Integration (Dev Onboarding + Client PDF Automation)

## Objective

Integrate `presenterm` (terminal markdown presentation, v0.16.1, already installed) as a first-class DX surface: (A) a ≤4-slide onboarding deck that replaces README-skimming for new devs, and (B) a one-command pipeline that turns a markdown template into a client-facing PDF so devs stop hand-formatting progress updates.

## Context

- **Project mood:** Orbe is a dark-first, immersive, fast, magical CRM (Nequi/Nu/WhatsApp vibe). Decks should feel the same — dark surfaces with a purple-magenta primary and teal accent. A purely-black/white terminal theme would betray the brand.
- **Pain addressed:** new devs read 4 root docs (README/SPEC/CONVENTIONS/DESIGN, ~600 lines) before writing code; external actors (clients) get ad-hoc progress updates that cost dev time to assemble.
- **Tooling match:** Orbe is justfile-driven (20 recipes, symbol groups ▶◆▲■●◇). presenterm fits the same CLI-first, no-JS, single-binary ethos. No new runtime, no Node.
- **PDF backend verified present:** `typst`, `weasyprint`, `chromium` all on PATH. presenterm 0.16.1 uses typst for `--export-pdf`.
- **Cognitive-load rule (SPEC §G):** every screen, doc, deck, recipe is judged by how much working memory it costs a new dev. ≤4 slides, terse copy, explicit references — not decoration.

## Theme decision — `catppuccin-mocha` (built-in)

**Chosen theme:** `catppuccin-mocha` (presenterm built-in, selected via `-t` flag).

**Why:**
- Dark-first by default — matches Orbe's `#0A0A0A` base mood.
- Mauve accent `#cba6f7` — closest built-in palette to Orbe's purple `#6B21A8` / purple-light `#9333EA`.
- Zero custom theme files. No schema drift, no validation risk, no `~/.config/presenterm` pollution.
- One flag (`-t`) on every recipe invocation — reproducible across machines.

**Explicit decision (user, m0058):** "Use default theme that is closer to the projected dark magenta and black theme." — i.e. built-in, not custom. We rejected the custom `themes/orbe.yaml` route because the presenterm 0.16.1 theme schema rejected our `theme:` root key (must be selected via `-t`), and even with the right schema, an on-brand-perfect palette is not load-bearing — the deck's *content* (≤4 slides, mood-as-prose) carries the brand far more than the chrome does.

## Scope

### Includes
- `docs/decks/` resource directory (JuanJo.Decimal v0.1 — Areas=3 letters, Categories=4 letters).
  - `3a.Onbo/onboarding.md` — 4 slides, new-dev target audience.
  - `3b.Clnt/template.md` + `out/` — client PDF pipeline.
- New `◇ decks` justfile group: `deck`, `deck-pdf`, `deck-client`, `deck-html`. All pass `-t catppuccin-mocha`.
- CONVENTIONS.md §1 update + SPEC §C (C12) + README Structure/Quick Start.
- `.gitignore` for generated PDFs (preserve `.gitkeep`).

### Excludes
- Custom theme file (`themes/orbe.yaml`) — rejected; catppuccin-mocha is good enough.
- Per-call config file (`presenterm.yaml`) — rejected; flags only.
- Speaker-notes streaming (`-P`/`-l`) — YAGNI for solo/async dev onboarding.
- HTML export wiring beyond a single recipe — PDF is the primary client deliverable.
- Auto-rebuild PDFs in `just dev` watch loop — decks are async artifacts.
- Live data binding from SurrealDB into client PDFs — Phase 2; templates use `${PLACEHOLDER}` + `envsubst` for now.
- `deck-progress` for release notes — single overview template covers client scope.

## Architecture decisions

### D1 — Location: `docs/decks/onboarding/` + `docs/decks/client/`
JuanJo.Decimal v0.1: Areas=3 letters (`Dec`=Decks), Categories=4 letters (`Onbo`=Onboarding, `Clnt`=Client). Decimal scheme extends naturally; root-level `Decks/` would violate the PRA convention.

### ~~D2 — Custom theme, not built-in~~ → SUPERSEDED by Theme decision above
Custom `themes/orbe.yaml` attempted, then deleted. Built-in `catppuccin-mocha` (selected via `-t`) replaces it. Less code, less drift risk.

### ~~D3 — Per-call config via `-c`, not global~~ → SUPERSEDED by Theme decision above
`presenterm.yaml` attempted, then deleted. No config file at all — flags only. Simpler.

### D4 — Snippet execution for onboarding only
Onboarding deck uses `+exec_replace` blocks (`-X` flag) to run real `just dev` and render its output inline — proves the dev loop works on first read. Client deck **never** executes snippets (deterministic output, no machine-specific paths leaking to external actors). `deck-pdf` sets `DECK_STATIC=1` so the live snippet is pre-rendered (the snippet block is gated by `<!-- skip_slide: env DECK_STATIC -->` so the PDF variant skips it cleanly).

### D5 — Placeholder substitution via `envsubst`, not templating engine
Client template uses `${VAR}` tokens. `just deck-client CLIENT="..."` pipes through `envsubst` before presenterm. Zero new deps.

### D6 — Mood transmitted explicitly, not as decoration
Slide 4 is **prose describing the UX feel** — references (Nequi/Nu/WhatsApp), what "fast" means in concrete UX terms (tap card → inline adjust → no reload), tactile language (dark surfaces, soft glows). Cognitive-load principle: tell the reader what to feel, don't show them a palette and hope they infer it.

### D7 — Single client overview template, scope-locked
One `template.md` covers the client use case now. No `deck-progress` recipe in v1. Easy to add when a real need surfaces.

## Proposed file tree (actual)

```
docs/decks/
├── 3a.Onbo/
│   ├── onboarding.md          # 4 slides: What / Stack / Start / Mood
│   └── onboarding.pdf         # generated by `just deck-pdf`
├── 3b.Clnt/
│   ├── template.md            # ${CLIENT_NAME}, ${DATE}, ${STATUS_SUMMARY}, ${NEXT_MILESTONES}, ${CONTACT}
│   └── out/                   # generated PDFs — gitignored (`.gitkeep` preserved)
├── README.md                  # how to edit decks, regenerate, add new ones
```

No `presenterm.yaml`. No `themes/`. Theme via `-t catppuccin-mocha` flag in each recipe.

## Onboarding deck content spec (4 slides, hard cap)

Per user: "what is the project about, what do they expect, how to begin, project mood in a glance."

**Slide 1 — What** (title slide, centered, vertical jump)
- `# Orbe` (display weight)
- `> CRM for Colombian SMEs. Inventory-first. Grow from there.`
- `MIT · MVP complete · Rust end-to-end`
- `<!-- alignment: center -->` + `<!-- jump_to_middle -->`

**Slide 2 — Stack & Shape**
- Compact table: Dioxus 0.7 · Axum 0.8 · SurrealDB 3.2 · jj · plain CSS
- ASCII clean-arch diagram (Core→Flow→Gate→Wire + UI via `#[server]`)
- One line: "No JS, no Node, one toolchain (`cargo` + `dx`)."
- `<!-- column_layout: [2, 3] -->` — table left, diagram right.

**Slide 3 — Start in 60 seconds** (the money slide)
- ```bash
  +exec_replace:{cmd="just dev", lines=12}
  ```
- Below: "Read next: `SPEC.md` → `CONVENTIONS.md` → `GUIDE.md`."
- `<!-- skip_slide: env DECK_STATIC -->` — `deck-pdf` (with `DECK_STATIC=1`) skips the live snippet for a deterministic export.
- `<!-- speaker_note: just dev runs dx serve on :8080 -->`

**Slide 4 — Mood** (UX prose, NOT swatches)
- One heading: "How it should feel"
- 3 short paragraphs (~2 sentences each):
  1. **Fast.** Tap a product card, the quantity adjusts inline — no page reload, no spinner. Numbers animate from old to new. Every action is one tap away; chrome stays hidden until you reach for it.
  2. **Bold.** Numbers are huge. Tap targets are huge. The whole screen is the primary action — no decorative chrome competing for attention.
  3. **Tactile.** Dark surfaces (`#0A0A0A`) with a purple-magenta glow (`#6B21A8`) and teal accents (`#2DD4BF`). Inspired by Nequi (instant, big-number payment feel), Nu (warm, soft), WhatsApp (chat-fast, presence-aware).
- Closing line: "If a screen feels like a desktop app from 2003, we missed it."
- `<!-- end_slide -->` closes.

> Constraint: ≤4 slides is a hard cap. If a 5th topic surfaces, it goes into `docs/decks/README.md`, not a slide.

## Client PDF pipeline spec

`template.md` structure (placeholder-driven, deterministic, NO `+exec_replace`):
```
# ${CLIENT_NAME} — Orbe Overview
_Generated ${DATE}_

## Status
${STATUS_SUMMARY}

## What's working
${WHAT_WORKS}

## Next milestones
${NEXT_MILESTONES}

## Contact
${CONTACT}
```

`just deck-client CLIENT="Acme" DATE="2026-07-02"` flow:
1. `envsubst < docs/decks/client/template.md > /tmp/orbe-client.md` with env from just args.
2. `presenterm -t catppuccin-mocha /tmp/orbe-client.md --export-pdf -o docs/decks/client/out/orbe-overview-${DATE}.pdf`
3. Print output path.

Placeholders are explicit `${UPPERCASE}` — never silent magic. Dev fills them via just args or by setting env vars.

## Justfile integration (actual)

New `◇ decks` group, 4 recipes — already applied:

```just
_decks := "docs/decks"

# · ◇ decks
[doc('present onboarding deck (terminal, interactive, runs +exec_replace)')]
[group('decks')]
deck:
    @printf '\n  \033[1m\033[95m◇\033[0m deck\n  \033[36m────────────────────────\033[0m\n\n'
    presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.md -X

[doc('export onboarding as PDF (skips live snippet; static)')]
[group('decks')]
deck-pdf:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-pdf\n  \033[36m────────────────────────\033[0m\n\n'
    DECK_STATIC=1 presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.md --export-pdf -o {{_decks}}/3a.Onbo/onboarding.pdf
    @printf '  \033[96m→ {{_decks}}/3a.Onbo/onboarding.pdf\033[0m\n'

[doc('export client overview PDF — args: CLIENT DATE SUMMARY NEXT CONTACT')]
[group('decks')]
deck-client CLIENT DATE='today' SUMMARY='Update in progress.' NEXT='TBD' CONTACT='Orbe team':
    #!/usr/bin/env bash
    set -euo pipefail
    @printf '\n  \033[1m\033[95m◇\033[0m deck-client\n  \033[36m────────────────────────\033[0m\n\n'
    @mkdir -p {{_decks}}/3b.Clnt/out
    @CLIENT_NAME="{{CLIENT}}" DATE="{{DATE}}" STATUS_SUMMARY="{{SUMMARY}}" NEXT_MILESTONES="{{NEXT}}" CONTACT="{{CONTACT}}" \
        envsubst < {{_decks}}/3b.Clnt/template.md > /tmp/orbe-client.md
    @presenterm -t catppuccin-mocha /tmp/orbe-client.md --export-pdf -o {{_decks}}/3b.Clnt/out/orbe-overview.pdf
    @printf '  \033[96m→ {{_decks}}/3b.Clnt/out/orbe-overview.pdf\033[0m\n'

[doc('export onboarding as HTML (web embed)')]
[group('decks')]
deck-html:
    @printf '\n  \033[1m\033[95m◇\033[0m deck-html\n  \033[36m────────────────────────\033[0m\n\n'
    presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.md --export-html -o {{_decks}}/3a.Onbo/onboarding.html
```

`deck` swapped into `essentials` row 6 (replacing `info`). `◇ decks` block added to `help`. `6 groups` comment → `7 groups`.

## Steps — status

| # | Step | Status |
|---|---|---|
| 1 | Scaffold `docs/decks/` tree | ✅ done (m0039) |
| 2 | ~~Write `presenterm.yaml` + `themes/orbe.yaml`~~ | ❌ SUPERSEDED (catppuccin-mocha instead) |
| 3 | Write `3a.Onbo/onboarding.md` (4 slides, slide 4 = UX prose) | ✅ done (m0041) |
| 4 | Iterate: `presenterm -t catppuccin-mocha ... --validate-overflows` until clean | ✅ done (m0060, exit 0) |
| 5 | Write `3b.Clnt/template.md` with placeholders | ✅ done (m0041) |
| 6 | Add `◇ decks` recipes to `justfile`; wire into `essentials` + `help` | ✅ done (m0045, m0063) |
| 7 | Test each recipe: `just deck-pdf`, `just deck-client` | ⏳ PENDING runtime test |
| 8 | Update `CONVENTIONS.md` §1 + `README.md` Structure/Quick Start | ✅ done (m0041-m0042) |
| 9 | Add `**/out/*.pdf` to `.gitignore`; write `docs/decks/README.md` | ✅ done (m0041-m0063) |
| 10 | Full verification pass | ⏳ PENDING |

## Verification — acceptance criteria

```bash
# 1. Theme renders without error
presenterm -t catppuccin-mocha docs/decks/onboarding/onboarding.md --validate-overflows -x  # ✓ m0060

# 2. Client template renders without error
presenterm -t catppuccin-mocha docs/decks/client/template.md --validate-overflows  # ✓ m0061

# 3. Justfile recipes wire up
just --list                    # should show 7 groups including [decks]
just deck                      # interactive, 4 slides
just deck-pdf                  # produces 3a.Onbo/onboarding.pdf, non-empty
just deck-client CLIENT="Acme" DATE="2026-07-02" SUMMARY="MVP complete" \
                  NEXT="- Reports v1 by week 4" CONTACT="Juanjo"
                               # produces 3b.Clnt/out/orbe-overview.pdf

# 4. Conventions held
rg "docs/decks" CONVENTIONS.md      # documented ✓
rg "C12" SPEC.md                         # constraint registered ✓
test ! -d ~/.config/presenterm           # no global pollution ✓

# 5. Old paths swept
rg "1\.Db|2\.Cf|1a\.Sch|2\.Sd|3\.Decks|1\.Onboarding|2\.Client" --hidden -g '!.git/' -g '!.codegraph/'
# expected: 0 hits

# 6. Slide count cap ≤ 4
rg -c "end_slide" docs/decks/onboarding/onboarding.md   # expected: 1

# 7. Compile sanity (no path changes since m0036)
cd crates/orbe-api && cargo check                     # ✓ m0036

# 8. No obsolete files
rg "orbe\.yaml|presenterm\.yaml" docs/decks/       # expected: 0
```

**Done =** all above green + a new dev can run `just deck` and answer "what is Orbe, what stack, how do I start, what does it feel like" without opening any other file.

## Risks + assumptions

| # | Risk | Severity | Mitigation / verification |
|---|---|---|---|
| R1 | ~~presenterm theme schema differs across versions; `orbe.yaml` keys rejected on 0.16.1~~ | **resolved** | Custom theme abandoned; `catppuccin-mocha` is built-in. |
| R2 | `+exec_replace` on slide 3 runs real `cargo`/`just` — slow first run, may fail in sandboxed CI | medium | `deck-pdf` sets `DECK_STATIC=1` to skip the live snippet block; interactive `deck` runs live. |
| R3 | PDF export via typst requires fonts installed system-wide | low | catppuccin-mocha uses bundled terminal fonts; no DM Sans/JetBrains Mono dep for the deck. PDF is on-brand-enough via the theme's mauve accent. |
| R4 | `envsubst` not on every dev machine | low | Standard on util-linux. |
| R5 | `R/3.Dec/` is a new area precedent (3rd area after `1.Dat`, `2.Cfg`) | low | Decimal scheme extends naturally; documented in CONVENTIONS §1. |
| R6 | Client PDFs accidentally leak internal details (paths, dev names) via `include:` | **high** | Client template uses only `${PLACEHOLDER}` + hand-curated sections. **No `+exec_replace` in client path.** No `include:` of repo files. Reviewed per-export. |
| R7 | presenterm `--export-pdf` output is large (typst embeds fonts) | low | Acceptable for client deliverable. Measure in step 7. |

### Resolved concerns (originally flagged for verification)
- ~~Exact `footer.style: template` syntax for 0.16.1~~ → moot; we use catppuccin-mocha defaults.
- ~~Whether `code.theme_name: Carbonfox` ships built-in~~ → moot; we use catppuccin defaults.
- `--validate-overflows` exit code semantics → verified m0060: exit 0 on clean, no overflow errors on either deck.

## Out of scope (explicit non-goals)

- ❌ A deck-per-feature (one deck per module) — premature; revisit when >3 modules exist.
- ❌ `deck-progress` recipe — single overview template covers client scope now.
- ❌ Auto-rebuild PDFs in `just dev` watch loop — decks are async artifacts.
- ❌ Hosting HTML exports on the Axum server — separate concern (static `assets/` later).
- ❌ Multi-language decks — Spanish version deferred until first Colombian SME user trial.
- ❌ Custom theme / config file — built-in theme is enough.

---

## Decisions locked (from user)

- ✅ `docs/decks/onboarding/` + `3b.Clnt/` placement (decimal-compliant).
- ✅ 4-slide cap: What / Stack / Start / Mood.
- ✅ Client scope: single overview template (`deck-client`).
- ✅ Mood slide = explicit UX prose (cognitive-load principle).
- ✅ Convention upgrade v0.1 applied + SPEC §C4 + §C12 + §G cognitive-load rule.
- ✅ Built-in theme `catppuccin-mocha` (user m0058); no custom theme file.

## Implementation status

- ✅ Directory tree + onboarding.md + template.md + README.md
- ✅ justfile `◇ decks` group (4 recipes)
- ✅ CONVENTIONS.md §1 + README.md Structure/Quick Start
- ✅ SPEC.md §C (C4 updated to v0.1, C12 added) + §T T5/T6 paths
- ✅ .gitignore updated
- ⏳ Runtime verification: `just deck-pdf` + `just deck-client` produce non-empty PDFs (pending)
- ⏳ Old-path sweep final pass