# The sixteen Windows-side survivors, and the mutants recorded on the host that takes the arm

Written at step 9, 2026-10-10, on the tree the three witness runs measured (HEAD `781563cd59a6` plus the chunk's
edits). At step 9 no Windows grade in this file was taken by this chunk: step 11 held the dispatch for the
founder's word (`inputs#I3`, `inputs#I4`), and every "Windows, this chunk" cell read "not graded: held by step 11".

**The Windows cells were filled on 2026-10-10 after the dispatch the founder allowed** (`inputs#I5`): run
38036448183 on `dd5161d55743`, attempt 1, nine jobs green (`windows-dispatch.md`). Each grade below is that job's
own outcome line for the mutant at its "now" coordinate. Of the sixteen, twelve read caught and four are no longer
generated; none reads missed, timeout or unviable.

What the chunk did measure, and where:

- on the Linux dev host, three mutation runs (`witness-runs.md`) and `cargo mutants --list` on cargo-mutants
  27.1.0 over the final tree (the "gone" readings);
- for the Windows target on the Linux host, only that the gated test code compiles and lints
  (`cargo clippy … --target x86_64-pc-windows-msvc`, gate entry 3). That proves a gated test builds, not that it
  passes or that it kills anything;
- the first run of the Windows-gated tests is the ordinary CI read of step 10 (`operator-pass.md`).

Coordinates: "was" is the coordinate at the audit's commit `e304994` (run 37761947926), equal at `781563c`;
"now" is the coordinate on the final tree, from `cargo mutants --list`.

## The sixteen

Fifteen were graded MISSED on `windows-2025` at run 37761947926 (`e304994`); `cleanup.rs:106:35` was graded MISSED
there at run 37174673472 (`60c569b`).

### Nine closed by a test that runs on `windows-2025`, at their own site

