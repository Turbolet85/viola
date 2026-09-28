# Cascade dispositions — 2026-09-28-hook-perf-gate

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (17 patterns, every retired claim of the pass: the seam
count and its wordings "one ratified exception" / "the one carve-out" / "the one variable outside" / "one other
variable" / "only env var outside"; the retired export paths `perf/*.json` / `perf/hook-`; the feature spelling
`--features fake-agent --target-dir`; the CI count `15 check-runs` / `(8; `; the pending phrasing `"Hook perf gate"
chunk`; `no forced panic`; `not built yet`; `no line over 4 KiB`; the retired two-session design `perf-stamped` /
`perf-unstamped`; `fixture stdin`; the G2-in-`test`-only wording `presence-checks it` / ``G2 (`test`)``; the retired
same-job rationale `separate perf job` / `same per-OS job`; `runs the hyperfine gates`; the retired gate detail code
`` `perf-*.json` ``), baseline `85ae5aa` (the pre-CI parent). Every pattern's control fired on the pre-pass masters.
Listing: `sweep.txt` in this run dir. Beside it, one leaf read (`grep -rnE 'hyperfine|--perf|perf job|\bG2\b|seam|
forced[- ]panic|cli_controls|check-runs|over 4 KiB|4 KiB'` over CLAUDE.md, `.claude/rules`, `.claude/docs`) for leaves
that restate a changed fact without a retired token.

## Rows

| row | disposition |
|---|---|
| architecture.md:107, :172, :586 (pump-seam, `edited`) | amended this pass — the pump seam now stands beside the hook seam; true |
| architecture.md:374 (pump-seam) | no change — describes the pump seam itself; true |
| security-plan.md:232, :688, :690, :693 (pump-seam) | no change — the pump seam's own row and its 2026-09-27 Decisions Log entry (history); true |
| security-plan.md:427, :585 (pump-seam, `edited`) | amended this pass; true |
| test-plan.md:927, :1897 (pump-seam) | no change — the pump seam's forced-window test; true |
| playbook.md:50 (pump-seam, base) | no change — quotes the 2026-09-27 ratification as history; true |
| .claude/rules/security.md:34 (pump-seam, one-variable, leaf) | re-derived — two seams + the G2 exact-path exemption |
| .claude/docs/conventions.md:13 (pump-seam, one-exception, leaf) | re-derived — two ratified exceptions |
| .claude/docs/gotchas.md:104 (pump-seam, leaf) | no change — the pump seam's forced-window test; true |
| .claude/docs/security-summary.md:39 (pump-seam, leaf) | re-derived — two env carve-outs |
| architecture.md:97 (one-exception) | no change — a different claim sharing the token (`viola-pty`'s `PtyError` per-crate error exception) |
| .claude/docs/obs-summary.md:31 (hook-perf-gate, leaf) | re-derived — the per-OS `perf` job, four rows |
| .claude/docs/services/viola-state.md:41 (hook-perf-gate, leaf) | re-derived — the over-4 KiB half landed |
| .claude/docs/services/viola.md:41 (hook-perf-gate, leaf) | re-derived — perf arm built + the forced-panic seam line |
| .claude/docs/tests-summary.md:44 (hook-perf-gate, leaf) | re-derived — built, gated in the `perf` job |
| test-plan.md:509 (not-built-yet) | no change — a different claim (unbuilt selectors such as `boot --ui`, `run --e2e`); true |
| .claude/docs/commands.md:5 (not-built-yet, leaf) | no change — the general "surfaces not built yet" note; true |
| obs-plan.md:621 (no-4kib-line, `edited`) | amended this pass — the phrase now describes only the 8-process check, beside the landed over-4 KiB half; true |
| test-plan.md:929, :1184 (fixture-stdin) | no change — the `hook_events` / `--home` hook tests, which do take test-written fixture stdin; not the perf rows |
| .claude/docs/commands.md:16 (g2-test-only, leaf) | re-derived — the `test` and `perf` jobs presence-check jq; the G2 script refuses without it |
| obs-plan.md:1757 (same-perf-job) | no change — §12 Decisions Log history (the same-job decision as it was taken); the current truth is §9 step 1, amended |
| test-plan.md:656 (perf-glob-code, `new`) | this pass's own text ("every present `perf-*.json` is judged") — true; the retired detail code is gone from :657 |
| 0-row patterns (one-carve-out, perf-dir-json, feat-spelling, ci-count, no-forced-panic, two-sessions, run-hf-gates) | each control fired on the pre-pass masters; no stale site remains |

## Leaves re-derived from the leaf read (no retired token, a changed fact)
- `.claude/docs/stack.md:34` — mirrors arch §Stack Code quality verbatim: recomputed from `architecture.md:38` (G2's script, hyperfine 1.20.0).
- `.claude/docs/commands.md:37` — the "Built today" `run` grammar gains `--perf`; `:69` — G2 is the script with its probe and exact-path exemption.
- `.claude/rules/verification-harness.md:45` — `run --perf` joins the runner-seam arms, boot/cleanup through `PerfSession`.
- CLAUDE.md `GENERATED:setup:*` blocks — recomputed from arch (overview · modules · warnings · pointer table · architecture): no block states a seam, perf, G2 or CI-count fact; no change.
- `.claude/docs/stack.md:49`, `.claude/docs/tests-summary.md:60`, `.claude/rules/testing.md:34`, `.claude/docs/gotchas.md:65`, `.claude/docs/obs-summary.md:28` — read; true as they stand (hyperfine listed; G2 over home role files; G2 reference).

## Curation homes and judgment bases
No `curation` row; the one `base` row (playbook.md:50) is history and stays.
