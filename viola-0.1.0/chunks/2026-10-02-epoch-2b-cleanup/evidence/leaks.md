# Leaks A and B (steps 4–5) — measured

/implement run `2026-10-02T19-38-47-implement`. The "before" counts are in `host-snapshot.md`. Readings come from this
session's runs. "C: copy" is a gitignore-filtered copy of the tree in the session scratchpad (see `m2-diagnosis.md`).

## Leak A — the throwaway git repos in `%TEMP%`
**Premise falsified.** Plan step 4 held that git's read-only object files make Windows refuse `TempDir`'s removal, which
leaves the dir. Measured:
- A committed throwaway repo holds 3 read-only object files (python probe).
- `throwaway_repo_leaves_no_dir_after_drop` with the planned guard **removed**, so `git_repo()` returns a plain
  `TempDir`: **green** (C: copy, nextest unit, 0.12 s). std's Windows `remove_dir_all` deletes read-only files, so a dropped
  repo goes whole.
- A full viola-e2e unit run on the unguarded fixtures (225 passed): **0** new `%TEMP%/.tmp*` dirs.
- The two in-repo witness runs (root integration, 80 failures between them): **0** new `.tmp*` dirs.

So no self-test that ends leaves a repo behind. The 13 497 git-repo dirs in `%TEMP%` cluster on the mutation and coverage
days (by-day table in `host-snapshot.md`). That fits runs whose test processes were terminated before their `TempDir`
dropped. Those are runner kills, which no drop guard can reach. Under testing.md 2026-09-24 ("a guard that repeats what the
call already guarantees … leave it out"), **the guard is not written**. `run.rs`'s `git_repo()` / `mini()`, `base.rs` and
`mutants.rs` stay as at HEAD. `throwaway_repo_leaves_no_dir_after_drop` lands over the plain `TempDir` as the witness of
the measured fact (gate 6). It has no remove-the-guard red because there is no guard. This is surfaced for the wrap as a
premise disproof (scope §3 / plan step 4).

The 399 throwaway cargo projects and 132 empty dirs of 10-01 did not recur either (0 new dirs of any shape).

## Leak B — the ownerless homes under `target/e2e-home`
- Two full in-repo witness runs (D1, D2: 40 failures each) left **0** entries in `target/e2e-home`. No test drops its home
  while a child still holds a file in a run that is not killed.
- The owner record landed on 09-27 (`39b8297`). 718 of the 4 768 historic ownerless homes are dated 09-24 to 09-26 and
  predate it. The rest were removed by the operator's `cargo clean` before they could be read whole.
- Mechanism kept closed anyway: std's `remove_dir_all` stops at the first entry it cannot delete, **in listing order**.
  An entry beside the home that sorts after `owner.json` (a test's scratch dir, e.g. `sweep/`) lets a plain removal delete the
  record first and then stop, which leaves an ownerless remnant no sweep may take. `tests/support/home.rs` now removes an
  owned dir through `remove_owned`, which deletes the record last. Both `TestHome`'s drop and `sweep_gone_owners` use it. The
  owner-record rule is unchanged: reclaim only by pid + start time, never by name or age.
- `remove_owned_keeps_the_owner_record_while_a_file_is_held` (C: copy, cli_instance_state):
  - guard removed (a plain `remove_dir_all`): **red**, `the record went first` (tests\cli_instance_state.rs:485);
  - guard restored: **green**.
  A first form of the case, with the held file under `home/` (sorted before `owner.json`), read green both ways. It was
  reshaped to the order that separates the two.

### Leak B on D:'s red runs — measured after the gates (correction to the first reading above)
The gates' D: runs (entries 10, 11, 13 and 19, about 300 failures between them) left **22** ownerless `viola-test-*`
homes. Every one is a stamped-home remnant: `home/ledger/stamps.json` and its `.lock`, usually the pinned
`home/bin/<key>/viola.exe`, and no `owner.json`. Two-sided on D:, same command (the 5-binary witness set), the ownerless
dirs new after each run:

| home.rs | run 1 | run 2 | run 3 |
|---|---|---|---|
| this chunk's (`remove_owned`) | 1 | 2 | — |
| HEAD-like (drop and sweep on a whole-dir `remove_dir_all`) | 0 | 3 | — |
| this chunk's, plus `wait_bounded` waiting out its kill (`tests/support/verify.rs`) | 2 | 3 | 2 |

Each run had 42–43 failures. The rates do not separate, so **`remove_owned` neither causes nor closes this class**, and
neither does waiting out `wait_bounded`'s kill. The `verify.rs` change was reverted (byte-equal to HEAD): its guard has no
measured effect. No live process was found running from `target/e2e-home` after the runs, and the mechanism that removes
the record yet keeps these files is not established.

What is established: **this class appears only where tests fail at D:'s deadlines.** The C: copies hold **0** ownerless
homes after about ten all-green or nearly-green runs (the M2 witness arms, three full-package rounds, the M1 baselines,
coverage, unit runs). The HEAD copy holds one home *with* its record, from its single load-stall failure; the sweep can
take it. Leak B is therefore a consequence of M2's failing tests and goes with M2's decision. The plan's "no new ownerless
home after one in-repo run" criterion is **not met** in-repo while M2 is open.

`remove_owned` stays: it closes the ordering hazard its own case demonstrates (red without, green with). It is not claimed
as the fix for the D: remnants.
