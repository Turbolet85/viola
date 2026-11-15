# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T11-19-59)

**Outcome: green, and the operator pass is done.** The chunk completes on v2-02. v2-03 is not verified this round, on
the founder's own word, and is un-claimed at the wrap. Stopped before the wrap, on the operator's word (inputs#I35).

Setup's approval line: `2026-11-15T11-17-37-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json` (plan rev
15).

## Step 9 — `hold: stands`
`crates/viola-core/src/lib.rs`: `DIALOG_HOLD_MAX`'s doc comment loses «PROVISIONAL: measured live only to about 660 s»
for «Measured live at the ceiling itself: a wrapper expiry after 3 600 013 ms on claude 2.1.325, the hook still
waiting.» The value stays 3600, and the old words stand nowhere else in the tree (swept).

## The gates
- 13–17 and 21–25: `green 10 · red 0` (run `2026-11-15T11-19-59-implement`).
  - 13: `place: {"carried": ["Mcp.tools-list-1.json", "Screen.ready.json", "Screen.turn.json"], "cli_places": false, "placed": []}`.
  - 14: `106 tests run: 106 passed`.
  - 15: `1 test run: 1 passed`.
  - 16: `3280 tests run: 3280 passed, 0 skipped` (69.8 s).
  - 17: fmt clean.
  - 21: `stamp-check: pass`; 22: `screens-check: clean`; 23: `rehearse-reuse: green`;
    24: `rehearse-trust-form: green`; 25: `status-row: present`.
- 9–12, the live round, as fired in earlier runs: 9 red (stamp green, recording refused) · 10 red (no `session-start`) ·
  11 red twice (the first raised before any start; the second on `hits: 0`, its status row later read `present` by
  entry 25) · 12 red on v2-03's two clauses, its hold reading green. Starts 9 of 9; none was made in this run.

## The operator pass (`evidence/operator-pass.txt`)
- **18, hygiene:** `hygiene: clean — read 421 …`, then landed (tx 1460).
- **The pre-CI commit:** `4c02d2affcbbcc74f6ac8d862f2f83b02250d6bb chore(2026-11-15-live-round-on-the-trimmed-product):
  operator pre-CI commit, for the run this chunk's verdict reads`. 375 files; the only one outside the pipeline's tree
  is `crates/viola-core/src/lib.rs`.
- **19, push:** `56c99714..4c02d2af  build/viola-0.2.0 -> build/viola-0.2.0`. HEAD equals the remote.
- **20, CI:** `4c02d2affcbb verdict: green · checks 7/7 · wall 632 s · runs ci#43708704119 completed/success`.

## Matrix
- **v2-02 → implemented.** Its ref is `evidence/operator-12.txt`'s hold reading (`dialog-expired` source `wrapper`,
  `deadline_hit=true`, `duration_ms=3600013`, no key during the hold) plus the doc comment. It is cleared by
  `cleared-v2-02.md`: the founder's four lines are all answered, `hold: stands` among them (inputs#I5, inputs#I33).
- **v2-03:** no ref. It is un-claimed at the wrap with a dated note on the founder's word, plus a CARRY (card
  `2026-11-15T11-17-37-phase/p5-card-1.md`, «Beyond the plan»).

## Files changed
- `crates/viola-core/src/lib.rs` — step 9.
- Work dir: `evidence/operator-pass.txt` (new) and the gate entries' own writes (the summary lines of 23 and 24's
  rehearsals).

## Deviations
None. Scope read: `scope: clean — changed 1 · listed 1 · recorded 0`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 23 and 24's rehearsal sessions | this run (gate entries) | terminated — `census_held=0` in each closing line |
| `cargo nextest` (entries 14–16) | this run (gate entries) | terminated — exited with their entries |

## Decisions & corrections
- **The operator's word (inputs#I35):** step 9, the remaining entries, v2-02's ref and the operator pass, including
  the pre-CI commit and the push; then stop before the wrap. The commit and push were made on that word, not by this
  skill's own reach.
- **A guard refusal:** a `rec.py get` piped into `head` was refused by the bash guard. It was re-issued as a plain read
  of the gate logs, and no `rec.py` output was filtered.
