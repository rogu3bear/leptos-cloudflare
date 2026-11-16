---
artifact: acceptance-criteria
version: "1.0"
created: 2026-11-15
status: locally-qualified
---

# Acceptance Criteria: agent-ready GitHub template

## Story Context

An application owner gives this template to a coding agent. The agent can find
its operating contract, run locally, change the application and select the proper
provider-control lane without relying on the maintainer's machine. cfctl unlocks
governed operations where installed and authorized; independent adopters retain a
standalone path. The owner reviews this candidate before further publication.

This slice changes onboarding, documentation organization and its affected link
checks only. Existing adoption/runtime acceptance remains in
[acceptance-criteria.md](acceptance-criteria.md). It adds no application feature,
provider operation, new controller, hosted CI or release-guard exception.

## Happy Path

### AC-T1: Discover control from a public copy

**Given** a public source copy without the maintainer's ignored files

**When** an agent starts with the root instructions and README

**Then** it can identify source owners, local commands, required verification and
provider/publication boundaries without a missing private prerequisite.

### AC-T2: Work without Cloudflare control

**Given** build prerequisites but no Cloudflare credential or cfctl

**When** an agent follows local setup and identity adoption

**Then** it can run the local application and required verification without
provisioning provider resources or installing a controller.

### AC-T3: Discover governed capabilities

**Given** cfctl is installed and the owner selected a governed account

**When** an agent follows the optional control guide

**Then** it discovers the installed capabilities and binds account, profile and
target before planning, exact approval, execution and independent readback.

## Edge Cases

### AC-T4: Keep independent operation available

**Given** an independently operated account without cfctl

**When** an agent prepares a provider change

**Then** it can find the standalone credential and target-specific deployment
procedure without a maintainer-home script or controller prerequisite.

## Error States

### AC-T5: Recover without bypassing controls

**Given** a missing build tool, failed local check or unavailable governed capability

**When** an agent reaches that failure

**Then** it reports the blocker and supported recovery while preserving work,
without claiming readiness, weakening checks or switching control planes.

## Non-Functional Criteria

### AC-T6: Keep the root navigable without losing useful work

**Given** operational guides and internal planning records

**When** an adopter opens the repository root and follows documentation links

**Then** the agent contract and Quick Start are prominent, public guides have valid
links, and personal/internal planning records are excluded from the public copy
while their unique source material is preserved privately.

### AC-T7: Review before publication

**Given** the changed candidate and its verification evidence

**When** preparation finishes

**Then** the owner receives a local reviewable candidate and the proposed change
summary before any new push, merge, template-setting change or live deployment.

## Notes

- “Home directory” is interpreted as this repository's root. Personal home files,
  credentials, caches and unrelated work are outside this slice.
- GitHub already reports `is_template: true`; no settings change is needed.
- Local fixtures prove onboarding and runtime contracts, not independent customer
  adoption, current account permissions or live provider behavior.
- The user's review is the next publication boundary; prior PR21 approval does
  not authorize publishing this new candidate.
