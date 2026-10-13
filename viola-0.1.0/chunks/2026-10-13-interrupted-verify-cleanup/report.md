# Report — 2026-10-13-interrupted-verify-cleanup

**Chunk:** viola verify stopped by Ctrl-C or TERM removes only what that run created; the child-program name and --settings through their one homes
**Date:** 2026-10-13 (wrap run `.andromeda/runs/2026-10-13T20-00-28-wrap/`)
**Commits:** `c22161e chore(2026-10-13-interrupted-verify-cleanup): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-13T18:18:44Z; base `d9f01b1b8642`)

## Changes (structured — detectors read this)
- **Files:** `src/cmd/verify/stop.rs` (new) · `src/cmd/verify.rs` · `src/cmd/verify/typed.rs` · `src/run/version_gate.rs` ·
  `schemas/diag-line.v1.json` · `tests/cli_verify.rs` · `tests/support/verify.rs` · `.github/workflows/windows-mutants.yml`
  (the out-of-list companion, see Deviations). Basis: `gate.py scope` — `clean — changed 8 · listed 7 · recorded 1`.
- **Symbols / APIs:**
  - **`cmd::verify::stop`** (`src/cmd/verify/stop.rs`, new, 1-119), the root package's one signal / console-control
    handler. Verify is the only verb that installs it; no other verb's signal behaviour changes.
    - `install()` (16-20), `stopped()` (22-25) and `set()` (27-30) over one process-global `AtomicBool`; the handler's
      only act is that store.
    - `stops_on(event)` (32-36), a plain decision: `0` (`CTRL_C_EVENT`) and `1` (`CTRL_BREAK_EVENT`) stop, every
      other event does not.
    - Unix (43-56): `on_signal` is installed for `SIGINT` and `SIGTERM` through `libc::sigaction` with `SA_RESTART`.
    - Windows (58-75): `console_routine`, through `windows-sys` `SetConsoleCtrlHandler`, returns TRUE with the flag
      set for Ctrl-C and Ctrl-Break, and FALSE for every other event (close, logoff, shutdown keep the OS default).
    - A failed install is ignored (fail-open). SIGKILL and a crash keep their default and leave the dirs by hand.
  - **`src/cmd/verify.rs`**:
    - `enum Ended { Exit(u8), Interrupted }` (85-89).
    - `verify` (69-78) prints the pair `unable: verify was stopped before it finished` / `hint: run viola verify again
      to stamp this CLI` through `human::refuse`, logs `run::log_self_exit(1, Some("interrupted"))` under a
      `VIOLA_NAME`, and exits 1.
    - `measure` installs the stop as its first act. It reads the stop at the version probe's end (179-181), at the
      print probe's end (204-206), when `typed::measure` returns `None` (228-230), and once more after the probe dir
      is dropped, before `check_rows` (242-244). After that the run completes as before.
    - The default program reads `viola_agent_claude::PROGRAM`.
    - `run_logged` passes `stop::stopped` to `run_bounded`; the spawn pair is unchanged. A stopped child's
      `process-exit` carries `duration_ms` and no `child_exit_status`.
  - **`src/run/version_gate.rs`**:
    - `run_bounded` gains a sixth input, `stopped: fn() -> bool`, read before the child's exit on every 10 ms poll.
      At the stop it kills and reaps the child (`fn end`, 85-88).
    - `Bounded` gains `stopped: bool`, which tells the stop from the deadline's kill.
    - Two callers remain: verify's `run_logged` and `run`'s `version_gate`. The latter passes `never_stopped`, so
      `run`'s gate is unchanged in behaviour.
  - **`src/cmd/verify/typed.rs`**:
    - `Inputs` gains `stop: fn() -> bool`. `measure` returns `Option<TypedRun>` (`None` = stopped) and runs the four
      runs in one loop with one stop read before each (78-89, `RunStep`); Run A is now `untrusted_run`.
    - One polling function, `poll_until(clock, until, stopped, found)` (217-225), serves `wait_for` (241-249),
      `wait_capture` (both gain a stop input), `Run::settle`, `Run::box_wait`, and the two waits of
      `end_by_ctrl_c`.
    - `ctrl_c_ending` gains `stopped`: with the stop set it sends no Ctrl-C (or no second one) and goes to the kill.
    - `Run::paste_text` (690-693) refuses a paste once the stop is set.
    - The pump wait after a kill, `ended_by` in `Run::finish`, does NOT observe the stop.
    - Runs C and D's `--settings` read `viola_agent_claude::statusline::SETTINGS_FLAG`.
  - **Interfaces:** no IPC method, endpoint, event kind, port, socket or env var is added.
- **Crates / modules:** module added: `src/cmd/verify/stop.rs` (root package, `mod stop;` beside `mod typed;`). No crate
  added or removed.
- **Dependencies:** none added or bumped. The handler uses the root package's existing `libc` (Unix) and `windows-sys`
  (Windows, the workspace feature list's `Win32_System_Console`). `Cargo.toml` and `Cargo.lock` are unchanged: the
  preservation guard's command, run with only `.github/workflows/windows-mutants.yml` excluded, exits 0.
- **Schema / config:** `schemas/diag-line.v1.json:82` — `process-exit.detail` gains `interrupted`, giving 12 members
  (the closed enum's members as listed in research.md §Files inspected, plus this one). No config key.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `process-exit.detail` 11 → 12 members (`schemas/diag-line.v1.json:82`). The masters list members, not a count:
    pattern `detail-enum`, standing 6 — `obs-plan.md:308`, `:621`, `:834` (the `detail` code catalog),
    `design-system.md:789`, `registries/contracts/obs-plan/logging-stack.md:12`,
    `registries/contracts/obs-plan/otel-sdk-init.md:21`.
  - `windows-mutants.yml`: matrix items 9 → 10, and the root `viola` package's items 4 → 5 (the new
    `viola-cmd-verify-stop` · `src/cmd/verify/stop.rs`). Pattern `mutants-jobs`, standing 20; the two sites that
    state the count:
    - `security-plan.md:357` («a nine-item matrix … the root `viola` package is four of them, split by file»);
    - `test-plan.md:1163` («nine `mutants (<label>)` jobs … the root package `viola` is four of them»).

    The «all nine jobs» of run 38036448183 (`registries/contracts/architecture/ci-cd-approach.md:2`,
    `test-plan.md:1226`) is a past run's reading and stays.
  - The root `Instant::now() + WITHIN` count is unchanged: a fixed-string grep under `tests/` reads 28 sites in 20
    files, 0 added against `d9f01b1b8642`. Its sites stand: `architecture.md:416`,
    `registries/contracts/test-plan/5-command-implementation.md:33` (pattern `within-count`, standing 2).
  - `tests/cli_verify.rs` gains one rstest table of two cases.
- **Dev-tool versions:** none.
- **Harness / gate surface:** `tests/support/verify.rs` gains `StartedVerify { running, pid, receipt }` and
  `verify_started(home, fixtures, version, cwd, tmp, name)` (444-491). It is a verify started and not waited, built as
  `verify_with` but without `--turn-stop`, in a cwd the test owns, with `--trusted-root` that cwd and `TMPDIR` / `TMP`
  / `TEMP` a temp dir the test owns. No harness command, status shape, CI step or nextest profile changed.
  `.github/workflows/windows-mutants.yml` gains one matrix item (`package: viola`, `label: viola-cmd-verify-stop`,
  `files: src/cmd/verify/stop.rs`); nothing was dispatched.
- **Cross-project / external claims:**
  - CI on `c22161e791ff`, `ci#38453057445`:
    - attempt 1: `verdict: red · checks 15/15`, `not success: test (macos-latest)=cancelled`. The job read `steps: 0`,
      no runner name; annotation «The job was not acquired by Runner of type hosted even after multiple attempts»;
      macOS arm64 capacity notices.
    - attempt 2, after `gh run rerun 38453057445 --failed`: `verdict: green · checks 15/15 · attempt 2`. The stop cases
      ran: `test (macos-latest)` job 115496299136 (stop table + signal unit cases), `test (ubuntu-latest)` job
      115488617646 (the same), `test (windows-2025)` job 115488617598 (the three `console_routine` cases). The PASS
      lines are in `evidence/operator-pass.md`.
  - inputs (`inputs.py verify`: `3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0
    · n/a 3 · uncited 1 · unparsed 0`):
    - `I1 · message · the operator, the /andromeda-phase invocation, 2026-10-13T18:24Z · n/a` (cited by scope.md,
      research.md and plan.md);
    - `I2 · message · the operator, the P4 fork dialog, 2026-10-13T18:40Z · n/a` (cited by plan.md);
    - `I3 · message · the operator, the P5 review prompt, 2026-10-13T18:46Z · n/a`, cited here as inputs#I3. The
      operator's own word, verbatim: «yes. The card stands as written, the four decisions included; real Windows
      delivery goes to "Windows-only live measurements" as proposed.» Its card is
      `.andromeda/runs/2026-10-13T18-24-05-phase/p5-card-1.md`, whose `## Beyond the plan` row is, verbatim, with
      the operator's authority (his own word):
      > | 1 | Event delivery to a running `viola verify` on Windows (a real console's Ctrl-C / Ctrl-Break reaching the routine) is left to the existing working-route entry "Windows-only live measurements", whose BLOCKED-ON (an interactive Windows host) stands | owner for a part left out |
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - The plan's preservation guard (plan.md `## Test Commands`, the `git diff --quiet d9f01b1b8642 -- . …` entry; note
    «no other source, … workflow or config») cannot hold beside the repository's contract test
    `tests/contract_windows_mutation_scope.rs`. That test, run inside `pre-push`, requires every product file with a
    `cfg(windows)` gate to be listed in `.github/workflows/windows-mutants.yml`. The plan's step 1 puts the Windows
    console routine in the new `stop.rs`, and the first `pre-push` read red: `unlisted: src/cmd/verify/stop.rs`. With
    the companion line, the guard as written exits 1; with only that path excluded it exits 0. The research's Files
    to modify did not list the workflow either. Ownership: the plan's own text; the operator's direction at this wrap
    is to record it as a deviation and not edit the guard. Plan-template wording for guards over Windows-gated new
    files is a pipeline matter.
