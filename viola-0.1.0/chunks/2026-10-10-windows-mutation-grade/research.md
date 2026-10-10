# Codebase Research — 2026-10-10-windows-mutation-grade

## Scope
- **Depth:** deep on the three crates and the workflow · **Reads:** 19 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — in full, 13 additions read (2026-09-24
  `--in-diff` never regenerates an earlier miss; 2026-09-25 host-excluded misses; 2026-09-26 judge a run by its
  verdict, never by comparing counts; 2026-10-03/04 the `TMP` / `TMPDIR` rules; 2026-10-10 cargo-mutants keeps one
  earlier run's logs) · `.claude/rules/ci.md` — 3 additions applied (a running dispatch's job log is read through
  `gh api …/jobs/<id>/logs --allow-escape-sequences`; a dispatch joins its sha's checks, so the push's CI verdict
  is read before the dispatch and the dispatch's row with `--name mutants`; `gh run list --commit` takes the full
  sha) · `.claude/rules/testing.md` — 35 additions read, eight applied (named under Conventions).
- **Platform issues consulted:** none — no CI red was folded at the take-up and the plan holds no CI-reading entry
  outside the operator leg. cargo-mutants' own behaviour was measured on the pinned 27.1.0 instead (Measured facts).
- **External inputs:** `inputs#I1` — the operator's take-up direction: zero live `claude` sessions, the Windows
  runner as the proof loop with its dispatches counted, no split and no boundary widening, technical forks to the
  operator. `inputs#I2` — the three P4 forks as the operator settled them. `inputs#I3` — the operator's P5
  review: no test sends a kill at a process it does not own; founder ruling C2 stands, so no dispatch before the
  founder's own word and no master sentence about that ruling amended.

## Files inspected
- `crates/viola-state/src/strict.rs` (full) — the check, its Windows reader and every test. The shared test
  module holds the pure-verdict cases for all five refusals; the Windows test module holds one admitted case
  (`check_stamps_of_a_home_under_the_workspace_target_passes`) and no refused one.
- `crates/viola-state/src/fs.rs` (1–520) — `restrict`, the Windows `protect` / `dacl_of`, the retry loop, the
  Windows tests of the replace helpers and of the protected DACL.
- `crates/viola-pty/src/lib.rs` (355–450, 725–830, 1000–1080) — `host_stdin`, `console::is_console`, and the test
  rig that re-spawns the test binary inside a PTY as a `reads` / `reads-win32` child reporting each read.
- `crates/viola-e2e/src/harness/run/mutants.rs` (1–283) — the mutation arm: `mutants`, `mutate`,
  `mutants_suite`, the document it builds. It reads `outcomes.json` by its four counts and `missed.txt` /
  `timeout.txt` by line; it reads no per-mutant record and holds no exclusion.
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (1–110) — `prepare` and `HOST_SCRATCH = cfg!(windows)`.
- `crates/viola-e2e/src/harness/cleanup.rs` (60–145, 286–330) and `harness/mod.rs` (196–222) — `cleanup_one`,
  its kill deadline, `ProcessId::kill` / `wait_gone`, and the deadline test, which is `cfg(target_os = "linux")`.
- `.github/workflows/windows-mutants.yml` (full, 53 lines) and `tests/contract_windows_mutation_scope.rs`
  (1–40, 150–265).
- `.config/nextest.toml` (full) — the `mutants` profile and its three overrides.
- The audit: `.andromeda/runs/2026-10-08T10-08-51-code-audit/proposals.md` (F1–F3 and the first answered CARRY),
  `cover.py` (full), `win-outcomes.json` (every row, through `.andromeda/runs/2026-10-10T02-04-59-phase/winrows.py`).
- `viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/windows-dispatch.md` (full) — the
  first dispatch's per-job table and its 34 graded coordinates.
- `git show d5deb01^:crates/viola-e2e/src/harness/cfg_legs.rs` (outline, 410 lines) — the retired cfg reader.
- `.andromeda/test-plan.md` (the Mutation gate paragraph, whole) and `scripts/code-graph.py` (100–260).

