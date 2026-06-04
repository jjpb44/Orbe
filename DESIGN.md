# Orbe Design System

## 1. Philosophy

Three pillars:

- **Immersive** — full-screen moments, no chrome waste, content IS the interface
- **Fast** — instant feedback, optimistic UI, skeleton loading, no spinners
- **Magical** — subtle motion, micro-interactions that feel alive, transitions that surprise

## 2. Palette

| Token | Hex | Usage |
|---|---|---|
| Purple | `#6B21A8` | primary, CTAs, brand |
| Purple Light | `#9333EA` | hover, active states |
| Purple Dim | `#3B0764` | backgrounds, depth |
| Teal | `#2DD4BF` | success, accents, highlights |
| Teal Dim | `#0F766E` | secondary actions |
| Black | `#0A0A0A` | base background |
| Surface | `#111111` | cards, panels |
| Surface Alt | `#1A1A1A` | elevated surfaces |
| Text | `#F5F5F5` | primary text |
| Text Muted | `#9CA3AF` | secondary text |

Dark-first. No light theme for MVP.

## 3. Typography

- **Display**: DM Sans (geometric, modern)
- **Body**: DM Sans
- **Mono accent**: JetBrains Mono (code, numbers, technical data)
- Scale: fluid type scale, bold weights for hierarchy
- Numbers: tabular figures in JetBrains Mono for stock quantities, prices

## 4. Spacing & Layout

- 4px base grid
- Spacing scale: 4, 8, 12, 16, 24, 32, 48, 64
- Cards: 16px padding, 12px border-radius
- Mobile-first, max-width 480px for content
- Bottom navigation (thumb zone)

## 5. Motion

| Duration | Use |
|---|---|
| 150ms | micro-interactions |
| 250ms | standard transitions |
| 400ms | page transitions |

Easing:
- Standard: `cubic-bezier(0.4, 0, 0.2, 1)`
- Decelerate: `cubic-bezier(0, 0, 0.2, 1)`

Patterns:
- Stock badge: pulse on change (teal glow)
- Card entry: stagger fade-up (50ms delay between items)
- Modal: scale from 0.95 + fade (250ms)
- Page transitions: slide + fade (400ms)
- Button press: scale(0.97) + haptic feedback on mobile

## 6. Component Tokens (UnoCSS shortcuts)

| Token | Description |
|---|---|
| `o-btn` | purple bg, white text, rounded-lg, px-4 py-2, hover:brightness-110 |
| `o-btn-ghost` | transparent, purple text, border purple |
| `o-card` | surface bg, rounded-xl, p-4, shadow-lg |
| `o-input` | surface-alt bg, rounded-lg, px-3 py-2, focus:ring-2 purple |
| `o-badge` | rounded-full, px-2 py-0.5, text-xs, font-mono |
| `o-badge-ok` | teal bg, black text |
| `o-badge-low` | red bg, white text (stock < 5) |
| `o-table` | full-width, surface-alt header, alternating rows |

## 7. Iconography

- Lucide icons (consistent stroke width, 24px default)
- Bold, recognizable at small sizes
- No filled variants — outline only for consistency

## 8. Patterns

- **Empty states**: illustration + CTA, never blank screens
- **Loading**: skeleton screens, never spinners
- **Errors**: inline, contextual, with retry action
- **Success**: subtle teal flash + checkmark, auto-dismiss toast
- **Lists**: virtual scroll for 100+ items, pull-to-refresh on mobile
- **Forms**: single-column, labels above, validation on blur

## 9. Accessibility

- WCAG 2.1 AA minimum
- Touch targets ≥ 44px
- Color contrast ≥ 4.5:1 for text
- Focus rings visible (purple, 2px)
- Reduced motion: respect `prefers-reduced-motion`
