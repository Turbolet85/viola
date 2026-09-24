
## 2026-09-24-fake-agent-and-test-data-fixtures — test homes, test env vars and test-side paths registered
**Section:** §Occupied Resources (Environment variables · Filesystem · Repository) · §Infrastructure Patterns directory tree
**Change:**
- Env vars: `AGENT_RUN_KEEP_FAILED` registered (read only by the root test chain). `AGENT_RUN_KEEP_HOMES` is now read by `viola-harness` and the root test chain. None is read by `viola`.
- e2e-home: `viola-session-*/home` (harness) and `viola-test-*/home` (root rstest) both named.
- Repository gains `fixtures/fake-scripts/`, `schemas/fake-script.v1.json` and `crates/viola-core/proptest-regressions/`.
- Filesystem gains the test-home-only `fake/<name>.control` / `fake/<name>.receipt.ndjson`.
- The tree gains `tests/support/`, `schemas/`, `fixtures/fake-scripts/` and viola-core `proptest-regressions/`.

**Why:** report Spec claims disproved 1 (architecture.md:374 claimed `viola-session-*` held every test home; root tests use `viola-test-*`) and 2 (:359 said only `viola-harness` reads `AGENT_RUN_KEEP_HOMES`); report Files / Env vars / Symbols (Wrapper::boot paths); plan expected amendment.
- Rejected: `tests/support/` as a Repository entry (registry over-reach; the tree carries it), and a §Stack Testing row (the invariant holds, because [Deferred] :106 hands test libraries to test-plan, which pins them).
- Sweep `every harness and test home`, `read by \`viola-harness\` and never`, `e2e-home`, `AGENT_RUN_KEEP` over all seven masters: 2 arch sites amended (:359, :374). The CI-jobs line :448 (`AGENT_RUN_KEEP_HOMES=1` set by ci.yml) needs no change. The test-plan and obs-plan `e2e-home` sites name the directory, not a prefix claim, so they need no change. test-plan :596 (harness cleanup removes its own `viola-session-*` parent) is accurate as written, and `viola-session-row` hits in a11y/test-plan are UI element names.