- **Expected amendments (from plan):** every search is the site sweep `report-patterns.toml` (`cascade.py sweep
  --pre-pass`, baseline `c22161e7`); counts are each pattern's `standing` figure.
  1. architecture §Occupied Resources → Filesystem (Ctrl-C / TERM remove the run's dirs; SIGKILL, a crash, a Windows
     close / logoff / shutdown leave them by hand) — carried: Symbols / APIs, `cmd::verify::stop`.
     - `verify-dirs` standing 10: architecture `:91`, `:100`, `:372`, `:412`, `:413`; security-plan `:131`, `:282`,
       `:283`, `:291`; obs-plan `:740`.
     - `stop-signals` standing 9: architecture `:47`, `:91`; security-plan `:131`; test-plan `:725`, `:833`, `:1025`,
       `:1226`; registries `contracts/test-plan/5-command-implementation.md:85`,
       `bootstrap-phases-derive-for-route-setup-project.md:15`.
  2. architecture §Occupied Resources → Claude Code integration names (verify reads `PROGRAM` and `SETTINGS_FLAG`; the
     clause owing the move is retired) — carried: Symbols / APIs. `owed-literals` standing 3: architecture `:371`,
     security-plan `:278`, registries `contracts/architecture/crate-dependency-direction.md:6`.
  3. architecture §Conventions, CLI exit codes (exit 1 covers an interrupted `verify`) — carried: Symbols / APIs,
     `verify`. `verify-exit1` standing 2: architecture `:137`, `:360`.
  4. architecture §Infrastructure Patterns → Crate dependency direction (the root's `libc` `sigaction` and
     `windows-sys` `SetConsoleCtrlHandler` roles) — carried: Dependencies. `root-ffi` standing 1: registries
     `contracts/architecture/crate-dependency-direction.md:11`.
  5. security-plan §Data Protection (Interactive probe dirs and Probe captures removed on a stop, with the stated
     exceptions; the Code-bearing artefacts clause owing the `--settings` move retired) — carried: Symbols / APIs.
     Searches `verify-dirs` (security-plan `:131`, `:282`, `:283`, `:291`) and `owed-literals` (security-plan `:278`).
  6. obs-plan §6, the `detail` code catalog (`interrupted` on `process-exit` for `verify`, 12 schema members; its
     Decisions Log entry by the D-35/D-36 template — after U35 the record is the sidecar entry) — carried: Schema /
     config. `detail-enum` standing 6, as listed under Counts.
  7. obs-plan §4 Edge flows (the `verify` bullet's interrupted exit) — carried: Symbols / APIs. Sites: `verify-output` /
     `unable-pair` obs-plan `:739`, `verify-dirs` obs-plan `:740`.
  8. design-system §Surface: cli → Exit-code phraseology and Component Patterns 5 (the interrupted pair, a planning form
     on the operator's word, the founder's wording owed, `inputs#I2`) — carried: Symbols / APIs, `verify`.
     - `unable-pair` (standing 42 over seven masters) in design-system: `:24`, `:724`, `:750`, `:777`–`:788`, `:818`,
       `:826`, `:834`.
     - `verify-output`: design-system `:826`.
  9. layout-templates §Surface: cli, Output structure — `viola verify` (the interrupted form beside exit 0 / 1) —
     carried: Symbols / APIs.
     - `verify-output`: layout-templates `:473`, `:496`, `:598`.
     - `unable-pair`: layout-templates `:520`, `:535`, `:539`, `:541`–`:545`, `:588`, `:590`.
  10. a11y-plan §1 cli Notes and §8 Error recovery (the pair; the stop path adds no time bound) — carried: Symbols /
      APIs. `unable-pair`: a11y-plan `:90`, `:838`.
  11. test-plan §5 Integration Test Strategy, CLI (the `tests/cli_verify.rs` bullet gains the stop table; the root
      `WITHIN` count by its rule) — carried: Harness / gate surface and Counts (the count is unchanged).
      - `cli-verify-tests` standing 3: test-plan `:656`, obs-plan `:741`, registries
        `contracts/architecture/ci-cd-approach.md:20`.
      - `within-count` standing 2, as under Counts.
  - Not in the plan's list, carried under Counts: the Windows mutation matrix count (security-plan `:357`, test-plan
    `:1163`).
  - The P5 card row's Windows delivery owner → P5 route-resolve (a CARRY on "Windows-only live measurements").
