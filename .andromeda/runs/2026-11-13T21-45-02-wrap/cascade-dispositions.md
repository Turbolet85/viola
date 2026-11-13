# Cascade dispositions — 2026-11-13-rows-proposal-rule-work

## Step 2 — the sweep
Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-11-13T21-45-02-wrap/cascade-patterns-test-plan-1.toml --patterns-file .andromeda/runs/2026-11-13T21-45-02-wrap/cascade-patterns.toml` (baseline cb0b16ab), after the landings of TX 998–1004; one part, `sweep-pass-1.1.txt`, read whole.
Totals, copied: `total (7 patterns) · 10 rows over 8 files` · `per class · new 0/0 · standing 7/6 · leaf 3/2 · curation 0/0 · base 0/0`.
Pattern files: `cascade-patterns-test-plan-1.toml` (the dry run of `ops-main-test-plan.md`, whose landing followed: `tests1-1` the guard's old name); every other dry run printed `patterns: none` (prose alone, or no set). Main's own patterns, `cascade-patterns.toml`: the old wording of each amended claim — `m-symlink-unwrap` (a symlinked instance path reading `unwrapped-name`), `m-class-list` (the three hygiene classes), `m-key-resize` (a key after a resize), `m-every-payload` (every payload type derives Redact), `m-logs-0600` (the event log and diagnostics 0600 rule), `m-hint-shared` (the hint keyed by the reason). Not looked for: the hint strings themselves (unchanged), the email check (every row already states it), the record scrub (unchanged in every master).

Zero-row patterns, each with its control fired: `tests1-1` (test-plan.md:589), `m-class-list` (test-plan.md:851), `m-logs-0600` (security-plan.md:367) — statements about those patterns.

Rows:
- `.andromeda/security-plan.md:165` m-symlink-unwrap · contract/instance-unreachable-detail/unwrapped-name — amended (ops-main-security-plan.md, TX 1005): the symlink clause was false.
- `.andromeda/obs-plan.md:523` m-symlink-unwrap · contract/no-endpoint-instance/classification — no change: it states the reading function itself («anything else … reads unwrapped-name … and the instance check stays first»), which is still true; the check before it now refuses a link.
- `.andromeda/a11y-plan.md:206` m-symlink-unwrap · contract/instance-unreachable-detail/unwrapped-name — amended (ops-main-a11y-plan.md, TX 1006).
- `.andromeda/architecture.md:146` m-key-resize · decision dec-xucsxkvc — no change: «a test that writes a key after a resize waits for the child to observe the size» stays true (a conditional; the resize test now writes none).
- `.andromeda/test-plan.md:477` m-key-resize ×2 · decision/key-after-resize-waits-for-size — no change, the same conditional.
- `.claude/docs/gotchas.md:109`, `:110` m-key-resize leaf — to step 3 (the route entry «Key after a resize» and the conditional — true).
- `.claude/docs/services/viola-pty.md:31` m-key-resize leaf — to step 3.
- `.andromeda/obs-plan.md:990` m-every-payload edited · decision/veil-redact-payload-types — amended this pass (TX 1001), its new exception standing beside the rule.
- `.andromeda/design-system.md:262` m-hint-shared edited · contract/refusal-hints/refusal-line — amended this pass (TX 998); the keying clause kept, the shared-hint clause added.

Binds (step 2's lateral pairs): test-plan §3 ↔ obs-plan §3 / §6 and the a11y schema — no harness, log-format or violation-schema row changed this pass; nothing to keep consistent.

## Leaves
Set call: `cascade.py leaves` pass mode, baseline cb0b16ab — `total · pass · 10 group(s) · 17 leaves · 273789 B · unmapped 2` (`sweep leaves · 2 file(s) from sweep-pass-1.1.txt`).
Apply: `total · blocks 6 · applied 6` — every finding written by `--apply … --only`, none by hand, none declined.
- design-system+layout-templates · pass · 2 findings · applied 2 (design-summary.md:59 stale — the shared hint; frontend.md:26 omission — the answer hints and no hint for an opaque detail) · by hand 0 · declined 0
- obs-plan · pass · 1 finding · applied 1 (obs-summary.md:47 stale — Redact on content-bearing types only) · by hand 0 · declined 0
- security-plan-1 · pass · 1 finding · applied 1 (api.md:29 stale — unwrapped-name for an absent path or a regular file) · by hand 0 · declined 0
- security-plan-2 · pass · 2 findings · applied 2 (security.md:12 stale — the same; security.md:14 omission — the append open refuses a link and narrows through the handle) · by hand 0 · declined 0
- a11y-plan · added · security-plan+…+a11y-plan (CLAUDE.md) · security-plan+obs-plan · test-plan · test-plan+obs-plan · pass · 0 findings each.
- Unmapped: `.claude/rules/host-linux.md` (the host leaf — the host's own statements, no source states them) and `.claude/rules/testing-src.md` (a pointer list into testing.md, no master statement) — no comparer, no edit.