| # | was | now | mutation | the test that fails under it | Linux reading | Windows, this chunk |
|---|---|---|---|---|---|---|
| 1 | `strict.rs:30:5` | same | `check_stamps → Ok(())` | `strict::tests::check_stamps_of_a_path_that_cannot_be_statted_is_unreadable` (every OS, added by the chunk before this one); second kills, Windows only: `strict::win::tests::check_stamps_refuses_a_ledger_folder_everyone_may_write` and `…_a_stamps_file_everyone_may_write` | caught (the previous chunk's run, `witness-viola-state-outcomes.txt` there); not regenerated here, its line is outside this chunk's diff | caught (run 38036448183, `mutants (viola-state)`) |
| 2 | `strict.rs:34:23` | same | the `NotFound` guard → `true` | `strict::tests::check_stamps_of_a_path_that_cannot_be_statted_is_unreadable`; no new case, as planned | caught (the same earlier run) | caught (run 38036448183, `mutants (viola-state)`) |
| 3 | `strict.rs:43:5` | same | `check_path → Ok(())` | `strict::win::tests::check_stamps_refuses_a_ledger_folder_everyone_may_write`, `…_a_stamps_file_everyone_may_write` (each asserts `check_path` on the widened object) | caught (the same earlier run, by the Unix mode cases) | caught (run 38036448183, `mutants (viola-state)`) |
| 4 | `strict.rs:163:9` | `172:9` | `win::check → Ok(())` | the same two, and `strict::win::tests::persistent_acls_of_a_volume_that_cannot_be_read_is_none` (it asserts `check` reads `Unreadable`) | not compiled on Linux: the recipe leaves it out under `windows` (`host-excluded.md`) | caught (run 38036448183, `mutants (viola-state)`) |
| 5 | `strict.rs:170:9` | `179:9` | `win::persistent_acls → Some(true)` | `strict::win::tests::persistent_acls_of_a_volume_that_cannot_be_read_is_none` | not compiled on Linux: left out under `windows` (`witness-state.json`, the row at `179:9`) | caught (run 38036448183, `mutants (viola-state)`) |
| 6 | `fs.rs:187:13` | `198:13` | `\|\|` → `&&` in `win::dacl_of` (the operand before `dacl.is_null()`) | `fs::tests::set_dacl_refuses_an_sddl_that_carries_no_dacl` | not compiled on Linux: left out under `windows` (`witness-state.json`, the row at `198:13`) | caught (run 38036448183, `mutants (viola-state)`) |
| 7 | `lib.rs:434:9` (viola-pty) | same | `console::is_console → true` | `tests::piped_stdin_is_read_as_the_bytes_written_to_it` | not compiled on Linux: left out under `windows` (`linux-witness.json`) | caught (run 38036448183, `mutants (viola-pty)`) |
| 8 | `lib.rs:434:9` | same | `console::is_console → false` | `tests::console_read_keeps_a_ctrl_z_and_goes_on_to_the_next_read` | the same | caught (run 38036448183, `mutants (viola-pty)`) |
| 9 | `lib.rs:436:76` | same | `!=` → `==` in `console::is_console` | both of the two above (a console then reads as none, a pipe as one) | the same | caught (run 38036448183, `mutants (viola-pty)`) |

The two viola-pty cases run on every OS and pass on Linux (the default selection, gate entry 5); there they
exercise the rig, not `is_console`. What each rests on, none measured on a Windows host by this chunk:

- a test that widens a DACL and expects `Writable`: `SetNamedSecurityInfoW` takes the SDDL
  `D:P(A;…;FA;;;<user>)(A;…;FA;;;SY)(A;;0x2;;;WD)`, and the reader then finds an allow ACE for Everyone
  (`S-1-1-0`) with `FILE_WRITE_DATA`;
- `persistent_acls` of a drive letter with no volume is `None` (the test takes the highest letter from `Z` down
  to `D` whose root does not exist);
- a descriptor converted from `O:SY` carries no DACL, and `GetSecurityDescriptorDacl` then leaves the DACL
  pointer as it was set, null;
- a lone `^Z` written into the rig's ConPTY reaches the raw child as one read of `1a` through viola's reader,
  and std's console reader would have ended the input there;
- `ReadConsoleW` fails on a pipe handle.

Each is asserted by its own test, so `test (windows-2025)` reads it at step 10. A test that passes there shows
the unmutated code behaves so; that it fails under its mutant is the dispatch's reading.

Added after step 10 (2026-10-10T03:45Z): every test the three tables of the sixteen name passed in `test (windows-2025)` of
`ci#38021000200` on `dd5161d55743`, attempt 1, each read as its own `PASS` line (`operator-pass.md`). So the five
readings above hold on that runner for the unmutated code. No row's "Windows, this chunk" cell changes: a pass of
the test is not a grade of the mutant.

### Three closed by a decision moved into a plain function, tested on every OS

| # | was | now | mutation | restatement | the test | Linux reading | Windows, this chunk |
|---|---|---|---|---|---|---|---|
| 10 | `strict.rs:192:37` | `84:11` | `&` → `\|` in the persistent-ACL bit | `win::persistent_acls` calls `volume_keeps_acls(flags)`, a plain function of the volume flags | `strict::tests::volume_keeps_acls_reads_the_persistent_acls_bit` (five labelled cases) | caught (`witness-state-outcomes.txt`) | caught (run 38036448183, `mutants (viola-state)`) |
| 11 | `strict.rs:192:37` | `84:11` | `&` → `^` | the same | the same | caught (the same file) | caught (run 38036448183, `mutants (viola-state)`) |
| 12 | `cleanup.rs:106:35` (viola-e2e) | `130:9` | `+` → `-` on the kill deadline | `cleanup_one` calls `kill_deadline(Instant::now())`, a plain function of an instant | `harness::cleanup::tests::kill_deadline_lies_five_seconds_after_the_instant_it_is_given` (it starts no process and sends no kill) | caught (`witness-reader-outcomes.txt`) | caught (run 38036448183, `mutants (viola-e2e)`) |

The function `volume_keeps_acls` has five mutants on this tree and all five read caught on Linux. `kill_deadline`
has three: `+` → `-` caught, `+` → `*` and the `Default::default()` body unviable (`witness-runs.md`). The Linux
case `cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill` was not edited (`inputs#I3`).

### Four that leave the generated set, behaviour unchanged

For each, the Linux reading is `cargo mutants --list --package viola-state --features fake-agent --file
crates/viola-state/src/fs.rs --file crates/viola-state/src/strict.rs` on the final tree (120 lines): the mutant
is not generated.

| # | was | mutation | restatement | the equality it rests on | pinned by | Windows, this chunk |
|---|---|---|---|---|---|---|
| 13 | `strict.rs:207:44` | `\|` → `^` in `win::owner_and_dacl` | the flags are the one literal `OWNER_AND_DACL = 0x5` | `OWNER_SECURITY_INFORMATION` (0x1) and `DACL_SECURITY_INFORMATION` (0x4) share no bit, so `\|` and `^` give the same value | `strict::win::tests::owner_and_dacl_is_the_named_flags` (Windows only) | not generated: the job lists no such mutant (run 38036448183, `mutants (viola-state)`) |
| 14 | `fs.rs:161:47` | `\|` → `^` in `win::protect` | the flags are the one literal `PROTECTED_DACL = 0x8000_0004`, in `win::set_dacl` | `DACL_SECURITY_INFORMATION` (0x4) and `PROTECTED_DACL_SECURITY_INFORMATION` (0x8000_0000) share no bit | `fs::tests::protected_dacl_is_the_named_flags` (Windows only) | not generated: the job lists no such mutant (run 38036448183, `mutants (viola-state)`) |
| 15 | `fs.rs:186:13` | `\|\|` → `&&` in `win::dacl_of` (the operand before `present == 0`) | `present == 0` is dropped; the guard is `call == 0 \|\| dacl.is_null()` | the pointer is set null before the call, and a descriptor with no DACL leaves it so: `present == 0` repeated what `dacl.is_null()` reads | `fs::tests::set_dacl_refuses_an_sddl_that_carries_no_dacl` asserts the refusal for such a descriptor (Windows only) | not generated: the job lists no such mutant (run 38036448183, `mutants (viola-state)`) |
| 16 | `fs.rs:18:5` | `restrict → Ok(())` on a non-Unix build | `restrict` is two functions: a `cfg(unix)` one that sets the mode, and a `cfg(not(unix))` one whose whole body is `Ok(())` | a body that is only `Ok(())` equals its own replacement, and cargo-mutants 27.1.0 generates no mutant for it (measured at P3 in a scratch crate; here the list holds one `restrict` line, the Unix function's, at `19:5`) | the five call sites are unchanged; the Unix function's mutant read caught on Linux (`witness-state-outcomes.txt`) | not generated: the job lists no such mutant (run 38036448183, `mutants (viola-state)`); the Unix function's `fs.rs:19:5` is left out there under `unix` |

Rows 13 and 14 rest on arithmetic. Row 15 rests on a behaviour of `GetSecurityDescriptorDacl` this chunk did not
measure on Windows; its test is the measurement, at step 10. Row 16's Unix function is left out on a Windows host
by the exclusion (the recipe lists `fs.rs:19:5` under `unix` for that host, `host-excluded.md`).

None of `check_path`, `check_stamps`, `win::check` or `win::persistent_acls` is closed by an argument alone: rows
1 to 5 each name a test.

## Mutants recorded on the host that takes the arm

No new test is owed for these (scope §2, §5). Each was graded on `windows-2025` by an earlier dispatch; the
column "a later dispatch" is empty until one reads them again.

| was | now | mutation | Linux dev host | `windows-2025` | measured by | a later dispatch |
|---|---|---|---|---|---|---|
| `fs.rs:270:18` | `281:18` | `+=` → `*=` in `replace_private_with` | missed; not measured here: the retry arm is behind `HOST_IS_WINDOWS = cfg!(windows)` | caught | run 37761947926, `e304994` (`win-outcomes.json`, the audit's run dir) | caught, 10 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:274:20` | `285:20` | the retry guard → `false` | the same | caught | the same | caught, 10 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:275:21` | `286:21` | `&&` → `\|\|` | the same | caught | the same | caught, 10 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:274:29` | `285:29` | `<` → `==` | the same | caught | the same | caught, 0 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:274:29` | `285:29` | `<` → `>` | the same | caught | the same | caught, 0 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:274:29` | `285:29` | `<` → `<=` | the same | caught | the same | caught, 3 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:290:19` | `301:19` | the content guard → `true` in `replace_private_shared` | caught (the previous chunk's record) | caught | run 37761947926, `e304994` | caught, 4 s test (run 38036448183, `mutants (viola-state)`) |
| `fs.rs:290:19` | `301:19` | the content guard → `false` | caught (the same record) | caught | the same | caught, 4 s test (run 38036448183, `mutants (viola-state)`) |
| `scratch.rs:48:5` (viola-e2e) | same | `prepare → Ok(None)` | missed; not measured here: `HOST_SCRATCH = cfg!(windows)` | caught | run 37174673472, `60c569b` (`windows-dispatch.md` of chunk 2026-10-04-windows-boundary-mutation-workflow) | caught, 4 s test (run 38036448183, `mutants (viola-e2e)`) |
| `scratch.rs:54:8` | same | delete `!` in `prepare` | the same | caught | the same | caught, 4 s test (run 38036448183, `mutants (viola-e2e)`) |

The Windows grades above are of the source at those two commits. `fs.rs` moved in this chunk (its product lines
too: `restrict`, `set_dacl`, `dacl_of`), so the six retry-loop mutants and the two guard mutants have no Windows
grade at this source; the retry loop and `replace_private_shared` themselves are not edited. `scratch.rs` is
byte-identical to `60c569b` (the preservation guard, gate entry 12, names it).

Added after the dispatch (2026-10-10T08:51Z): run 38036448183 graded all ten at this source, and all ten read
caught (the last column). Three of the retry-loop mutants read caught after 10 s of test time, where the other
seven took 0 to 4 s. The job log carries the grade and the times only, so whether an assertion failed or a test
was ended at its 10 s kill line was not read for those three.

None of these is recorded as equivalent, and none as caught on a host that did not take its arm.