## Graph impact (rust plane; the first refresh under the script changed at `781563c`)
The refresh, `python scripts/code-graph.py refresh`, 2026-10-10T02:17:07Z → 02:17:36Z: `tree-refresh[rust]: 4373
nodes / 22801 edges - 27s` and `tree-refresh[ts]: 7 nodes / 1 edges - 0s`. Neither line carries the fallback note,
so the all-features index did not fall back. Query trace: `.andromeda/runs/2026-10-10T02-04-59-phase/tree-query-2026-10-10-windows-mutation-grade.json`.
- **`check_stamps`** — one product caller, `read_stamps_strict` @ `crates/viola-state/src/stamps.rs:78`, itself
  called by `version_gate` @ `src/run/version_gate.rs:19` and `:136`. That is the one entry point that runs the
  check today. Four test callers in `strict.rs` (`:524`, `:532`, `:543`, `:566`).
- **`check_path`** — one caller, `check_stamps` @ `strict.rs:33`.
- **`restrict`** — five call sites in `fs.rs`: `create_private_dir` (`:51`, `:59`), `open_private_append`
  (`:210`), `open_private_lock` (`:232`), `replace_private_with` (`:265`).
- **`mutants_suite`** — `run` @ `crates/viola-e2e/src/harness/run.rs:27` and `mutate` @ `run/mutants.rs:264`, plus
  four test sites. A changed signature reaches `run.rs`.
- **`cleanup_one`** — `cleanup` @ `cleanup.rs:45` and the Linux-only deadline test @ `:323`.
- **`host_stdin`** — `pump_child` @ `src/cmd/run.rs:507`, the fake agent @ `src/bin/viola-fake-agent.rs:729`, and
  the rig's `report_reads` @ `crates/viola-pty/src/lib.rs:747`.
- **`persistent_acls`, `owner_and_dacl`, `protect`, `dacl_of`, `is_console`** — 0 rows. The index is built for
  the host's cfg, so a `cfg(windows)` body is not on this plane: an index gap by gate, not a leaf. Their callers
  were read from source (`win::check` @ `strict.rs:164–165`, `protect_outside_profile` @ `fs.rs:132`, `protect` @
  `fs.rs:153`, `host_stdin` @ `lib.rs:357`).

