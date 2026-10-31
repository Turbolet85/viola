# a11y · standing conventions

- S1 · each new CLI case runs inside `nextest-integration` / `nextest-e2e` locally and the per-OS `coverage` suite in CI, through `scripts/agent-run.*` `run`, with no new `suite` value and no separate a11y driver · `## 9. CI Integration` and `## 11. A11y Anti-Patterns` § CI
- S2 · a new output assertion uses assert_cmd, predicates, trycmd or the portable-pty outer PTY, stands in for SC 1.3.2, 1.4.1 and 3.3.1 only, and claims no WCAG conformance · `## 4. ARIA Patterns & Roles` and `## 11. A11y Anti-Patterns` § Strategy
- S3 · each new fixed CLI string is pinned by trycmd or by literal, so a copy change fails CI until the pin moves on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S4 · output-discipline invariants pass on all three OS legs, with `retries: 0`, no retry-once policy and `#[ignore]` banned · `## 10. SLO Invariants & A11y Budgets` and `## 11. A11y Anti-Patterns` § SLO
- S5 · no new test parses the rendered `claude` TUI screen or takes a screenshot as evidence · `## 11. A11y Anti-Patterns` § Universal
