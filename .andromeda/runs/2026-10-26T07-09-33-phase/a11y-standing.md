# a11y · standing conventions

- S1 · every new CLI output-discipline or refusal test runs inside `nextest-integration` / `nextest-e2e` (CI: the per-OS `coverage` suite) under `scripts/agent-run.* run`, on all three OS legs, adding no `suite` value and no separate a11y driver · §1 A11y Scope Summary (CI integration) / §9 CI Integration
- S2 · a new refusal path has its `unable <reason> <detail>` + final `hint:` layout pinned by trycmd and its typed exit code and hint-free `--json` asserted by assert_cmd · .andromeda/a11y-plan-detail/cognitive-accessibility.md
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:25) cognitive-errors
- S3 · no new fixed CLI string goes unpinned; a copy change fails CI until its pin is updated · .andromeda/a11y-plan-detail/cognitive-accessibility.md
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:34) cognitive-plain-language
- S4 · no SGR on `unable`, and none under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive` in any new output · .andromeda/a11y-plan-detail/visual-design-verification.md
  (.andromeda/a11y-plan-detail/visual-design-verification.md:8) visual-sgr
- S5 · a new test is zero-flake: `retries: 0`, no `#[ignore]`, a flake keeps the chunk red; the per-OS `gate --require coverage,doctest` admits no skip · §10 SLO Invariants & A11y Budgets
- S6 · no assertion is written up as WCAG conformance evidence for the cli or tui · §11 A11y Anti-Patterns › Strategy