## Measured facts
- **The workflow's mutant counts at HEAD** (`cargo mutants --list --package <p> --features fake-agent --file <f> |
  wc -l`, which builds nothing): viola-pty `lib.rs` 80, `sideload.rs` 9 · viola-state `pin.rs` 36, `fs.rs` 54,
  `strict.rs` 69 · viola `src/cmd/run.rs` 50, `src/run/env.rs` 33, `src/main.rs` 34, `src/panic_frames.rs` 23,
  `src/conpty.rs` 0 · viola-e2e `scratch.rs` 19, `cleanup.rs` 32, `browser.rs` 17. Job totals 89 · 148 · 159 · 40 ·
  140 · 68, equal to run 37761947926's `Found` lines.
- **The pinned recipe's reading at HEAD** (`.andromeda/runs/2026-10-10T02-04-59-phase/forecast.py`: `cargo mutants --list --json` per matrix item
  through `cover.py`): under `rustc --print cfg --target x86_64-pc-windows-msvc` it leaves out 29 of 644 —
  viola-pty 5, viola-channel 4, viola-state 6 (all `strict.rs`), viola 13 (`src/panic_frames.rs` 9 under
  `cfg(unix)`, `src/cmd/run.rs` 4 under `cfg(all(windows, not(target_arch = "x86_64")))`), viola-e2e 1
  (`cleanup.rs`, `cfg(unix)`), viola-agent-claude 0. That is the CARRY's 24 + 4 and the one viola-e2e twin. Under
  the Linux host's own cfg the same recipe leaves out 160.
- **cargo-mutants 27.1.0 on three restatements** (`.andromeda/runs/2026-10-10T02-04-59-phase/list-probe-lib.rs`, `cargo mutants --list` in a
  scratch crate): a function whose whole body is `Ok(())` gets no `Ok(())` replacement and so no mutant; the same
  stub written as a `cfg(not(unix))` block inside a shared function gets one; a flag written `0x1 | 0x4` gets
  `|` → `&` and `|` → `^`, and the literal `0x5` gets none; a plain `flags & 0x8 != 0` function gets five
  mutants, all reachable by a test on every OS.
- **Windows-gated code type-checks on this host.** `CARGO_TARGET_DIR=target/wincheck cargo check -p viola-state -p
  viola-pty -p viola-e2e --tests --target x86_64-pc-windows-msvc` exits 0 in 3.3 s warm, and `cargo clippy -p
  viola-state -p viola-pty --tests --target x86_64-pc-windows-msvc -- -D warnings` exits 0 in 0.7 s
  (`rustup target list --installed` names the target). Nothing here runs a Windows test.
- **The two earlier dispatches** (`gh run view <id> --json jobs`):
  - run 37174673472 on `60c569b`: pty 10 m 38 s, channel 9 m 47 s, state 9 m 15 s, agent-claude 5 m 08 s, viola
    26 m 49 s, viola-e2e 16 m 12 s. Wall 1609 s.
  - run 37761947926 on `e304994`: pty 12 m 32 s, channel 9 m 21 s, state 15 m 37 s, agent-claude 6 m 21 s, viola
    120 m 27 s (cancelled), viola-e2e 4 m 26 s (baseline failure).
- **The `viola` job, by file** (`.andromeda/runs/2026-10-10T02-04-59-phase/winrows.py` over `win-outcomes.json`): of the 125 graded, `run.rs` 50,
  `main.rs` 34, `panic_frames.rs` 23, `env.rs` 18. The 15 never graded are all `src/run/env.rs` (33 listed, 18
  graded: 9 caught, 9 unviable). 125 mutants in a 7227 s job is about 54 s a mutant once roughly 7 minutes of
  setup, prebuild and baseline are taken off (an estimate: the fixed part was not read from the log).
- **`ci.yml`'s wall** on the last two pushes: 380 s and 460 s (Setup's read; the cleanup chunk's plan).

## The sixteen survivors, site by site
The fifteen of the CARRY, and one more the first dispatch recorded. Each row states what a test must distinguish.
- `strict.rs:30:5` (`check_stamps → Ok(())`) and `:34:23` (the `NotFound` guard → `true`): the shared test
  `check_stamps_of_a_path_that_cannot_be_statted_is_unreadable` (added after the audit) expects
  `Err(Refused::Unreadable)` for a path holding a NUL. Under either mutant the call returns `Ok(())`, so the test
  fails on any host where the stat of that path fails with a kind other than `NotFound`. On Linux both grade
  caught (the cleanup chunk's record). On Windows std refuses a NUL in a path before any system call, with
  `InvalidInput`; that reading is from std's documented behaviour, not from a Windows run.
- `strict.rs:43:5` (`check_path → Ok(())`) and `:163:9` (`win::check → Ok(())`): no Windows test refuses a real
  path. The verified half of the audit's claim. A killing test needs a real folder or file whose DACL grants a
  write right to a foreign SID, checked through `check_stamps` or `check_path`.
- `strict.rs:170:9` (`win::persistent_acls → Some(true)`): on an NTFS path the function returns `Some(true)`, so
  only an input for which it returns `None` separates them: a path whose volume cannot be read. The pure verdict
  for a volume without persistent ACLs is already tested on every OS (`windows_verdict(.., false)`).
- `strict.rs:192:37` (`&` → `|`, `&` → `^`): `flags & PERSISTENT_ACLS != 0`. A hosted runner has no volume with
  the bit clear, so the reader cannot separate them. The decision as a plain function of `flags` can, on every OS
  (the scratch probe's five mutants).
- `strict.rs:207:44` and `fs.rs:161:47` (`|` → `^` between two distinct flag constants): bit-identical values,
  so no test can fail. `testing.md` 2026-09-27 names the remedy: the combined value as one literal, pinned by a
  test against the named flags, as `WRITE_RIGHTS` already is (`strict.rs:69`, pinned at `:314`).
- `fs.rs:186:13` and `:187:13` (`||` → `&&` in `win::dacl_of`): the guard is `call == 0 || present == 0 ||
  dacl.is_null()`. A descriptor with a NULL DACL (`present` set, the pointer null) separates the second mutant.
  The first, `(call == 0 && present == 0) || dacl.is_null()`, differs from the original only where `present` is 0
  and the pointer is not null, which the API never returns for a pointer initialised null: `present == 0`
  repeats what `dacl.is_null()` already reads (`testing.md` 2026-09-24, extended 2026-09-27).
- `fs.rs:18:5` (`restrict → Ok(())`): on a non-Unix build the function's body is `let _ = (path, mode); Ok(())`,
  so the mutant is the function. No test can fail and the run's verdict has no arm for an argued mutant.
- `lib.rs:434:9` (`is_console → true`, `→ false`) and `:436:76` (`!=` → `==`): `host_stdin` (`:355–360`) takes
  viola's `ReadConsoleW` reader on a console stdin and `std::io::stdin()` otherwise. The rig's two `reads` cases
  run with a console stdin and pass under `→ false`, because the bytes they send read the same through either
  reader. What differs: std's console reader ends a read at Ctrl-Z and viola's keeps it
  (`console_input_keeps_every_ctrl_z`, on a scripted source), and `ReadConsoleW` fails on a handle that is not a
  console.
- `crates/viola-e2e/src/harness/cleanup.rs:106:35` (`+` → `-` on the kill deadline): **graded MISSED on Windows**
  at run 37174673472 (`windows-dispatch.md`, "A shared-body Windows survivor"). Its killing test
  `cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill` is `cfg(target_os = "linux")`: it uses PID 1,
  which a kill cannot end. Read again at P5 on the operator's direction (`inputs#I3`: no test sends a kill at a
  process it does not own):
  - a process the test starts cannot stand in. `ProcessId::of` (`harness/mod.rs:185–195`) returns `None` for a
    zombie, so a killed Unix child reads gone at once; and nothing in user mode holds a terminated Windows
    process in the process table for a time the test controls. A case that only samples the moment between the
    kill and the first poll is a race (`testing.md` 2026-09-27);
  - the expression can be restated so a test on every OS reads it. Measured on cargo-mutants 27.1.0
    (`.andromeda/runs/2026-10-10T02-04-59-phase/list-probe-deadline.rs.txt`, `cargo mutants --list`): the
    deadline as a plain function of an instant gets `+` → `-`, `+` → `*` and a `Default::default()` body. The
    first stays killable by a case asserting the returned instant lies five seconds after the given one; the
    other two do not compile for `Instant`. The remedy is `testing.md` 2026-09-24's (a decision in a plain
    function tested on every OS); that rule's letter names an OS-gated function, and here the OS-gated thing is
    the only killing test.

