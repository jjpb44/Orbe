# Decks — Orbe presentation source

Two surfaces, two tools, no config files, no JS:

| Surface | Tool | Why |
|---|---|---|
| Onboarding (devs, live in terminal) | presenterm 0.16.1 | interactive, `+exec_replace` shows real output |
| Client overview (PDF, shipped) | typst 0.15 | one-shot compile, no TTY, no shell hacks |

## Onboarding — `just deck`

`onboarding/onboarding.{en,es}.md` — 4 slides (What / Stack / Start / Mood), bilingual.
Slide 3 runs `+exec_replace` to show live `just dev` output; for the static PDF
(`just deck-pdf`) that block is rendered as code.

Theme: `catppuccin-mocha` (presenterm built-in) via `-t`, chosen for the mauve/purple
accent on deep dark — closest built-in to Orbe's `#6B21A8` on `#0A0A0A`. No custom
theme file; less surface area.

**EN/ES parity** is structural, not literal: same slide count, same mermaid count,
same doc-path set. Enforced by `scripts/check-bilingual-parity.sh` (`just deck-parity`),
which also runs inside `just ci`.

## Client PDF — `just deck-client`

`client/overview.typ` — typst template taking five `--input` values:

```bash
just deck-client \
  "Acme SAS" \
  "2026-11-09" \
  "Inventory MVP shipped. CRUD live. Smoke tests passing." \
  "Reports v1 (CSV export) by week 4" \
  "Juanjo · hello@orbe.dev"
```

Positional, in that order. (just treats `name=value` on the CLI as a variable
override, not as a recipe argument — so do not use `CLIENT="…"` here.)

Writes `client/out/orbe-overview.pdf` (gitignored).

Why not presenterm here: exporting a client PDF through presenterm needs
`script -qc "stty …"` — a fake TTY to satisfy a terminal renderer. typst compiles
the same content in one process, with real typography, no shell trickery.
`client/template.md` is kept as the plain-markdown source of the wording.

## Adding a deck

1. Onboarding-style (interactive): drop `onboarding/<name>.md` with
   `<!-- end_slide -->` separators, add a `◇ decks` recipe mirroring `deck-en`.
2. Client-facing: edit `client/overview.typ`. It is a template, not a slide deck.
3. If you add a second bilingual pair, extend `scripts/check-bilingual-parity.sh`
   and wire it into `just ci`.

## Validate before commit

```bash
presenterm -t catppuccin-mocha docs/decks/onboarding/onboarding.en.md --validate-overflows -x
typst compile docs/decks/client/overview.typ --input client=Test --input date=today \
      --input summary='s' --input next='n' --input contact='c' /tmp/orbe-check.pdf
```

Exit non-zero = overflow or compile error. Fix and retry.
