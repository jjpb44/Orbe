#!/usr/bin/env bash
# check-bilingual-parity.sh — structural parity check for Orbe EN/ES deck pair.
# Usage: check-bilingual-parity.sh <deck.en.md> <deck.es.md>
# Exits 0 on parity, 1 on mismatch.
# Compares: slide count (end_slide markers), mermaid refs (```mermaid blocks),
# doc-path refs (paths starting with docs/ or Backend/). Structural only,
# not word-for-word.
set -euo pipefail

EN="${1:-Backend/W/3.Dec/3a.Onbo/onboarding.en.md}"
ES="${2:-Backend/W/3.Dec/3a.Onbo/onboarding.es.md}"

[[ -f "$EN" ]] || { echo "❌ missing: $EN"; exit 1; }
[[ -f "$ES" ]] || { echo "❌ missing: $ES"; exit 1; }

# Helpers — count in file $1 the lines matching regex $2.
count_in() { rg -c -- "$2" "$1" 2>/dev/null || echo 0; }

# Slide count — explicit <!-- end_slide --> markers (file-end counts as one).
en_slides=$(count_in "$EN" '<!-- end_slide -->')
es_slides=$(count_in "$ES" '<!-- end_slide -->')

# Mermaid block count — ```mermaid fences (opening + closing pair = 2 markers).
en_mm=$(count_in "$EN" '^```mermaid$')
es_mm=$(count_in "$ES" '^```mermaid$')

# Doc-path refs — paths under docs/ or Backend/. Disallow markdown links with
# non-image hrefs (i.e. no [text](path) without leading !).
en_docs=$(rg -oN '\b(docs/[A-Za-z0-9_./-]+|Backend/[A-Za-z0-9_./-]+)' "$EN" | sort -u | wc -l)
es_docs=$(rg -oN '\b(docs/[A-Za-z0-9_./-]+|Backend/[A-Za-z0-9_./-]+)' "$ES" | sort -u | wc -l)

# Doc-path content parity — same sorted set of paths on both sides.
en_paths=$(rg -oN '\b(docs/[A-Za-z0-9_./-]+|Backend/[A-Za-z0-9_./-]+)' "$EN" | sort -u | sed 's/.*://')
es_paths=$(rg -oN '\b(docs/[A-Za-z0-9_./-]+|Backend/[A-Za-z0-9_./-]+)' "$ES" | sort -u | sed 's/.*://')

printf '  EN slides=%d mermaid=%d docs=%d\n' "$en_slides" "$en_mm" "$en_docs"
printf '  ES slides=%d mermaid=%d docs=%d\n' "$es_slides" "$es_mm" "$es_docs"

fail=0
[[ "$en_slides" == "$es_slides" ]] || { echo "❌ slide count mismatch ($en_slides vs $es_slides)"; fail=1; }
[[ "$en_mm" == "$es_mm" ]]         || { echo "❌ mermaid block count mismatch ($en_mm vs $es_mm)"; fail=1; }
[[ "$en_paths" == "$es_paths" ]]   || { echo "❌ doc-path set differs"; diff <(echo "$en_paths") <(echo "$es_paths") | head -20; fail=1; }

if [[ "$fail" -eq 0 ]]; then
    echo "✅ parity OK (slides=$en_slides, mermaid=$en_mm, docs=$en_docs)"
    exit 0
fi
exit 1
