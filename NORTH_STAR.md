# North Star

`leptos-cf` is a public starter template for production-grade Leptos applications on Cloudflare Workers.

The template should make the correct path obvious for both humans and coding agents:

- SSR, hydration, server functions, static assets, D1, and WebSockets have explicit ownership boundaries.
- Build tooling is pinned or resolved from checked-in sources, not ambient machine state.
- Security posture is fail-closed by default: no committed secrets, scoped Cloudflare tokens, session-scoped demo data, CSP/anti-framing headers, and bounded request bodies.
- Every release claim is backed by local proof that matches the Cloudflare deployment surface.

If a future change makes agents guess about routing, credentials, build tools, asset serving, or state ownership, the template has drifted.
