# Design System

## Design Direction

The signature moment is a large editorial promise paired with a source-derived request-path plate. The selected authority is `HORIZON.md` Direction A and `docs/design/edge-field-guide-concept.png`: warm field-manual paper, charcoal ink, coral request routes, cyan browser edges, amber Worker edges, green durable state, compact monospaced annotations, and honest proof labels. Generated copy and status marks are not factual authority.

## Typography

Use a local/system serif display stack for editorial headlines, a modern system sans for body copy, and a system monospace stack for evidence labels and commands. No font network dependency is added. Fluid headings use `clamp()`, body remains at least 1rem with 1.6 line height, and reading measures remain near 65 characters.

## Tokens

- Spacing: 4, 8, 12, 16, 24, 32, 48, 64, 96.
- Color: paper, paper-deep, ink, ink-muted, rule, coral, cyan, amber, green; status never relies on color alone.
- Radius: mostly square plates with restrained 2–8px rounding on controls.
- Shadow: one quiet paper-lift shadow; diagrams use rules rather than floating cards.
- Motion: opacity/transform only, short and optional; reduced-motion disables nonessential transitions.

## Components

| Component | Decision | Status |
|---|---|---|
| Site shell | One main landmark; shared header/footer and responsive nav. | In progress |
| Action link/control | Typed size and tone with native semantics. | Planned |
| Evidence tag | Names proof plane in text and color. | Planned |
| Request path | Accessible ordered steps, horizontal wide and vertical narrow. | Planned |
| Plate | Shared section rule, eyebrow, number, and content rhythm. | Planned |
| Lab controls | Keep server-action behavior; migrate visual geometry. | Planned |

## UX Audit Findings

| Issue | Heuristic | Severity (0-4) | Fix | Status |
|---|---|---:|---|---|
| Nested main landmarks | Structure and accessibility | 4 | Shell owns main; pages emit sections/articles. | In progress |
| Contact scope is ambiguous | Match between system and real world | 4 | Explicitly name D1 storage and no delivery promise. | In progress |
| Todo-first identity misrepresents product | Match and information scent | 4 | Move demonstration under Lab. | In progress |
| No visible focus system | Accessibility and control | 3 | Shared `:focus-visible` treatment. | Planned |
| Mobile composer rule targets wrong layout mode | Flexibility and efficiency | 3 | Use a real flex-column narrow rule. | Planned |
| Status claims can look equivalent | Visibility of system status | 3 | Evidence tags with textual provenance. | Planned |

## Microinteraction Inventory

| Interaction | Trigger/Rules/Feedback/Loops | Fix | Status |
|---|---|---|---|
| Navigation | Native route change; active state text plus rule. | Add shared focus and current-route treatment. | Planned |
| Task mutations | Submit; immediate disabled/pending feedback; success or actionable error. | Preserve behavior and normalize controls. | In progress |
| Contact mutation | Validate before dispatch; pending then explicit local-D1 result. | Preserve behavior and clarify scope. | In progress |
| Request map | Optional decorative trace on load/hover only. | Disable under reduced motion; content never depends on it. | Planned |
