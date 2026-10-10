# Scope — 2026-10-10-windows-mutation-grade · Windows mutation grade

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, under
`### Epoch 4 — Session state & governance`): Windows mutation grade — the strict-modes and DACL checks pinned by
Windows-side refusal tests, host-excluded twins left out of the count, every job inside its ceiling, workflow green —
plus five CARRY blocks (all five folded below; completeness check `route.py pins` → the run dir's trail: five rows on
the entry, 1270 · 981 · 1581 · 732 · 549 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- **no live `claude` session**: the plan holds zero;
- **the proof loop is the Windows runner**: the plan names how many `windows-mutants.yml` dispatches it needs, their
  wall time, and what is provable on the Linux dev host before the first one;
- a gate entry that only reads a record takes no `artifact` key, and a preservation guard lists no file a step edits;
- the chunk is sized against one builder window;
- **the founder is away**: a split or a boundary widening would be held, not answered, so the plan needs neither;
  technical forks go to the operator.

**Settled at P4** (`inputs#I2`, the operator's answers in the fork dialog, verbatim in the snapshot):
- **the `viola` job is split per file**: four jobs under the package — `src/cmd/run.rs`; `src/main.rs` with
  `src/conpty.rs`; `src/run/env.rs`; `src/panic_frames.rs` — and the 120-minute ceiling stays;
- **`restrict` becomes two cfg-gated functions**, the non-Unix one a bare `Ok(())`. The `testing.md` line it
  reverses (2026-09-24, a trivial other-OS stub inside one shared function) is amended at the wrap with this case
  named as its exception and the measured reason;
- **the retired syn reader is revived** for the host exclusion, and viola-e2e takes back syn and proc-macro2.

**Settled at P5** (`inputs#I3`, the operator's review, verbatim in the snapshot):
- **no test sends a kill at a process it does not own**: the kill-deadline mutant (`cleanup.rs:106:35`) is
  separated by a process the test starts itself, or restated by a rule `testing.md` names, or recorded as not
  closed. Research found no own-process form and one restatement (`research.md`, the sixteen survivors);
- **founder ruling C2 stands until his own word**: the mutation workflow is dispatched only at the epoch boundary.
  The plan does steps 1 to 9, the Linux witnesses, the push and the ordinary CI read, and then a stop rule holds
  the dispatch and the entry that reads it. No master sentence about that ruling is amended without his word.

**Where the coordinates stand.** Every coordinate the five CARRYs name was re-read against the tree at `781563c`
(P1): `crates/viola-state/src/strict.rs` `30:5` · `34:23` · `43:5` · `163:9` · `170:9` · `192:37` · `207:44`;
`crates/viola-state/src/fs.rs` `18:5` · `161:47` · `186:13` · `187:13` · `270:18` · `274:20` · `274:29` · `275:21` ·
`290:19`; `crates/viola-pty/src/lib.rs` `434:9` · `436:76`; `src/cmd/run.rs` `385:5`;
`crates/viola-e2e/src/harness/cleanup.rs` `106:35` · `140:9`;
`crates/viola-e2e/src/harness/run/mutants/scratch.rs` `48:5` · `54:8`. Each names the token the CARRY says it names.
Since the audit's commit `e304994` three of the named files moved (`git diff --numstat e304994 HEAD`):
`strict.rs` +10, `fs.rs` +23, `.config/nextest.toml` +8 −6; the others are byte-identical. The audit's run dir
(`.andromeda/runs/2026-10-08T10-08-51-code-audit/`) holds `proposals.md`, `cover.py` and `win-outcomes.json`.

**CI read at Setup** (`ci.py conclusion`, the last wrap's flip through HEAD):
- `d778ece` (the wrap commit): `verdict: green` · checks 15/15 · wall 380 s · `ci#38014916900`.
- `781563c` (the setup upgrade, U03): `in progress` · `ci#38015503374` — **verdict not yet available**; not read as
  green. It carries no red to disposition. Read again at P5, 2026-10-10T02:34Z: `verdict: green` · checks 15/15 ·
  wall 431 s, the same run.

## What this chunk builds

### 1. The fifteen Windows-side survivors: a refusal test or a recorded equivalence argument each
Source: CARRY 1 (the Epoch 3 boundary audit, F3 and the table under its first answered route CARRY;
security-relevant). Run 37761947926 of `windows-mutants.yml` on `e304994` graded these MISSED in code the
`windows-2025` runner builds — 11 in Windows-only bodies, 4 in shared bodies:

| site | mutation | body |
|---|---|---|
| `strict.rs:30:5` | `check_stamps → Ok(())` | shared; caught on Linux |
| `strict.rs:34:23` | `check_stamps`' `NotFound` guard → `true` | shared; missed on both hosts at the audit |
| `strict.rs:43:5` | `check_path → Ok(())` | shared; caught on Linux |
| `strict.rs:163:9` | `win::check → Ok(())` | Windows-only |
| `strict.rs:170:9` | `win::persistent_acls → Some(true)` | Windows-only |
| `strict.rs:192:37` | `&` → `\|`, and `&` → `^`, in `win::persistent_acls` | Windows-only (two mutants) |
| `strict.rs:207:44` | `\|` → `^` in `win::owner_and_dacl` | Windows-only |
| `fs.rs:18:5` | `restrict → Ok(())` | shared; caught on Linux |
| `fs.rs:161:47` | `\|` → `^` in `win::protect` | Windows-only |
| `fs.rs:186:13`, `:187:13` | `\|\|` → `&&` in `win::dacl_of` | Windows-only (two mutants) |
| `lib.rs:434:9` (viola-pty) | `console::is_console → true`, `→ false` | Windows-only (two mutants) |
| `lib.rs:436:76` (viola-pty) | `!=` → `==` in `console::is_console` | Windows-only |

- Each takes a Windows-side refusal test or a recorded equivalence argument (the entry's words). The strict-modes
  entry points come first: a test that runs on the `windows-2025` runner fails when the check answers `Ok`.
- `[premise-corrected: the run's verdict has no arm for an argued mutant — test-plan §3 `run` step 4 reads zero
  missed from the run's own outcomes, and a mutant the host compiles and grades missed keeps the job red whatever
  `evidence/` says]` An equivalence argument alone closes none of them. A site no test can separate leaves the
  generated set by a behaviour-preserving restatement the project's rules name, with its argument recorded beside it.
- `[premise-corrected: `strict.rs` gained the shared case
  `check_stamps_of_a_path_that_cannot_be_statted_is_unreadable` after the audit; under `check_stamps → Ok(())` it
  fails on any host]` The CARRY's mechanism claim ("No test that runs there fails when the strict-modes check
  answers `Ok`") holds at HEAD for `check_path` (`43:5`) and `win::check` (`163:9`): no Windows test refuses a real
  path. It no longer holds for `check_stamps` (`30:5`, `34:23`): that shared case kills both on Linux, and on a
  Windows build it is expected to (std refuses a NUL in a path before any system call) but has not been graded
  there.
- `[premise-corrected: research.md, the sixteen survivors]` Five of the fifteen cannot be separated by any test on
  a Windows build, not three: `fs.rs:161:47` and `strict.rs:207:44` (`|` → `^` between two distinct flags, a
  bit-identical value), `fs.rs:18:5` (`restrict`'s whole body under `cfg(not(unix))` is `Ok(())`), `fs.rs:186:13`
  (it differs from the original only for a descriptor the API never returns), and the two at `strict.rs:192:37`
  through the reader (no volume on a hosted runner has the bit clear). `testing.md` names the remedy for each
  kind: a combined flag as one pinned literal (2026-09-27), the decision as a plain function tested on every OS
  (2026-09-24), a repeated guard removed (2026-09-27). `fs.rs:18:5` has two possible restatements and is a fork.
- `[premise-corrected: `testing.md` 2026-09-28 bars only a reshape that takes a body out of cargo-mutants'
  generated set to meet the count]` No product behaviour changes. A site is closed by a test where one can fail,
  and otherwise by one of the restatements above; no function is reshaped to hide a mutant a test could kill.
- A sixteenth survivor stands beside the fifteen: `crates/viola-e2e/src/harness/cleanup.rs:106:35` (§5).

### 2. The six retry-loop mutants: their record as measured on the host that takes the arm
Source: CARRY 2 (the same audit's Linux survivor table, split by host on the operator's answer).
- `fs.rs:270:18` (`+=` → `*=`), `:274:20` (the match guard → `false`), `:275:21` (`&&` → `||`), `:274:29` three
  times (`<` → `==`, `>`, `<=`), all in `replace_private_with`'s retry loop: missed on the dev host, where
  `HOST_IS_WINDOWS = cfg!(windows)` is false, and caught on the `windows-2025` runner at run 37761947926
  (`win-outcomes.json` in the audit's run dir).
- No new test is owed. Owed is their record as measured on the host that takes the arm (test-plan §10, the Mutation
  gate paragraph: a mutant behind a `cfg!(windows)` const is recorded not measured on the other host and owed to
  the route entry that measures it).

### 3. Host-excluded mutants left out before they count missed
Source: CARRY 3 (audit F2 and its answer to the first audit CARRY).
- Run 37761947926 graded 43 missed: 24 `cfg(unix)` twins the Windows build excludes (viola-pty 5, viola-channel 4,
  viola-state 6, `src/panic_frames.rs` 9; on Linux 21 caught, 3 unviable, none missed), 4 `sideload_outcome`
  mutants no measured host compiles (`src/cmd/run.rs:385:5`, `cfg(all(windows, not(target_arch = "x86_64")))`), and
  the fifteen of §1.
- The jobs read green only when a mutant whose covering predicate is false on its host is left out before it counts
  missed. The pinned recipe is the audit's `cover.py`, read under
  `rustc --print cfg --target x86_64-pc-windows-msvc`.
- The exclusion's home is the harness's `run --mutants` (the verdict row then reads over the measurable set), and
  it applies on every host, so its logic is provable on Linux with `cfg(windows)` spans standing in. Verified:
  `run/mutants.rs` holds no exclusion at HEAD, and the pinned recipe run over the workflow's 644 mutants at HEAD
  leaves out 29 under the Windows runner's cfg (the 24 + 4 above and one in viola-e2e) and 160 under the Linux
  host's own (research.md, Measured facts).
- `cleanup.rs:140:9` (viola-e2e, `delete !`) sits in a `#[cfg(unix)]` block at HEAD: on a Windows build it is
  excluded by this section, not graded. Verified: it is the one viola-e2e mutant the recipe leaves out, and the
  first dispatch recorded it host-excluded.
- A harness that reads `cfg` gates existed before: `harness::cfg_legs` (syn-based, 2026-09-26) was retired with the
  CI mutation legs on 2026-09-28 and is in git history. Whether it is revived or the audit's recipe is ported is a
  fork.
- A mutant behind a `cfg!` const (`HOST_IS_WINDOWS`, `HOST_SCRATCH`) compiles on both hosts and is not left out by
  this section: on the host that does not take the arm it stays recorded "not measured here" (test-plan §10).

### 4. Every job inside its ceiling
Source: CARRY 3.
- The `mutants (viola)` job was cancelled at its 120-minute ceiling with 125 of 140 graded and no harness document.
  The unmutated baseline read 123 s build + 104 s test and a graded mutant's test phase a median 89 s, against
  88 s + 10 s and 131 mutants in 26 m 49 s at the first dispatch (run 37174673472, `60c569b`).
- The CARRY's own marker, kept as written: "the cause of that growth is not measured". It was not measured at P3
  either. What P3 read: the job's 140 mutants are `src/cmd/run.rs` 50, `src/main.rs` 34, `src/run/env.rs` 33,
  `src/panic_frames.rs` 23 and `src/conpty.rs` 0; the 15 never graded are all in `src/run/env.rs`; the graded 125
  took about 54 s each.
- The `viola` job fits by a raised ceiling or a per-file split, "chosen at take-up": a technical fork, the
  operator's.
- The `mutants (viola-e2e)` job graded 0 of 68 on the previous entry's kill (audit F1: its unmutated baseline timed
  out under the `mutants` profile). Chunk `2026-10-09-epoch-3-cleanup` landed that baseline's fix: the
  `verify_window_` override stands first in `[profile.mutants]` (`.config/nextest.toml`). The Windows job has not
  run since, so this chunk's first dispatch is that fix's Windows reading.

### 5. The Windows grades still unread
Source: CARRY 3 (its last sentence) and CARRY 4 (chunk `2026-10-09-epoch-3-cleanup-ii`).
- `[premise-corrected: `evidence/windows-dispatch.md` of chunk 2026-10-04-windows-boundary-mutation-workflow, "A
  shared-body Windows survivor"]` `cleanup.rs:106:35` (the kill deadline's `+`) has a Windows grade: MISSED at run
  37174673472 on `60c569b`. Its killing test is `cfg(target_os = "linux")`. It is a sixteenth survivor. It takes
  no Windows case that kills a process (`inputs#I3`): the deadline is restated as a plain function a test reads on
  every OS. `cleanup.rs:140:9` is the `cfg(unix)` twin of §3.
- `[premise-corrected: the same record's table of 34 graded coordinates]` `scratch.rs:48:5` (`prepare`'s body
  replaced by `Ok(None)`) and `:54:8` (`delete !`) were graded caught on the `windows-2025` runner at run
  37174673472; the three viola-e2e files the workflow scopes are byte-identical since `60c569b`
  (`git diff --numstat 60c569b HEAD`). They are the two missed of the Linux whole-member score (718 mutants, 656
  caught, 2 missed, 0 timeout, 60 unviable in 5356 s). Owed is their record on the host that takes the arm, and
  this chunk's dispatch reads them again.
- `[premise-corrected: `win-outcomes.json`, the `state` rows]` `fs.rs:290:19` has a Windows grade: both of its
  guard mutants read caught at run 37761947926 on `e304994`, and `fs.rs` has moved since only inside its test
  module. Owed is the record, and this chunk's dispatch reads it again.

### 6. The workflow reads green
- The entry's last clause: `windows-mutants.yml`, dispatched on the chunk's pushed commit, ends with every job's
  harness document `ok:true` — the sixteen killed by a test or restated (the entry's "killed or argued", as
  corrected in §1), the twins left out, the `viola` job inside its ceiling.
- The workflow stays what founder ruling C2 made it: dispatch-only with no inputs, report-only, never a gate and
  never a dependency of `ci.yml`, with no cache, upload, secret, `concurrency:` or `needs:` (verified against the
  architecture, security and test extracts). This chunk adds no mutation job to `ci.yml` and no agent command to
  the harness. The masters' sentence on the job's shape (one job per package) moves with the split and is an
  expected amendment at the wrap. "Dispatched only during the epoch-boundary audit" is founder ruling C2 and is
  not amended (`inputs#I3`); "its jobs read red while scoped files carry `#[cfg(unix)]` twins" waits for a
  dispatch to read otherwise.
- The reading itself is held: no dispatch is fired before the founder's own word (`inputs#I3`). The chunk brings
  the workflow to where a dispatch would read green and stops.

### 7. The security-plan sentence, brought to the workflows as this chunk leaves them
Source: CARRY 5 (chunk `2026-10-09-epoch-3-cleanup-ii`'s wrap, its security-plan detector's note).
- security-plan's Threat Model Summary lists "mutation (ubuntu and windows legs plus a union verdict)" among
  `ci.yml`'s jobs and names no perf job, while its §Dependency Security says "`ci.yml` runs no mutation job"; at
  `781563c` `ci.yml`'s jobs read `test`, `perf`, `msrv`, `fuzz-replay`, `lint`, `release`, `supply-chain`.
- Phase amends no spec: the sentence is an expected amendment at this chunk's wrap, named in the plan.

## Boundaries
- Zero live `claude` sessions; no `viola verify` run against the real CLI (`inputs#I1`).
- No split and no boundary widening (`inputs#I1`): no new dated exception in `security.md`, no new env seam, no
  new G2 exemption, no sixth agent command, no Decisions Log entry that needs the founder.
- Not this chunk: the entry "Windows-only live measurements" (its `BLOCKED-ON` an interactive Windows host stands;
  this chunk's Windows work runs on the hosted runner, headless).
- Not this chunk: a Linux mutation score. The Linux survivors were disposed by the two cleanup chunks.

## Surfaces and contracts it touches
- Tests in `crates/viola-state/src/strict.rs`, `crates/viola-state/src/fs.rs` and `crates/viola-pty/src/lib.rs`
  (Windows-gated cases, and two that run on every OS), the restatements of §1 in the first two, and the kill
  deadline's restatement with its every-OS case in `crates/viola-e2e/src/harness/cleanup.rs`.
- The harness's mutation run (`crates/viola-e2e/src/harness/run/mutants.rs` and a new module beside
  `run/mutants/scratch.rs`): the exclusion, and the document's fields that carry it (test-plan §3, the `run`
  contract). No closed enum gains a value.
- `.github/workflows/windows-mutants.yml` and `tests/contract_windows_mutation_scope.rs`, which keeps its file lists
  and tool pins.
- test-plan §10 (the Mutation gate paragraph) and §3 (`run` step 4); security-plan (the sentence of §7 and the CI
  integration paragraph).
- The chunk's `evidence/`: the equivalence arguments, the six retry-loop mutants' record, the dispatch's per-job
  reading.

## The proof loop
- On the Linux dev host first: the exclusion logic and its unit tests, one real mutation run that shows the
  exclusion on a real `outcomes.json`, the workflow contract test, the shared-body tests, the restatements read
  through `cargo mutants --list`, the Linux gates.
- `[premise-corrected: `cargo check -p viola-state -p viola-pty -p viola-e2e --tests --target
  x86_64-pc-windows-msvc` exits 0 on this host, and clippy for that target does too]` A Windows-gated test body
  does type-check and lint here; it does not run here.
- On the Windows runner: the Windows-gated tests first run in `ci.yml`'s `test (windows-2025)` on the operator
  pass's push; the mutation grade needs a `windows-mutants.yml` dispatch on that pushed commit. The plan counts the
  dispatches and their wall time, and fires none before the founder's word (`inputs#I3`).
