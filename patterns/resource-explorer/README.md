# Resource Explorer

**Problem**: Agents repeatedly rebuild collection filters, record context, edit
forms, review screens, and failure recovery. This recipe joins those steps in a
working Leptos 0.8 flow backed by the starter's existing session-scoped records.

## Run the workflow

Follow the credential-free local setup in [README](../../README.md), then open
`/patterns/resource-explorer`. Add two records, search for one, open it, edit its
title or status, review the saved and draft values, and save. The same record is
visible in `/lab`; this recipe uses real D1 persistence.

The gallery and this recipe compile the same implementation. There is no copied
example component to keep synchronized:

| Responsibility | Source to reuse or adapt |
| --- | --- |
| Collection, native GET filters, create action | [mod.rs](../../src/components/resource_explorer/mod.rs) |
| Filter/back-link encoding | [query.rs](../../src/components/resource_explorer/query.rs) |
| Record loading and related context | [detail.rs](../../src/components/resource_explorer/detail.rs) |
| Draft → review → acknowledged save | [editor.rs](../../src/components/resource_explorer/editor.rs) |
| Typed commands and shared title validation | [api.rs](../../src/api.rs) |
| Scoped queries and conditional update | [todos.rs](../../src/server/todos.rs) |
| Existing controls and visual tokens | [ui.rs](../../src/components/ui.rs), [main.css](../../style/main.css) |

## State and behavior contract

- `q` and `status` belong to the URL. Native GET submission works before
  hydration, browser history restores the filters, and detail/back/related links
  preserve them. Other query values do not travel with these links.
- D1 owns saved records. Search runs before the 100-row visible limit; counts
  describe the complete matching set. A truncation notice asks the user to narrow
  their search. Search is literal substring matching; SQLite's built-in `lower`
  supplies ASCII case folding, not locale-aware Unicode search.
- Related records are real records from the same session with the same saved
  status, limited to five links. They have their own loading and retry boundary.
  Both resources are constructed once at route entry, outside reactive view
  branches. Async SSR can evaluate a branch repeatedly; constructing a resource
  there creates unused serialized values that can affect later client navigation.
- Drafts belong to the mounted editor. Back to editing preserves a draft;
  discard or leaving the page ends it. Drafts are not stored in URLs or browser
  storage. Creation and editing require hydration; browsing remains useful
  without JavaScript because these routes use `SsrMode::Async` in [app.rs](../../src/app.rs).
- Save sends the reviewed title/status and their previously saved values.
  The server validates the title and compares both mutable fields in one scoped
  `UPDATE … RETURNING`. A conflict preserves the draft and requires explicit
  review of current values. This is field comparison, not a revision history;
  an intervening change back to identical values is not distinguishable.
- Retrying a save whose response was lost recognizes already-applied values.
  Pending controls prevent duplicate browser dispatch. A success is displayed
  only after server acknowledgment. Failed saves keep the draft available.
- Missing and other-session records remain unavailable; no foreign record data
  is returned. This is session isolation, not account authentication or a roles
  system. Adopt authorization rules at your application's server boundary.

No new dependency, table, binding, browser storage, provider operation, or schema
migration is required. The existing lab's retention and session limits apply.

## Adapt to a second domain

Keep the URL/native-form and edit/review interaction structure. Replace the
record fields, status vocabulary, related-record rule, and typed server commands
together. Put validation and authorization in the domain's server owner; map
its conflict and acknowledgment semantics to the editor. Reuse the controls and
tokens before introducing another table or form framework. When a domain needs
pagination or version history, implement those in its data contract rather than
claiming this bounded example already supplies them.

The interaction reference is Cloudscape's [resource management pattern](https://cloudscape.design/patterns/resource-management/view/):
keep collection context while inspecting one record. The review step follows
GOV.UK's [check answers pattern](https://design-system.service.gov.uk/patterns/check-answers/).
The implementation uses native HTML and the starter's own visual system; it
does not copy either library's component code or add their dependencies.

## Verification

Run `./scripts/verify.sh`. The existing local Worker harness applies migrations
to its disposable local D1 store and invokes
[test-resource-explorer.mjs](../../scripts/test-resource-explorer.mjs). It checks
SSR collection/detail/related/empty states, search beyond the visible limit,
literal punctuation, status/count agreement, scoped writes, stale-review
conflicts, response-loss retries, validation, and saved readback.

The Rust query tests cover URL encoding and filter ownership. For rendered
acceptance, exercise creation → filter → detail → edit → review → save → reload;
also check Back to editing, discard, browser Back, network failure/retry,
keyboard focus, and narrow layouts. Include a direct detail-page load followed
by changing its status and returning to a filtered collection: every visible row
must still satisfy the current URL filters. After saving, navigate to a related
record, change the first record's status in another tab, and revisit it: related
records must use the freshly loaded status. A source/reference check alone does
not establish this browser behavior. Local proof does not establish deployment.
