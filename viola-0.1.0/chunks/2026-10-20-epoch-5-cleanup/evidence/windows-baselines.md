# Item (3): the Windows mutation leg's three failed baselines (plan steps 6, 7, 8)

The chunk runs no mutation, witness or dispatch (inputs#I1 §1). Each Windows reading below is either a recorded run
read by its id, or owed by name to the next boundary's Windows leg.

## (3a) `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` (step 6)

- The failed reading: run 39326811932, `mutants (viola-e2e)`'s baseline. The plain control read `Ok(())` against
  `Err(Writable)`.
- The change: the plain sibling is now granted `BUILTIN\Users` write explicitly. That is a direct spawn of
  `%SystemRoot%\System32\icacls.exe <plain> /grant *S-1-5-32-545:(OI)(CI)(W)`, its exit asserted 0, so it reads
  `Err(Writable)` whatever its parent's DACL. The scratch side and both assertions are unchanged.
- Here: the Windows-target clippy compiles it (block entry 4, green). CI's `test (windows-2025)` leg is read in the
  operator pass (`evidence/operator-pass.md`).
- **Owed to the next boundary's Windows leg:** the case green inside cargo-mutants' copy in the host scratch
  (`mutants (viola-e2e)`'s baseline). The hypothesis it rests on (the copy is owner-only, so the old control inherited
  nothing to refuse) is unmeasured.

## (3b) `path6_the_boundaries_and_the_window_word` (step 8): recorded, not changed

| reading | profile | kill | wall |
|---|---|---|---|
| CI `test (windows-2025)`, run 39383206773 (`0807ee3c`), its JUnit | `ci` | 20 s | 5.502 s |
| the dispatch, run 39326811932, `mutants (viola-cmd-verify-stop)`'s baseline | `mutants` | 10 s | killed at 10 s |
| this host, the block's entry 14 (`NEXTEST_PROFILE=mutants`, the chunk's tree) | `mutants` | 10 s | 5.807 s |
| this host, P5's baseline of the same entry (the untouched tree) | `mutants` | 10 s | 5.955 s |

- The difference between CI's 5.5 s and the dispatch's kill is not measured. Measuring it on the runner that failed
  is a dispatch, which the chunk makes none of (testing.md 2026-09-28; inputs#I1 §1). No kill line moves for it.
- **Owed:** the next boundary's Windows leg reads `path6_the_boundaries_and_the_window_word` inside its kill, or names
  its phases.

## (3c) `mutants (viola-pty)`: 78 unviable against 11 caught (step 7)

- The failed reading: run 39326811932, job 118314687311. Every mutant from 21:27:00 on was `unviable` in a 0–4 s
  build, right after the three `try_wait` mutants were caught (log lines 439–442).
- The change: the test guard kills unconditionally, and the PTY children run a per-test copy of the test binary
  (`evidence/viola-pty-leak.md`).
- Here: the remove-the-guard reading cannot separate the guards on Linux, where the master's close hangs up the child
  (`evidence/viola-pty-leak.md`). Every `viola-pty` case is green here (block entries 7 and 23). The three CI legs
  are read in the operator pass.
- **Owed to the next boundary's Windows leg:** `unviable` ≤ `caught` in `mutants (viola-pty)`.