## Patterns detected
- **Decisions as plain functions, readers per OS** (`strict.rs:1–5`, `:55`, `:90`): the module's own stated shape.
- **A combined flag as one pinned literal** (`strict.rs:69`, its pin at `:314–329`).
- **A Windows test home under the workspace's `target/e2e-home`** (`strict.rs:377–397`, `fs.rs:485–519`): never
  `%TEMP%`, so the protected DACL is set at creation and the CI scans cover it.
- **The DACL read back, not the SDDL written** (`fs.rs:499–512`): the assertion is over `strict::win::allows`'
  SIDs, masks and flags.
- **A host-keyed arm as a compile-time const** (`scratch.rs:11`, `fs.rs:243`), each pinned by a test of the const.
- **The harness's tool calls go through the `Runner` seam** (`run.rs:34`,
  `dyn FnMut(&mut Command) -> (Option<i32>, String)`), so a test passes a stand-in and never nests a tool.
- **The retired cfg reader** (`cfg_legs.rs` at `d5deb01^`, 410 lines with its tests): syn 2.0.119 and proc-macro2
  1.0.107 with `span-locations`, a visitor collecting the `cfg` attributes covering a line, a three-valued `eval`
  where only a predicate proven false drops a leg. Both crates are still in `Cargo.lock` at those versions, as
  transitive dependencies.

