# Fan-out results — 2026-09-24-quality-gates

Seven doc-agents (Explore), one batch, 2026-09-24. The returns needed no stripping. Four docs carried proposals and have raw twins (`.raw-fanout-{doc}.md`).

| doc | verdict | proposals |
|---|---|---|
| architecture | proposals | 14: D-arch-resources ×5 (fuzz/ workspace, artifacts outputs, Workspace crates note, [Module Boundaries], tree) · D-arch-decisions ×9 (runtime row, Code quality ×2, Dependency policy, CI/CD row, Setup steps, workflows bullet, Jobs wired today, tree comments) |
| security-plan | proposals | 11: D-security-deps ×10 (fuzz lockfile outside cargo deny: `deny.toml` additions, Pinning, CI integration, Threat Model trust boundary, Threat Model CI/CD, Decisions Log; pinned-action set; Threat Model entry point; toolchain Pinning; toolchain CI integration) · D-security-auth ×1 (secret-scanning-ci-gate: mutants.out CARRY closed) |
| design-system | no drift | `proposals: []`: no UI surface; every tokens flag n/a |
| layout-templates | no drift | `proposals: []`: no product surface; the harness is not a `viola` verb |
| test-plan | proposals | 24: D-tests-obs-harness ×10 (§3 gate Inputs JUnit, §9 Test report junit, §3 gate body, §3 gate CI placement, §3 preamble usage details, §9 Coverage/Quality rows, §10 Mutation union, §3 run --leg, §12 ×2) · D-tests-framework ×11 (§3 regex, §10 Stack regex, §11 CI regex ban, §3 --fuzz-replay, §9 Fuzz row, §3 ci-tool-install, §9 tool install, §9 MSRV row, §9 Mutation row, §9 Matrix builds, §11 CI single-job ban, §9 Test report mutants.out) · D-tests-coverage ×3 (§6 Property suite, §2 Property row, §12) |
| obs-plan | proposals | 3: D-obs-pii ×3, severity escalate (§9 Mutation consumer, §8 item 6 unscanned uploads, §9 Platform nightly) |
| a11y-plan | no drift | `proposals: []`, with an out-of-invariant note: a11y-plan:280/499/604/1115/1153 name `nextest-integration` / `nextest-e2e` as the CLI output-discipline suites, while CI now reports them under `coverage`. The orchestrator raises this at Validate. |
