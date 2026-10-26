# a11y · standing conventions

- S1 · Each new CLI output-discipline case runs inside `nextest-integration` / `nextest-e2e` locally and the per-OS `test` job's single `coverage` suite in CI, on all three OS legs, under `gate --require coverage,doctest` with no skip · §1 A11y Scope Summary (CI integration) and §9 CI Integration
- S2 · No new `suite` enum value (such as `a11y`) and no separate a11y driver outside `scripts/agent-run.* run` · §11 A11y Anti-Patterns › CI
- S3 · Each new refusal: trycmd pins the `unable <reason> <detail>` + final `hint:` layout, assert_cmd pins the typed exit code and a hint-free `--json` · .andromeda/a11y-plan-detail/cognitive-accessibility.md (Error recovery)
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:25) cognitive-errors
- S4 · Each new fixed CLI string pinned by trycmd or literal · .andromeda/a11y-plan-detail/cognitive-accessibility.md (Plain language)
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:35) cognitive-plain-language
- S5 · Each new human-mode output: no SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`, every SGR span beside its word, colour never the sole cue · .andromeda/a11y-plan-detail/visual-design-verification.md
  (.andromeda/a11y-plan-detail/visual-design-verification.md:8) visual-sgr
- S6 · No WCAG conformance claim rests on assert_cmd / trycmd / portable-pty results · §11 A11y Anti-Patterns › Strategy
- S7 · Zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy, no soft a11y SLO · §10 SLO Invariants & A11y Budgets and §11 › SLO
