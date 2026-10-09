# Codebase Research — 2026-10-09-epoch-3-cleanup

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 34 (the take-up); the revision of 2026-10-09T16:35Z added 14
  reads and 4 greps, listed under "The revision" below
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 10 Session Additions (never pipe
  `boot`; a mutation run's `TMPDIR` short and NOCOW on this host; judge a mutation run by its verdict, never by counts
  against another run; a `test(=name)` filter needs the full path; a stalled-start red is a finding about the backing).
  `.claude/rules/testing.md` — read whole, 35 Session Additions (a kill line moves only for a floor that is by design,
  with a planted-hang control, 2026-09-28 ext. 2026-10-05; a root test with no window prefix runs under the `mutants`
  profile's 10 s kill, 2026-09-24 ext. 2026-10-06; no `[[gate]]` entry holds a mutation run, 2026-10-04; a live
  round's product command runs with the bridge's `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed and `--home` given,
  2026-10-07; every new guard test carries its remove-the-guard run, 2026-09-25). `.claude/rules/host-linux.md` — whole.
- **Platform issues consulted:** the red job's own log, `gh api repos/Turbolet85/viola/actions/jobs/113260250076/logs`
  (run 37761947926 on `e304994`), read 2026-10-09: line 1216 `TIMEOUT [  30.009s] (250/250) viola-e2e::harness_lifecycle
  boot_with_an_unknown_cli_version_is_verify_failed`, line 1225 `Summary [  43.985s] 250 tests run: 249 passed (1 slow),
  1 timed out, 0 skipped`, line 1232 `"reason":"mutants-exit-4"`. The failure's signature is this repository's own
  nextest kill line (`.config/nextest.toml:45-47`, 15 s × 2) and reads the same on the dev host (30.003 s, the audit's
  reading 1). It is not a hosted-platform failure, so no tracker or release-note search applies to it.
- **External inputs:** `inputs#I1` — the operator's direction at take-up (the cap of 5 live starts on 2.1.287 by path,
  the kill form as a fork, the two stale citations folded, a priced split card at P4). `inputs#I2` — the host-pressure
  reader `hostwatch.py` in the overseer's repository, a pointer at its commit `7d315d4e`.
  `inputs#I4` — the review's direction: the live sessions run headless on a plain pty the rig opens itself,
  rehearsed under the fake agent before any live start.
  `inputs#I3` — the P4 fork answers: the split in two (the founder's word), the `verify_window_` name, the
  `empty-text` detail with its hint (the founder's word) and its rung (the overseer's), every stale citation site.
  `inputs#I7` — the operator's word at the revision: implement stopped on the plan; the `artifact` key leaves the
  two readers of the live records; one lone-inner-CR reading is added as start 4 of 5 on the changed build, with no
  behaviour change; steps 1 to 15 stand as done.

## Files inspected
- `crates/viola-core/src/lib.rs` (100-194) — `RefusalReason` (5 values) and the closed `NotDelivered` enum (5 variants:
  `InputNotReady`, `NoPromptSubmitted`, `TurnRunning`, `UnknownDialog`, `ControlCharacter`, `:122-128`) with `as_str`
  (`:131-139`); `validate_paste_text` (`:181-187`) refuses on `is_refused_control` alone and is shared by `send` and
  `answer`.
- `crates/viola-agent-claude/src/hook.rs` (202-204, 640-672, 717-728) — `typed_text` is `text.trim_end_matches('\n')`;
  its rstest table `typed_text_drops_every_trailing_newline_and_nothing_else` holds nine cases, one of them
  `a_cr_before_it_stays("x\r\n", "x\r")` and one `only_newlines("\n\n", "")`.
- `src/run/send.rs` (62-68, 124-129, 299-371, test names) — the wrapper's `send`: `validate_paste_text` at `:318`, the
  typed text taken once at `:321`, then the wheel, `classify(text, …)` at `:329`, the in-flight text at `:337`,
  `ctx.issue(text.len())` at `:360`, `paste(text)` at `:361`. `SendSlot::claim` compares `f.text == text` (`:129`).
  Six unit tests carry `trailing_newline` in their names (`:924`, `:930`, `:948`, `:1525`, `:1548`, `:1568`).
- `src/cmd/send.rs` (75-82, 136-239) — the client: `read_text` (over the cap or non-UTF-8 is exit 2), `deliver` runs
  `validate_paste_text` at `:187` before any frame and logs `send-refused{side:"client"}` through `refused_client_side`
  (`:231-239`); `Out::unable` picks the hint by detail through `human::send_hint` (`:149`).
- `src/human.rs` (192-228, 396-416, 528-542) — `send_hint` is keyed by cause (five causes and `human-typing`); three
  unit tests hold the cause list (`send_hint_is_the_design_string`, `write_send_unable_…`, `no_hint_names_release`).
- `schemas/diag-line.v1.json` (59-62, 102-103) — `send-refused.detail` is `{"type":"string"}`, not an enum; the closed
  detail enum at `:103` is `parse-rejected`'s.
- `.config/nextest.toml` (whole, 59 lines) — `[profile.mutants]` 5 s × 2; its overrides in order: `package(viola-e2e)`
  15 s × 2 (`:45-47`), `test(/send_window_/)` 15 s × 2 (`:50-52`), `test(/verify_window_/)` 15 s × 3 (`:56-58`).
  `[profile.ci]` 30 s × 4 with `test(/verify_window_/)` 20 s × 3 as its first override (`:25-27`).
- `crates/viola-e2e/tests/harness_lifecycle.rs` (225-304) — the killed test at `:265`: `boot` with `cli_version`
  `9.9.9` over a scratch workspace, asserting `reason: verify-failed`, `exit_code: 1`, no supervisor spawned. The file
  holds 8 tests (`grep -c '^#\[test\]'`).
- `crates/viola-e2e/src/harness/boot.rs` (grep) — boot step 4 runs `viola verify -- … --cli-version <v> --fixtures …
  --screens …` (`:186-199`); `INSTANCE_DEADLINE` 20 s.
- `src/cmd/verify/typed.rs` (470-515) — `settled`: a quiet screen with a signature literal settles at once, one with
  none settles only at `from + GATE_MAX_WAIT`.
- `crates/viola-agent-claude/src/screen.rs` (grep) — `GATE_MAX_WAIT` is 8500 ms (`:16`).
- `tests/contract_lints.rs` (196-270) — `test_deadlines_sit_below_the_nextest_kill_line` reads the first
  `[profile.mutants.overrides]` table and asserts it holds the line `filter = 'package(viola-e2e)'` (`:228-234`).
- `crates/viola-state/src/events.rs` (84-218), `stamps.rs` (14-43), `strict.rs` (22-51), `fs.rs` (255-304) — the six
  viola-state survivor sites, each at the audit's coordinate.
- `crates/viola-agent-claude/src/ledger.rs` (636-690, 866-913, 1150-1188) — the six survivor sites; `dialog_variants`
  at `:874-913`.
- `src/bin/viola-fake-agent.rs` (318-492) — `close_hooks` (`:327-329`), `run_hook` (`:395-442`), `submit` (`:452-…`).
- `src/cmd/mod.rs` (118-147), `src/cmd/hook.rs` (190-229) — `cli_sink` and `handle_dialog`'s oversize check.
- `tests/cli_answer.rs` (50-92, 128-166), `tests/cli_wait_last.rs` (126-164), `tests/support/mod.rs`,
  `tests/support/piped.rs` (1-40) — the scaffolding copies and the shared module.
- `crates/viola-e2e/src/harness/supervise.rs` (56-67, 118-126), read at the P5 review for `inputs#I4` — the form the
  harness boots `viola run` on: `--home <home> run <name> -- <fake agent> --cli-version <v> --fixtures <dir>
  --screens --trusted-root <cwd>` on an outer pty, and its stop, Ctrl-C written to the pty every 500 ms until the
  wrapper exits. `target/harness/debug/viola-fake-agent` stands on the host.
- The audit's `record.json` (`commands`, `tool_versions`) and `proposals.md` (whole); the 2026-10-08 chunk's
  `evidence/live-readings.ndjson`, `live-sessions.ndjson`, `live-start.sh`, `local-live.md`, `live-drive.py` (head).
- `hostwatch.py` at `inputs#I2` (865-945) — verbs `read`, `start`, `stop`, `status`; flags `--from`, `--to`, `--last`,
  `--trail`, `--for`.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-09-epoch-3-cleanup.json`, plane `rust`)
- **`typed_text`** — 1 product caller: `send` @ `src/run/send.rs:321` (and the import at `:19`); 2 test callers in
  `crates/viola-agent-claude/src/hook.rs` (`:670`, `:814`). The strip has one site.
- **`validate_paste_text`** — 4 product callers: `deliver` @ `src/cmd/send.rs:187`, `send` @ `src/run/send.rs:318`,
  `deliver` @ `src/cmd/answer.rs:105`, `answer` @ `src/run/dialog.rs:329`. A check placed inside it reaches `answer`.
- **`NotDelivered`** (type) — 33 reference sites in 5 files: `crates/viola-core/src/lib.rs` 13, `src/run/send.rs` 9,
  `src/cmd/send.rs` 6, `src/run/dialog.rs` 4, `src/cmd/answer.rs` 1 (query 3, `rows` 5).
- **`send_hint`** — 3 product callers: `unable` @ `src/cmd/send.rs:149`, `unreachable` @ `src/cmd/send.rs:157`,
  `unable_unreachable` @ `src/cmd/client.rs:154`; 6 test call sites in `src/human.rs`.
- **`dialog_variants`** — 1 product caller: `record` @ `src/cmd/verify.rs:401`; 4 test call sites in `ledger.rs`.
- **`record`** (`src/cmd/verify.rs`) — 1 caller: `measure` @ `src/cmd/verify.rs:234` (query 2, `rows` 1).
- **`cli_sink`** — 6 call sites, all in `dispatch` @ `src/cmd/mod.rs:87`, `:92`, `:97`, `:102`, `:107`, `:112`.
- **`has_email`** — `screen_text_fault` @ `ledger.rs:1155`, its unit test @ `:2205`, and the root test
  `tests/contract_fixture_hygiene.rs:197`. cargo-mutants runs only the mutated package's tests, so the root test kills
  nothing in `ledger.rs`.
- **`check_stamps`** — `read_stamps_strict` @ `crates/viola-state/src/stamps.rs:78` and three unit tests in `strict.rs`.
- **`replace_private_shared`** — `pin_exe` @ `crates/viola-state/src/pin.rs:82`, `pin_companions` @ `:172`,
  `pin_and_plugin` @ `src/cmd/run.rs:329`; one unit test @ `fs.rs:304-306`.
- **`close_hooks`** — `read_stdin` @ `src/bin/viola-fake-agent.rs:728`, `:737`.

## Measured facts (this take-up, 2026-10-09)
- No source moved since the audit's commit: `git diff --numstat e304994 HEAD -- '*.rs'` prints 0 lines, so every
  audit coordinate is a HEAD coordinate.
- Mutants per survivor file (`cargo mutants --list --package <member> --file <path>`, cargo-mutants 27.1.0, lines
  counted): `events.rs` 56 · `stamps.rs` 26 · `strict.rs` 69 · `fs.rs` 54 (viola-state: 205 of the unit's 269) ·
  `ledger.rs` 304 (of viola-agent-claude's 462) · `viola-fake-agent.rs` 132 · `src/cmd/mod.rs` 4 · `src/cmd/hook.rs`
  74 (root package: 210 of 1047). The audit's whole-unit walls: viola-state 1133 s, viola-agent-claude 644 s, viola
  16623 s (about 16 s a mutant), viola-e2e about 12 s a mutant over 718.
- The scaffolding copies in root tests (`grep -n -E '^(pub )?fn (events|wait_events|…)\(|^struct (Running|Ran)'
  tests/*.rs`): `events` in 7 files (`channel_paste_validation`, `cli_answer`, `cli_instance_state`, `cli_send`,
  `cli_wheel`, `hook_events`, `tui_wheel`), `wait_events` in 6, `struct Running` with its `Drop` in 2 (`cli_answer`,
  `cli_wait_last`), `struct Ran` in 4, a `viola(…) -> Ran` runner in 3. Every `events` copy already reads through
  `support::ndjson::read_lines`, and every wait already uses `Watch` and `WITHIN`.
- The audit's instruments stand on the host at the recorded versions: `jscpd` 5.0.16, `rust-code-analysis-cli` 0.0.25,
  `cargo-mutants` 27.1.0. Its commands are in `record.json` `commands.duplication` and `commands.complexity`, with
  their summarizers `a13.py` and `cx.py` in the audit's run dir.
- The live rig of the 2026-10-08 chunk still stands: the stamped home `target/e2e-home/viola-live-4043089/home`
  (`ledger/stamps.json`, mode 0600, 857 B, dated 2026-10-08; the host is up since 2026-10-04, so the tmpfs was not
  lost), `claude` 2.1.287 by path under the user's mise installs (2.1.288 and 2.1.289 stand beside it), and the product
  build `target/release-check/release/viola`, sha256 prefix `2bf1b8ab19e16c8e`, the build starts 7 and 8 ran on.
- Test modules: 70 files under `src/` and `crates/*/src/` carry `#[cfg(test)]`
  (`grep -rl '#\[cfg(test)\]' src crates/*/src --include=*.rs | wc -l`), as the entry says.
- `.claude/docs/gotchas.md` holds no line on the `mutants` profile's 10 s kill
  (`grep -n -i -E '10 s|mutants. profile|kill line|CONFIRM_WINDOW'`: 0 hits).
- No cargo-mutants exclusion file exists (`.cargo/` is absent), so a survivor disposed by argument still reads
  `missed` in `outcomes.json`.
- CI on `59e791e` settled green after Setup read it in progress: `ci#37948581286`, 15 of 15, wall 418 s.

## Patterns detected
- **Validate, take the typed text once, one text everywhere** (`src/run/send.rs:318-361`): the four consumers
  (classification, in-flight match, `text_bytes`, paste) all read the binding made at `:321`. A wider strip inside
  `typed_text` reaches all four with no other edit.
- **A refusal is checked on both sides** (`src/cmd/send.rs:187`, `src/run/send.rs:318`): the client refuses before any
  frame and logs `side:"client"`; the wrapper checks again. Both sites are in the root bin, which depends on
  `viola-agent-claude`, so both can read the typed text; `viola-core` cannot.
- **One hint per cause, keyed by the detail's own word** (`src/human.rs:209-228`), with the cause list repeated in
  three unit tests.
- **The window classes carry their reason in the test name** (`.config/nextest.toml:17-27`, `:49-58`):
  `send_window_` and `verify_window_` are matched by name in both profiles; `tests/cli_verify.rs:829`
  `verify_window_without_screens_fails_every_interactive_row` is the root case with the same no-screen shape.
- **A live session's rig lives in the chunk's `evidence/`** (the 2026-10-08 chunk: `live-compositor.sh`,
  `live-start.sh`, `live-drive.py`, the start ledger `live-sessions.ndjson` with one `start` row written before each
  start and a `lock-read` row before each window step).
- **A leaf doc is written by implement when the chunk owns the line** (the 2026-10-02 cleanup's plan step 14 wrote
  `.claude/docs/gotchas.md`, listed under its Files to modify with a count gate).

## Conventions to follow
- **Test names**: `<subject>_<condition>_<expected>`; a case table is rstest `#[case::label]`, and in `viola-e2e` a
  labelled array in one `#[test]` (no dev-dependencies there).
- **Kebab-case wire values live in `viola-core` only** (`crates/viola-core/src/lib.rs:120-139`), with their literal
  pinned in a unit table (`:398-401`).
- **Human lines only through `src/human.rs`**, one `write_all` per refusal with its hint.
- **Unit tests stay inline** as `#[cfg(test)] mod tests` (the orphans gate), and a helper split out of a function
  stays in the same file.
- **A mutation witness is a direct run on the survivor's file in its owning package**, under `NEXTEST_PROFILE=mutants`
  and a short NOCOW `TMPDIR`, its `outcomes.json` copied to `evidence/` (the run archive keeps ten).

## Scope premise closure
- Every `[inferred]` bullet of `scope.md` was closed against the reads above; `scope.md` is amended in the same pass.
- Verified, the tag dropped: the operator's direction (against `inputs#I1`); both audit instruments and their
  commands; the scaffolding coordinates; the three survivors that sit inside functions §2 splits
  (`ledger.rs:900:76` in `dialog_variants` `:874-913`; `viola-fake-agent.rs:473:46` and `:473:57` in `submit`);
  `strict.rs:34:23` standing in both hosts' lists; the trailing-CR reading (line 10: `prompt_text_bytes` 31 of
  `sent_bytes` 32, `prompt_equals_sent_text_without_cr` true) against the exact compare at `src/run/send.rs:129`;
  the reader's verbs (`inputs#I2`); the gotchas gap; the other stale citation sites.
