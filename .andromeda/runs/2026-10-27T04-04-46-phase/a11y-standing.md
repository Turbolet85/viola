# a11y · standing conventions

- S1 · each new test runs inside the existing `nextest-integration` / `nextest-e2e` suites (CI: the per-OS `test` job's single `coverage` suite on all three OS legs), adds no `suite` value and no separate driver, and the `gate --require coverage,doctest` allows no skip · §1 A11y Scope Summary (CI integration) and §9 CI Integration of the body
- S2 · each new test is zero-flakiness: `retries: 0`, no `#[ignore]`, a flake keeps the chunk red until its cause is fixed · §10 SLO Invariants & A11y Budgets of the body
- S3 · each new or changed fixed CLI string (a refusal reason, detail or hint line) is pinned by trycmd or a literal, so a copy change fails CI · .andromeda/a11y-plan-detail/cognitive-accessibility.md
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:89) cognitive-plain-language
- S4 · each new human-mode CLI refusal ends with a `hint:` line (only `unknown`, exit 14, has none), carries a typed exit code, and its `--json` form is one document with no hint · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S5 · each new log or diagnostic line for a wrapped run goes to `diagnostics/`, never the terminal · .andromeda/a11y-plan-detail/a11y-scope-summary.md