- **Coverage of new surfaces:**
  - `SIGINT` / `SIGTERM` / console control event → `viola verify`: validation ✓ (a closed decision, `stops_on` 0 | 1;
    Unix `SIGINT` / `SIGTERM` only) · instrumentation ✓ (`process-exit{detail:"interrupted"}`, the spawn pairs kept) ·
    PII n/a · tests unit + integ (`stop.rs` cases, `typed.rs` poll / ending cases, `version_gate.rs` stop cases, the
    `cli_verify` SIGINT / SIGTERM table) · a11y ✓ (the pair is plain ASCII, `assert_plain`; one `unable:` and one
    `hint:` line, pattern 2) · tokens n/a.
  - The interrupted stderr pair (CLI) → validation n/a · instrumentation ✓ · PII n/a (fixed text, no path or pid) ·
    tests integ (exact bytes) · a11y ✓ · tokens n/a.

## Deviations from intent
- **Where the pair is printed.** Plan step 5 says `measure` prints the pair. `measure` returns `Ended::Interrupted`
  instead, and `verify` prints it before logging the self exit. The lines, their order and the exit are the same.
- **`run_bounded`'s stop.** It is a `fn() -> bool` input read before the child's exit on every poll, and `Bounded`
  carries `stopped`. This makes the stopped outcome deterministic and distinct from the deadline's kill.
