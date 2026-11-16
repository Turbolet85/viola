# Outcome — 2026-11-16-survey-key-keeps-the-wheel (implement run 2026-11-16T15-55-51-implement)

**Outcome: green.** Plan rev 10 (revision 2), re-entered at step 1. No live start.

## Files changed
- Product: none. At step 1, `git restore -- crates/viola-agent-claude/src/screen.rs src/cmd/run.rs src/run/gate.rs
  src/run/wheel.rs` exited 0, and `git diff --stat -- src crates` printed nothing after it. The four files are
  byte-equal to HEAD 498b4533.
- Evidence (gitignored, landed in the store): `.andromeda/cache/chunks/2026-11-16-survey-key-keeps-the-wheel/evidence/live-starts.md`
  — a `## Revision 2` section was appended (step 3). No other evidence file was touched.

## Gates (trail `.andromeda/runs/2026-11-16T15-55-51-implement/gate-2026-11-16-survey-key-keeps-the-wheel.json`, one firing)
`entries 15 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 3`
- 1 the rollback probe: green, last line `0` (it was red at revision 2's baseline, `4`)
- 2 `cargo fmt --all --check`: green
- 3 clippy: green
- 4 `cargo check`: green
- 5 the wheel/gate/send/run modules: green, `304 tests run: 304 passed, 733 skipped` (equal to its baseline)
- 6 `viola-agent-claude`: green, `572 tests run: 572 passed, 0 skipped` (equal to its baseline)
- 7 the integration set: green, `76 tests run: 76 passed, 0 skipped` (equal to its baseline)
- 8 the fixtures probe: green, last line `0`
- 9 the `CLAUDE_FORCE_DISPLAY_SURVEY` probe: green, exit 1, last line `0`
- 10 the starts ledger: green, last line `6` (no seventh start)
- 11 `agent-run.sh run --unit`: green
- 12 `agent-run.sh pre-push`: green, 86.99 s
- 13–15: `not run — leg operator`; they belong to the operator pass.

No deferral: every gate ran. Smoke: `skipped — no boot-path / UI-surface change`. Matrix: no `chunk==` row (v2-21 was
un-claimed at P5), so no ref was written.

## Deviations
none. The scope read: `scope: clean — changed 0 · listed 0 · recorded 0 … excluded 260`. scope record: none.

## Surfaced / stuck
none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started outside the gate tool's entries | — | — |

The census (`gate.py census --comm viola --comm claude --comm viola-fake-agent --args projects/viola/target/debug/viola`)
reads `census: 14 running`. None of them is from this tree's `target/debug/viola`. They are the overseer sessions, the
prototype builders (`viola-lab/prototype`), the product's `viola wait` and the plugin's `viola mcp`. Not this run's.

## Decisions & corrections
- The fix-loop protocol's «NEVER `git checkout --` the working tree» stood against plan step 1's `git restore`. The
  plan's step and the operator's invocation («step 1 the rollback of the four product files») directed the restore,
  and the overseer holds the diff as a patch outside the repository (plan step 1).

## Next
The OPERATOR PASS: entry 13 (`gate.py hygiene`, fired as written, before the land and the pre-CI commit), entry 14
(the push, the operator's), entry 15 (`ci.py conclusion --sha HEAD --wait 1200`, fired as written). Then
`/andromeda-wrap-session`.
