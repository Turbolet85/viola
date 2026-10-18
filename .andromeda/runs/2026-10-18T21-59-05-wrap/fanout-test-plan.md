proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: "The `--local-live` bullet's literal reads `LEDGER_ROWS`: `[&str; 24]` under `cfg(unix)` — the twenty-three ids as listed, then `local-command-compact` after `agents-join` — and the 21-id form (the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) under `cfg(not(unix))`, `LIVE_PASS` under the same split, `run_local_live_reads_the_relied_rows` reading `local-command-compact` last; the dated live-firing sentences stay as history (this chunk made no `--local-live` firing: its round was one `viola verify --record`)."
    sidecar: "test-plan §3 → 5-command implementation: LEDGER_ROWS / LIVE_PASS at 24 (Unix) / 21 (elsewhere), local-command-compact appended last (chunk 2026-10-18-compact-post-condition-ledger-row)."
    rationale: "Report Harness / gate surface: the harness's `LEDGER_ROWS` and `LIVE_PASS` (`crates/viola-e2e/src/harness/run.rs`) at 24 / 21 with `local-command-compact` last, `run_local_live_reads_the_relied_rows` reads it last; Counts `ledger-lits` and `rows-count` name `test-plan/5-command-implementation.md:43`. No log format or status shape changed (Harness / gate surface: no status shape changed), so obs-plan §3 needs no paired edit."
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:43
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: "`boot` step 4's relied set reads `LedgerRow::relied()`: all twenty-four on Unix, elsewhere twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`."
    sidecar: "test-plan §3 → 5-command implementation: boot's stamped relied set 24 / 21 with local-command-compact."
    rationale: "Report Symbols / APIs: `LedgerRow::LocalCommandCompact`, the twenty-fourth member of `LedgerRow::ALL`, relied on every OS, `LedgerRow::relied()` 24 on Unix and 21 on Windows (was 23 / 20); Harness / gate surface: the boot's stamp at twenty-four rows; Counts `rows-count` names `test-plan/5-command-implementation.md:8`."
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: "The relied-set parenthetical reads: the verdict `verified` reads `LedgerRow::relied()`: 24 rows on Unix, 21 on Windows, `agents-join` then `local-command-compact` appended last, both relied on every OS."
    sidecar: "test-plan §4: relied set 24 / 21, local-command-compact appended last."
    rationale: "Report Symbols / APIs (`LedgerRow::ALL: [Self; 24]`, relied 24 / 21, no arm in `relied_on`); Counts `rows-count` names test-plan 505."
    basis: .andromeda/test-plan.md:505
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered
    change: "`tests/cli_verify.rs`'s step lines read `MM` the relied count (`LedgerRow::relied()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`; `STEPS_PASS` and `ROW_IDS` are `[&str; 24]` on Unix and `[&str; 21]` on Windows)."
    sidecar: "test-plan §5: cli_verify step lines [NN/24] / [NN/21], last local-command-compact; STEPS_PASS / ROW_IDS 24 / 21."
    rationale: "Report Counts `rows-count` (step lines `[NN/24]` / `[NN/21]`, the last `[24/24] local-command-compact …` / `[21/21] local-command-compact …`) and `ledger-lits` (`STEPS_PASS` / `ROW_IDS` at 24 / 21, test-plan 586 ×2); Outcome (design / layouts / a11y) met by `tests/cli_verify.rs`."
    basis: .andromeda/test-plan.md:586
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered
    change: "`contract_ledger_probes.rs` lists `ROW_IDS: [&str; 24]` (the three statusline ids relied on Unix only, `resume-session`, `agents-join` and `local-command-compact` relied on every OS), `<n>` the relied count: 24 on Unix, 21 on Windows."
    sidecar: "test-plan §5: contract_ledger_probes ROW_IDS 24, relied 24 / 21, local-command-compact named."
    rationale: "Report Counts `rows-count` and `ledger-lits` (test-plan 598); Outcome (tests): `contract_ledger_probes` `24 pass  0 fail`."
    basis: .andromeda/test-plan.md:598
    dependent-of: D-tests-obs-harness
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Coverage triggers
    change: "The contract-suite trigger reads: the fake agent's behaviour matches the recorded fixtures, the spine, the dialog variants, the framing variants, the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`) and the compact variant (`fake_agent_compact_replay_matches_every_recorded_compact_set`); live `viola verify` refreshes the framing variants through Run B's five added pastes (`paste-<n>`, `compact-<n>`, `clear-<n>`; the newline text recorded as `paste-3`, the `/compact` paste's SessionStart as `SessionStart.compact-1.json`)."
    sidecar: "test-plan §1 Coverage triggers: Run B's five added pastes, the compact-<n> framing variant and fake_agent_compact_replay_matches_every_recorded_compact_set."
    rationale: "Report Symbols / APIs: `src/cmd/verify/typed.rs` `framing_turns` pastes `PROBE_COMPACT_COMMAND` after the newline turn, Run B pastes six compiled texts; `ledger::framing_variants` names the compact SessionStart `compact-1`; Coverage of new surfaces names the integ test `contract_fake_agent_drift` `fake_agent_compact_replay_matches_every_recorded_compact_set`; Counts `runb-pastes` and `framing-var` name test-plan 349."
    basis: .andromeda/test-plan.md:349
  - detector: D-tests-coverage
    severity: warning
    section: §2 Test Strategy → Test pyramid (Contract row)
    change: "The framing-tier fixtures are recorded by Run B's five added pastes as `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `compact`, `clear`; six under `2.1.287` — `paste-1`, `paste-2`, `paste-3`, `compact-1`, `clear-1` ×2 — none under `2.1.288`), and `contract_fake_agent_drift` also replays the compact variant byte for byte (`fake_agent_compact_replay_matches_every_recorded_compact_set`)."
    sidecar: "test-plan §2 Contract row: five added Run B pastes, six framing variants under 2.1.287 with compact-1, the compact replay test."
    rationale: "Report Counts `runb-pastes` (test-plan 379 @c570) and `framing-var` (test-plan 379 ×6, five under `2.1.287` → six, `compact-1` added); Files: new `fixtures/claude/2.1.287/SessionStart.compact-1.json`; Outcome (tests): drift replays `compact-1` byte for byte."
    basis: .andromeda/test-plan.md:379
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered
    change: "`tests/cli_verify.rs`'s run list reads `--framing` for Run B's five added pastes (the long, tag-like and newline texts, `/compact`, `/clear`), and the tag-turn modal case reads pasting neither the newline text nor either local command (`/compact`, `/clear`)."
    sidecar: "test-plan §5: cli_verify's Run B five added pastes; the tag-turn modal pastes neither local command."
    rationale: "Report Symbols / APIs (`framing_turns`: Run B now pastes six compiled texts, `/compact` before `/clear`); Counts `runb-pastes` names test-plan 586 @c251; Counts: the tag-turn modal summary moved with the compact row failing."
    basis: .andromeda/test-plan.md:586
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → 5-command implementation
    change: "`boot` step 4 reads Run B with its five added compiled pastes — the long, the tag-like and the newline texts, then `/compact`, then `/clear`."
    sidecar: "test-plan §3 → 5-command implementation: boot's Run B five added compiled pastes, /compact before /clear."
    rationale: "Report Symbols / APIs (`framing_turns`: `/compact` pasted after the newline turn, `/clear` last as before); Expected amendment 4 names test-plan §3 `run`; Counts `runb-pastes` (five added pastes)."
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: "After the sentence on chunk 2026-10-13-paste-newline-ledger-row's four added turns, add: since chunk 2026-10-18-compact-post-condition-ledger-row Run B makes five added turns (the `/compact` paste joined before `/clear`, its answer waited up to the probe deadline); that chunk's integration entry read 46.58 s, and no kill line moved."
    sidecar: "test-plan §3 → Bootstrap phases: Run B five added turns since the compact chunk; no kill line moved."
    rationale: "Report Symbols / APIs (`framing_turns` waits `local_command_answered` up to `PROBE_DEADLINE` for the `/compact` paste); Gates: the integration entry green (46.58 s); Files lists no `.config/nextest.toml` change. The site the report's `dialogless` pattern names here (@c2923) names `dialogless_home` only and states no count — no change there."
    basis: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Seed strategies (Recorded hook payloads row)
    change: "Run B's framing payloads read `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (six today, under `2.1.287` only), and the count reads the 2.1.287 set holds 34 files by `ls fixtures/claude/2.1.287 | wc -l`."
    sidecar: "test-plan §7 Recorded hook payloads: six framing payloads with compact-<n>; the 2.1.287 set 34 files."
    rationale: "Report Counts `set-287` (33 → 34 files, test-plan 931 ×3) and `framing-var` (five → six, `compact-1` added); Files: new `fixtures/claude/2.1.287/SessionStart.compact-1.json`."
    basis: .andromeda/test-plan.md:931
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture hygiene
    change: "The walk reads `2.1.287`'s six recorded framing variants, `UserPromptSubmit.paste-1.json`, `UserPromptSubmit.paste-2.json`, `UserPromptSubmit.paste-3.json`, `SessionStart.compact-1.json`, `SessionEnd.clear-1.json` and `SessionStart.clear-1.json`."
    sidecar: "test-plan §7 Fixture hygiene: six framing variants walked, SessionStart.compact-1.json named."
    rationale: "Report Counts `framing-var` (test-plan 965 ×3, five recorded framing variants → six); Files: `tests/contract_fixture_hygiene.rs` modified; Outcome (tests): hygiene passes `compact-1`; Schema / config: scrubbed, schema `claude-fixture.v1.json`."
    basis: .andromeda/test-plan.md:965
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fixture library
    change: "The recorded framing variants read (`paste-<n>`, `compact-<n>`, `clear-<n>`)."
    sidecar: "test-plan §7 Fixture library: compact-<n> among the framing variants."
    rationale: "Report Counts `framing-var` (test-plan 920; rows listing the stems → changed, `compact-1` named)."
    basis: .andromeda/test-plan.md:920
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: "After the `agents-join` arm add: the `local-command-compact` arm (since chunk 2026-10-18-compact-post-condition-ledger-row, `local_command_compacted`) passes only when `compact_start` (the first SessionStart of source `compact` after the newline turn's prompt) finds one whose non-empty `session_id` equals Run B's first SessionStart `session_id`, no UserPromptSubmit carries `/compact`, and `compact_latency_ms` is within `COMPACT_CONFIRM_WINDOW`: `check_local_command_compact_passes_on_the_recorded_shape`, `check_local_command_compact_fails_on_each_broken_post_condition` and `check_local_command_compact_fails_without_a_capture`, beside `ledger_rows_are_the_twenty_four_measured_behaviours_in_order`."
    sidecar: "test-plan §4: the local-command-compact arm and its three check_local_command_compact_ unit functions."
    rationale: "Report Symbols / APIs (the `check` arm `local_command_compacted`, `compact_start`, `TypedRun::compact_latency_ms`); New text, by line: ledger.rs 3318-3326, 3328-3367, 3369-3386 (the three test functions); Outcome (arch) names `ledger_rows_are_the_twenty_four_measured_behaviours_in_order`. test-plan §4 names every other arm's cases; the new arm's are unnamed."
    basis: crates/viola-agent-claude/src/ledger.rs:3303-3387
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: "The version gate's older-stamp oracle (`src/run/version_gate.rs` `stamps_verdict_needs_the_dialog_rows`) reads: an all-`pass` stamp of the twenty-three pre-chunk ids reads unverified (`(false, None)`, the twenty-three rows of an older stamp), and adding `local-command-compact` verifies (`(true, None)`)."
    sidecar: "test-plan §4: the older-stamp oracle at twenty-three older ids, verified with local-command-compact added."
    rationale: "Report Symbols / APIs (`src/run/version_gate.rs` `stamps_verdict_needs_the_dialog_rows`: the twenty-three pre-chunk ids all passing read `(false, None)`, adding `LocalCommandCompact` reads `(true, None)`); Counts `older-stamp` reports the control unfired, a master that states it being a detector's catch: test-plan states it, worded as the twenty-one (Unix) or eighteen (Windows) older ids. Whether the Windows clause (one of the eighteen and `resume-session` verifies on Windows only) still holds is not stated by the report — the orchestrator's re-read."
    basis: src/run/version_gate.rs:275-281
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: "The local-command decision reads twelve `send_local_command_` functions in `src/run/send/classify.rs` — the eight as listed and, since chunk 2026-10-18-compact-post-condition-ledger-row, `…_compact_is_confirmed_by_its_same_session`, `…_compact_is_not_confirmed_by_another_session_start`, `…_compact_window_expiry_is_no_prompt_submitted_at_its_own_bound` and `…_compact_with_a_trailing_newline_is_confirmed_by_its_same_session`; and after the `/clear` clause: `/compact` on a verified CLI is confirmed only by a `session-start` with cause `compact` and the remembered (same) session id — another cause, another id, no id or nothing remembered claims nothing, no `prompt-submitted` claims it — and its expiry at `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL; `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send), clock-driven, gives `no-prompt-submitted` with the cursor (`send_compact_confirm_window_is_its_own_bound` reads `window_ms(slot.window())` 40 000)."
    sidecar: "test-plan §4: twelve send_local_command_ functions; /compact's same-session post-condition and its own 40 s window at the unit tier."
    rationale: "Report Counts `send-tests` (test-plan 515 @c1783: eight → twelve functions, the four named); Symbols / APIs (`SendSlot::session_started`, `SendSlot::window()`, `COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL); New text, by line: classify.rs 325-366, 368-420, 436-480, 482-525; Outcome (arch / a11y) and Deviation 6."
    basis: src/run/send/classify.rs:313-525
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover
    change: "The trailing-newline cases read eight tests whose names carry `trailing_newline`, four of them in `src/run/send/classify.rs` (the fourth `send_local_command_compact_with_a_trailing_newline_is_confirmed_by_its_same_session`, since chunk 2026-10-18-compact-post-condition-ledger-row), and four local-command cases; the clause on which of them take LF, CR and CRLF tails as rstest cases re-read for the new function."
    sidecar: "test-plan §4: trailing_newline tests seven → eight (classify.rs four), local-command cases three → four."
    rationale: "Report New text, by line: classify.rs 482-525 adds `send_local_command_compact_with_a_trailing_newline_is_confirmed_by_its_same_session`; Deviation 2 (it covers `/compact` with a trailing newline; the unconfirmable trailing-newline table lost its three `/compact` rows). The report does not state the new function's rstest tails."
    basis: src/run/send/classify.rs:482-525
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered
    change: "The run without the framing replay reads failing `local-command-compact`, `local-command-clear` and, on Unix, the three statusline rows (`stamped 2.1.0  19 pass  5 fail` on Unix, `19 pass  2 fail` on Windows)."
    sidecar: "test-plan §5: cli_verify's no-framing summary 19 pass 5 fail / 19 pass 2 fail, local-command-compact failing."
    rationale: "Report Counts `dialogless` (test-plan 586 @c1237: the no-framing summaries of `tests/cli_verify.rs` now `19 pass  5 fail` / `19 pass  2 fail`)."
    basis: .andromeda/test-plan.md:586
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered
    change: "The tag-turn modal case's summary reads `21 pass  3 fail` on Unix, `18 pass  3 fail` on Windows, `paste-newline` failing with `local-command-compact` and `local-command-clear`."
    sidecar: "test-plan §5: cli_verify's tag-turn modal summary 21 pass 3 fail / 18 pass 3 fail."
    rationale: "Report Counts (the other `tests/cli_verify.rs` summaries: the tag-turn modal `21 pass  3 fail` / `18 pass  3 fail`, no master site found by the patterns — a master that states them is a detector's catch; test-plan 586 states `21 pass  2 fail` / `18 pass  2 fail`)."
    basis: .andromeda/test-plan.md:586
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent
    change: "Under `--framing` the probed `/clear` fires the recorded `clear-1` SessionEnd then SessionStart and no UserPromptSubmit, and the probed `/compact` fires the set's recorded `SessionStart.compact-1` with `session_id` set to the last SessionStart the agent fired (the launch one, or a replayed clear's; only a SessionStart whose fixture exists moves it) and no UserPromptSubmit; a set without the variant fires nothing for it (unit `fake_compact_replay_carries_the_last_fired_session_id`)."
    sidecar: "test-plan §7 Fake agent: the --framing compact replay (SessionStart.compact-1 with the last fired session id, no UserPromptSubmit)."
    rationale: "Report Symbols / APIs (`src/bin/viola-fake-agent.rs`: the compact stem, `Agent::session`, no option or seam added; unit case `fake_compact_replay_carries_the_last_fired_session_id`, fake-agent 1919-2004); Reverted / negative API facts (the session moves only on a SessionStart with a fixture); Counts `framing-var` names test-plan 942."
    basis: src/bin/viola-fake-agent.rs:1919-2004
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent
    change: "The `--framing` option's description adds: the probed `/compact` fires the recorded `SessionStart.compact-1` with the last fired SessionStart's `session_id` and no UserPromptSubmit, a set without the variant firing nothing for it."
    sidecar: "test-plan §7 Fake agent modes: --framing's compact replay."
    rationale: "Report Symbols / APIs (`src/bin/viola-fake-agent.rs` under `--framing`); Counts `framing-var` names test-plan 957."
    basis: .andromeda/test-plan.md:957
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §7 Test Data & Fixtures → Fake agent
    change: "The payload clause reads: no other payload is built save the compact replay's `SessionStart.compact-1` under `--framing`, whose `session_id` is set to the last SessionStart the agent fired; without `--resume` every other payload is the recorded bytes."
    sidecar: "test-plan §7 Fake agent: the compact replay is the one built payload beside the --resume launch."
    rationale: "The site states that no payload is built beyond the `--resume` launch and that without `--resume` every payload is the recorded bytes; report Symbols / APIs: under `--framing` the compact stem fires `SessionStart.compact-1` with `session_id` set to the last fired SessionStart's id — a built payload without `--resume`. Counts `framing-var` names test-plan 940."
    basis: .andromeda/test-plan.md:940
    dependent-of: D-tests-coverage
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events"
    change: "Replace the sentence on `/compact` with: `/compact` on a verified CLI is confirmed by its post-condition alone, a `session-start` with cause `compact` and the session's own (unchanged) `agent_session_id`, the line before `send-confirmed{cursor}`, exit 0, inside its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL) or expiring `not-delivered`/`no-prompt-submitted` at it (`send_compact_on_a_verified_cli_is_confirmed_by_its_same_session`, chunk 2026-10-18-compact-post-condition-ledger-row); on an unverified CLI it is `unconfirmable` at once, `send-issued` then `send-confirmed{confirmed:false}` and no window (`send_compact_on_an_unverified_cli_is_unconfirmable_at_once`)."
    sidecar: "test-plan §6 Path 2: /compact on a verified CLI confirmed by its same-session SessionStart; send_compact_on_a_verified_cli_is_unconfirmable_at_once retired."
    rationale: "Report Retired: the `/compact` reads `unconfirmable` at once on a verified CLI behaviour and the test `send_compact_on_a_verified_cli_is_unconfirmable_at_once` (replaced by `send_compact_on_a_verified_cli_is_confirmed_by_its_same_session` and `send_compact_on_an_unverified_cli_is_unconfirmable_at_once`); Counts `compact` / `readback` / `send-tests` name test-plan 666 @c541 / @c681."
    basis: tests/cli_send.rs:740-781
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events"
    change: "The `local` outcome's parenthetical reads: a ledger-listed command with no measured post-condition: `/remote-control` on any version, `/compact` and `/clear` on an unverified CLI."
    sidecar: "test-plan §6 Path 2: the local outcome lists /compact only on an unverified CLI."
    rationale: "Report Retired (`/compact` unconfirmable at once on a verified CLI) and Symbols / APIs (`LOCAL_COMMANDS`: `/compact` carries `Some(PostCondition::CompactedSession)`, `/remote-control` keeps `None`; on an unverified CLI `/compact` stays `Confirms::Nothing`); Counts `compact` names test-plan 666."
    basis: .andromeda/test-plan.md:666
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §1 Test Scope Summary → Critical paths
    change: "The confirmed-`send` path, after the `/clear` clause, adds: `/compact` on a verified CLI waits for its post-condition only (a `session-start` with cause `compact` and the session's own unchanged id) inside its own window, `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL), and is never claimed by a `prompt-submitted`; a missed post-condition is `not-delivered`/`no-prompt-submitted`."
    sidecar: "test-plan §1 Critical paths: /compact on a verified CLI waits for its same-session post-condition in its 40 s window."
    rationale: "The site names `/clear` as the only listed command that waits on a verified CLI, so `/compact` (listed, its post-condition read as unmeasured) reads as `unconfirmable` at once — the claim the report's Retired bullet retires; Symbols / APIs (`PostCondition::CompactedSession`, `SendSlot::window()`); Counts `timeout-ext` names test-plan 197 as changed where it names `/compact` on a verified CLI."
    basis: .andromeda/test-plan.md:197
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Path 2 — confirmed `send` with CL-1 events"
    change: "The human-mode outcomes step reads: a `/clear` or a `/compact` confirmed by its post-condition prints `[RB] read back` like any confirmed send (the unconfirmable line stays for `/remote-control` and for any listed command on an unverified CLI)."
    sidecar: "test-plan §6 Path 2: a confirmed /compact prints [RB] read back."
    rationale: "Report Outcome (design / layouts / a11y): a human-mode `viola send` of `/compact` on a stamped home prints `[RB] read back` with its cursor, exit 0, and on an unstamped home the unconfirmable line; the site names only `/clear` as confirmed by its post-condition. The report states no line number for this step."
    dependent-of: D-retired
note: "D-tests-framework found no drift (nextest gates and cargo test/rstest as §2/§4; the case_4 red seen once under plain `cargo test`, a runner §4 does not gate on, is the report's WATCH pin, not a framework change). Coverage gaps carried in the report and not proposed as amendments: Deviation 6 (the `run.confirm_window` span's `window_ms` field proved at the unit tier only, no captured-span test) and `tests/tui_channel_fds.rs`'s retry having no forced red-before-green (scope record); test-plan's E2 lines 800 / 804 still hold. Two basis entries cite source ranges from the report's New text, by line (ledger.rs 3303-3387 is a row's first-to-last within one file's list: 3303-3316 to 3369-3386 under the added range 3303-3387). The `verify_window_` no-screen case's time against the mutants profile's 45 s with the sixth paste is not stated by the report."