- **One stop read covers the interactive runs.** The four runs run in one loop, with one stop read before each, so no
  run starts after the stop. After Run D, `verify`'s post-drop read applies.
- **No paste after the stop.** `Run::paste_text` refuses a paste once the stop is set, and `ctrl_c_ending` reads the
  stop before its first Ctrl-C too. Together these hold the "no byte to a PTY child on the stop path" criterion even
  for a stop between a wait's success and the next paste.
- **The helper returns the pid.** `verify_started` returns the pid with the guard and the receipt: the case's
  `libc::kill` needs it. `tests/support/cli.rs` is untouched.
- **Unit cases beyond step 7's list**, each pinning a stop path:
  - `run_bounded_with_the_stop_set_reads_stopped` (every OS) and `never_stopped_is_never_set`;
  - `poll_until_ends_at_a_stop_set_while_it_waits`;
  - `wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition`;
  - `ctrl_c_ending_at_the_stop_sends_no_more_and_kills`.
- **No new `Instant::now() + WITHIN` site.** The receipt watch uses the existing `fake::wait_for`, so the plan's note
  that the stop case moves the root count did not happen (measured 28 / 20, unchanged).
- **Entry 9, the preservation guard, reads red on `.github/workflows/windows-mutants.yml`.** The repository's contract
  test forced that one matrix line. The operator approved it at implement («Add the workflow line — one matrix line,
  nothing dispatched; surface entry 9 as a plan defect at the wrap», 2026-10-13, given here), and nothing was
  dispatched. The operator's direction at this wrap, verbatim: «Entry 9 (the preservation guard) reading red on
  windows-mutants.yml is recorded as a deviation: the repo's contract test forced that one matrix line, the operator
  approved it, nothing was dispatched. Do not edit the guard.»
