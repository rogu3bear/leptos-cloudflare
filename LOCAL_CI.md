# Local CI

Remote CI is intentionally absent from this repository. Do not add GitHub
Actions or other hosted workflow triggers without an explicit operator decision.

Authoritative local gate:

Before the full gate, set `LEPTOS_RELEASE_BASELINE` to the recorded outgoing
bundle ID, or `first-release` for a new application’s initial release.
See [release switches](RELEASE.md#hydration-assets-during-release-switches).


```bash
./scripts/verify.sh
```

For narrower iteration, use the repo playbook and then run `./scripts/verify.sh`
before release or review claims.
