# Decks — Orbe presentation source

Two decks, one built-in theme, zero config files. Presenterm 0.16.1, no JS, no Node.

## Theme — `catppuccin-mocha` (built-in)

Dark-first, mauve/purple accent, deep dark background — closest match to Orbe's
purple `#6B21A8` on black `#0A0A0A`. Selected via `-t catppuccin-mocha` on every
recipe. No custom theme file; less surface area.

## Onboarding — `just deck`

`3a.Onbo/onboarding.md` — 4 slides for new devs (What / Stack / Start / Mood).
Slide 3 runs `+exec_replace` to show live `just dev` output; for the static
PDF (`just deck-pdf`), set `DECK_STATIC=1` so the snippet block is skipped.

## Client PDF — `just deck-client`

`3b.Clnt/template.md` — placeholder-driven overview. Never executes snippets.
Substitute with `envsubst`:

```bash
just deck-client \
  CLIENT="Acme SAS" \
  DATE="2026-07-02" \
  SUMMARY="Inventory MVP shipped. CRUD live. Smoke tests passing." \
  NEXT="- Reports v1 (CSV export) by week 4
- Multi-user auth by week 8" \
  CONTACT="Juanjo · hello@orbe.dev"
```

PDF writes to `3b.Clnt/out/orbe-overview.pdf` (gitignored).

## Adding a deck

1. Pick a name under `3.Dec/` (Area=3 letter, Category=4 letter, optional
   ID per CONVENTIONS §1 / JuanJo.Decimal v0.1).
2. Write a `.md` with `<!-- end_slide -->` between sections.
3. Add a recipe to the `◇ decks` group in `justfile` (mirror the existing
   `deck` / `deck-pdf` patterns — keep the banner + `presenterm -t catppuccin-mocha`).

## Validate before commit

```bash
presenterm -t catppuccin-mocha \
  Backend/W/3.Dec/3a.Onbo/onboarding.md --validate-overflows -x
```

Exit non-zero = overflow detected. Fix and retry. (`-x` enables `+exec` blocks
needed by slide 3.)