- The runner-only bullet (the red `mutants (viola-e2e)` job) is closed against its own run and stands verified, with
  its coordinates: the log lines are quoted in the Platform slot above.
- Corrected: §4's hypothesis. The mechanism is re-derived by read: with `cli_version` `9.9.9` no
  `fixtures/claude/9.9.9/` exists (the committed sets are `2.1.283`, `2.1.287`, `2.1.288`), so the fake agent draws no
  screen literal, and `settled` (`src/cmd/verify/typed.rs:478-498`) then ends each of verify's four interactive runs
  only at `GATE_MAX_WAIT`, 8.5 s: at least 34 s by construction, against a 30 s kill. The four waits' own durations in
  this test are not measured; the chunk reads them before the kill moves.
- Corrected: §6's "a refusal detail joins a closed list" has more consumers than the entry names. The list is
  `NotDelivered` (33 reference sites in 5 files); the hint table and its three test lists are a second closed list;
  `diag-line.v1.json` does not close `send-refused.detail`, so no schema moves.
- Corrected: §8. The same stale bare numbers stand at `.andromeda/security-plan.md:209` (four times) and
  `.andromeda/test-plan.md:992` (twice, `:127`); a third stale bare number of another entry stands in the architecture
  key file `registries/contracts/architecture/project-directory-structure.md:49` (`:93`, the self-healing entry, which
  `architecture.md:50` cites as `working-route.md:109`).
