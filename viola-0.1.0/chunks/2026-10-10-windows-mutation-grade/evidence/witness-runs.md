# Step 8 — the three Linux witness runs

One-shot measurements on the Linux dev host, one at a time, never gate entries. Each entry's first part is written
before the run is waited on (the same text stands in the implement run dir's `witness-journal.md`); its result is
added when the run returns. No run is repeated to change its reading.

Common to all three: the working tree on HEAD `781563cd59a6` with the chunk's edits uncommitted; `TMPDIR` on the
repository's sibling `../viola-mutants-scratch` (NOCOW: `lsattr -d` reads `C`); outputs under the ignored
`target/witness-wmg/`; cargo-mutants 27.1.0 and cargo-nextest 0.9.146, the CI pins.

## Witness 1 — the workflow's viola-pty item, on Linux

- Written 2026-10-10T03:03:40Z, before the launch.
- `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --package viola-pty --file crates/viola-pty/src/lib.rs --file crates/viola-pty/src/sideload.rs`
- Outputs: `target/witness-wmg/w1.stdout` (the document), `w1.stderr` (the outcome lines), `w1.exit`.
- The recipe's reading of this item on this tree: 89 mutants, 25 left out on Linux.
- Next: save the document as `linux-witness.json`, compare its `host_excluded` with the recipe's list.
- Returned 2026-10-10T03:10:14Z, harness exit 0. The document (`linux-witness.json`): `ok:true`, verdict
  `package`, package `viola-pty`, tested 89, caught 53, survived 0, no failure. cargo-mutants' own summary line:
  `89 mutants tested in 6m: 25 missed, 53 caught, 11 unviable` (`linux-witness-outcomes.txt`, 92 lines).
- `host_excluded`: 25 rows, every one under the predicate `windows` (16 in `lib.rs`, 9 in `sideload.rs`). The
  pinned recipe (`cover.py`, through the implement run dir's `forecast.py`, on this tree) lists 25 for the item on
  Linux. The two sets are equal by name and by predicate; no row stands on one side only. The plan predicted 25
  at HEAD.
- The three `console::is_console` mutants (`lib.rs:434:9` twice, `:436:76`) are among the 25: Linux does not
  compile that body, so this run says nothing about their kill. That reading is the Windows runner's.
- The host over the run's window (`hostwatch.py read --from 03:03:40Z --to 03:10:14Z --for viola`): `QUIET`, 0 s
  stalled on IO. The test-home backing read `tmpfs` with 22.3G free after it.

## Witness 2 — the reader, the exclusion and the deadline function

- Written 2026-10-10T03:10:39Z, before the launch.
- `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --file crates/viola-e2e/src/harness/run/mutants/host.rs --file crates/viola-e2e/src/harness/run/mutants.rs --file crates/viola-e2e/src/harness/cleanup.rs`
- A scoped run over the chunk diff (verdict `scoped`): only lines the chunk changed in the three files.
- Outputs: `target/witness-wmg/w2.stdout`, `w2.stderr`, `w2.exit`.
- Looked for: zero missed, and `replace + with - in kill_deadline` read `caught` (its kill on this host).
- Next: read every survivor's log before witness 3 starts; a survivor takes a case in step 4, 5 or 6.
- Returned 2026-10-10T03:22:35Z, harness exit 0. The document (`witness-reader.json`): `ok:true`, verdict
  `scoped`, base `d778ececb7dd` (the last master flip, derived), tested 94, caught 88, survived 0, no failure
  and no `host_excluded` field: nothing was left out. cargo-mutants' own summary line: `94 mutants tested in 12m:
  88 caught, 6 unviable` (`witness-reader-outcomes.txt`, 97 lines).
- By file: `host.rs` 73 caught, 2 unviable; `mutants.rs` 14 caught, 1 unviable; `cleanup.rs` 1 caught, 3
  unviable.
- The deadline function, `cleanup.rs:130`: `replace + with - in kill_deadline` read `caught`;
  `replace + with * in kill_deadline` and `replace kill_deadline -> Instant with Default::default()` read
  `unviable` (neither compiles for an `Instant`), as measured before the plan was written. The fourth
  `cleanup.rs` line is `replace cleanup_one -> Report with Default::default()` at `:90:5`, `unviable`.
- No missed and no timed-out mutant: no case is owed to step 4, 5 or 6.
- The host over the run's window (`hostwatch.py read --from 03:10:39Z --to 03:22:35Z --for viola`): `QUIET`, 0 s
  stalled on IO. The backing read `tmpfs` with 22.3G free after it.

## Witness 3 — the strict-modes check and owner-only creation

- Written 2026-10-10T03:22:46Z, before the launch.
- `TMPDIR=../viola-mutants-scratch bash scripts/agent-run.sh run --mutants --file crates/viola-state/src/strict.rs --file crates/viola-state/src/fs.rs`
- A scoped run over the chunk diff.
- Outputs: `target/witness-wmg/w3.stdout`, `w3.stderr`, `w3.exit`.
- Looked for: zero missed after the exclusion; the five `volume_keeps_acls` mutants and the Unix `restrict`
  mutant read `caught`; the Windows-gated diff lines in `host_excluded`, each under its predicate.
- Next: step 9's records, the recipe re-read on the final tree, then the whole gate block.
- Returned 2026-10-10T03:26:22Z, harness exit 0. The document (`witness-state.json`): `ok:true`, verdict
  `scoped`, base `d778ececb7dd`, tested 22, caught 6, survived 0, no failure. cargo-mutants' own summary line:
  `22 mutants tested in 3m: 16 missed, 6 caught` (`witness-state-outcomes.txt`, 25 lines).
- The six caught, all in bodies Linux compiles: `fs.rs:19:5 replace restrict -> io::Result<()> with Ok(())`
  (the `cfg(unix)` function), and the five mutants of `volume_keeps_acls` at `strict.rs:84` (the body replaced
  by `true` and by `false`, `!=` → `==`, `&` → `|`, `&` → `^`).
- `host_excluded`: 16 rows, every one under `windows`, all in `win::` bodies: `fs.rs` `protect` (142:9),
  `set_dacl` (149:9), `dacl_of` (194:9, 198:13, 197:94); `strict.rs` `persistent_acls` (179:9 three times,
  201:15), `owner_and_dacl` (206:9 seven times). Each stands in the recipe's Linux list for the viola-state item
  under the same predicate. The recipe lists 69 for the whole item; this run mutated only the chunk's lines.
- So on Linux this run says nothing about the Windows-gated diff lines, and the document says so by name. Their
  grades are the Windows runner's.
- The host over the run's window (`hostwatch.py read --from 03:22:46Z --to 03:26:22Z --for viola`): `QUIET`, 0 s
  stalled on IO. The backing read `tmpfs` with 22.3G free after it.

## After the three

- No mutation run was started after 03:26:22Z. No source or test file changed between the first launch and the
  last return, so the three runs measured one tree.
- Each run's stderr opens with cargo's own build lines, which carry the repository's absolute path; those files
  stay under the ignored `target/witness-wmg/`. Their outcome lines are the three `*-outcomes.txt` files here.