- **Scope record** (`gate.py scope`: `clean — changed 8 · listed 7 · recorded 1 (companion 1 · mechanical 0 ·
  in-intent 0 · widening 0)`):
  - companion: `.github/workflows/windows-mutants.yml` · serves `src/cmd/verify/stop.rs` · word: «Add the workflow line
    — one matrix line, nothing dispatched; surface entry 9 as a plan defect at the wrap» — the operator, 2026-10-13.

## Decisions & corrections
- The operator's ruling at implement on the plan defect: the companion `windows-mutants.yml` matrix line, nothing
  dispatched, entry 9 surfaced at the wrap (2026-10-13, given here; `scope-record.md`).
- The operator's ruling on the CI re-run: «the founder's no-dispatch direction covers the mutation workflow, not a
  re-run of the ordinary CI job a runner never picked up» (2026-10-13, given here). `gh run rerun 38453057445
  --failed` followed.
- The operator's wrap direction: entry 9 is recorded as a deviation, and the guard is not edited (2026-10-13, the
  `/andromeda-wrap-session` invocation).
- A sweep hazard, measured. A new product file with a `cfg(windows)` gate needs a `windows-mutants.yml` matrix line:
  `tests/contract_windows_mutation_scope.rs`, run inside `pre-push`, reads red `unlisted: <path>` otherwise. A plan
  guard that freezes workflows contradicts it. P5's control for the guard minted `stop.rs` without running that test
  (`p5-controls.md`).
- A sweep hazard, measured. A `journal.md` line appended into the committed run dir after the pre-CI commit dirtied
  the tree, so the push entry's `git diff --quiet && git diff --cached --quiet` guard refused (exit 1, history
  unmoved). The line was set aside, the file restored to HEAD, the push re-fired green, and the line put back.
  Journal an operator-pass step only after the push, or outside the tree.
- A CI reading, measured. A job GitHub never ran reads `cancelled` with `steps: 0`, an empty `runner_name` and the
  annotation «The job was not acquired by Runner of type hosted even after multiple attempts»; its log download
  answers 404 `BlobNotFound`. The cause is the runner, read from the check run's annotations
  (`gh api …/check-runs/<id>/annotations`), never from the log.