- At the revision (`inputs#I7`): the word's two additions entered §6 as `[inferred]` and were closed in the same
  pass against "The revision" below, the tag dropped. Verified: the two readers are red by the freshness atom alone
  (the gate trail's two `show` records); three starts stand of the cap of five; the changed build stands at
  `62bf6028d95fe34e`; a lone inner CR passes the validator and the strip unchanged; `live-run.md` names the shape as
  not measured. The stop falsified no scope bullet: the `artifact` key was the plan's, stated in no scope line.

## The split and the forks, as answered at P4 (`inputs#I3`)
- The chunk is split in two on the founder's word. The lists below hold the part that stays: the `viola-e2e` kill, the
  trailing CR and CRLF strip with the empty-text refusal, the three rule homes with the gotchas line, and the stale
  citations (wrap amendments, so they add no file here). M1, M2, M3, the eighteen Linux survivors and the `viola-e2e`
  whole-unit score leave; their reads above stand for the entry this chunk's wrap mints.
- The kill takes the `verify_window_` name. The new detail is `empty-text`, for any text whose typed text is empty,
  beside `control-character` on both sides. The citation item reaches every stale site found.
- Leaned at P4, per the entry's own measured cost (`testing.md` is 19.5 KB and `paths:` reaching all of `src/` would
  load it on every product-code edit): the testing rule's reach into `src/` is a small src-scoped file, not a widening
  of `testing.md`'s `paths:`.

