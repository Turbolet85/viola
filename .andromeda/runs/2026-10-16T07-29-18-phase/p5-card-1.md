# P5 review card 1: plan revision for 2026-10-14-resume-and-compact-ledger-rows

**The plan:** `viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md` (revised)
**The revision's run dir:** `.andromeda/runs/2026-10-16T07-29-18-phase/`
**The cause:** the operator's word, `inputs#I4` (`relay-1.md`)

## Why the plan is revised
- `/andromeda-implement` stopped on entry 15, the preservation guard.
- Every other gate is green, and both live rounds are done: 12 of 12 starts spent, `stamped 2.1.287  22 pass  0 fail`
  and `local-live passed 1 failed 0`.
- The guard reads red on one file, `tests/support/home.rs`. That file is a step 9 companion recorded in
  `scope-record.md`: `dialogless_home`'s summary literal moves 17/14 → 18/15.
- The guard's exclusion list did not name it. The take-up's sweep keyed on `… pass` literals in `tests/cli_verify.rs`,
  and this reader builds its line from an integer.

## What changes
| Where | Change |
|---|---|
| `[[gate]]` entry 15 (the preservation guard) | adds `':!tests/support/home.rs'`; `note` says so; `baseline` re-read |
| step 9 | names `tests/support/home.rs:386` (`dialogless_home`), and the run-count readers measured at implement |
| Codebase touchpoints | `tests/support/home.rs` added. `tests/support/verify.rs`'s line now states what was done: the shaped session id and the `after` parameter |
| new `## Revision` section | the steps already done with evidence, and the re-entry |
| `scope.md` | the sweep premise corrected `[premise-corrected: …]`, plus a revision section citing `inputs#I4` |
| `research.md` | both lists rewritten to the write set actually made (15 modify + 1 new). `screen.rs`, `slot.rs` and `cli_revive.rs` are dropped as unbuilt branches. One sweep row added |

No step, live round, acceptance criterion or other gate entry changes. Every other entry keeps its `baseline`
(`gate.py baseline`: seen green 8 · new (key) 9 · leg 9).

## The re-entry
- `/andromeda-implement` re-enters at block entry 15 and goes on through the block from there.
- No live leg (entries 7–11) is fired again: the cap is spent (`inputs#I1` fork 3, `inputs#I4`).
- Then the operator pass: entries 24–26 (hygiene, the clean-tree push, `ci.py conclusion --sha HEAD --wait 1800`).

## Entry 15's baseline and controls (`p5-controls.md`)
- On the real tree (the stopped run's edits): **green**.
- Clone at `880b056377f4`:
  - unmodified: green;
  - must-pass (home.rs, ledger.rs, the variant, chunk evidence, the friction log): green;
  - must-fail `src/run/send/slot.rs` edited: red;
  - must-fail `tests/support/mod.rs` edited: red (the exclusion is the one file, not the directory);
  - must-fail an untracked `src/p5_control.rs`: red.

## Top acceptance criteria (unchanged)
- `LedgerRow::ALL` holds 22 rows, the last `resume-session`, relied on every OS: `MM` is 22 on Unix and 19 on Windows.
- The `resume-session` check passes only on Run E's first SessionStart of source `resume` whose id equals Run B's first
  id (5 named cases).
- `/compact` on a verified CLI reads `[  ] unconfirmable`, exit 0.
- `SessionStart.resume-1.json` is recorded live and replayed byte for byte.
- `pre-push` is green, and CI reads `verdict: green` on the final HEAD.

## Mechanical checks (re-run after the last plan edit)
- dry-run parses 26 entries, with no unknown key, wrong type or undeclared unset handle.
- `planlint`: 0 hits.
- `scope lists: parsed 16 items`.
- `inputs`: 4 entries, 0 unparsed, 0 uncited.
- Size: about 300 prose lines.
- Validation-1: aligned. The scope's revision section is what the plan does.

## Matrix
- No capability is claimed by this chunk (`matrix.py show --chunk`: 0), and none is claimed now.
- `v1-34` stays pooled; its take-up note stands. There is no concretization preview.

## Beyond the plan
none