## Conventions to follow
- `testing.md` 2026-09-27: a crate's security property takes a test inside that crate; cargo-mutants runs only
  the mutated package's tests.
- `testing.md` 2026-09-24 (the awaited line; the mutant-reachable wait): a new test's wait detects the watched
  process's exit and stays under the kill line — 10 s for a root or crate test under the `mutants` profile, 30 s
  for `viola-e2e` (`.config/nextest.toml:39–55`).
- `testing.md` 2026-09-28: no body is reshaped to leave cargo-mutants' generated set. The three restatements
  above are of another kind: each removes an operand or a body no test can observe, by a rule the same file names.
- `testing.md` 2026-10-03: an item used only in `cfg(windows)` test code is imported inside that item.
- `testing.md` 2026-09-25: every new guard test carries its remove-the-guard run. For a Windows-gated test that
  run is the dispatch itself: the mutant is the guard neutralised and `caught` is the test red.
- `testing.md` 2026-10-04: no `[[gate]]` entry is a mutation run; a scoring or kill run is a one-off witness with
  its counts in `evidence/`.
- test-plan §4 (the `viola-e2e` exception): the harness's unit cases are a labelled case table in one `#[test]`.
- architecture, Occupied Resources: a harness document carries counts and repo-relative names only.

## New files to create
- `crates/viola-e2e/src/harness/run/mutants/host.rs` — the host exclusion and its case table
- `viola-0.1.0/chunks/2026-10-10-windows-mutation-grade/evidence/` — the survivor record, the host-excluded record, the Linux witness, the dispatch reading, the operator pass

## Files to modify
- `crates/viola-state/src/strict.rs` — the Windows refusal cases, the unreadable-volume case, the bit decision as a plain function, the combined read flags as one pinned literal
- `crates/viola-state/src/fs.rs` — `restrict` as two cfg-gated functions, the combined set flags as one pinned literal, the DACL setter taking its SDDL, `dacl_of`'s repeated operand and its no-DACL case
- `crates/viola-pty/src/lib.rs` — the rig's two new child cases
- `crates/viola-e2e/src/harness/cleanup.rs` — the kill deadline as a plain function of an instant, and its case on every OS
- `crates/viola-e2e/src/harness/run/mutants.rs` — the exclusion applied before the counts are read, the document's new field
- `crates/viola-e2e/src/harness/run.rs` — the caller of `mutants_suite`, if its signature moves
- `crates/viola-e2e/Cargo.toml` — syn and proc-macro2, and the unused-dependency note for proc-macro2
- `Cargo.toml` — the two `[workspace.dependencies]` lines
- `Cargo.lock` — viola-e2e's two dependency lines
- `.github/workflows/windows-mutants.yml` — the `viola` item split in four, a label per item for the job name
- `tests/contract_windows_mutation_scope.rs` — the job names stay distinct

## Open questions
- none — the three plan-decision forks P3 raised were settled by the operator at P4 (`inputs#I2`): the `viola` job
  is split per file; `restrict` becomes two cfg-gated functions; the retired syn reader is revived. The retired
  reader's workspace lines, for the revival (`git show d5deb01^:Cargo.toml`): `proc-macro2 = { version =
  "=1.0.107", features = ["span-locations"] }` and `syn = { version = "=2.0.119", default-features = false,
  features = ["full", "parsing", "printing", "visit"] }`. It knew a target's family and OS only: its `eval` reads
  `target_arch` as unknown, so the four `src/cmd/run.rs:385:5` mutants would stay in the count unless the host's
  architecture is added. It judged one line, not a span, and read no `mod` declaration in a parent file.