## The revision (2026-10-09T16:35Z): the stop, as found
`/andromeda-implement` stopped at its P2 before the operator pass, on the plan (`inputs#I7`). Read where the stopped
run left it, nothing re-run:
- **The stop.** The implement run dir's gate trail (`.andromeda/runs/2026-10-09T15-46-10-implement/`, the whole-block
  run of 16:24:23Z): 27 entries, 22 green, 2 red (entries 22 and 23), 3 not run (the `operator` leg). Its `show`
  calls for 22 and 23 both read `exit 0 ✓`, a log holding the one line `true`, and `artifact STALE`: 0 s older than
  the run for `evidence/live-sessions.ndjson`, 240 s older for `evidence/live-readings.ndjson`. So both filters
  hold and the freshness atom alone reddens them.
- **Why the atom cannot hold.** gate-contract.md §What the tool reads: an `artifact` is `fresh` when its mtime is
  after the command began, and plan-template's `artifact` key is "a file … that the entry PRODUCES". Both entries
  are `jq` readers; the files are written by the hand-driven rig of the live steps, which is no listed entry
  (test-plan §1 Untestable zones). A reader keyed `artifact` reads `STALE` at every firing, at implement and at the
  wrap's light gate alike. The key was the plan's defect; no scope bullet stated it.
