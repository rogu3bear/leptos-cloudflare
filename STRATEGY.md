# Strategy: a reference implementation that survives adoption

## Diagnosis

The field guide now exists: the route tree exposes Start, Architecture, Patterns,
and bounded D1 labs. The earlier todo-first homepage diagnosis is historical.
The current engineering challenge is making a working reference application
survive a new name, different page content, and a subsequent application change
without losing its runtime and release guarantees. Previous initialization
deleted still-consumed code and verification depended on field-guide identity.
That is an adoption boundary problem, not evidence that more features are needed.

This diagnosis is a testable hypothesis about user value. The repository has no
measured public adoption baseline or demonstrated competitive advantage. If
independent adopters complete these changes easily but stall elsewhere, revise
the diagnosis using their observed failure point.

## Guiding policy

Keep one inspectable Leptos/Workers reference and a small, explicit reusable
runtime contract. Concentrate on a complete clone-to-application-change journey.
Application code owns domain semantics; cfctl owns infrastructure operations in
this operator workspace. Independent public adopters retain a standalone
Wrangler path. The starter must not require the operator's home directory.

The source of potential advantage is that examples, runtime integration, and
local proof can be inspected together. This is a plausible design advantage,
not a claim of market demand or superiority over an unmeasured competitor.

## Coherent actions

1. Keep identity adoption non-provisioning and non-destructive. Preserve the
   functional sample and schema until an application's consumers are replaced.
2. Keep runtime verification independent of field-guide copy while retaining
   explicit reference-site acceptance. Required security checks must fail
   visibly when their prerequisites are missing.
3. Record one renamed application build and a changed page or workflow against
   the same runtime checks. Then test a subsequent change with an independent
   adopter; use that evidence to decide whether any further extraction is needed.

The repository maintainer owns source and local verification; application owners
own their cutovers, and provider operators own credential and release decisions.
No staffing commitment, delivery date, customer result, or provider completion
is implied by this document. Current criteria and direct evidence are in
`docs/acceptance-criteria.md`; review the diagnosis after the first independently
observed adoption, or after any failure exposing a different binding constraint.

## What we will not do during this repair

- Add default auth, billing, email delivery, or a broad pattern marketplace.
- Build a new framework or generator that strips arbitrary application domains.
- Add an alternative account controller or parent-directory rotation dependency.
- Require cfctl for independently operated public clones.
- Rework the site design or add Pages/CSR/Containers without a named need.
- Call source, local proof, publication, provider deployment, or customer success
  interchangeable evidence.

## Alternatives and challenge

A documentation-only field guide would reduce adoption promises but would give
up the reusable starter outcome. A separate generated seed may eventually help,
but would add another artifact to maintain before the variation boundary is
proved. A batteries-included app would concentrate many unrelated requirements
in the core. Prefer the current small contract split while testing it against
an actual application change; switch direction if measured failures reject it.

The primary objection is that these engineering repairs may not improve public
adoption. That remains unresolved until someone outside the implementation
exercise tries the path. Do not turn a passing fixture into demand evidence.
