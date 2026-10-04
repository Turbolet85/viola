# layouts extract

## No domain coverage
The chunk is test-and-gate tooling only (mutation scoring, pre-push and gate-tool migration, the harness `git diff` prefix pin). It creates or changes no web-spa or cli surface element, output line, focus order, breakpoint, modal or empty state. Its nearest touchpoint, the `viola run` passthrough per layout-templates §Surface: cli §Output structure — `viola run`, is only exercised by tests here and does not change.