- **The steps done.** Their evidence stands in the chunk's `evidence/`: `live-preconditions.md` (four preconditions
  held; the build after the gates), `rehearsal.md` (held, four readings), `e2e-floor.md` (four waits of 8577, 8561,
  8563 and 8562 ms; PASS at 34.482 s under `ci`; S6 did not fire), `planted-hang.md` (TIMEOUT at 45.005 s under
  `mutants`, PASS at 84.513 s under the default profile, the plant's marker counting 0; the order case red on the old
  order and green on the moved one), `guard-red-green.md` (three guards removed and restored: 12, 6 and 31 red of 70),
  `live-run.md`, `live-sessions.ndjson` (8 rows: the rehearsal, three `start` rows with `cap` 5, three `census`
  rows and the final one), `live-readings.ndjson` (seven `reading` lines and one `equality` line). `gate.py scope`
  at 16:25:30Z read `clean`: changed 10, listed 10.
- **Start 3's finding, which the new reading sits beside.** `after-inner-crlf` (readings line 7 and its `equality`
  line): the CLI submitted the text with its CR LF as one LF (`prompt_equals_sent_with_each_crlf_as_one_lf` true,
  `prompt_equals_sent_unchanged` false), the prompt was filed `human`, the wheel went to the human and the send ended
  `not-delivered` / `no-prompt-submitted`, exit 13. `live-run.md` closes it with "not measured here is an inner CR
  standing alone, with no LF after it". That is the reading the word adds.
