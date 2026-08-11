# Improve Website Plan

## Context

The existing template UI has observable usability and information-architecture defects, but the future public site has no live conversion baseline. Therefore source-derived UX repairs may proceed inside the create-website cut, while the conversion gate remains `awaiting-evidence`. The one action per page is documented in `WEBSITE.md`.

## Phase Status

| Phase | Skill | Status | Artifact | Date |
|---|---|---|---|---|
| 1 | cro-methodology | awaiting-evidence | `METRICS.md`, `WEBSITE.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 2 | ux-heuristics | in-progress | `DESIGN.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 3 | refactoring-ui | in-progress | `DESIGN.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 4 | web-typography | in-progress | `DESIGN.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 5 | storybrand-messaging | done | `POSITIONING.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 6 | high-perf-browser | pending | `METRICS.md`, `WEBSITE.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 7 | made-to-stick | done | `POSITIONING.md`, `EXPERIMENTS.md` | 2026-08-05 |
| 8 | design-everyday-things | in-progress | `DESIGN.md`, `EXPERIMENTS.md` | 2026-08-05 |

## Key Decisions

| Date | Phase | Decision | Rationale |
|---|---|---|---|
| 2026-08-05 | 1 | Hold conversion optimization at the evidence gate. | No analytics, recordings, survey, or traffic baseline exists. |
| 2026-08-05 | 2 | Repair nested landmarks, misleading labels, and route hierarchy now. | These are source-observed usability defects, not conversion guesses. |
| 2026-08-05 | 3 | Introduce shared tokens and parent-owned spacing. | Current page-local rules visibly drift. |

## Next Actions

- [ ] Finish severity 3–4 source-observed UX repairs (Codex, 2026-08-05).
- [ ] Record browser performance and accessibility evidence after implementation (Codex, 2026-08-05).
- [ ] Establish an ethical, privacy-respecting adoption baseline after launch (owner/operator, after launch).
