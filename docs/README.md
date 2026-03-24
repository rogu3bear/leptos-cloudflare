# Guides

Documentation for building full-stack Rust applications on Cloudflare with Leptos.

## Start Here

- **[How Leptos Works](how-leptos-works.md)** -- The compilation model, WASM vs SSR, hydration, server functions, reactivity. Read this first if you're new to Leptos or full-stack WASM.

## Deployment Models

Two paths depending on your needs:

- **[Edge Deployment](edge-deployment.md)** -- Everything on Cloudflare's edge. Workers for compute, D1 for SQL, R2 for files, KV for cache, Queues for background work, Durable Objects for coordination. No servers to manage. This is the default and recommended starting point.

- **[Hybrid Deployment](hybrid-deployment.md)** -- Edge Workers + Containers for heavy workloads (native binaries, long-running processes, ML) and/or Tunnels for connecting private infrastructure. For when you outgrow the Worker sandbox.

## Building

- **[Building Features](building-features.md)** -- Practical guide to extending this starter. Adding routes, server functions, D1 tables, components, and styles. Covers error handling, secrets, and verification.

## For AI Agents

- **[Agent Playbook](agent-playbook.md)** -- Structured instruction set for AI coding agents (Claude Code, Codex, etc.) to bootstrap and develop on this project. Exact commands, expected outputs, file ownership map, verification protocol, and troubleshooting.
