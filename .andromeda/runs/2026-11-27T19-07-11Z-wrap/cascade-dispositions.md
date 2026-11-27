# Cascade dispositions — the 2026-11-27T19-07-11Z-wrap 0-pending wrap

## Sweep
Skipped: every `rows-land` dry run of this pass (architecture · security-plan · test-plan · obs-plan · a11y-plan)
printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file; step 3's leaves read each changed row's
delta` — the 43 operations each set a decision's `state` cell `provisional → standing` and no text cell, so no file of
the pass holds a pattern. The 112 ratification entries (`rec.py amend`, TX 5123–5129) add sidecar entries and change no
row. Step 3's leaves read each changed row's delta.

## Leaves
Mode: pass (`cascade.py leaves`, baseline `a80df0e5`, no chunk marker), 15 groups · 26 leaves · unmapped 2
(`.claude/rules/host-linux.md` (host) · `.claude/rules/testing-src.md` — neither restates a decision's state; no
change). One comparer a group, batch `leaf`, `fanout.py check`: `VERDICT: 15 of 15 pass · failed: none`.
`cascade.py leaves --apply`: `total · blocks 9 · applied 9`.

- a11y-plan — 1 finding, applied (`.claude/docs/a11y-summary.md:53`, «`RESPAWN_REAP_BOUND`, PROVISIONAL» → standing);
  by hand 2 in the same line, landed by `rec.py harness land` (TX 5135): «(40 s, PROVISIONAL, architecture [Delivery
  Confirmation])» on `COMPACT_CONFIRM_WINDOW`, whose architecture and test-plan rows now stand, and «PROVISIONAL bounds»
  on the dialog hold, which no master row marks provisional (`rec.py find --regex` over the seven: 0 hits) — the
  comparer replaced one of the line's three tags.
- architecture-1 — 2 findings, applied (`.claude/docs/services/viola-agent-claude.md:11`, `:33`, the
  `COMPACT_CONFIRM_WINDOW` tag).
- architecture+security-plan+test-plan+obs-plan+a11y-plan — 1 finding, applied (`.claude/docs/gotchas.md:84`).
- security-plan-1 — 1 finding, applied (`.claude/docs/security-summary.md:65`, the switch's tag).
- test-plan-1 — 4 findings, applied (`.claude/docs/tests-summary.md:21`, `:26`, `:44`, `:61`).
- architecture-2 · architecture-3 · architecture-4 · architecture+security-plan · architecture+test-plan+obs-plan ·
  obs-plan · security-plan-2 · security-plan+obs-plan · test-plan-2 · test-plan+obs-plan — 0 findings.
- Declined: none. Left as it stands: `.claude/docs/obs-summary.md:31` «spine provisional» — the row
  `obs-plan decision/spine-hook-budget` now stands but its rule text still reads «provisionally 1.0 s», unchanged by
  the directive's §1 («No row's TEXT changes»), so the leaf agrees with its row; named in the last card.
