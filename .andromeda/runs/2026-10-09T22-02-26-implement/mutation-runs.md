# Mutation run journal — 2026-10-09-epoch-3-cleanup-ii

Rule (plan step 1, `inputs#I2`): one entry is appended BEFORE every mutation run. When a run returns, this file is
read back from disk before anything else, then the run's end, exit and verdict line are appended. The journal, not
memory, is what the next step is read from. Paths are repository-relative; the scratch is
`<repo parent>/viola-mutants-scratch` (NOCOW, read empty before run 1).

Standing rules while any run below is in flight: no full suite, no `pre-push`, no second mutation run. Editing,
`cargo check`, `cargo fmt` and a filtered unit run are fine. A red where no assertion failed on a value, or a
`timeout` line in a mutation record, is read against the backing
(`test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`) and the host record
(`hostwatch.py read --from <start> --to <end> --for viola`) before anything else; it is never re-run for green.

The run dir of this implement run: `.andromeda/runs/2026-10-09T22-02-26-implement/`.
The chunk dir: `viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/`.

## Run 1 — the `viola-e2e` whole-unit score (plan step 2)

- start (UTC, read from the clock just before the launch): see the line appended below
- command: `TMPDIR="$(dirname "$PWD")/viola-mutants-scratch" bash scripts/agent-run.sh run --mutants --package viola-e2e`
- stdout: `.andromeda/runs/2026-10-09T22-02-26-implement/run1-e2e-score.stdout.json`
- stderr: `.andromeda/runs/2026-10-09T22-02-26-implement/run1-e2e-score.stderr.txt`
- tree: HEAD `14f1fb5`, no chunk edit in the tree (only the phase's take-up and bookkeeping are uncommitted)
- expected wall: 78 min measured once (2026-10-04), up to 2.5 h
- host before the launch: backing `tmpfs` behind the `target/e2e-home` link; host record QUIET for viola over the
  last 15 min (0 s stalled on IO); load average 6.9 on 32 cores; no cargo, nextest or cargo-mutants process alive
- while it runs: no source edit until its stderr shows the tree copied and the unmutated baseline done; then plan
  step 3 (`evidence/scalars.py`, its `selftest`, the two "before" readings into `evidence/scalars.md`), then
  sections B, C, D, E (edits only, `cargo check`, `cargo fmt`, filtered unit runs)
- NEXT STEP WHEN IT RETURNS: plan step 13. Read this journal from disk. Read the stdout document and the stderr
  outcome lines. Append the end, the exit and the verdict line here. Any `timeout` line: read the backing and
  `hostwatch.py` over the run's window first. Then copy the document to `evidence/e2e-score.json`, the outcome
  lines (lines opening with caught / missed / timeout / unviable, any case) to `evidence/e2e-score-outcomes.txt`,
  and write `evidence/e2e-score.md` (command, start, end, wall, the four counts, the verdict rule's reading, every
  missed or timed-out mutant by repo-relative name, the count of `.tmp*` directories left in the scratch, what else
  ran in the window). No `outcomes.json`, no absolute path. Then continue sections B to E if not finished; the
  witness runs (step 14) start only after B to E are in the tree and `run --unit` and `run` read green.
- launched (UTC): 2026-10-09T22:03:20Z
- the launch command also writes `run1-e2e-score.exit.txt` (the harness exit and the end time) when the run ends
- 22:05Z: the tree copied (`cargo-mutants-viola-*.tmp` in the scratch) and the baseline running; 718 mutants found
- NOTE for hygiene: `run1-e2e-score.stderr.txt` opens with cargo `Compiling` lines that carry the repository's
  absolute path. Before the operator pass's hygiene read, move the stderr files of every run out of the run dir
  (to the session scratchpad) and keep only the outcome lines in `evidence/`.

### State of the tree while run 1 is in flight (22:17Z)

- Plan step 3 done: `evidence/scalars.py`, `evidence/scalars.md` with the "before" row (3.36 %, 7 named
  fragments, 4 over the ceiling) and the base test counts (unit 1471, integration 353).
- Sections B, C, D, E are ALL in the tree: 18 files modified, 2 new (`tests/support/events.rs`,
  `tests/support/cli.rs`), exactly research's lists; no scope-record line needed so far.
- Read green on the edited tree: `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --features
  fake-agent -- -D warnings`; the 14 new cases and the restated functions' tests (27 of 27, a filtered nextest run
  in `target/harness`, build jobs limited to 8); the count probe reads 0.
