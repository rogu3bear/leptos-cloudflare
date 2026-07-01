# Local CI

Remote CI is intentionally absent from this repository. Do not add GitHub
Actions or other hosted workflow triggers without an explicit operator decision.

Authoritative local gate:

```bash
./scripts/verify.sh
```

For narrower iteration, use the repo playbook and then run `./scripts/verify.sh`
before release or review claims.
