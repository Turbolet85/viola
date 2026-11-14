@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=115
last: - 2026-11-14: A remove-the-guard control whose entry runs a 
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=42 class=stale
leaf-text: The `viola_e2e::fixtures` copy lands with its first E2E consumer; Path 1's flow is not one, landing on the root chain as `tests/path1_run_start.rs`.
source: dec-cxlxjspx «Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through»
replace-with: The chain exists once, as the sync root copy in `tests/support/`: viola-e2e has no copy and boots its sessions through its own `harness_session`; Path 1's flow lands on the root chain as `tests/path1_run_start.rs`.
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=29 class=omission
leaf-text: Readiness is a bounded 100 ms file-state probe, staged:
source: ctr-eo3m22cw «harness boot · timeout — Readiness is bounded at 20 s per instance after the build.»
replace-with: Readiness is a file-state probe polled every 100 ms against a deadline of 20 s per instance after the build, staged:
