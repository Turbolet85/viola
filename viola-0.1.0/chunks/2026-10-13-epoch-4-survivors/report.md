# Report — 2026-10-13-epoch-4-survivors

**Chunk:** Epoch 4's 47 mutation survivors disposed by owning-package test, restatement or not-measured-here record
**Date:** 2026-10-13
**Commits:** `ad7fb3c` chore(2026-10-13-epoch-4-survivors): operator pre-CI commit, for the run this chunk's verdict
reads (the only commit since the last wrap's `c6e1301`; `git log --since 2026-10-13T11:06:47Z`)

## Changes (structured — detectors read this)
- **Files:** `crates/viola-pty/src/lib.rs` · `crates/viola-agent-claude/src/statusline.rs` ·
  `crates/viola-state/src/fs.rs` · `crates/viola-state/src/board.rs` · `src/run/wheel.rs` · `src/run/dialog.rs` ·
  `src/cmd/hook.rs` · `src/cmd/send.rs` · `src/run/send/slot.rs` · `src/cmd/verify/typed.rs` (test module only). These
  are the ten files research listed (`gate.py scope`: changed 10 · listed 10 · recorded 0). No new source file.
  Evidence is in `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors/evidence/` (`survivors.ndjson`, `survivors.md`,
  `operator.md`).
- **Symbols / APIs:**
  - **`viola_state::fs`** (`crates/viola-state/src/fs.rs`):
    - New private fn `replace_private_retrying(path, bytes, mode, pause: &mut dyn FnMut(), retry: &dyn Fn(Option<i32>)
      -> bool)` holds the replace loop that `replace_private_with` held. The retry guard reads `attempts <
      REPLACE_ATTEMPTS && retry(e.error.raw_os_error())`.
    - `replace_private_with` keeps its signature (`pub(crate)`). Its body is one call:
      `replace_private_retrying(…, &|raw| retry_replace(raw, HOST_IS_WINDOWS))`.
    - Behaviour is unchanged: the mode is set before any byte, `sync_all`, `persist`, at most `REPLACE_ATTEMPTS` = 100
      attempts, and only raw OS error 5 on a Windows host is retried (`retry_replace` and the consts are unchanged).
    - Its remaining callers are unchanged and still call `replace_private_with`: `replace_private` (`fs.rs`) and
      `write_snapshot_with` (`crates/viola-state/src/snapshot.rs`, untouched by this chunk — the preservation guard).
  - **`src/cmd/send.rs`:** new private fn `issue_line_shows(json: bool, terminal: bool) -> bool` (`!json &&
    terminal`). `Out<'_>::open` calls it with `io::stdout().is_terminal()`. The `[  ] open` line's text, stream
    (stderr) and timing are unchanged.
  - **`crates/viola-pty/src/lib.rs`, test module only:** `report_paths` names the two real-PTY report files
    `<temp dir>/viola-pty-watch/<test name>.<pid>.report` and `<test name>.<pid>.test.report`. `<pid>` is
    `std::process::id()` of the test process; until now they were named after the test alone. Their keep-on-failure
    and remove-on-pass behaviour is unchanged.
  - **New test-only path:** `src/cmd/hook.rs`'s new case makes a `tempfile` dir `viola-answers-*` under
    `<current dir>/target/` and reaches it by the relative path `../<current dir's name>/target/<tmp>/answers`. The dir
    is removed on drop.
  - **New or changed unit cases**, each in the package that owns the file it kills (ranges from *New text, by line*):
    - `statusline.rs` — `statusline_shell_is_bin_sh_with_the_command_as_one_argument` (changed: the whole value,
      `Some(["/bin/sh", "-c", command])` on Unix, `None` elsewhere).
    - `fs.rs` — `replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` and
      `replace_private_retrying_returns_at_once_when_retry_refuses`, each over a target that is a non-empty directory.
    - `board.rs` — `board_read_sums_the_skip_counts_over_every_instance` (changed: an unlisted key planted in both logs,
      `unknown_fields: 2`), `board_scan_refuses_a_home_that_cannot_be_statted_and_writes_no_line` and
      `board_scan_of_an_instances_dir_that_cannot_be_listed_is_an_error` (both `cfg(unix)`, mode 000, restored before
      drop).
    - `wheel.rs`:
      - 4 typing rows: `sgr_mouse_one_past_the_bound`, `dcs_ended_by_bel`, `osc_one_past_the_bound`,
        `kitty_flags_with_two_fields`.
      - 4 reply rows: `sgr_mouse_at_the_bound`, `osc_at_the_bound`, `da1_with_no_field`, `da2_with_no_field`.
      - `classifier_split_reads_carry_the_unfinished_sequence` (2 cases).
      - `wheel_pause_whose_record_cannot_be_appended_is_internal`.
      - `REPLIES` is unchanged.
    - `dialog.rs` — `continuation_survives_a_pre_tool_use_for_another_tool`,
      `millis_is_the_whole_milliseconds_capped_at_u64_max`.
    - `hook.rs` — `answer_reads_no_relative_answers_dir_that_resolves`.
    - `send.rs` — `issue_line_shows_only_for_a_human_on_a_terminal` (4 cases).
    - `send/slot.rs` — `send_expired_window_closes_the_send_so_a_late_line_is_not_claimed`.
    - `typed.rs` — `feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does`,
      `max_gap_leaves_out_a_chunk_after_the_window`.
- **Crates / modules:** none added or removed. Five packages changed: `viola`, `viola-state`, `viola-agent-claude`,
  `viola-pty`, and `viola-e2e` (none of its files — named here because its 2 `scratch.rs` survivors keep their record).
- **Dependencies:** none (`Cargo.toml` and `Cargo.lock` unchanged — the preservation guard, gate entry 9 green).
- **Schema / config:** none.
- **Spec-master edits:** none this chunk (implement edits no master).
- **Counts / qualifiers moved:** none — verified. The site sweep's four fired patterns found no count this chunk
  changed (`report-patterns.toml`; `cascade.py sweep --pre-pass` over the masters and registries: snapwriter standing
  1 · ptywatch standing 1 · notmeasured standing 1 · issueline standing 14). Each standing row is a statement about a
  mechanism, not a count; the Expected amendments lines below take them.
- **Dev-tool versions:** none.
- **Harness / gate surface:**
  - No harness verb, CI step or status shape changed. The chunk's gates are the plan's block: the ledger read (new),
    the `typed.rs` hunk guard (new) and the preservation guard (new) beside the standing entries.
  - **The mutation records**, for test-plan §10's Mutation gate paragraph (the Epoch 4 boundary audit's 47 survivors,
    `.andromeda/runs/2026-10-12T06-54-29-code-audit/record.json` `mutation.survivors`, now in
    `evidence/survivors.ndjson`):
    - 22 are disposed by a test and 7 by a restatement plus its test: `fs.rs` 6 and `send.rs` `Out::open` 1. All 29
      read red under the hand-applied mutant and green restored (`evidence/survivors.md`).
    - 2 `scratch.rs` `prepare` rows stay "not measured here", by the test-plan §10 record (run 38036448183).
    - 16 `typed.rs` rows (`framing_turns` 5, `SharedPty` 5, `Run::end_by_ctrl_c` / `wait_pump` / `finish` 6) are
      owed to the route entry "Epoch 4 survivors II".
    - No mutation run was made (`inputs#I1`): the Epoch 5 boundary audit's own run is the reader that confirms each
      kill. The plan's forecast for that run: the Linux whole-member runs read 29 fewer missed over these files.
  - **`fs.rs`'s five const-gated mutants** (`:281:18`, `:285:20`, `:285:29` ×3) now sit in a function a Linux run
    reaches: the retry decision is a parameter, so the loop's bound is exercised on every host.
  - **The kill filter form:** a hand-applied mutant's case runs through `bash scripts/agent-run.sh run --unit --filter
    'test(/(^|::)tests::{name}$/)'`. `test(=tests::{name})` selects only a crate-root module's inline tests (a
    `lib.rs` `mod tests`); for a submodule's tests (`cmd::hook::tests::…`, `board::tests::…`) nextest reads "no tests
    to run" (`nextest-exit-4`). As measured this chunk: 19 of 29 first firings selected nothing
    (`.andromeda/runs/2026-10-13T16-01-52-implement/journal.md`).
- **Cross-project / external claims:**
  - **CI:** run `ci#38430155254` (push, completed/success) measured `ad7fb3c1f7a8`. `ci.py conclusion`: verdict green,
    checks 15/15, wall 610 s (`evidence/operator.md`).
  - **The audit record** (in this repo): `.andromeda/runs/2026-10-12T06-54-29-code-audit/record.json`, its
    `mutation.survivors` (47 rows).
  - **`inputs.py verify`** (this wrap; I4 snapped at this wrap, before the report):
    - I1 · message · the operator, the /andromeda-phase invocation arguments, 2026-10-13T14:19Z · n/a · cited.
    - I2 · message · the founder, by dialog («Делить на две»), relayed verbatim by the operator, 2026-10-13 · n/a ·
      cited.
    - I3 · message · the operator, the P5 word on `.andromeda/runs/2026-10-13T14-19-57-phase/p5-card-1.md` ("yes. The
      card stands as written.") · n/a · cited here. Its card, `.andromeda/runs/2026-10-13T14-19-57-phase/p5-card-1.md`,
      `## Beyond the plan`, verbatim, the operator's word (I3) on a row resting on the founder's own answer (I2):
      > | 1 | A new working-route entry **"Epoch 4 survivors II"**, placed directly after "Epoch 4 survivors" under `### Epoch 5 — Driver surface`. It carries `typed.rs`'s 16 survivors by audit coordinate: `framing_turns` `:271:84` ×2, `:286:29`, `:288:40` ×2; `SharedPty` `resize` `:403:9`, `close` `:415:9`, `child_pid` `:419:9` ×3; `Run::end_by_ctrl_c` `:648:12`, `:651:12`, `:648:45`, `:651:45`; `Run::wait_pump` `:659:9`; `Run::finish` `:671:53`. It also carries research.md §Measured facts 4 and 5 (no `Pty` double seam; `SharedPty::resize` / `child_pid` uncalled by any product path) and the disposal forms of test-plan §10. It inherits `inputs#I1`'s direction (no mutation run, witness run or dispatch; no live `claude`) and rests on the founder's split (`inputs#I2`). | route insertion (the wrap mints it) |
    - I4 · message · the operator, the /andromeda-wrap-session invocation arguments, 2026-10-13T16:59Z · n/a · cited
      here. Mint "Epoch 4 survivors II" from that row, as the founder approved (`inputs#I2`); amend the three leaves
      the implement report names; the Bash guard now refuses a pipeline tool piped into a filter.
    - Summary as printed before I4: `inputs: 3 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0
      · unreachable 0 · n/a 3 · uncited 1 · unparsed 0` (the uncited one, I3, is cited by this bullet).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `.claude/rules/verification-harness.md` Session Additions 2026-09-27: "an inline unit test is `tests::name`".
    Measured false for a test outside a crate-root module, whose full path is `<module>::tests::name`: 19 filters of
    the form `test(=tests::name)` selected no test (`nextest-exit-4`; implement `journal.md`). This is a curation home,
    not a master: masters 0 (the `filterform` pattern's unfired-control refusal line). It routes to P3 curation as an
    in-place extension (the operator's direction, `inputs#I4`).
  - research.md's harness summary repeated that form. research.md is immutable; this records it, nothing to amend.
- **Expected amendments (from plan):**
  - test-plan §10 — the Mutation gate paragraph. Carried: **Harness / gate surface** (the mutation records). Search
    `notmeasured` (`not measured here`, ignore case): test-plan standing 1, `test-plan.md:1226` ×2 (@c2166, @c3159);
    other masters 0; leaf `.claude/docs/tests-summary.md:42`.
  - architecture §Established Decisions [Snapshot writer]. Carried: **Symbols / APIs** (`replace_private_retrying`).
    Search `snapwriter` (`replace_private_with|retry_replace|REPLACE_ATTEMPTS|replace_private_retrying`): architecture
    standing 1, `architecture.md:51` ×2; other masters 0; leaves `.claude/docs/services/viola-state.md:23`, `:24`.
  - architecture §Occupied Resources — the pty watch report named with the process id. Carried: **Symbols / APIs**
    (`report_paths`); branch b was taken (the Outcome, step 2). Search `ptywatch` (`viola-pty-watch|<test
    name>\.report|<test>\.report`): architecture standing 1, `architecture.md:415` ×2; other masters 0; leaves
    `.claude/docs/gotchas.md:111` and `.claude/docs/services/viola-pty.md:33`.
  - test-plan §3 `run` step 4 or §10 — two mutation jobs must not share a test's report file (HYPOTHESIS). Carried:
    **Harness / gate surface** and the Outcome's step-2 reading.
    - The attribution is recorded, not established: under the mutant alone all three tests read red, and the
      two-job run was not re-run (`inputs#I1`).
    - Search `mutjobs` (`cargo-mutants jobs|two jobs|JOBS = 2|-j 2|one TMPDIR|shared TMPDIR`): masters 0, the
      sweep's unfired-control refusal line ("mutjobs, filterform never fired over the seven masters' pre-pass text
      (ad7fb3c1)").
    - Where test-plan §10 already names the Linux mutation `TMPDIR` (`test-plan.md:1226`, the "On the Linux dev host
      every mutation run takes `TMPDIR=…`" sentence), the record sits beside it.
- **Coverage of new surfaces:**
  - `issue_line_shows` → validation n/a · instrumentation n/a (a pure decision, no span; `Out::open` unchanged) · PII
    n/a · tests unit · a11y n/a · tokens n/a.
  - `replace_private_retrying` → validation n/a · instrumentation n/a (no span on the replace before or after) · PII
    n/a · tests unit · a11y n/a · tokens n/a.
  - No new external surface, hot-path operation or UI element.

## Deviations from intent
1. **Step 8's relative path.** The answers dir is a tempdir under the current dir's `target/`, reached as
   `../<current dir's name>/target/<tmp>/answers`, not a tempdir under the system temp dir. A system temp dir on
   another Windows drive (`windows-2025`: the checkout on `D:`, the temp dir on `C:`) has no relative path from the
   current dir. The path is still built from `..` components and the tempdir's path, as the step says.
2. **Two reds are not a plain assertion.**
   - `board.rs:164:29` `+=`→`-=` reads red as `attempt to subtract with overflow` at the mutated statement. The sum
     starts at 0, so no case can reach an assertion under that mutant. Its `*=` twin reads red at the assertion
     (`unknown_fields: 0` against 2).
   - `board.rs:45:19` first read red at the case's own `.expect("read")`. The case was changed to assert the whole
     value, and re-read red at that assertion.
3. **The kill filter.** 19 of the 29 pairs first ran no test under the filter the research summary named; they were
   re-run under the regex form (**Harness / gate surface**). Only the re-run's readings are in `survivors.md`.
4. **Step 2 took branch b.** No new resize case was written. The three existing tests are the disposal, under
   per-process report names.

Scope record: none — `gate.py scope` clean, 0 recorded (changed 10 · listed 10 · recorded 0 · excluded 89).

## Decisions & corrections
- The operator's direction for the implement run, by the invocation:
  - no live `claude` start, no mutation run, witness run or workflow dispatch, and a kill shown by a hand-applied
    mutant;
  - pipeline-tool output to a file, read whole, and a scripted `.rs` write runs rustfmt in the same call;
  - each long gate run journaled in the run dir before the wait;
  - the operator pass with the `ci.py conclusion` read.
  All held: `journal.md` has five entries, and `evidence/operator.md` carries the pass.
- **The operator's word at this wrap (`inputs#I4`):**
  - Mint "Epoch 4 survivors II" from `p5-card-1.md` row 1, as the founder approved (`inputs#I2`) — owner P5.
  - Amend the three leaves the implement report names (`.claude/docs/services/viola-pty.md:33`,
    `.claude/docs/gotchas.md:111` — through the architecture amendment's cascade; the verification-harness rule's
    2026-09-27 entry — through P3, a curation home).
  - "The Bash guard now refuses a pipeline tool piped into tail, head, grep, cut, sed or awk: redirect to a file and
    read it whole." A standing rule now enforced by a guard. The same rule reached implement as "a rule landing
    soon" (`inputs#I2`) and is in the user memory (`pipeline-tool-output-to-file`).
- **Sweep hazard (a filter that selects nothing):** nextest `test(=tests::name)` matches only a crate-root module's
  inline tests. For a submodule's test it selects nothing, and the harness reads `nextest-exit-4` for the red AND the
  green run, so a hand-applied mutant script reads "red" falsely. Evidence: implement `journal.md` line 3; the first
  firing's `failures: ["nextest-exit-4"]`. The fix is `test(/(^|::)tests::name$/)`, and a red is valid only with a
  panic line and no `nextest-exit` failure.
- **A mutant with no assertion-level red:** `+=`→`-=` on a `u64` sum that starts at 0 panics with an overflow at the
  mutated statement under the test profile, whatever the case asserts (`board.rs:164:29`, `survivors.md`).
- **A relative path between two Windows drives does not exist:** a test that needs a relative path to a temp dir makes
  that dir under the current dir (step 8; `windows-2025` checks out on `D:`, with its temp dir on `C:`).
- **A recurrence of the standing learning "a time written into a record ahead of the clock":** the stamp-ahead hook
  refused the journal's first line (16:10Z written at 16:02:54Z); it was corrected to the read clock (implement P1
  evolve record, `guard-refused:1`).
- **Hand-applied mutant discipline that held:** apply by text, rustfmt in the same script, confirm the bytes changed,
  run, restore from a byte copy, confirm the bytes are equal, run again. 98 harness calls, every file restored
  (`git status` after each batch showed only the ten listed files).

## Outcome
**Acceptance criteria, against the diff:**
- **29 cases, red under their mutants and green restored — met.** All 31 disposed survivors have a nextest case in
  their owning package: 29 kills by test or restatement, plus the 2 `scratch.rs` records, which stand as test-plan §10
  holds. Each case reads red under the hand-applied mutant and green restored, with both readings in
  `evidence/survivors.md`. 27 of the 29 reds are the case's own assertion; `board.rs:164:29` `-=` reads red by the
  overflow at the mutated statement (Deviations 2). None is disposed by argument.
- **The ledger entry — met:** `last line 1` (gate entry 7 green). 47 rows: 22 `test` · 7 `restatement` · 16 `owed` ·
  2 `not-measured-here`, every `by` filled.
- **No mutation run, witness run or dispatch, and no live `claude` — met:** none was made; the 98 harness calls are
  `run --unit --filter`.
- **`replace_private_with`'s contract — met.** Its existing cases pass, the two Linux cases read the attempt count (99
  pauses) and the immediate return, and its two callers are unchanged (gate entry 9 green).
- **`shell_argv` — met:** exactly `["/bin/sh", "-c", command]` on Unix, `None` on Windows; the only shell-out is
  unchanged.
- **`board::scan` — met.** An unstattable home is `ScanRefused::Home`, an unlistable `instances/` is `Err`, and
  neither writes a line (`capture` reads none). `unknown_fields` is summed across instances, and the `skipped` shape is
  unchanged.
- **The classifier — met.** No row was added to `REPLIES`, the existing typing, reply and split tables pass unchanged
  (`run --unit` 1997 passed), and each split-read case asserts both reads' verdicts.
- **`viola send`'s issue line — met:** the condition is unchanged (`!json && terminal`), and no human output line
  changed.
- **`typed.rs` — met:** it changes only inside its test module (gate entry 8 `last line 0`).
- **No manifest, lockfile, env var or feature change — met:** the preservation guard and `cargo deny` are green.
- **G1, G2 and G4 — met:** green.
- **`run --unit`, `pre-push` and CI — met.** `run --unit` `"ok":true`, `pre-push` `"ok":true` at `linux-tests`, and CI
  `verdict: green` on `windows-2025`, `macos-latest` and `ubuntu-latest`, run `ci#38430155254`.

**Gates (implement's block, `.andromeda/runs/2026-10-13T16-01-52-implement/gate-2026-10-13-epoch-4-survivors.json`):**
- `cargo fmt --all --check` — green.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green.
- `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings` — green.
- the G1 `rg` — green (exit 1, no output).
- `bash scripts/lint-probes.sh` — green.
- `bash scripts/agent-run.sh run --unit` — green, 1997 passed, 0 failed.
- the ledger read `python3 -c …` — green, last line 1.
- the `typed.rs` hunk guard — green, last line 0.
- the preservation guard `git diff --quiet c6e130181e51 …` — green.
- `cargo deny check` — green.
- `agent-run.sh cleanup` → `boot` → `status` → `g2-zero-panics.sh` → `schema-check` → `cleanup` (session
  `p-e4s-smoke`) — all green; `processes_gone` and `endpoint_gone` true.
- `bash scripts/agent-run.sh pre-push` — green: `"ok":true`, `"stage":"linux-tests"`, coverage 2431 passed, playwright
  1. Re-fired before the pre-CI commit (firing 2, 66.81 s), green.
- `gate.py hygiene` (`leg = 'operator'`) — fired as written by the operator pass: `hygiene: clean`.
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — fired through
  `--operator 19`: green, history moved `c6e13018→ad7fb3c1` on the remote-tracking refs.
- `ci.py conclusion --sha HEAD --wait 1800` (`leg = 'operator'`) — fired as written: `ad7fb3c1f7a8 verdict: green ·
  checks 15/15 · wall 610 s · runs ci#38430155254 completed/success`.
- No `defer` entry and no deferral; the run's walk-class line read `rust 3 … · uncommitted 95`, and every gate ran.
- Smoke: the plan's smoke entries ran as P2 gates (boot, status, cleanup) and were not re-driven.

**Watches:** none folded on this entry. Scope §3's observation of the root package's Windows mutation baseline stays
not exercised: no `windows-mutants.yml` dispatch inside the chunk.

**Step 2, measured:** the `resize` → `Ok(())` mutant alone read red in all three resize tests, at 7.006–7.056 s
(`wait_line` at `CHILD_WITHIN`). No child report showed `size 120x40`; the full reports are in `survivors.md`.

**Outcome basis:**
- This window holds the implement run whole: `.andromeda/runs/2026-10-13T16-01-52-implement/`, its
  `outcome-2026-10-13-epoch-4-survivors.md` the same report.
- The operator pass ran in that window: one commit, `ad7fb3c` (`git log --format='%h %s' c6e13018..HEAD`), no fix
  commit.
- The final HEAD's CI run, `ci#38430155254` on `ad7fb3c1f7a8`, is recorded in `evidence/operator.md`.
- After the pre-CI commit only records changed: implement's `journal.md`, its `outcome-…md` and
  `evidence/operator.md`.

**Process hygiene:** implement's census holds. The harness session `p-e4s-smoke` was terminated by entry 16, and the
98 mutant harness calls and the `pre-push` children returned. A `/proc/<pid>/exe` census at this wrap (2026-10-13
17:01:53Z) found 0 processes whose binary is under this repository. The host's `viola` processes are the operator's
bridge (`viola-lab/prototype`), another tree's build. The kept mutant-red reports stand under
`$TMPDIR/viola-pty-watch/` by design.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: c6e13018 (the parent of the oldest pre-CI commit ad7fb3c1) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/statusline.rs — added 6 line(s) in 1 range(s)
added: 385-390
### crates/viola-pty/src/lib.rs — added 5 line(s) in 3 range(s)
added: 851-852 · 860 · 862-863
### crates/viola-state/src/board.rs — added 47 line(s) in 4 range(s)
added: 463-502 · 636-637 · 647-650 · 663
  - 463-467 @464 «fn set_mode(path: &Path, mode: u32) {»
  - 469-484 @472 «fn board_scan_refuses_a_home_that_cannot_be_statted_and_writes_no_line() {»
  - 486-501 @490 «fn board_scan_of_an_instances_dir_that_cannot_be_listed_is_an_error() {»
### crates/viola-state/src/fs.rs — added 69 line(s) in 3 range(s)
added: 273-286 · 298 · 423-476
  - 274-276 «replace_private_retrying(path, bytes, mode, pause, &|raw| {»
  - 423-429 @424 «fn occupied_target(tmp: &tempfile::TempDir) -> std::path::PathBuf {»
  - 431-439 «fn assert_untouched(tmp: &tempfile::TempDir, target: &Path) {»
  - 441-463 @442 «fn replace_private_retrying_tries_exactly_the_attempts_while_retry_holds() {»
  - 465-475 @466 «fn replace_private_retrying_returns_at_once_when_retry_refuses() {»
### src/cmd/hook.rs — added 25 line(s) in 1 range(s)
added: 1092-1116
  - 1092-1115 @1096 «fn answer_reads_no_relative_answers_dir_that_resolves() {»
### src/cmd/send.rs — added 19 line(s) in 3 range(s)
added: 117-121 · 124 · 282-294
- 117-120 @118 «fn issue_line_shows(json: bool, terminal: bool) -> bool {»
  - 282-293 @287 «fn issue_line_shows_only_for_a_human_on_a_terminal(»
### src/cmd/verify/typed.rs — added 18 line(s) in 1 range(s)
added: 832-849
  - 832-842 @833 «fn feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does() {»
  - 844-848 @845 «fn max_gap_leaves_out_a_chunk_after_the_window() {»
### src/run/dialog.rs — added 23 line(s) in 1 range(s)
added: 743-765
  - 743-758 @746 «fn continuation_survives_a_pre_tool_use_for_another_tool() {»
  - 760-764 @761 «fn millis_is_the_whole_milliseconds_capped_at_u64_max() {»
### src/run/send/slot.rs — added 22 line(s) in 1 range(s)
added: 261-282
  - 261-281 @264 «fn send_expired_window_closes_the_send_so_a_late_line_is_not_claimed() {»
### src/run/wheel.rs — added 41 line(s) in 4 range(s)
added: 620-623 · 637-640 · 645-659 · 934-951
  - 645-658 @649 «fn classifier_split_reads_carry_the_unfinished_sequence(»
  - 934-950 @937 «fn wheel_pause_whose_record_cannot_be_appended_is_internal() {»
