# layouts extract

## No domain coverage
This chunk only touches test-side tooling: a WSL2 pre-push gate, `scripts/agent-run.{sh,ps1}`, the `viola-e2e` harness and operator docs. It changes no product surface (scope §Boundaries: "No product-crate behaviour changes"). layout-templates.md only covers the product `web-spa` Bay and the product `viola` CLI verbs (§Surface: cli: list/send/wait/verify/run). It has no section on harness or agent-run output. The amendment history file (`layout-templates-amendments.md`) does not exist, so that history is empty.
