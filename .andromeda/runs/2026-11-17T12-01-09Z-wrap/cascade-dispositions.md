# Cascade dispositions — 2026-11-17-booted-wrapper-does-not-outlive-its-test

## Step 2 — the sweep
patterns: none — ops-architecture.md's dry run printed `patterns: none — the call sets and retires no row`; ops-test-plan.md's `patterns: none: 2 set(s) and retire(s) changed prose alone — no file; step 3's leaves read each changed row's delta`; ops-main-test-plan.md and ops-security-plan.md landed nothing. No file holds a pattern: the sweep is skipped, and step 3's leaves read each changed row's delta.
Verbatim cross-master citations: P1's site sweep (`.andromeda/runs/2026-11-17T12-01-09Z-wrap/sweep-pre-pass-1.1.txt`, pattern `drop-guard`) listed every master row naming a Drop guard: architecture `dec-yll6cprb`, security-plan `dec-pzgvqgc3`, obs-plan `dec-4m453ix3` are other subjects (verify's probe dirs, an event emitted from a guard) — no change; test-plan `dec-dvrimbrq`, `dec-gpk2ut2j`, `dec-cxlxjspx`, `dec-izwwcxl2` state a guard that kills and waits, which the amended `decision/wrapper-drop-guard` keeps («a Drop guard that kills, then waits, still covers the normal end») — no change.
Lateral binds (test-plan §3 ↔ obs-plan §3 / §6, a11y schema): no harness command, log field or violation schema changed — no change.

## Leaves
`cascade.py leaves` (pass, baseline 2dc9b58d): `total · pass · 7 group(s) · 20 leaves · 327575 B · unmapped 2`.
- architecture-1 · pass · findings 0
- architecture-2 · pass · findings 0
- architecture-3 · pass · findings 0
- architecture-4 · pass · findings 0
- architecture+test-plan · pass · findings 0
- test-plan-1 · pass · findings 1 · applied 1 (`.claude/rules/testing.md:18#1` omission: the tied builder and its census)
- test-plan-2 · pass · findings 3 · applied 3 (`.claude/rules/verification-harness.md:28#1` stale, `:30#1` omission, `:42#1` omission: the CLI boot untied, the in-test session tied, the root children tied)
- by hand 0 · declined 0
- unmapped `.claude/rules/host-linux.md` (host): no master source — no step-3 edit. Unmapped `.claude/rules/testing-src.md`: names no changed row's subject (it scopes `src/` unit tests) — no change.
Tool total: `total · blocks 4 · applied 4`.
