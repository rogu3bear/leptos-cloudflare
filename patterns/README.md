# Patterns

This directory is the **second layer** of the leptos-cf project.

## Philosophy

The core template (`/`) is intentionally minimal. It demonstrates the essential contracts for a production-grade Leptos application on Cloudflare Workers:
- Build & deployment pipeline
- Local-first verification
- Edge SSR + hydration behavior
- Basic server functions and reactivity

Real applications need more. This `patterns/` layer exists to capture battle-tested, well-documented solutions to common problems **without polluting the core starter**.

## What is a Pattern?

A good pattern here has these qualities:

- **Self-contained** — Can be understood and adapted in isolation.
- **Production-leaning** — Shows real considerations (error handling, loading states, edge constraints, security, performance).
- **Educational** — Heavy on "why" and "how this interacts with the Cloudflare + Leptos model".
- **Composable** — Designed to be combined with other patterns.
- **Minimal surface** — Avoids unnecessary dependencies or complexity.
- **Verified** — Follows the same verification standards as the core (or clearly documents exceptions).

## Directory Structure

```
patterns/
├── README.md                 # This file
├── dynamic-entity-detail/    # Example pattern
│   ├── README.md
│   └── ...
└── shared-layout/            # Another example
    ├── README.md
    └── ...
```

Each pattern directory should contain:
- `README.md` — The primary documentation (problem, solution, trade-offs, integration notes, Cloudflare-specific gotchas).
- Minimal example code or clear references to where the pattern lives in the core (when appropriate).
- Any supporting files needed to make the example runnable in context.

## Contribution Guidelines

When adding a new pattern:

1. **Start with the problem**, not the solution.
2. Explain the Leptos + Cloudflare specific constraints and opportunities.
3. Show the minimal viable implementation + the "production hardened" version.
4. Document integration points with other patterns.
5. Update this README if you establish a new convention.

We prefer **high-quality, deeply documented patterns** over quantity.

## Relationship to the Core Template

- Patterns may reference or extend ideas from the core.
- The core should remain usable without adopting any patterns.
- Over time, exceptionally stable and valuable patterns may influence the core, but this should be rare and deliberate.

## Current Patterns

- [Dynamic Entity Detail](./dynamic-entity-detail/) — Typed server functions, ParamSegment routing, fine-grained reactivity, optimistic updates, and independent loading sections.
- [Shared Layout Composition](./shared-layout/) — Practical persistent UI layout that works cleanly with SSR/hydration.

More patterns (auth, realtime via Durable Objects, R2 uploads, background work, etc.) are planned.

## Long-Term Vision

This layer is the primary place where the template will grow its real-world value while protecting the minimal, trustworthy nature of the core starter. See `STRATEGY.md` in the project root for broader context.
