# Cascade dispositions — 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel (wrap 2026-11-20T07-09-09Z-wrap)

## Step 2 — the sweep
- The tool's pattern files: none. Every `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file` (architecture 3, security-plan 4, test-plan 2 and 1, a11y-plan 1) or `patterns: none — the call sets and retires no row` (obs-plan); no file was superseded.
- Main's own patterns, `cascade-patterns.toml` (6): `m-relabel-only` (the origin contract's old «driver is set only by the wrapper's relabel»), `m-human-moves` (a human-filed prompt always moving the wheel), `m-cc-seed` (the kept stamp seeded into CC alone), `m-second-send` (the hint reading after the second send alone), `m-turn-cell-attr` (the a11y except's old «after a turn end or send the box row's cell attributes»), `m-recorded-bytes` (the fake's closed set-field list). Each control fired on the pre-pass masters.
- Listing: `sweep-pass-1.1.txt` — `total (6 patterns) · 16 rows over 10 files` · `per class · new 1/1 · standing 7/3 · leaf 8/7 · curation 0/0 · base 0/0`.

Every row:
- architecture.md:110 `m-human-moves` dec-xbbudrvy (decision/human-prompt-confirms-wheel-take) — amended (its except, tx 3268).
- security-plan.md:360 `m-cc-seed` dec-cgqgi7dg (decision/live-rig-kept-stamp-owner-only) — amended (tx 3269).
- test-plan.md:568 `m-second-send` dec-wpcapchv (decision/live-rig-hint-cell-reading) — amended (tx 3270), the new text.
- test-plan.md:38 `m-turn-cell-attr` dec-lsyfvmke (decision/no-child-screen-assertions) — no change: its except names «the box row's cell attributes» with no timing, true of the fresh-box reading too.
- test-plan.md:148 `m-turn-cell-attr` dec-g6y7k5xo (decision/never-parse-rendered-child-screen) — no change, the same reason.
- test-plan.md:271 `m-turn-cell-attr` dec-fnoyzejm (decision/discipline-at-every-layer) — no change, the same reason.
- test-plan.md:1097 `m-turn-cell-attr` dec-j2mfetpl (decision/no-screen-parsing) — no change, the same reason.
- test-plan.md:877 `m-recorded-bytes` dec-d5qj3lkd (decision/fake-agent-replays-recorded-bytes) — amended (tx 3272).
- .claude/rules/events.md:20 `m-relabel-only` — leaf, step 3.
- CLAUDE.md:40 `m-cc-seed` — leaf, step 3.
- .claude/docs/security-summary.md:36 and :73 (×2) `m-cc-seed` — leaf, step 3.
- .claude/rules/testing.md:38, .claude/rules/verification-harness.md:13, .claude/docs/tests-summary.md:50 `m-turn-cell-attr` — leaf, step 3 (no timing stated: expected no change).
- .claude/docs/a11y-summary.md:46 `m-turn-cell-attr` — leaf, step 3 (quotes the a11y except's old wording).

Not looked for: the told-wake rule's own wording in the leaves (added, not retired — step 3's comparers read each changed row's delta). The rules homes (`## Session Additions`, `USER:session-learnings`, `docs/session-learnings.md`) and the bases: 0 rows.

## Leaves
`cascade.py leaves` (pass mode, baseline 33321186): `epoch · «Epoch 4 — Driver reliability: ledger rows and bounds» · markerless 5 · gated 0 · open` · `sweep leaves · 7 file(s) from sweep-pass-1.1.txt · 0 outside every group` · `total · pass · 13 group(s) · 26 leaves · 431365 B · unmapped 2`.
Apply: `total · blocks 19 · applied 16 · held 3`; nothing by hand.
- a11y-plan · pass · 1 finding · applied 1 (a11y-summary.md:46, the except's fresh-box reading).
- architecture-1 · pass · 1 · applied 1 (services/viola-agent-claude.md:6, the origin relabel).
- architecture-2 · pass · 2 · applied 2 (services/viola.md:27 and :41, the fired-wake except and relabel).
- architecture-3 · pass · 2 · applied 2 (rules/events.md:20 and :22).
- architecture+security-plan · pass · 0.
- architecture+security-plan+test-plan+obs-plan+a11y-plan · pass · 1 · applied 1 (CLAUDE.md:40, «CC or R3W»).
- architecture+test-plan+obs-plan · pass · 0.
- obs-plan · pass · 0.
- security-plan · pass · 6 · applied 6 (security-summary.md:36, :73 ×3, :71; rules/security.md:25).
- security-plan+obs-plan · pass · 0.
- test-plan-1 · pass · 3 · applied 1 (tests-summary.md:29, the fired wake beside the harness turns) · declined 2: tests-summary.md:21 #1 (`--stop-crons`, «0 to 4») and #2 (`--wake-after-ms`, «capped at 5 000») — each would put a figure the code states into a leaf (inputs#I7 item 6, inputs#I3 item 10); the fake's options are test-plan rows (contract/fake-agent-cli/stop-crons, …/wake-after-ms), and no reader of the summary acts against them unaware.
- test-plan-2 · pass · 2 · applied 1 (verification-harness.md:38, the closed set-field list gains `--stop-crons`, no figure) · declined 1: verification-harness.md:41 #1 (the two options with «0 to 4» and «5 000 at most») — the same reason.
- test-plan+obs-plan · pass · 1 · applied 1 (rules/observability.md:41, the typed mark and count uninstrumented).
- unmapped `.claude/rules/host-linux.md (host)` — the host's own statements, no comparer's (no step-3 edit). Unmapped `.claude/rules/testing-src.md` — scoped to `src/` test layout, no master of this pass its source; no change.
- The driver's own «set no scheduled wake» (CLAUDE.md `USER:session-learnings`) is a curation home, never a cascade edit: P3 owns its one-line pointer to the re-pin (inputs#I7 item 6).
