# Report — 2026-10-10-windows-mutation-grade

**Chunk:** Windows mutation grade — the strict-modes and DACL checks pinned by Windows-side refusal tests, host-excluded twins left out of the count, every job inside its ceiling, workflow green
**Date:** 2026-10-10T08:59Z
**Commits:** `dd5161d chore(2026-10-10-windows-mutation-grade): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since the base `781563cd59a6`; basis `git log --format='%h %s' 781563cd59a6..HEAD`)

## Changes (structured — detectors read this)
- **Files:** basis `git diff --name-only 781563cd59a6` plus `git status --short`, source and manifests only (11, equal to `gate.py scope`'s `changed 11 · listed 11`):
  `.github/workflows/windows-mutants.yml` · `Cargo.toml` · `Cargo.lock` · `crates/viola-e2e/Cargo.toml` · `crates/viola-e2e/src/harness/cleanup.rs` · `crates/viola-e2e/src/harness/run/mutants.rs` · `crates/viola-e2e/src/harness/run/mutants/host.rs` (new) · `crates/viola-pty/src/lib.rs` (test module only) · `crates/viola-state/src/fs.rs` · `crates/viola-state/src/strict.rs` · `tests/contract_windows_mutation_scope.rs`.
  Records: the chunk folder's `evidence/` (`survivors.md`, `host-excluded.md`, `witness-runs.md`, three witness documents with their outcome lines, `operator-pass.md`, `windows-dispatch.md`) and `inputs/` (I1 to I6).
- **Symbols / APIs:** no IPC method, endpoint, event, socket, port, env var, CLI verb or flag is added or changed. Product crate `viola-state` and test-only crate `viola-e2e`:
  - `viola_state::strict::volume_keeps_acls(flags: u32) -> bool`, new and `pub` (`strict.rs` 82-85): the persistent-ACL bit of a volume's flags as a plain function. Its one product caller is `strict::win::persistent_acls`. The private const `PERSISTENT_ACLS` moved from `strict::win` to the module's top level.
  - `strict::win::OWNER_AND_DACL: u32 = 0x5`, a private const replacing `OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION` in `win::owner_and_dacl`; a Windows test pins it to the two names.
  - `viola_state::fs::win`: the module is now `pub(crate)` (was private). New `pub(crate) fn set_dacl(path: &Path, sddl: &str) -> io::Result<()>`, the half of `win::protect` that applies an SDDL; `protect` calls it with `protected_sddl`. New `pub(super) const PROTECTED_DACL: u32 = 0x8000_0004`. `win::dacl_of` no longer tests `present == 0` (the guard is `call == 0 || dacl.is_null()`).
  - `viola_state::fs::restrict` is two functions, a `cfg(unix)` one that sets the mode and a `cfg(not(unix))` one whose body is `Ok(())`. Its five call sites are not edited. Behaviour is unchanged on every OS.
  - `viola-e2e`: `harness::cleanup::kill_deadline(now: Instant) -> Instant`, new and private (`cleanup.rs` 128-131); `cleanup_one` is its one caller. `harness::run::mutants::mutants_suite` gained a parameter, `host_excluded: u64`; its one product caller is `mutate` in the same file (basis `grep -rn "mutants_suite(" crates src tests`: 1 definition, 1 product call, 8 test calls, all in `mutants.rs`).
  - New module `harness::run::mutants::host` (`host.rs` 1-782, all items `pub(super)` or private): `Host {family, os, arch}`, the const `HOST` (from `std::env::consts`), `Excluded {name, cfg}`, `excluded_missed(root, outcomes, host) -> Vec<Excluded>`.
- **Crates / modules:** no crate added or removed. One module added: `crates/viola-e2e/src/harness/run/mutants/host.rs`. `viola-pty`: test module only (`lib.rs` 1053-1176).
- **Dependencies:** two, both direct dependencies of the test-only `viola-e2e` alone, pinned in `[workspace.dependencies]`: `proc-macro2 = "=1.0.107"` with the feature `span-locations`, and `syn = "=2.0.119"` with `default-features = false` and the features `full`, `parsing`, `printing`, `visit`. Both were already in `Cargo.lock` as transitive packages: the lockfile gains two dependency lines under `viola-e2e` and no package entry (`Cargo.lock` added lines 1672 and 1675). No product crate gains a dependency, no manifest gains a viola-crate edge or a feature. `cargo deny check` reads green with no new ignore (the gate entry).
- **Schema / config:** no file under `schemas/` changes (the preservation guard names the directory). No config key. The harness `run` document, which is not a `schemas/` format, gains one field: see Harness / gate surface.
- **Spec-master edits:** none. No master, key file or sidecar was edited before this wrap.
- **Counts / qualifiers moved:**
  - `windows-mutants.yml` jobs: 6 → 9. The one `viola` matrix item became four (`src/cmd/run.rs`; `src/main.rs` with `src/conpty.rs`; `src/run/env.rs`; `src/panic_frames.rs`). Rule: the count of `- package:` items under `matrix.include` (9; `grep -c '^          - package: viola$'` reads 4). Each item carries a `label`, and the job name is `mutants (${{ matrix.label }})`, no longer `mutants (${{ matrix.package }})`. Docs stating the old shape, by `six-package|six packages|six jobs` over the masters and key files (2 hits): `security-plan.md` (§Dependency Security, CI integration) and `registries/contracts/architecture/ci-cd-approach.md`; and by `windows-mutants` (19 hits over 6 files), the per-package wording at `test-plan.md:1149` (§9 Mutation row: "one `mutants (<package>)` job per package") and in `ci-cd-approach.md` ("One job, `mutants (<package>)`").
  - Mutants over the workflow's nine items: 642 (basis: `cargo mutants --list --json` per item through `forecast.py`, re-read 2026-10-10T08:02Z; 644 at the plan's HEAD; 508 over the six items at the first dispatch, the figure `test-plan.md:1212` carries with run 37174673472).
  - Mutants a Windows host never compiles, left out by the harness on `windows-2025`: 30 (viola-pty 5, viola-channel 4, viola-state 7, viola-cmd-run 4, viola-panic-frames 9, viola-e2e 1; basis: each job's `host_excluded` in run 38036448183, equal to the recipe's list). On Linux the recipe reads 152 over the same items; the harness read 25 for the viola-pty item there (witness 1).
  - The sentence at `test-plan.md:1212`, "Its jobs read red until the audit classifies the `#[cfg(unix)]` twins" (`read red until|classifies the twins`: 1 hit, test-plan): no longer true. The harness leaves those twins out itself, and run 38036448183 read all nine jobs green.
  - `obs-plan.md:1013` (§9 Mutation row) carries "9 of 9 caught, run 37174673472" for the `cfg(windows)` obs code in `src/panic_frames.rs`. Run 38036448183 reads that file as its own job: 23 tested, 14 caught, 0 unviable, 9 left out as `unix` twins, 0 missed.
  - Windows grades read in run 38036448183 (added at Validate, after the fan-out, because two detector proposals rested on `evidence/` for them; basis: each job's own outcome lines, recorded per coordinate in `evidence/survivors.md`): of the sixteen survivors twelve read caught and four are no longer generated; the six retry-loop mutants of `replace_private_with` and both `fs.rs:301:19` guard mutants read caught in `mutants (viola-state)`; the two `prepare` mutants (`scratch.rs:48:5`, `:54:8`) read caught in `mutants (viola-e2e)`. The retry-loop and `prepare` mutants sit behind a const (`HOST_IS_WINDOWS`, `HOST_SCRATCH`, each `cfg!(windows)`), so they compile on both hosts, are never left out, and still read missed on the Linux dev host. The four `src/cmd/run.rs:385:5` mutants are left out on both hosts (`all(windows, not(target_arch = "x86_64"))`) and graded on no host this project builds on; they are owed to no route entry yet.
  - CI test counts on the pushed sha (`ci#38021000200`): `windows-2025` 1886 (was 1861), `macos-latest` 1853 (was 1834), `ubuntu-latest` 1857 (was 1838); the harness reads unit 1502 and integration 355 on the dev host. No master states any of these (`\b(1861|1834|1838|1483|1502|1857|336|355)\b`: 0 hits over 37 files).
- **Dev-tool versions:** none. cargo-mutants 27.1.0 and cargo-nextest 0.9.146 were re-read at the CI pins on the dev host for the witness runs; zizmor ran at the CI pin. `syn` and `proc-macro2` are lockfile-resolved crates and not this line's subject.
- **Harness / gate surface:**
  - `run --mutants`, every verdict that runs cargo-mutants: after the run, the harness reads the run's own `outcomes.json` by record. A record whose summary is `MissedMutant` and whose whole span sits under a `cfg` predicate proved false for the host the harness was built for (on a node covering the span, or on the `mod` declaration that brings the file in) is left out of `survived`, of the missed count and of `failures`. `tested` still counts it. `survived` = missed − left out + timeout. `unviable <= caught` is judged as before. A caught, unviable or timed-out record is never left out, and a file, path, attribute or key the reader cannot read keeps the mutant counted. Decided keys: `unix`, `windows`, `target_family`, `target_os`, `target_arch`; any other key is unknown; `not` / `all` / `any` are three-valued. The host's facts are compile-time constants (`std::env::consts`): no child process, no environment read.
  - The `run` document's `mutants` object gains `host_excluded`: an array of `{name, cfg}`, the name cargo-mutants' own and the predicate as written in source, repo-relative. The field is written only when at least one mutant was left out (absent in three of the nine Windows jobs and in witness 2). No `verdict`, `reason`, `suite` or `event` value is added, and `suites[]` keeps its field names.
  - cargo-mutants' own summary line and outcome lines still say MISSED for a left-out mutant; only the harness document leaves it out.
  - `windows-mutants.yml`: nine matrix items, each with `label`; job name `mutants (<label>)`. Unchanged: `workflow_dispatch` only, no `inputs:`, `permissions: {}`, `timeout-minutes: 120`, `fail-fast: false`, the action pins, the tool line, the step bodies, and the header comment that states founder ruling C2.
  - `tests/contract_windows_mutation_scope.rs`: new case `workflow_job_labels_are_distinct_one_per_item` and its helper `label_problems` (181-204, 347-376).
  - No agent command, internal subcommand, CI job in `ci.yml` or `nightly.yml`, or pre-push stage is added.
- **Cross-project / external claims:**
  - CI, repository `Turbolet85/viola`: `ci#38021000200` (push) on `dd5161d55743`, attempt 1, green, 15/15. `windows-mutants#38036448183` (`workflow_dispatch`) on the same sha, attempt 1, `completed` / `success`, nine jobs green; read by `ci.py conclusion --sha HEAD --name mutants --wait 5400` as `verdict: green · checks 9/24`. The verdicts were taken on `dd5161d`; this wrap's commit adds records, masters and leaves on top and no source.
  - Earlier runs the records cite, not re-read here: `windows-mutants#37761947926` on `e304994` and `#37174673472` on `60c569b`.
  - The dispatch ran on the founder's own word, `inputs#I5`: given live in the overseer dialog at 2026-10-10T07:58:27Z after the dispatch was shown to him with three options priced, relayed verbatim by the operator. It allows one or two dispatches now, for this chunk only. One was used. It is an exception for this chunk and changes no wording of ruling C2.
  - The host pressure reader `hostwatch.py` lives outside this repository (the overseer's); implement read it live for the three witness windows and the gate block's window, each `QUIET`. Its readings are in `witness-runs.md` and `operator-pass.md`; no fact of a master rests on it.
  - `inputs.py verify` (2026-10-10, this wrap): `inputs: 6 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 6 · uncited 3 · unparsed 0`. No DRIFT.
    - `I1 · message: the operator, the /andromeda-phase invocation, 2026-10-10 · copy · n/a — a message has no live source`
    - `I2 · message: the operator, the P4 fork dialog, 2026-10-10 · copy · n/a — a message has no live source`
    - `I3 · message: the operator, the P5 review, 2026-10-10 · copy · n/a — a message has no live source`
    - `I4 · message: the operator, 2026-10-10, the /andromeda-implement invocation · copy · n/a — a message has no live source` (the implement directive, `inputs#I4`; printed UNCITED before this report existed)
    - `I5 · message: the founder, 2026-10-10T07:58:27Z, live in the overseer dialog; relayed verbatim by the operator · copy · n/a — a message has no live source` (`inputs#I5`; printed UNCITED before this report existed)
    - `I6 · message: the operator, 2026-10-10, the /andromeda-wrap-session invocation · copy · n/a — a message has no live source` (`inputs#I6`, snapped at this wrap: three operator notes)
- **Reverted / negative API facts:** none in the diff. Before any code, the plan's first draft held a Windows kill-deadline case that sent a kill at pid 4; the operator's review (`inputs#I3`) removed it, and no such test was written. No argued-equivalent list, raised ceiling, dispatch input or `--exclude-re` shipped (the plan's rejected approaches).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's wall-time note (plan.md, Implementation notes, "The proof loop"; a plan estimate, no master carries it): about 52 min for `viola-cmd-run` and 16 min or less for every other job. Read in run 38036448183: `viola-cmd-run` 26 m 11 s; `viola-main` 41 m 30 s, `viola-panic-frames` 45 m 07 s, `viola-run-env` 49 m 21 s, `viola-e2e` 18 m 02 s. Every job stayed inside 120 min.
  - The plan's forecast of 29 left out on Windows (viola-state 6): the final tree reads 30 (viola-state 7), because the Unix `restrict` is now a function of its own with one mutant, `fs.rs:19:5`. The plan asked to record what the dispatch read.
  - `test-plan.md:1212`, "Its jobs read red until the audit classifies the `#[cfg(unix)]` twins": made false by this chunk's own change and listed under Expected amendments below.
- **Expected amendments (from plan):**
  - test-plan §3 `run` (step 4, Verdict and Output format), the host exclusion and `host_excluded`: carried, Harness / gate surface. Search: `host_excluded|host-excluded` 0 hits over the 7 masters and 30 key files, so the field is stated nowhere yet; the `run` step lives in test-plan's §3 key files (`registries/contracts/test-plan/`) and §3 body.
  - test-plan §9 (Pipeline structure, Mutation row), nine jobs and the per-file split, the dispatch wording untouched: carried, Counts. Search: `windows-mutants` 7 hits in test-plan; the row is `test-plan.md:1149`.
  - architecture §Infrastructure Patterns → CI/CD approach (job shape), → Crate dependency direction (viola-e2e's syn and proc-macro2), → Project directory structure (the tree comment and the new module): carried, Counts, Dependencies and Crates / modules. Search: `windows-mutants` 3 hits in `registries/contracts/architecture/ci-cd-approach.md` and 1 in `project-directory-structure.md`; `proc-macro2|proc_macro2|\bsyn\b` 0 hits over 37 files; `cfg_legs` 0 hits.
  - security-plan §Dependency Security (CI integration), the "one job, six-package matrix" wording: carried, Counts. Search: `six-package|six packages|six jobs` 1 hit in security-plan, 1 in `ci-cd-approach.md`.
  - security-plan §Threat Model Summary (Infrastructure, CI/CD, the jobs sentence), no mutation job in `ci.yml` and the `perf` job named (the route CARRY from chunk 2026-10-09-epoch-3-cleanup-ii): not carried as a change of this chunk. This chunk changed neither `ci.yml` (the preservation guard names it) nor its job list. The fact itself: `ci.yml` holds no mutation job since 2026-09-28 and its fifteen checks are `lint`, `test`, `release` and `perf` on three OSes plus `supply-chain`, `msrv` and `fuzz-replay` (`operator-pass.md`, step 5). The plan lists the sentence because the CARRY sits on this chunk's route line. Search: `mutation job|mutants job` 3 hits in security-plan.
  - a11y-plan §10 (Standard+ invariants), "covers" read over the measurable set, if the detector finds it needs it: carried as a fact, Harness / gate surface (a host-excluded mutant leaves the missed count; zero missed reads over the measurable set). Search: `epoch-boundary` 1 hit in a11y-plan.
  - test-plan §10 (Mutation gate), only after a dispatch: the sentence "its jobs read red until the audit classifies the twins" and the Windows grades: carried, Counts and Outcome. The dispatch has read (run 38036448183). Search: `read red until|classifies the twins` 1 hit, `test-plan.md:1212`.
  - obs-plan §9 CI Integration (Pipeline integration, Mutation row), only after a dispatch: the invocation form after the split and the obs-code reading with its run: carried, Counts. Search: `windows-mutants` 2 hits in obs-plan; the row is `obs-plan.md:1013`.
  - Owed at the wrap, curation (plan, Implementation notes; `inputs#I2`): the `testing.md` line of 2026-09-24 on a trivial other-OS stub takes `restrict` as its named exception with the measured reason. Not a master: owner P3.
- **Coverage of new surfaces:** no new external-input surface, hot-path operation or UI element.
  - the `run` document's `mutants.host_excluded` (test-only harness output) → validation n/a (it reads the run's own `outcomes.json` and checked-out source; a path with a non-normal component is never read) · instrumentation n/a (the harness is not a product role) · PII redacted✓ (repo-relative names and predicates only; the unit case `run_mutants_leaves_out_a_missed_mutant_the_host_never_compiled` asserts no absolute path in a document holding a left-out mutant) · tests unit (the `host.rs` case table, three `mutants.rs` cases) and three Linux witness runs and nine Windows jobs · a11y n/a · tokens n/a
  - `viola_state::strict::volume_keeps_acls` (a pure function of a `u32`) → validation n/a · instrumentation n/a · PII n/a · tests unit, every OS (`volume_keeps_acls_reads_the_persistent_acls_bit`, five cases) · a11y n/a · tokens n/a
  - `viola_state::fs::win::set_dacl` (crate-visible, Windows only; called by `protect` and by tests) → validation: an SDDL carrying no DACL is refused✓ · instrumentation n/a · PII n/a · tests unit on `windows-2025` (`set_dacl_refuses_an_sddl_that_carries_no_dacl`, `protected_dacl_is_the_named_flags`, the two `check_stamps_refuses_…` cases) · a11y n/a · tokens n/a

## Deviations from intent
- Four plan points were left to the implementer and settled from neighbouring code (implement's own record, 2026-10-10T03:01Z): which path stands for a volume that cannot be read (a drive letter with no volume, the highest from `Z` down to `D`); `host_excluded` is not written when empty; the four job labels (`viola-cmd-run`, `viola-main`, `viola-run-env`, `viola-panic-frames`); a file declared by two `mod` lines.
- `crates/viola-e2e/src/harness/run.rs`, listed conditionally in the plan, was not edited: `mutants_suite`'s one product caller is in `mutants.rs`.
- The Windows left-out count is 30, not the plan's 29 (above).
- Step 11 ran in a later session than steps 1 to 10. The dispatch waited for the founder's word and was fired once after it was snapshotted (`inputs#I5`). A second dispatch was allowed and not used.
- Process: two of implement's edits went through a count-asserted script instead of the Edit tool (one line of `host.rs` after the format hook rewrote the file; the workflow matrix change).
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 11 · listed 11 · recorded 0`, base `781563cd`).

## Decisions & corrections
- The operator's review at P5 (`inputs#I3`): no test sends a kill at a process it does not own, so the Windows kill-deadline case does not target pid 4; and founder ruling C2 stands until his own word, so the plan stopped before any dispatch.
- The operator's wrap note 1 (`inputs#I6`): that rule is curated, and `cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill` (`crates/viola-e2e/src/harness/cleanup.rs`, a `cfg(target_os = "linux")` case that takes PID 1 as its target) stays and is named beside the rule. The note cites the `fn` at line 295, its line before this chunk; on this tree it is line 300 (read: `grep -n` on the name), the chunk having added five lines above it.
- The operator's wrap note 2 (`inputs#I6`): the playbook takes the rule the last wrap proposed (`fanout-results.md` T4/O1 of `2026-10-10T01-25-38-wrap`): a count in a master is amended with its rule named.
- The operator's wrap note 3 (`inputs#I6`): the founder's mutation ruling is reworded nowhere; only this chunk's exception is named, with its date (`inputs#I5`; one dispatch used).
- The three forks settled at P4 (`inputs#I2`): the `viola` job split per file instead of a raised ceiling; the retired syn reader revived instead of a hand port of the audit's recipe; `restrict` split in two.
- Step 4's rule, as the plan read it: `testing.md` 2026-09-24 names an OS-gated function; here the OS-gated thing was the only killing test, and the remedy (the decision in a plain function tested on every OS) was applied for the rule's stated reason. The operator did not object at review.
- Sweep hazards found:
  - cargo-mutants' summary line counts a host-excluded mutant as missed (`4 missed, 127 caught` on a green job). A run is judged by the harness document, never by that line.
  - A caught mutant's outcome line carries its test time and nothing else. Three retry-loop mutants read caught after 10 s of test (`fs.rs:281:18` `*=`, `285:20` → `true`, `286:21`): the job log cannot tell a failed assertion from a test ended at its 10 s kill line. The same shape as the CARRY on "Paste newline ledger row".
  - In the root package a graded mutant costs about one whole pass of the root tests on `windows-2025` (median 95 to 102 s in `viola-run-env` and `viola-panic-frames`), and a left-out mutant is still built and tested before it is left out. A job's wall follows its count of viable mutants, not its file's size.
  - `gh run list --commit` given a 7-character sha printed an empty list with exit 0 at this session's start (already a rule, `ci.md` 2026-10-09; met again).
  - A session that enters implement at a plan's stop rule finds the session-start ladder leading with the wrap, because the pre-CI commit is in history while an operator entry is still held.

## Outcome
Acceptance criteria, each re-asserted against the diff and the records.

Proven before the stop rule:
- (tests) New Windows-gated tests pass in `test (windows-2025)`: MET. `ci#38021000200` on `dd5161d55743`, `verdict: green · checks 15/15`, attempt 1; the six runner-only cases each read as a `PASS` line; `pre-push` read `ok:true` on the same tree (`operator-pass.md`).
- (security) Each of the sixteen stands in `evidence/survivors.md` with its test or restatement and its Linux reading; none of `check_path`, `check_stamps`, `win::check`, `win::persistent_acls` is closed by an argument alone: MET (rows 1 to 5 each name a test).
- (tests) The six retry-loop mutants, both `fs.rs:290:19` mutants and the two `prepare` mutants stand by coordinate with host, run and grade; the four `src/cmd/run.rs:385:5` mutants stand in `host-excluded.md` as not measured on any host: MET.
- (tests) The Linux witness for the viola-pty item leaves out the `cfg(windows)` mutants the recipe names: MET (25 and 25, equal by name and predicate; the `jq -e …/evidence/linux-witness.json` entry green).
- (arch) `windows-mutants.yml` still dispatch-only with no `inputs:`, cache, upload, secret, `concurrency:` or `needs:`; header states the ruling; `ci.yml` holds no mutation job: MET (the zizmor entry, the forbidden-key probe, the header count and the preservation guard green; the contract binary 4 of 4).
- (obs) `host_excluded` carries repo-relative names and predicates only: MET (the unit case; 0 absolute paths in all nine Windows documents).
- (a11y) The `run` document keeps its `suites[]` field names and gains no `suite` value; the three tui boundary cases still run on `windows-2025`: MET (the diff adds no `suite` value; `test (windows-2025)` 1886 passed, none skipped).
- (arch) viola-e2e gains exactly syn and proc-macro2; no viola-crate edge or feature; `cargo deny check` green with no new ignore: MET.
- (security) No environment read in a product crate, no config key, flag, `cfg` or feature bypassing a control, no dated exception, no test seam; no test added sends a kill at a process it did not start: MET against the diff (`kill_deadline_lies_five_seconds_after_the_instant_it_is_given` starts no process and sends no kill; the two viola-pty cases wait on children they spawn).
- (obs) G2 and G4 green over the smoke home and in `test (windows-2025)`; `schemas/diag-line.v1.json` gains no value: MET.

Held by step 11, proven after the founder's word (`inputs#I5`):
- (tests) Every `windows-mutants.yml` job prints a `run` document with `ok:true`, verdict `package`, zero missed, zero timeout, unviable not above caught; the held entry reads `verdict: green` over the nine checks: MET. Run **38036448183** on `dd5161d55743`, attempt 1; `evidence/windows-dispatch.md` holds each job's document verbatim.
- (tests) Every job ends inside its 120-minute ceiling with its document printed and no Timeout grade: MET (longest 49 m 21 s).
- (security) The Windows column of each of the sixteen is filled from that dispatch: MET (twelve caught, four no longer generated).
- (tests) Each job's `host_excluded` equals the pinned recipe's reading on the final tree: MET (30, equal by name and predicate in all nine; every row names its predicate).

Gates, by `run`, as recorded by the operator pass (the whole block fired once at 03:28:58Z to 03:31:09Z on the tree the pre-CI commit took):
- `cargo fmt --all --check` green (exit 0) · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green · `CARGO_TARGET_DIR=target/wincheck cargo clippy -p viola-state -p viola-pty -p viola-e2e --tests --target x86_64-pc-windows-msvc -- -D warnings` green
- `bash scripts/agent-run.sh run --unit` green (1502 of 1502) · `bash scripts/agent-run.sh run` green (unit 1502, integration 355) · `bash scripts/agent-run.sh run --integration --filter 'binary(contract_windows_mutation_scope)'` green (4 of 4)
- `zizmor .github/workflows/` green (`No findings to report`) · the forbidden-key probe (`! grep -nE 'secrets\\.|upload-artifact|…'`) green (no output) · `grep -c '^          - package: viola$' …` green (last line 4) · `grep -c 'timeout-minutes: 120' …` green (1) · `grep -c 'dispatched only during the' …` green (1)
- `git diff --quiet 781563cd59a6 -- .github/workflows/ci.yml …` (the preservation guard) green · the no-`#[ignore]`-added probe green (no output) · `jq -e '.mutants.verdict == "package" …' …/evidence/linux-witness.json` green · `cargo deny check` green
- `bash scripts/agent-run.sh cleanup --session p-wmg-smoke` green · `… boot --session p-wmg-smoke --instance builder` green (10.2 s) · `… status --session p-wmg-smoke` green (`state:"ready"`) · `bash scripts/g2-zero-panics.sh` green (`g2: clean`) · `bash scripts/agent-run.sh schema-check` green · the closing `cleanup` green (`processes_gone:true`, `endpoint_gone:true`)
- `bash scripts/agent-run.sh pre-push` green (`"ok":true`, `"stage":"linux-tests"`; coverage 1857 of 1857)
- `leg = 'operator'`, driven by hand and recorded in `evidence/operator-pass.md`: `… gate.py hygiene` (`hygiene: clean`) · `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (exit 0, `781563c..dd5161d`) · `… ci.py conclusion --sha HEAD --wait 1800` (exit 0, `verdict: green`, `ci#38021000200`) · `… ci.py conclusion --sha HEAD --name mutants --wait 5400` (exit 0, `verdict: green · checks 9/24`, run 38036448183; also in `evidence/windows-dispatch.md`)
- No entry carries `defer`. Smoke: the boot path changed (owner-only helpers); the smoke entries above are it.

Watches: none folded.

Outcome basis: the operator pass ran. The verdicts rest on its final state: one commit, `dd5161d`, no fix commit; the final HEAD's CI run `ci#38021000200` and the dispatch `windows-mutants#38036448183`, both recorded in `evidence/`. Implement's steps 1 to 10 ran in an earlier session whose conversation this wrap does not hold; their facts are taken from the chunk's own evolve records (5 records with `chunk` equal to this marker and `skill` `andromeda-implement`), from `evidence/` and from the diff. Implement's step 11 (the dispatch and entry 26) ran in this session, and its report is used as given. Between that report and this one: the operator's three wrap notes (`inputs#I6`), which change no fact of the chunk.

Process hygiene: step 11's census, measured against the host's process list at 2026-10-10T08:54Z: the `ci.py conclusion` reader, three job-state pollers and the `gh` and `cargo mutants --list` calls, all started by that run, all `terminated`; no row remained. Steps 1 to 10: the operator pass's record says the process list read after the gate block held no process of this repository (seven processes named `viola` were another tree's build, decided by `/proc/<pid>/exe`); that session's own census is unmeasured here — its conversation held it. This wrap started one background code-graph refresh, which exited 0.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 781563cd (the parent of the oldest pre-CI commit dd5161d5) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/windows-mutants.yml — added 17 line(s) in 7 range(s)
added: 15 · 25 · 28 · 31 · 34 · 37-47 · 49
### Cargo.lock — added 2 line(s) in 2 range(s)
added: 1672 · 1675
### Cargo.toml — added 4 line(s) in 2 range(s)
added: 199-200 · 209-210
### crates/viola-e2e/Cargo.toml — added 3 line(s) in 2 range(s)
added: 25-26 · 29
### crates/viola-e2e/src/harness/cleanup.rs — added 15 line(s) in 3 range(s)
added: 106 · 128-132 · 335-343
- 128-131 @129 «fn kill_deadline(now: Instant) -> Instant {»
  - 335-342 @336 «fn kill_deadline_lies_five_seconds_after_the_instant_it_is_given() {»
### crates/viola-e2e/src/harness/run/mutants.rs — added 183 line(s) in 21 range(s)
added: 13 · 47-48 · 51-55 · 58-61 · 189-191 · 196 · 208-210 · 216-217 · 274 · 276 · 278 · 280-282 · 291 · 298 · 359
       362 · 368-389 · 393 · 398 · 837 · 844-970
  - 368-388 @371 «fn mutants_suite_leaves_host_excluded_missed_out_of_the_survivors() {»
  - 844-856 @846 «fn missed_lines(outcomes: &str) -> String {»
  - 858-862 @860 «const GATED: &str = "#[cfg(target_os = \"none\")]\nfn never() -> u32 {\n    1\n}\n\»
  - 864-874 @865 «fn gated(line: u64, replacement: &str, summary: &str) -> Value {»
  - 876-891 @878 «fn gated_run(records: &[Value]) -> (Workspace, Outcome, tempfile::TempDir) {»
  - 893-901 @894 «fn strings(doc: &Value, out: &mut Vec<String>) {»
  - 903-932 @906 «fn run_mutants_leaves_out_a_missed_mutant_the_host_never_compiled() {»
  - 934-969 @937 «fn run_mutants_keeps_every_mutant_it_cannot_prove_host_excluded() {»
### crates/viola-e2e/src/harness/run/mutants/host.rs — new file · 782 line(s)
- 18-24 @20 «pub(super) struct Host {»
- 26-32 @28 «pub(super) const HOST: Host = Host {»
- 34-40 @37 «pub(super) struct Excluded {»
- 46-67 @48 «pub(super) fn excluded_missed(root: &Path, outcomes: &Value, host: Host) -> Vec<Excluded> {»
  - 49-66 «outcomes["outcomes"]»
- 69-74 @70 «fn position(at: &Value) -> Option<Position> {»
- 76-85 @78 «fn false_cfg(root: &Path, file: &str, span: Range, host: Host) -> Option<String> {»
  - 80-82 «if !file.components().all(|c| matches!(c, Component::Normal(_))) {»
- 87-89 «fn read(path: &Path) -> Option<syn::File> {»
- 91-119 @94 «fn declared_false(root: &Path, file: &Path, host: Host) -> Option<String> {»
  - 96-118 «loop {»
- 121-134 @123 «fn module_of(file: &Path) -> Option<(String, PathBuf)> {»
  - 126-133 «match stem {»
- 136-144 @137 «fn declarations(parsed: &syn::File, name: &str) -> Vec<Range> {»
  - 138-141 «let mut found = Declarations {»
- 146-149 «struct Declarations<'a> {»
- 151-158 «impl<'ast> Visit<'ast> for Declarations<'_> {»
  - 152-157 «fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {»
- 160-163 «fn range(span: Span) -> Range {»
- 165-174 @166 «fn false_cover(parsed: &syn::File, span: Range, host: Host) -> Option<String> {»
  - 167-171 «let mut cover = Cover {»
- 176-180 «struct Cover {»
- 182-188 «impl Cover {»
  - 183-187 «fn note(&mut self, attrs: &[Attribute], node: &impl Spanned) {»
- 190-194 @192 «fn holds(node: Range, span: Range) -> bool {»
- 196-269 @199 «impl<'ast> Visit<'ast> for Cover {»
  - 200-203 «fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {»
  - 205-208 «fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {»
  - 210-213 «fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {»
  - 215-218 «fn visit_item_const(&mut self, node: &'ast syn::ItemConst) {»
  - 220-223 «fn visit_item_static(&mut self, node: &'ast syn::ItemStatic) {»
  - 225-228 «fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {»
  - 230-233 «fn visit_local(&mut self, node: &'ast syn::Local) {»
  - 235-238 «fn visit_expr_block(&mut self, node: &'ast syn::ExprBlock) {»
  - 240-243 «fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {»
  - 245-248 «fn visit_expr_for_loop(&mut self, node: &'ast syn::ExprForLoop) {»
  - 250-253 «fn visit_expr_if(&mut self, node: &'ast syn::ExprIf) {»
  - 255-258 «fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {»
  - 260-263 «fn visit_expr_tuple(&mut self, node: &'ast syn::ExprTuple) {»
  - 265-268 «fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {»
- 271-282 @273 «fn false_predicate(attr: &Attribute, host: Host) -> Option<String> {»
  - 274-276 «if !attr.path().is_ident("cfg") {»
  - 278-280 «if eval(&meta, host) != Some(false) {»
- 284-329 @286 «fn eval(cfg: &Meta, host: Host) -> Option<bool> {»
  - 287-328 «match cfg {»
- 331-339 «fn all(values: &[Option<bool>]) -> Option<bool> {»
  - 332-338 «if values.contains(&Some(false)) {»
- 341-349 «fn any(values: &[Option<bool>]) -> Option<bool> {»
  - 342-348 «if values.contains(&Some(true)) {»
- 351-455 @352 «mod tests {»
  - 357-361 «const LINUX: Host = Host {»
  - 362-366 «const MACOS: Host = Host {»
  - 367-371 «const WINDOWS: Host = Host {»
  - 372-375 «const WINDOWS_ARM: Host = Host {»
  - 377-393 @378 «fn host_is_the_one_the_harness_was_built_for() {»
  - 395-398 «fn ev(cfg: &str, host: Host) -> Option<bool> {»
  - 400-430 @402 «fn eval_decides_family_os_and_arch_and_nothing_else() {»
  - 432-452 @434 «fn eval_all_and_any_are_three_valued() {»
- 456-458 «fn item_fn() {»
- 459-464 @460 «impl S {»
  - 461-463 «fn in_impl() {»
- 465-470 @466 «mod inline {»
  - 467-469 «fn in_mod() {»
- 471-476 «impl S {»
  - 472-475 @473 «fn impl_fn() {»
- 481-505 «fn statements() {»
  - 484-487 @485 «for _ in 0..1 {»
  - 488-491 @489 «unsafe {»
  - 492-495 @493 «{»
- 506-509 @507 «fn mac() {»
- 510-516 @511 «impl T {»
  - 512-515 @513 «fn nested() {»
- 517-520 @518 «fn unknown_key() {»
- 521-524 @522 «fn other_arch() {»
- 525-528 @526 «fn cfg_attr() {»
- 529-532 @530 «fn another_attribute() {»
- 533-536 @534 «fn before() {»
- 537-539 «fn after() {»
- 549-782 «"#;»
  - 551-571 @553 «fn tree() -> tempfile::TempDir {»
  - 573-586 @575 «fn found(text: &str, needle: &str) -> (Position, Position) {»
  - 588-604 @589 «fn record(root: &Path, file: &str, from: &str, to: &str, summary: &str) -> Value {»
  - 606-612 @607 «fn left_out(root: &Path, file: &str, from: &str, to: &str, host: Host) -> Option<String> {»
  - 614-664 @615 «fn excluded_missed_reads_the_cfg_on_each_node_kind() {»
  - 666-693 @668 «fn excluded_missed_keeps_a_span_that_leaves_its_node() {»
  - 695-742 @696 «fn excluded_missed_follows_the_mod_declarations_above_a_file() {»
  - 744-781 @746 «fn excluded_missed_leaves_out_only_missed_records_it_can_read() {»
### crates/viola-pty/src/lib.rs — added 124 line(s) in 1 range(s)
added: 1053-1176
  - 1053-1069 @1056 «fn shows_line(report: &std::path::Path, want: &str, mut exited: impl FnMut() -> bool) -> bool {»
  - 1071-1078 «fn read_lines(report: &std::path::Path) -> Vec<String> {»
  - 1080-1105 @1084 «fn console_read_keeps_a_ctrl_z_and_goes_on_to_the_next_read() {»
  - 1107-1111 @1108 «struct Piped {»
  - 1113-1123 «impl Drop for Piped {»
  - 1125-1175 @1129 «fn piped_stdin_is_read_as_the_bytes_written_to_it() {»
### crates/viola-state/src/fs.rs — added 74 line(s) in 12 range(s)
added: 16-17 · 19-27 · 74 · 86 · 93-97 · 143-149 · 164 · 171 · 191-192 · 495-496 · 498 · 505-546
  - 508-518 @510 «fn protected_dacl_is_the_named_flags() {»
  - 520-538 @524 «fn set_dacl_refuses_an_sddl_that_carries_no_dacl() {»
### crates/viola-state/src/strict.rs — added 140 line(s) in 11 range(s)
added: 79-86 · 149-150 · 156 · 162-165 · 201 · 216 · 340-364 · 412-423 · 433-438 · 442-510 · 612-622
- 82-85 @83 «pub fn volume_keeps_acls(flags: u32) -> bool {»
  - 612-621 @619 «fn volume_keeps_acls_reads_the_persistent_acls_bit(#[case] flags: u32, #[case] keeps: bool) {»
### tests/contract_windows_mutation_scope.rs — added 56 line(s) in 2 range(s)
added: 181-205 · 347-377
- 181-204 @183 «fn label_problems(yml: &str) -> Vec<String> {»
  - 186-189 «let labels: Vec<&str> = lines()»
  - 191-193 «if labels.len() != items {»
  - 195-199 «for label in labels {»
  - 200-202 «if !lines().any(|l| l == "name: mutants (${{ matrix.label }})") {»
- 347-376 @348 «fn workflow_job_labels_are_distinct_one_per_item() {»
  - 350-354 «let item = |package: &str, label: &str| {»
  - 355-359 «let two = format!(»
  - 363-366 «let unlabelled = format!(»
  - 369-372 «assert_eq!(»
