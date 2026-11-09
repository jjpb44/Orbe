// Orbe — client project overview (PDF).
//
//   just deck-client client="Acme SAS" date="2026-11-09" summary="…" next="…" contact="…"
//
// Deterministic: no evaluation, no network, no snippets. Every value arrives as
// a --input; the fallbacks below exist so `typst compile` alone never fails.
#let client  = sys.inputs.at("client",  default: "Client")
#let date    = sys.inputs.at("date",    default: "today")
#let summary = sys.inputs.at("summary", default: "Update in progress.")
#let next    = sys.inputs.at("next",    default: "TBD")
#let contact = sys.inputs.at("contact", default: "Orbe team")

// ── palette (DESIGN.md tokens) ───────────────────────────────────────────────
#let purple = rgb("#9333EA")
#let ink    = rgb("#0A0A0A")
#let muted  = rgb("#6B7280")
#let hair   = rgb("#E5E7EB")

#set document(title: "Orbe — project overview for " + client, author: "Orbe")
#set page(
  paper: "a4",
  margin: (x: 2.2cm, y: 2.4cm),
  footer: context [
    #set text(size: 8pt, fill: muted)
    Orbe · #date · #h(1fr) #counter(page).display()
  ],
)
#set text(font: ("DejaVu Sans", "Liberation Sans"), size: 10.5pt, fill: ink)
#set par(leading: 0.68em, justify: true)
#set heading(numbering: none)

#let rule = line(length: 100%, stroke: 0.6pt + hair)

// ── cover ────────────────────────────────────────────────────────────────────
#v(2.5cm)
#text(size: 9pt, fill: purple, weight: "bold")[PROJECT OVERVIEW]
#v(0.3cm)
#text(size: 34pt, weight: "bold")[Orbe]
#v(0.2cm)
#text(size: 13pt, fill: muted)[Inventory CRM for #client]
#v(0.6cm)
#rule
#v(0.25cm)
#text(size: 9pt, fill: muted)[Generated #date]

#pagebreak()

// ── status ───────────────────────────────────────────────────────────────────
#heading(level: 2)[Status]
#rule
#v(0.2cm)
#summary

// ── what's working ───────────────────────────────────────────────────────────
#v(1.2cm)
#heading(level: 2)[What's working]
#rule
#v(0.2cm)
#list(
  [Inventory schema, CRUD API and stock-movement ledger],
  [Single-binary Rust stack — Dioxus UI, Axum server, SurrealDB],
  [Plain-CSS design system with a dark-first palette],
  [Local CI gate: formatting, lints, layer boundaries, tests and a REST/RPC smoke suite],
  [Bilingual onboarding deck and a one-command client PDF],
)

// ── next ─────────────────────────────────────────────────────────────────────
#v(1.2cm)
#heading(level: 2)[Next milestones]
#rule
#v(0.2cm)
#next

// ── contact ──────────────────────────────────────────────────────────────────
#v(1.2cm)
#heading(level: 2)[Contact]
#rule
#v(0.2cm)
#contact