- Early scalar readings on the edited tree (not yet in `scalars.md`; step 16 re-takes them):
  `duplication: pct 2.84 · duplicated 1533 of 53987 · clones 187 · named fragments 0` (exit 0) and
  `complexity: over_ceiling 1 · dialog_variants 10 · record 2 · submit 11 · files 126 of 126` (exit 0).
- `cargo mutants --list` on the edited tree: the four restated mutants are gone from the generated set
  (`next_line` now lists `!= → ==` at 211:24; `parallel_both_before_first_post` lists `!= → ==` and `delete !`;
  `clear_start` lists only the `from + at` mutants; `end_turn` lists `== → !=` at 495:28 with no `&& → ||` on
  the long-paste line).
- Deviations to carry into the report: (1) the `cli_verify` helper serves FOUR tests, the plan names three (the
  fourth, `verify_without_framing_fails_only_the_clear_row`, holds the same block); (2) the instrument's file
  population is tracked plus untracked `*.rs` (the audit's `git ls-files` leaves the two new files out until they
  are committed); (3) the shared `Running` has a public constructor `Running::over(child)` because `cli_answer`'s
  two hook runners build their own command.
- NOT yet run (waits for run 1 to return): `run --unit`, `run`, the three witness runs (step 14), steps 15 to 17,
  the whole gate block (`gate.py run --plan … --run-dir … --marker …`, backgrounded), the P2/P3 evolve
  checkpoints, the scope read, the operator pass (pre-push, hygiene, the pre-CI commit, the push, the CI read with
  `run_attempt`), the report.
- Telemetry already appended: the `code` step record (22:16:23Z, ids a to d).

### Run 1 returned (journal re-read from disk at 23:32:45Z)

- end (UTC): 2026-10-09T23:32:36Z; wall 5356 s (89 min 16 s); harness exit 1
- cargo-mutants' own line: `718 mutants tested in 89m: 2 missed, 656 caught, 60 unviable`; baseline `12s build +
  35s test`; auto-set test timeout 178 s
- the document: `"ok":false`, suite `mutants` passed 656, failed 2, survived 2; `mutants.tested` 718, verdict
  `package`, package `viola-e2e`
- verdict rule: missed 2 (rule: 0) · timeout 0 (rule: 0) · unviable 60 <= caught 656
- the two missed, both in `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, function `prepare`: `48:5 replace
  prepare -> … with Ok(None)` and `54:8 delete !`. Read at source: `prepare` returns `Ok(None)` at once when
  `HOST_SCRATCH` (`cfg!(windows)`) is false, so on this host the first is the code's own value and the second is
  past the return. Both are "not measured here; owed to Windows mutation grade". This chunk kills neither.
- no `timeout` line. Backing: `tmpfs` behind the link. Host record over the window: QUIET for viola, 0 s stalled
  on IO, load peak 18.7, mean 9.7.
- left in the scratch: 17 `.tmp*` directories, nothing else (the tool removed its tree copy)
- NEXT: plan step 13 (the three evidence files), then `run --unit` and `run` on the edited tree, then run 2.
- 23:33Z: step 13 done: `evidence/e2e-score.json`, `evidence/e2e-score-outcomes.txt` (718 lines),
  `evidence/e2e-score.md`; the score reader's filter reads `true`, exit 0.
- 23:33:42Z to 23:34:38Z, on the edited tree: `run --unit` exit 0, `"ok":true`, unit 1484 of 1484; `run` exit 0,
  `"ok":true`, unit 1484, integration 354 (base: 1471 and 353; +13 unit, +1 integration, none dropped).

## Run 2 — witness, viola-state (plan step 14, first of three)

- command: `TMPDIR="$(dirname "$PWD")/viola-mutants-scratch" bash scripts/agent-run.sh run --mutants --package
  viola-state --file crates/viola-state/src/events.rs --file crates/viola-state/src/stamps.rs --file
  crates/viola-state/src/strict.rs --file crates/viola-state/src/fs.rs`
- stdout: `.andromeda/runs/2026-10-09T22-02-26-implement/run2-witness-viola-state.stdout.json`
- stderr: `.andromeda/runs/2026-10-09T22-02-26-implement/run2-witness-viola-state.stderr.txt`
- exit file: `run2-witness-viola-state.exit.txt`
- tree: HEAD `14f1fb5` plus every edit of sections B to E (the tree `run` just read green); NO edit while it runs
- expected: 203 mutants listed, about 15 min. Its document is expected to read `"ok":false` (the `cfg(windows)`
  bodies of `strict.rs` and `fs.rs`, and the six `replace_private_with` rows); step 15's comparison is the verdict
