# Reference application design

The starter uses an editorial field-guide layout. Replace its branding when
adopting it; the design is application content, not a runtime requirement.

Source of truth: `style/main.css`, `src/components/app_layout.rs`, shared UI
components and the route tree in `src/app.rs`.

- Use system serif headlines, system sans body text and monospace technical labels.
- Keep warm paper, charcoal text, visible rules and restrained coral accents.
  Runtime diagrams use distinct browser, Worker and durable-state colors.
- Keep text readable near a 65-character measure. Headings scale with `clamp()`.
- Preserve responsive navigation, one main landmark, skip link and visible focus.
- Use native controls and show pending, success, failure and retry states in text.
- Honor reduced motion; never make content or navigation depend on animation.
- Label source, local verification and provider evidence distinctly. Examples
  must not imply a deployed service, customer proof or a working support channel.

For UI changes, inspect wide and narrow layouts, keyboard navigation, hydration,
console errors and affected persistence/recovery workflows before claiming them
verified. Replacing the visual identity must preserve CSP and runtime boundaries.
