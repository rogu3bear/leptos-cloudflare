# Long-Term Strategy for leptos-cf

This document captures the intended evolution of the template so that future contributors (human or agent) make decisions aligned with the project's identity.

## Core Identity

leptos-cf is a **public, agent-first reference implementation** for production-grade Leptos applications on Cloudflare Workers.

Its highest value is:
- Reproducible, local-first verification
- Explicit, correct patterns for the edge (Workers + Assets + D1)
- Strong teaching of Leptos' unique strengths in a real deployment context
- Being a reliable foundation that agents can safely extend

## Strategic Principles

1. **Two-Layer Architecture (Core + Patterns)**
   - The main template (what `init.sh` produces) must stay minimal, stable, and extremely well-verified.
   - Real-world complexity lives in a separate "Patterns" surface (`/patterns/` or companion materials).
   - This prevents the core from rotting while still providing production-grade guidance.

2. **Local > Git-dependent**
   - All important verification must be runnable locally via clear scripts (`scripts/verify.sh` is the model).
   - CI exists only as a thin, reproducible executor of the local contracts.

3. **Agent-Native by Default**
   - Documentation, contracts, and verification should be written so that capable AI agents can bootstrap, extend, and maintain projects from this template with high reliability.
   - Explicit boundaries and "why" comments are more important than clever code.

4. **Own the Upgrade Story**
   - The biggest long-term tax in this ecosystem is major version upgrades (Leptos, worker-rs, cargo-leptos, wasm-bindgen).
   - Future work should prioritize clear migration paths and compatibility notes.

5. **Tasteful Minimalism**
   - The core starter demonstrates *how* to use Leptos well on the edge.
   - It does not try to be a complete application or include every feature.

## Current Focus Areas (as of this PR)

- Local verification as single source of truth
- Better demonstration of Leptos router (layouts, dynamic routes)
- Explicit examples of fine-grained reactivity (`Memo`) and server functions
- Suspense / progressive loading patterns suitable for edge SSR
- Educational comments that explain *why* certain patterns exist in this stack

The `/patterns/` directory has been created as the first concrete step of the two-layer architecture. See `patterns/README.md`.

## Future Horizons

- Formal Patterns Library (auth, realtime via Durable Objects, R2 uploads, background work, etc.)
- Structured contracts / PRPs that make agent extension even more reliable
- Living upgrade guides and compatibility matrix
- Optional "production seed" that composes core + key patterns

## Decision Filter

When considering changes, ask:
- Does this strengthen the core contracts or belong in the future Patterns layer?
- Can a capable agent discover and correctly apply this change?
- Does this make the template more or less trustworthy as a long-term reference?

This document should be updated as the strategy evolves.