- scratch before: 17 `.tmp*` directories left by run 1
- NEXT STEP WHEN IT RETURNS: read this journal from disk; append end, exit, counts; any `timeout` line → backing
  and `hostwatch.py` first, never a re-run for green; copy the document to `evidence/witness-viola-state.json` and
  the outcome lines to `evidence/witness-viola-state-outcomes.txt`; check the five kill rows read `caught`
  (`events.rs:96:19`, `events.rs:149:19`, `stamps.rs:28:19`, `strict.rs:34:23`, `fs.rs:290:19`, each "replace match
  guard … with true") and that every `missed` line is in the audit's `c-mutation-viola-state.json` `not_measured`
  or one of the six `replace_private_with` rows; then start run 3 (viola-agent-claude, `ledger.rs`), then run 4
  (root: `viola-fake-agent.rs`, `src/cmd/mod.rs`, `src/cmd/hook.rs`, `src/cmd/verify.rs`).
- launched (UTC): 2026-10-09T23:34:54Z

### Run 2 returned (journal re-read from disk at 23:50:06Z)

- end (UTC): 2026-10-09T23:49:58Z; wall 904 s; harness exit 1 (expected: the document reads `"ok":false`)
- cargo-mutants: `203 mutants tested in 15m: 65 missed, 129 caught, 9 unviable`; baseline `6s build + 3s test`;
  auto-set test timeout 20 s. The document: `mutants.tested` 203, verdict `package`, package `viola-state`, the
  four files.
- timeout 0 · unviable 9 <= caught 129
- the 65 missed, each matched by file and mutation text: 59 are in the audit's `c-mutation-viola-state.json`
  `not_measured` (all `cfg(windows)`), 6 are the `replace_private_with` rows owed to "Windows mutation grade";
  **0 not accounted for**
- the five kill rows read `caught`: `events.rs:96:19` (`current_len`), `events.rs:149:19` (`read_within`),
  `stamps.rs:28:19` (`read_capped`), `strict.rs:34:23` (`check_stamps`), `fs.rs:290:19` (`replace_private_shared`),
  each "replace match guard … with true"
- the restated guard: no `< → >` line in `LoggedLines::next_line`; its one comparison mutant `211:24 replace !=
  with ==` reads `caught`; all 13 lines of that function read `caught`
- backing `tmpfs`; scratch now holds 20 `.tmp*` directories
- copied: `evidence/witness-viola-state.json`, `evidence/witness-viola-state-outcomes.txt` (203 lines)

## Run 3 — witness, viola-agent-claude, `ledger.rs` (plan step 14, second of three)

- command: `TMPDIR="$(dirname "$PWD")/viola-mutants-scratch" bash scripts/agent-run.sh run --mutants --package
  viola-agent-claude --file crates/viola-agent-claude/src/ledger.rs`
- stdout: `.andromeda/runs/2026-10-09T22-02-26-implement/run3-witness-viola-agent-claude.stdout.json`
- stderr: `.andromeda/runs/2026-10-09T22-02-26-implement/run3-witness-viola-agent-claude.stderr.txt`
- exit file: `run3-witness-viola-agent-claude.exit.txt`
- tree: the same edited tree as run 2; NO edit while it runs
- expected: 303 mutants listed, about 7 min; no missed and no timeout line is expected
- NEXT STEP WHEN IT RETURNS: read this journal from disk; append end, exit, counts; a `timeout` line → backing and
  `hostwatch.py` first; copy to `evidence/witness-viola-agent-claude.json` and `…-outcomes.txt`; check the four
  kill rows `caught` (`647:81 == → !=` in `both_parallel_answered`; `&& → ||` at `898:68` in `joined_call`, the old
  `900:76` of `dialog_variants`; `replace is_local_char -> bool with true`; `1196:74 == → !=` in `has_email`) and the
  two restated rows gone (no `< → <=` in `parallel_both_before_first_post`; in `clear_start` only the `from + at`
  mutants at `694:24`); any missed line is this chunk's red: kill it with a test and re-run this witness. Then
  run 4 (root: `--package viola --file src/bin/viola-fake-agent.rs --file src/cmd/mod.rs --file src/cmd/hook.rs
  --file src/cmd/verify.rs`, 263 mutants listed, about 68 min).
- launched (UTC): 2026-10-09T23:50:29Z

### Run 3 returned (journal re-read from disk at 23:59:24Z)

- end (UTC): 2026-10-09T23:59:17Z; wall 528 s; harness exit 0; the document reads `"ok":true`
- cargo-mutants: `303 mutants tested in 9m: 283 caught, 20 unviable`; baseline `7s build + 0s test`; auto-set test
  timeout 20 s. `mutants.tested` 303, verdict `package`, package `viola-agent-claude`, file `ledger.rs`.
- missed 0 · timeout 0 · unviable 20 <= caught 283
- the four kill rows read `caught`: `647:81 replace == with != in both_parallel_answered`; `898:68 replace && with
  || in joined_call` (the old `900:76` of `dialog_variants`); `1187:5 replace is_local_char -> bool with true`;
  `1196:74 replace == with != in has_email`
- the two restated rows are gone: `parallel_both_before_first_post` lists `663:19 != → ==` and `670:9 delete !`,
  both `caught`, and no `< → <=`; `clear_start` lists five lines, all `caught`, the arithmetic ones `694:24 + → -`
  and `694:24 + → *` (the `from + at`), and no mutant of a `+ 1`
- backing `tmpfs`; scratch still holds 20 `.tmp*` directories
- copied: `evidence/witness-viola-agent-claude.json`, `evidence/witness-viola-agent-claude-outcomes.txt` (303)

## Run 4 — witness, the root package's four files (plan step 14, third of three)

- command: `TMPDIR="$(dirname "$PWD")/viola-mutants-scratch" bash scripts/agent-run.sh run --mutants --package
  viola --file src/bin/viola-fake-agent.rs --file src/cmd/mod.rs --file src/cmd/hook.rs --file src/cmd/verify.rs`
- stdout: `.andromeda/runs/2026-10-09T22-02-26-implement/run4-witness-viola.stdout.json`
- stderr: `.andromeda/runs/2026-10-09T22-02-26-implement/run4-witness-viola.stderr.txt`
- exit file: `run4-witness-viola.exit.txt`
- tree: the same edited tree as runs 2 and 3; NO edit while it runs (no source, no test)
- expected: 263 mutants listed, about 68 min (the audit's 15.9 s a mutant); no missed and no timeout expected
- NEXT STEP WHEN IT RETURNS: read this journal from disk; append end, exit, counts; a `timeout` line → backing and
  `hostwatch.py --from <launch> --to <end>` first, never a re-run for green; copy to `evidence/witness-viola.json`
  and `evidence/witness-viola-outcomes.txt`; check the five kill rows `caught` (`replace Agent::close_hooks with
  ()`; `437:74 == → !=` in `Agent::run_hook`; `495:28 == → !=` in `Agent::end_turn`, the old `473:57` of `submit`;
  `replace cli_sink -> Option<DetailSink> with None`; `214:72 > → >=` in `handle_dialog`) and the restated row
  gone (no `&& → ||` on the long-paste line of `end_turn`; `500:37 && → ||` is the tag line's); every mutant of
  `record`, `named_payloads`, `scrubbed_payloads`, `signature_screens`, `write_recording` and `end_turn` must read
  caught or unviable. Any missed or timed-out line is this chunk's red: kill it with a test in the root package
  and re-run THIS witness (about 68 min more). Then step 15 (`evidence/survivors.md`, `evidence/survivors.ndjson`,
  18 lines), step 16 (both scalar verbs, the "after" row and the after test counts 1484 / 354 in
  `evidence/scalars.md`), step 17 (the whole block through `gate.py run`, backgrounded; then the operator pass).
- launched (UTC): 2026-10-09T23:59:52Z
- 00:01Z, read while run 4 is in flight (no source or test edit): run 3's per-mutant logs, still on disk as
  `mutants.out.old/log/` (ignored by git), name the ONE test that failed under each kill-row mutant:
  - `647:81` (`both_parallel_answered`, `== → !=`): `ledger::tests::check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered`
  - `898:68` (`joined_call`, `&& → ||`): `ledger::tests::dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post`
  - `1187:5` (`is_local_char → true`): `ledger::tests::has_email_reads_local_at_domain_dot_tld::case_09_space_before_the_at`
  - `1196:74` (`has_email`, `== → !=`): `ledger::tests::has_email_reads_local_at_domain_dot_tld::case_10_hyphen_in_the_domain`
- Run 2's per-mutant logs were NOT read before run 3 replaced them (the tool keeps one earlier run). For the five
  viola-state kill rows the killing test is named by construction (each case calls the mutated function), not read
  from a log. When run 4 returns, read `mutants.out/log/` for its five kill rows BEFORE any other mutation run.

### Run 4 returned (journal re-read from disk at 01:06:15Z, 2026-10-10)

- end (UTC): 2026-10-10T01:06:06Z; wall 3974 s (66 min 14 s); harness exit 0; the document reads `"ok":true`
- cargo-mutants: `263 mutants tested in 66m: 238 caught, 25 unviable`; baseline `16s build + 38s test`; auto-set
  test timeout 195 s. `mutants.tested` 263, verdict `package`, package `viola`, the four files.
- missed 0 · timeout 0 · unviable 25 <= caught 238
- the five kill rows read `caught`, and the per-mutant log of each names its new case as the test that FAILED:
  `328:9 replace Agent::close_hooks with ()` → `tests::run_hook_after_close_hooks_runs_nothing`;
  `437:74 == → !=` in `Agent::run_hook` → `tests::run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold`;
  `495:28 == → !=` in `Agent::end_turn` (the old `473:57` of `submit`) →
  `tests::submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint`;
  `133:5 replace cli_sink -> Option<DetailSink> with None` →
  `cli_wait_last wait_in_a_home_whose_diagnostics_is_a_file_keeps_the_chain_in_the_detail_file`;
  `214:72 > → >=` in `handle_dialog` →
  `cmd::hook::tests::handle_dialog_with_a_payload_at_the_frame_bound_prints_the_decision_body`
- the restated row is gone: `Agent::end_turn` lists `495:28 == → !=`, `500:37 && → ||` (the tag line) and `500:48
  == → !=`, all `caught`; no `&& → ||` on the long-paste line. Every mutant of `record`, `named_payloads`,
  `scrubbed_payloads`, `signature_screens`, `write_recording` and `end_turn` reads caught or unviable.
- nextest TIMEOUT lines INSIDE per-mutant logs (not the mutation record's `timeout` outcome, which is 0): 133 of
  the 263 logs hold one; 234 hold a FAIL line; 4 mutants are graded `caught` by the profile's 10 s kill alone, no
  FAIL line: `viola-fake-agent.rs:378:16` and `:378:38` (`delete !` in `replay_dialogs`), `:97:17` (the `--screens`
  match arm deleted), `src/cmd/verify.rs:284:9` (`ProbeDir::typed → Default::default()`). None is on a line this
  chunk edited; under each the stamped-home fixture's `viola verify` never finishes. Read before anything else, per
  the operator's rule: backing `tmpfs`; host record over 23:59:52Z..01:06:06Z `QUIET for viola`, 0 s stalled on
  IO, io some peak 15 %, load peak 20.6, mean 10.1. Not re-run. Named in `evidence/survivors.md` for the wrap.
- scratch now holds 34 `.tmp*` directories (17 after run 1, 20 after runs 2 and 3)
- copied: `evidence/witness-viola.json`, `evidence/witness-viola-outcomes.txt` (263 lines)
- 01:07Z: `evidence/survivors.ndjson` built (18 rows) by the builder now kept as `evidence/survivors-build.py`; the
  survivor reader's filter reads `true`, exit 0.
- 01:07:35Z: step 16's readings taken: `duplication: pct 2.84 · duplicated 1533 of 53987 · clones 187 · named
  fragments 0` exit 0; `complexity: over_ceiling 1 · dialog_variants 10 · record 2 · submit 11 · files 126 of 126`
  exit 0; `selftest: 0 mismatches` exit 0.
- NEXT: write `evidence/survivors.md` (step 15); step 16 (both scalar verbs, the "after" row, the after test
  counts); move the four `run*-*.stderr.txt` files out of the run dir into the session scratchpad (they carry the
  repository's absolute path); then step 17: the whole block, `python -X utf8 <tools>/gate.py run --plan
  viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/plan.md --run-dir .andromeda/runs/2026-10-09T22-02-26-implement
  --marker 2026-10-09-epoch-3-cleanup-ii`, backgrounded; verdict = the printed SUMMARY line. No mutation run is
  left to make.

### After the four runs (01:09Z, 2026-10-10)

- Steps 15 and 16 done: `evidence/survivors.md`, `evidence/survivors.ndjson`, `evidence/survivors-build.py`,
  `evidence/scalars.md` with the "after" row (2.84 %, 0 named fragments, `over_ceiling` 1) and the after test
  counts (1484 / 354).
- The four `run*-*.stderr.txt` files were moved to the session scratchpad (`raw-stderr/`); the run dir keeps each
  run's stdout document and exit file. No file of the run dir holds a home path now.
- 01:09:17Z: the whole gate block launched through `gate.py run` (backgrounded). Host just before it: QUIET for
  viola, load average 2.3.
- THEN: read the SUMMARY line; a red → `gate.py show --n <n>`, fix, `--only`, then the whole block again. On
  green: the P2 evolve checkpoint (`references/evolve/fix-loop.md`), `matrix.py show --chunk` (the chunk claimed no
  capability at take-up, expected: nothing to record), the P3 note (the smoke entries ran as P2 gates) and its
  evolve checkpoint (`smoke.md`), the scope read (`gate.py scope --chunk-dir … --run-dir … --marker …`), then the
  OPERATOR PASS on the operator's word (given with this invocation): `pre-push` (entry 18, through the tool:
  `--entry 18`), hygiene (entry 19, by hand), the commit `chore(2026-10-09-epoch-3-cleanup-ii): operator pre-CI
  commit, for the run this chunk's verdict reads`, the guarded push (entry 20), the CI read (entry 21:
  `ci.py conclusion --sha HEAD --wait 1800`) with `run_attempt` read from the run itself: the final sha needs
  green on its FIRST attempt. Record every entry in `evidence/operator-pass.md`. A red is fixed by a new commit on
  top, never an amend, never a force push, and the pass repeats.

### The block and the operator pass (2026-10-10)

- 01:11Z: the block read `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 · not-run 3` on its first firing.
  Telemetry appended: `fix-loop` (01:11:50Z, ids a to c) and `smoke` (01:12:20Z, id a). The matrix shows 0
  capabilities claimed by this chunk: nothing recorded. Scope read: `clean — changed 20 · listed 20`.
- The operator's word for this run is snapshotted as `inputs#I3` (`relay-1.md` in this run dir).
- 01:12:55Z to 01:13:59Z: `pre-push` again through the tool (`--entry 18`): green, 63.73 s.
- 01:14:03Z and 01:14:32Z: hygiene `clean` twice (read 66, then 67 with `evidence/operator-pass.md`).
- 01:14:39Z: the pre-CI commit `632f6a7edf29` (94 files). 01:14:44Z to 01:14:46Z: the guarded push, exit 0,
  `14f1fb5..632f6a7  HEAD -> build/viola-0.1.0`; the remote head read live at `632f6a7edf29`, 0 ahead.
- NEXT: entry 21, `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion
  --sha HEAD --wait 1800` (backgrounded). When it returns: read `verdict:`; read `run_attempt` from the run itself
  (`gh api repos/Turbolet85/viola/actions/runs/<id>`), and `gh run list --commit $(git rev-parse HEAD)` with the
  FULL sha. Green on attempt 1 → complete `evidence/operator-pass.md` (the commit, the push, the CI read, "after
  the pass"), then the P4 report: outcome, files changed, deviations, the process census. A red → read the failing
  job's log first (the NUL-byte cases on `windows-2025` / `macos-latest` are the first reading there); fix by a NEW
  commit on top and repeat the pass from `pre-push`; the final sha then needs its own first-attempt green.
- 01:15:03Z to 01:22:49Z: the CI read: `632f6a7edf29 verdict: green · checks 15/15 · wall 447 s · runs
  ci#38012420489 completed/success`; `run_attempt` 1, the only run on the sha, all fifteen jobs `success` at
  attempt 1. The new cases read `PASS` in the `windows-2025` (13, the read-only directory case is `cfg(unix)`) and
  `macos-latest` (14) test jobs' logs. No fix commit: `632f6a7` is the final sha.
- 01:23Z: `evidence/operator-pass.md` completed; the scope read on the new base (`14f1fb5f`, the pre-CI commit's
  parent) and hygiene with `--marker` both read `clean`. Census: no process of this repository alive. Left behind:
  34 `.tmp*` directories in the mutation scratch (250 MB), 26 `viola-session-*` homes of run 1 on the tmpfs base,
  `mutants.out/` and `mutants.out.old/` (ignored by git), the raw stderr captures in the session scratchpad.
- The implement run ends here with the report. NEXT is the wrap (`/andromeda-wrap-session`), the operator's call.
