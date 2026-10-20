group: test-plan-1
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
read: .claude/rules/testing.md · 102 · «- 2026-10-20: A probe word the right implementation also wri»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «one of twelve argv options»
  source: .andromeda/test-plan.md:962 «thirteen argv options (no env; the count's rule is architecture's list of them)»
  replace-with: «one of thirteen argv options»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and the test-only screen switch `--tag-turn-screen <phase>` draws the named screen after the tag-like paste's turn alone)»
  source: .andromeda/test-plan.md:962 «the test-only screen switch `--long-turn-screen <phase>` (with `--framing` and `--turn-stop`: after the long paste's turn alone, after its Stop, the agent draws `Screen.<phase>.json` of its set»
  replace-with: «and the test-only screen switches `--tag-turn-screen <phase>` and `--long-turn-screen <phase>` draw the named screen after the tag-like paste's turn alone and after the long paste's turn alone)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(sets `2.1.283`, `2.1.287` and `2.1.288`, 12 dialog variants each in 2.1.287 and 2.1.288, six framing variants, two status line variants, one resume variant and one agents fixture (`Agents.recorded-1.json`, Run B's own row of its one `claude agents --json` read) in 2.1.287 and the superseded relayed fixtures included;»
  source: .andromeda/test-plan.md:970 «today the sets `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`: the four spine `{SessionStart,UserPromptSubmit,Stop,SessionEnd}.default.json` in each, `2.1.287`'s four superseded relayed dialog fixtures»
  replace-with: «(sets `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`, 12 dialog variants each in 2.1.287, 2.1.288 and 2.1.302, six framing variants, two status line variants, one resume variant and one agents fixture (`Agents.recorded-1.json`, Run B's own row of its one `claude agents --json` read) in each of 2.1.287 and 2.1.302, and 2.1.287's superseded relayed fixtures included;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «2.1.287 stamped at every relied row, 24 on Unix and 21 on Windows, 2.1.283 and 2.1.288 drift-only)»
  source: .andromeda/test-plan.md:603 «for each stamped set (`fixtures/claude/2.1.287/` and `fixtures/claude/2.1.302/` today)»
  replace-with: «2.1.287 and 2.1.302 stamped at every relied row, 24 on Unix and 21 on Windows, 2.1.283 and 2.1.288 drift-only)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «(2.1.287, recorded by Run E since chunk 2026-10-14-resume-and-compact-ledger-rows)»
  source: .andromeda/test-plan.md:936 «Run E's resume payload as `SessionStart.resume-<n>.json` (one under each of `2.1.287` and `2.1.302`)»
  replace-with: «(2.1.287 and 2.1.302, recorded by Run E since chunk 2026-10-14-resume-and-compact-ledger-rows)»