- A sweep hazard, measured. A grep of nextest status lines with `viola::(…)?[^ ]*name` cannot match an integration
  test: its line is `viola::<binary> <test name>`, with a space between the two. The first read returned no stop-table
  line on the Ubuntu job until the pattern named `viola::cli_verify verify_stopped…`.
- A process attribution, measured. A `sleep 5` in the census belonged to another project's session (its parent bash
  polled an andromeda-pulse task file). Decide by the parent's command line and `/proc/<pid>/exe` against the
  repository root.
- A guard refusal: the Bash guard's `cd` arm refused a `cd` into the session scratchpad once. It was re-issued with
  absolute paths.
- The remove-the-guard controls (testing.md 2026-09-25) ran as a scripted neutralise → run → restore-in-`finally`,
  with the restore read back by sha256 (`evidence/guards.md`). Each of three guards read red on both cases with its
  own panic line.
- The operator's invocation directions at implement held throughout (`inputs#I1`): no live `claude` start, no mutation
  or witness run, no workflow dispatch, signals only at processes the test started, record times from `date -u` in
  the same call, pipeline-tool output to a file read whole, and each long gate run journalled before the wait.

## Outcome
- **Acceptance criteria, against the diff:**
  - (security / arch) The SIGINT and SIGTERM cases exit by themselves. The print probe's, Run A's and Run B's dirs are
    gone; the planted `.viola-verify-0`, `.viola-verify-0-plan`, `viola-verify-foreign` and `ledger/probes/0/` stand
    with their bytes; nothing is removed by a pattern or a listing (the guards are the dirs' own drop guards) — met.
  - (layouts / design / a11y) stdout is empty, stderr is exactly the pair (plain), and the exit is 1 — met.
  - (arch / tests) No `ledger/stamps.json` — met.
  - (obs) Every spawn pair is closed, the log ends `process-exit{subject:"self", exit_code:1,
    detail:"interrupted"}`, no `panic` line, and every line validates against the 12-member schema — met.
  - (security) No byte is written to a PTY child after the stop: `paste_text` and `ctrl_c_ending` read it. The child is
    killed and the pump joined before the dir drops, and `ended_by` observes no stop (unit case) — met.
  - (tests) The Windows routine, called directly on `windows-2025`, passed its three cases (CI job 115488617598);
    event delivery is not claimed — met.
  - (arch / security) The uninterrupted run is unchanged (`cli_verify` 72 passed, `cli_version_gate` green), and the
    literal probe reads `last line 0` — met.
  - (arch / security) No dependency, feature, seam or environment variable is added; `Cargo.toml`, `Cargo.lock` and the
    fake agent are unchanged; `cargo deny check` is green — met. The guard's command as written reads red on the one
    workflow file, which is the deviation above, not a dependency, feature, seam or variable.
  - (tests) `run --unit` reads `"ok":true`; `pre-push` reads `"ok":true` at `linux-tests`; CI reads `verdict: green` on
    HEAD `c22161e791ff` across `windows-2025`, `macos-latest` and `ubuntu-latest` (ci#38453057445, attempt 2) — met.
  - (tests) No mutation run, witness run, workflow dispatch or live `claude` start (the CI re-run of the ordinary job
    was on the operator's word, not a dispatch) — met.
- **Gates** (the final whole-block run at implement; the light gate re-runs them at P7):
  - green:
    - `cargo fmt --all --check`;
    - `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
    - `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings`;
    - `rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .` (exit 1, no output);
    - `bash scripts/lint-probes.sh`;
    - `bash scripts/agent-run.sh run --unit`;
    - `bash scripts/agent-run.sh run --integration --filter 'binary(cli_verify) | binary(cli_version_gate) |
      binary(contract_diag_schema)'` (72 passed);
    - the literal probe `python3 -c …` (last line 0).
  - red: `git diff --quiet d9f01b1b8642 -- . ':!src/cmd/verify.rs' …` — `exit 0 ✗ (exit 1)` on
    `.github/workflows/windows-mutants.yml` alone. This is the operator-recorded deviation, not a not-this-chunk's red.
  - green:
    - `cargo deny check`;
    - `bash scripts/agent-run.sh cleanup --session p-ivc-smoke`;
    - `… boot --session p-ivc-smoke --instance builder`;
    - `… status --session p-ivc-smoke`;
    - `bash scripts/g2-zero-panics.sh`;
    - `bash scripts/agent-run.sh schema-check`;
    - `… cleanup --session p-ivc-smoke` (`processes_gone`, `endpoint_gone` true);
    - `bash scripts/agent-run.sh pre-push` (`"ok":true`, `linux-tests`).
  - `leg = 'operator'`, recorded in `evidence/operator-pass.md`:
    - `gate.py hygiene`: `hygiene: clean`;
    - the guarded push: green on its second firing, `c22161e7` pushed;
    - `ci.py conclusion --sha HEAD --wait 1800`: `verdict: green · checks 15/15 · attempt 2 · ci#38453057445`.
  - Smoke: the boot path, which stamps through an uninterrupted `viola verify`, is green as P2 gates; it was not
    re-driven at P3.
- **Watches:** none.
- **Outcome basis.** The operator pass ran: `c22161e` is the one commit since `d9f01b1`, and CI attempt 2 on it is the
  verdict of record (`evidence/operator-pass.md`). This window holds the implement run whole; its persisted report
  is `.andromeda/runs/2026-10-13T18-48-52-implement/outcome-2026-10-13-interrupted-verify-cleanup.md` (outcome
  `surfaced`).
- **Process hygiene:**
  - From implement P4's census: the gate entries' processes, the `p-ivc-smoke` harness session (cleanup
    `processes_gone:true`), the three remove-the-guard runs and the unit case's `/bin/sleep 30` were all terminated.
    The viola-lab prototype `viola` and the `claude` processes are not this run's.
  - Re-measured at this wrap: 0 processes whose `/proc/<pid>/exe` lies under the repository root.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: d9f01b1b (the parent of the oldest pre-CI commit c22161e7) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/windows-mutants.yml — added 3 line(s) in 1 range(s)
added: 48-50
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 82
### src/cmd/verify.rs — added 52 line(s) in 22 range(s)
added: 7-9 · 11 · 29 · 69-78 · 80 · 85-90 · 92-94 · 105 · 121-124 · 127 · 145 · 152 · 159 · 179-181 · 187 · 196
       204-206 · 222 · 228-230 · 242-244 · 266 · 269-270
  - 69-78 «let (code, detail) = match measure(&home, args)? {»
- 85-89 @86 «enum Ended {»
  - 179-181 «if answer.stopped {»
  - 204-206 «if printed.stopped {»
  - 228-230 «let Some(typed) = typed else {»
  - 242-244 «if stop::stopped() {»
### src/cmd/verify/stop.rs — new file · 119 line(s)
- 16-20 @18 «pub(super) fn install() {»
- 22-25 @23 «pub(super) fn stopped() -> bool {»
- 27-30 @28 «pub(super) fn set() {»
- 32-36 @34 «fn stops_on(event: u32) -> bool {»
- 38-41 @39 «extern "C" fn on_signal(_: libc::c_int) {»
- 43-56 @44 «fn install_os() {»
  - 45-55 «for signal in [libc::SIGINT, libc::SIGTERM] {»
- 58-67 @61 «extern "system" fn console_routine(event: u32) -> windows_sys::core::BOOL {»
  - 63-65 «if stops {»
- 69-75 @70 «fn install_os() {»
- 77-119 @78 «mod tests {»
  - 83-90 @84 «fn stops_on_reads_ctrl_c_and_ctrl_break_only() {»
  - 92-103 @97 «fn install_turns_the_signal_into_the_stop_with_the_process_alive(#[case] signal: libc::c_int) {»
  - 105-118 @110 «fn console_routine_stops_on_ctrl_c_and_ctrl_break_and_leaves_close_to_the_os(»
### src/cmd/verify/typed.rs — added 240 line(s) in 47 range(s)
added: 11-13 · 31 · 53 · 59 · 65-67 · 69-74 · 78-91 · 93-99 · 101 · 105 · 136 · 155 · 167 · 171 · 196 · 202 · 204
       217-225 · 227-229 · 231-232 · 241-250 · 279 · 283-289 · 291 · 315 · 318 · 331 · 334 · 376-377 · 383 · 385-387
       586-587 · 590 · 595-598 · 600 · 628 · 682 · 690-696 · 698-703 · 707 · 709-715 · 738-739 · 744 · 748-753 · 760-761
       1157 · 1173-1281
  - 78-83 «let runs: [&RunStep<'_>; 4] = [»
  - 84-89 «for run in runs {»
- 241-249 @242 «fn wait_for(»
  - 283-289 «let submitted = wait_capture(»
  - 385-387 «poll_until(clock, until, &stopped, |now| {»
  - 595-598 «if stopped() {»
  - 690-693 @691 «fn paste_text(&self, text: &str) -> bool {»
  - 1173-1198 @1177 «fn ctrl_c_ending_at_the_stop_sends_no_more_and_kills(»
  - 1200-1210 @1201 «fn poll_until_returns_at_once_at_the_stop_without_asking() {»
  - 1212-1222 @1213 «fn poll_until_ends_at_a_stop_set_while_it_waits() {»
  - 1224-1246 @1227 «fn poll_until_without_the_stop_waits_to_its_answer_or_its_deadline(»
  - 1248-1265 @1249 «fn wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition() {»
  - 1267-1280 @1269 «fn ended_by_still_waits_with_the_stop_set() {»
### src/run/version_gate.rs — added 79 line(s) in 14 range(s)
added: 9 · 28-29 · 33 · 38-40 · 47 · 63-67 · 69 · 72-73 · 78-92 · 142 · 329 · 333 · 351 · 356-399
  - 78-82 «Ok(Bounded {»
- 85-88 «fn end(child: &mut Child) {»
  - 356-373 @359 «fn run_bounded_with_the_stop_set_reads_stopped() {»
  - 375-393 @379 «fn run_bounded_with_the_stop_set_kills_and_reaps_a_running_child() {»
  - 395-398 @396 «fn never_stopped_is_never_set() {»
### tests/cli_verify.rs — added 137 line(s) in 1 range(s)
added: 808-944
- 808-822 @810 «fn names(dir: &Path) -> Vec<String> {»
  - 811-819 «let mut names: Vec<String> = fs::read_dir(dir)»
- 824-834 @826 «fn plant_foreign(dir: &Path) {»
  - 828-832 «fs::DirBuilder::new()»
- 836-943 @845 «fn verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1(»
  - 854-859 «let foreign = [»
  - 860-862 «for dir in &foreign {»
  - 864-871 «let started = support::verify::verify_started(»
  - 872-874 «let lines = fake::wait_for(&started.receipt, "Run B's pasted prompt", |l| {»
  - 882-885 «assert_eq!(»
  - 888-891 «let cwds: Vec<PathBuf> = of_kind(&lines, "cwd")»
  - 893-895 «for dir in &cwds {»
  - 898-905 «for dir in &foreign {»
  - 908-909 «let role =»
  - 910-918 «let shape: Vec<(&str, &str)> = role»
  - 919-933 «assert_eq!(»
  - 937-939 «let validator =»
  - 940-942 «for (n, line) in role.iter().enumerate() {»
### tests/support/verify.rs — added 50 line(s) in 3 range(s)
added: 7 · 14 · 444-491
- 444-449 @445 «pub struct StartedVerify {»
- 451-490 @455 «pub fn verify_started(»
  - 463-466 «let receipt = home»
  - 472-484 «let child = Command::new(VIOLA)»
  - 485-489 «StartedVerify {»
