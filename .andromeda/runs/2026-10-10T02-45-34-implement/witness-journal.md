# Witness journal — 2026-10-10-windows-mutation-grade, step 8

Three Linux mutation runs, one at a time, each a one-shot measurement (the operator's direction, `inputs#I4`).
Each entry is written before the run's wait and read back from this file when the run returns. A run is never
repeated to change its reading. A red where no assertion failed on a value is read against the test-home backing
and `hostwatch.py` before anything else.

Common to all three: the tree is the working tree on HEAD `781563cd59a6` with the chunk's edits uncommitted;
`TMPDIR` is the repository's sibling `../viola-mutants-scratch` (NOCOW, read with `lsattr -d`: `C` set); the
outputs go under the ignored `target/witness-wmg/`; the test-home backing reads `tmpfs`, 22.3G free, before the
first run. cargo-mutants keeps one earlier run's per-mutant logs, so a survivor's log is read from
`mutants.out/log/` before the next run starts.

## Witness 1 — the workflow's viola-pty item, on Linux

- Written: 2026-10-10T03:03:40Z, before the launch.
- Command, from the repository root:
  `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --package viola-pty --file crates/viola-pty/src/lib.rs --file crates/viola-pty/src/sideload.rs`
  (the scratch given by its absolute path on the command line).
- Outputs: `target/witness-wmg/w1.stdout` (the harness document), `target/witness-wmg/w1.stderr` (cargo-mutants'
  outcome lines), `target/witness-wmg/w1.exit` (the harness exit).
- Expected by the recipe (`forecast.py` in this run dir, on this tree): 89 mutants, 25 left out on Linux.
- Next step at its return: read this entry back; read `w1.exit` and the document; save the document as
  `evidence/linux-witness.json`; compare its `host_excluded` names with the recipe's 25 and record both counts and
  every difference; read any survivor's log before witness 2 starts; then write witness 2's entry.
- Returned 2026-10-10T03:10:14Z (this entry read back from disk first). Harness exit 0. Document: `ok:true`,
  verdict `package`, tested 89, caught 53, survived 0, failures none; cargo-mutants' own line: `89 mutants tested
  in 6m: 25 missed, 53 caught, 11 unviable`. `host_excluded` holds 25 rows, each under `windows`; the recipe's
  Linux list for the item holds 25; the two sets are equal by name and by predicate, no row on one side only.
  Saved as `evidence/linux-witness.json`, its outcome lines as `evidence/linux-witness-outcomes.txt`.
- Host over the run's window (`hostwatch.py read --from … --to … --for viola`): `QUIET`, 0 s stalled on IO.
  Backing after it: `tmpfs`, 22.3G free. No survivor, so no log to read before witness 2.

## Witness 2 — the reader, the exclusion and the deadline function

- Written: 2026-10-10T03:10:39Z, before the launch.
- Command, from the repository root:
  `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --file crates/viola-e2e/src/harness/run/mutants/host.rs --file crates/viola-e2e/src/harness/run/mutants.rs --file crates/viola-e2e/src/harness/cleanup.rs`
  (the scratch given by its absolute path). A scoped run over the chunk diff: its base is derived, and it mutates
  only lines the chunk changed in those three files.
- Outputs: `target/witness-wmg/w2.stdout`, `w2.stderr`, `w2.exit`.
- Looked for: zero missed; `replace + with - in kill_deadline` read `caught`.
- Next step at its return: read this entry back; read `w2.exit`, the document and the outcome lines; for every
  missed or timed-out mutant read its log under `mutants.out/log/` BEFORE witness 3 starts, then decide the case
  it takes in step 4, 5 or 6 (a survivor is work, not a re-run); save the document as
  `evidence/witness-reader.json`; then write witness 3's entry.
- Returned 2026-10-10T03:22:35Z (this entry read back from disk first). Harness exit 0. Document: `ok:true`,
  verdict `scoped`, base `d778ececb7dd`, tested 94, caught 88, survived 0, failures none, no `host_excluded`
  field (nothing was left out); cargo-mutants' own line: `94 mutants tested in 12m: 88 caught, 6 unviable`. By
  file: `host.rs` 73 caught and 2 unviable, `mutants.rs` 14 caught and 1 unviable, `cleanup.rs` 1 caught and 3
  unviable. `replace + with - in kill_deadline` at `cleanup.rs:130:9` read `caught`.
- No missed and no timed-out mutant, so no log to read and no case owed to step 4, 5 or 6.
- Host over the run's window: `QUIET`, 0 s stalled on IO. Backing after it: `tmpfs`, 22.3G free.
- Saved as `evidence/witness-reader.json`, its outcome lines as `evidence/witness-reader-outcomes.txt`.

## Witness 3 — the strict-modes check and owner-only creation

- Written: 2026-10-10T03:22:46Z, before the launch.
- Command, from the repository root:
  `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --file crates/viola-state/src/strict.rs --file crates/viola-state/src/fs.rs`
  (the scratch given by its absolute path). A scoped run over the chunk diff.
- Outputs: `target/witness-wmg/w3.stdout`, `w3.stderr`, `w3.exit`.
- Looked for: zero missed after the exclusion; the five `volume_keeps_acls` mutants and the Unix `restrict`
  mutant read `caught`; the Windows-gated diff lines named in `host_excluded`, each under its predicate.
- Next step at its return: read this entry back; read `w3.exit`, the document and the outcome lines; read any
  survivor's log before anything else runs a mutation; save the document as `evidence/witness-state.json`; then
  step 9's records, the forecast re-read on the final tree, and the whole gate block.
- Returned 2026-10-10T03:26:22Z (this entry read back from disk first). Harness exit 0. Document: `ok:true`,
  verdict `scoped`, base `d778ececb7dd`, tested 22, caught 6, survived 0, failures none, `host_excluded` 16 rows,
  every one under `windows`; cargo-mutants' own line: `22 mutants tested in 3m: 16 missed, 6 caught`.
- The six caught: the Unix `restrict` (`fs.rs:19:5`) and the five `volume_keeps_acls` mutants (`strict.rs:84`).
- The sixteen left out are all in `win::` bodies (`protect`, `set_dacl`, `dacl_of` twice plus its `Default`,
  `persistent_acls` four, `owner_and_dacl` seven); each stands in the recipe's Linux list for the viola-state item
  under the same predicate (16 of the recipe's 69 for the whole item; this run mutated only the chunk's lines).
- No survivor after the exclusion, so no log to read.
- Host over the run's window: `QUIET`, 0 s stalled on IO. Backing after it: `tmpfs`, 22.3G free.
- Saved as `evidence/witness-state.json`, its outcome lines as `evidence/witness-state-outcomes.txt`.

All three witness runs have returned. No mutation run is started after this line by this run.