- **Why it needs its own start.** After `after-inner-crlf` the wheel stood at the human (`snapshot_wheel` `human`),
  and only a human hands it back, so start 3 could take no further reading; the lone-CR text's outcome is unknown and
  may end the same way. One reading, one start: start 4 of the cap of 5 (`live-sessions.ndjson`: `start` rows `n`
  1, 2 and 3; re-derived: `jq -s '[.[] | select(.kind == "start")] | length'` prints 3).
- **The text reaches the CLI as typed.** In the tree as the stopped run left it: `validate_paste_text` allows CR
  (`crates/viola-core/src/lib.rs:191-197`, `'\n' | '\r' | '\t' => false`), and `typed_text` is
  `text.trim_end_matches(['\r', '\n'])` (`crates/viola-agent-claude/src/hook.rs:207-209`), which leaves a CR that
  is not at the end. So a text with one inner CR and no newline at its end is issued and pasted unchanged; what the
  CLI submits for it is the unknown the reading records.
- **The build start 4 runs on.** `target/release-check/release/viola` read at 16:37:12Z: sha256 prefix
  `62bf6028d95fe34e`, 3 998 456 B, the hash start 3 ran on (`live-sessions.ndjson` line 6). The stamped home stands
  (`ledger/stamps.json` mode 0600, 857 B), `target/e2e-home` is the link and its backing reads `tmpfs`. The
  release-check entry cleans and rebuilds that binary, so start 4 is made before the block is re-fired and the hash is
  read again right before its ledger row.
- **The rig takes a new reading without a new script.** `evidence/live-read.py reading <id> <session> <home> <name>
  <private dir> <text file>` reads any id and compares against the text file in the private directory (`:137-236`);
  its three built-in equalities are for endings, so the inner-CR equalities are written as an `equality` line beside
  the reading, as `after-inner-crlf`'s were.
- **The write set does not move.** The revision edits the plan, the two readers and the chunk's own `evidence/`; no
  file of the two lists below changes with it, and no file joins them.
- Not re-read at the revision: six of the seven extracts and every history file of the take-up
  (`.andromeda/runs/2026-10-09T15-03-42-phase/`). Neither change reaches a master section they carry. The tests
  extract was re-read for "Live readings are not a test layer" (test-plan §1 Untestable zones), which the added
  reading falls under.

## New files to create
- `.claude/rules/ci.md` — the rule home for the workflows directory and the operator pass
- `.claude/rules/testing-src.md` — the src-scoped pointers to the testing rule for test modules inside product code

## Files to modify
- `crates/viola-core/src/lib.rs` — the new not-delivered detail and its literal
- `crates/viola-agent-claude/src/hook.rs` — the typed text without trailing CR and LF, and its case table
- `src/run/send.rs` — the wrapper's empty-text refusal and the trailing-newline unit cases
- `src/cmd/send.rs` — the client's empty-text refusal before any frame
- `src/human.rs` — the new cause's hint and the three cause lists
- `tests/cli_send.rs` — the cross-process cases
- `tests/channel_paste_validation.rs` — the raw-client wrapper half of the new refusal
- `.config/nextest.toml` — the override order of the mutants profile
- `crates/viola-e2e/tests/harness_lifecycle.rs` — the killed test's name
- `tests/contract_lints.rs` — the reader of the mutants profile's overrides
- `.claude/rules/verification-harness.md` — the host-pressure reader's line
- `.claude/docs/gotchas.md` — the mutants profile's 10 s kill line

## Open questions
- none — the three plan decisions were answered at P4 (`inputs#I3`).
