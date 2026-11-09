# Plan — Bilingual EN/ES Onboarding for Orbe

## Inspect complete (Q/W/Z structure)

```
docs/decks/
├── 3a.Onbo/
│   ├── onboarding.md       (2263 B, EN, 4 slides, modified TODAY)
│   └── onboarding.pdf      (14696 B, from prior session)
├── 3b.Clnt/
│   ├── out/
│   └── template.md         (733 B, EN, 5 slides, envsubst placeholders)
└── README.md               (1715 B, EN, modified TODAY)
```

**Current content is EN** (the existing `onboarding.md` is English — title "Orbe", tagline, prose, references). The user's redirect "EN and ES i18n is important" means: ADD ES as a parallel file, don't replace EN.

**No CI, no LLM step, no diff threshold, no mermaid in Orbe** — justfile recipes are the only regen surface. EdTech's CI pipeline doesn't translate.

## Plan: 3 NEW + 4 EDIT (no CI, no LLM, no threshold)

| # | Op | Target | What |
|---|---|---|---|
| 1 | RENAME | `3a.Onbo/onboarding.md` → `onboarding.en.md` | preserve current EN as `.en` |
| 2 | NEW | `3a.Onbo/onboarding.es.md` | Spanish mirror, 4 slides, mood-as-prose (es-CO tone) |
| 3 | NEW | `scripts/check-bilingual-parity.sh` | structural parity: slide count, mermaid refs, doc-path refs |
| 4 | EDIT | `justfile` `◇ decks` group | add `deck-en`, `deck-es`, `deck-parity`; default `deck` → `.en.md`; update `deck-pdf` to either file |
| 5 | EDIT | `SPEC.md` §C12 | append bilingual + parity requirement |
| 6 | EDIT | root `README.md` | update R/3.Dec/ Quick Start mention |
| 7 | EDIT | `docs/decks/README.md` | bilingual section + parity mention |

## on a slide per language

EN already exists (verbatim from `onboarding.md`):

1. **What** — `# Orbe` centered, "CRM for Colombian SMEs. Inventory-first."
2. **Stack & shape** — layer table + ASCII clean-arch (UI→Wire→Flow→Core/Gate→SurrealDB)
3. **Start in 60 seconds** — `+exec_replace: just dev`, "Read next: SPEC.md → CONVENTIONS.md"
4. **Mood** — Fast/Bold/Tactile, Nequi/Nu/WhatsApp references

ES mirror (same structure, Colombian Spanish tone):
1. **Qué es** — `# Orbe` centrado, "CRM para pymes colombianas. Inventario primero."
2. **Stack y forma** — tabla de capas + ASCII clean-arch (Spanish column headers)
3. **Empezar en 60 segundos** — `+exec_replace: just dev`, "Lee después: SPEC.md → CONVENTIONS.md"
4. **Mood** — Rápido/Osado/Táctil, referencias Nequi/Nu/WhatsApp

Hard cap: 4 slides (matches EN, matches prior cognitive-load decision).

## Parity check (script spec)

`scripts/check-bilingual-parity.sh <en.md> <es.md>`:
- Compares: `<!-- end_slide -->` count, `diagrams/*.png` refs, `docs/*.md` refs
- Exit 1 on any mismatch, 0 on parity
- Pure structural — never compares wording
- `set -euo pipefail`, ~25 lines bash

## Justfile recipe plan

```just
[group('decks')]
deck:                      # alias for deck-en (international default)
    @just deck-en

[doc('present onboarding (EN, default)')]
[group('decks')]
deck-en:
    @script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.en.md -X" /dev/null

[doc('present onboarding (ES, Colombian market)')]
[group('decks')]
deck-es:
    @script -qc "stty rows 50 cols 200; presenterm -t catppuccin-mocha {{_decks}}/3a.Onbo/onboarding.es.md -X" /dev/null

[doc('check structural parity between EN/ES decks')]
[group('decks')]
deck-parity:
    @bash {{_decks}}/scripts/check-bilingual-parity.sh {{_decks}}/3a.Onbo/onboarding.en.md {{_decks}}/3a.Onbo/onboarding.es.md
```

`deck-pdf` updated to render BOTH languages → `onboarding.en.pdf` + `onboarding.es.pdf` (or accept `LANG=en|es` arg). Default = both.

## 3 confirmations needed before implement

1. **Default language** for `just deck` (no suffix) → **EN** (international devs, default) or **ES** (Colombian market primary)?
2. **`template.md`** → **keep single EN** (placeholders are language-neutral; client fills with their language), or split into `template.{en,es}.md` too?
3. **Mermaid diagrams** → **skip** (existing ASCII clean-arch is terse + cognitive-load-friendly), or **add 1 mermaid** (richer arch slide)?

**Out of scope (locked from EdTech translate):** no CI, no LLM step, no diff threshold, no EdTech touches.

→ "go" with EN default + single template + no mermaid → I execute.
→ different choice on any of the 3 → tell me which.
