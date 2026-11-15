@history_item n=1 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a running turn refuses `send` exit 13 `turn-running` until `turn-ended`; every test that sends twice must end the first turn with a scripted Stop over the gated `fixtures/fake-scripts/path3.json`; a bare `release` leaves a turn running, `pause` then `release` clears it (`path5_a_turn_left_running_is_cleared_by_pause_then_release`). This is the exact state the API-error end leaves stuck: the chunk's red-before-fix test sits beside these cases, and its fix must make a StopFailure clear turn-running as Stop does, with no `pause` + `release`.

@history_item n=2 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the turn-running control negative) — `tests/cli_controls_not_disableable.rs` holds a running turn → `send` 13 `turn-running` under every `FAKE_AGENT_HOOK_PANIC` setting. The API-error end must not open a way to bypass this control; the row stays green when a StopFailure ends the turn.

@history_item n=3 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the fake agent quiesces its hooks before it exits) — the fake agent waits for a running hook before exiting and starts no other. A new fired StopFailure step must keep that ordering (trap: a hangup mid-hook truncated a coverage profile).

@history_item n=4 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — Path 3 as landed: `tests/cli_wait_last.rs` (`path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`) over `fixtures/fake-scripts/path3.json`, plus `tests/chaos_wait_vanish.rs`. The site and the gated-script form a "wait on the API-error turn end returns" witness extends.

@history_item n=5 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — named the `turn-ended` data keys as exactly `last_assistant_message`, `background_tasks` (`EventKind::TurnEnded`, `state_events_background_tasks_and_option_details_are_known_keys`), the unit cases `turn_ended_background_tasks_*` in `hook.rs` and `hook_a_recorded_stop_carries_its_background_task_count`. This is the register a new `cause` key extends: the additive-key precedent (unit + on-disk round trip + a recorded-stop integration case), and the known-keys case must take the new key.

@history_item n=6 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the wake set is six by `EventKind::WAIT_WAKE` and `state_events.rs` round-trips 16 kinds by the length of `EventKind::ALL`; viola-core writes table-driven cases as literal tables (no rstest). Counts carry their rule: if the chunk adds a kind instead of a key, both counts move by rule; if `turn-ended` carries the cause, the wake set is unchanged.

@history_item n=7 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the torn append as landed) — the event reader's three counts (`three_counts`): known-kind lines with unknown keys are counted once and returned. The scope's premise that mixed binary versions stay readable after a `cause` key is added rests on this; an older reader reading `turn-ended{cause}` counts `unknown_fields`, which a test may need to account for.

@history_item n=8 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fake-agent contract as built, consumer-first) — the operator's direction: consumer-first, no shapes invented before a recorded fixture; payloads are `<fixtures>/<cli-version>/<Event>.<variant>.json`. No `StopFailure.*` fixture is recorded, so a synthetic StopFailure payload is a departure this ruling bears on: it must be stated, not slipped in.

@history_item n=9 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** (fixture naming and `--fixtures` root) — fixtures are named by the CLI's own PascalCase hook event (`StopFailure.<variant>.json`) with no mapping table on either side; the fake agent looks them up by that name. Any StopFailure fixture or synthetic payload follows that name form.

@history_item n=10 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the fake agent's resume payload) — precedent for a payload the fake agent sets without a recorded fixture (resume `source`/`session_id`), recorded as such, its recorded payload owed to a later ledger-row entry; argv options counted by rule. The nearest precedent for the StopFailure payload being synthetic until a live recording.

@history_item n=11 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (relayed dialog fixtures; five perf rows) — print mode raises no dialog hook, so fixtures came from another source (relayed live captures) until a re-probe recorded them; and once the dialog hook was a registered event its perf row joined `gate --require perf` by name (`perf::ROWS`). A newly registered StopFailure hook raises both questions: its fixture source, and whether a perf row is owed.

@history_item n=12 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares the fake agent's hook stdin byte for byte with each recorded set and holds the hook order to the spine literal SessionStart → UserPromptSubmit → Stop → SessionEnd. A fake agent that can fire StopFailure must not break that spine literal, and with no recorded StopFailure there is nothing to compare it against.

@history_item n=13 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. Test Scope Summary: 5 rows) — `decision/fake-agent-contract-suite` now lists the spine, dialog, framing, resume and compact variants; `decision/fixtures-recorded-by-live-verify` the recorded families. A StopFailure variant would extend these rows, but only once one is recorded by live verify.

@history_item n=14 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the fake agent's argv options counted by rule (thirteen then), the hygiene walk and Recorded hook payloads over `2.1.287` and `2.1.302`. A new fake-agent option or mode to emit StopFailure moves that count and its `opts_*` parse cases (`decision/fake-agent-argv-only`: options from argv, no env).

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — the latest precedent for adding a ledger row: `LedgerRow::ALL`/`relied()`, `[NN/MM]` lines, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` arrays, the older-stamp oracle, the `cli_verify` summaries, the fixture set's file count and the drift replay all move together. This bears on the scope's P3/P4 question of a StopFailure ledger row with a `viola verify` probe.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` is all 24 rows on every build, `[&str; 24]` literals ungated. This is the current base a new ledger row would extend to 25.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a ledger row whose live reading depended on the account state, not the CLI, read red live and was routed out. A trap for a StopFailure row: forcing an API error live is not reproducible, so a probe that cannot be measured without one is a weak row.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy: 5 rows) — `decision/send-refusal-order` now reads control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted; `decision/version-gate-needs-all-relied-rows` gained «a retired row it also holds changing nothing». The refusal order is what the next `send` after an API-error end must pass, and the relied-rows rule governs a stamp once a row is added.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary: 6 rows) — `decision/snapshot-replay-fields` recovers links, agent_session_id and wheel only; `contract/run-start-events/order` is two records. The pattern of what the wheel and turn state recover from the log, which bears on where turn-running lives in the code while the board reads `idle`.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 6 rows) — `contract/viola-send/exit-11` is retired and never reused, an older wrapper's word reading `unknown`, exit 14. The precedent for closed-set compatibility: a cause word outside the set reads a fixed fallback, never the peer's string.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the closed causes at the unit tier) — `viola_core::UnreachableCause` is a closed cause set in viola-core with no serde path, each word held by `unreachable_cause_is_each_its_own_code_word`. The direct precedent for the chunk's closed turn-end cause set in viola-core, with no Claude-specific name.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the reply screen and the fault screen) — `viola_core::is_code_word` (1 to 64 bytes `[a-z0-9-]`, `code_word_literal_table`); a peer's free text maps to a fixed word (`unknown`) and never reaches a document. The same screen the CLI `error` string's mapping into the closed cause set should follow, with the fallback member for an unknown value.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the after-every-test error-body check, replaced) — `error_body_hit` / `assert_error_body_clean` with a planted value (class `planted`), reporting place and class, never bytes. The ready tool for proving the API-error text (`last_assistant_message` / `error_details`) reaches no event, snapshot or log line. A failing witness must print no canary into JUnit.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error-body enumeration in the contract suite) — `tests/contract_error_bodies.rs` with literal tables and `thiserror_messages_interpolate_nothing`. A new error-carrying surface (the StopFailure payload's error text) is checked against this census.

@history_item n=25 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** (scoped log captures through with_scoped_default) — every scoped log capture in root-bin unit tests goes through `crate::test_support::with_scoped_default`, never a bare `with_default`, or a line can be lost. Applies to any unit case asserting that no log line holds the error text.

@history_item n=26 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — the hook stdin property (512 cases) and the `hook_stdin` fuzz target landed. A new StopFailure payload parse in viola-agent-claude is a parser surface under `decision/property-fuzz-for-parsers` / `decision/parser-property-fuzz-coverage`.

@history_item n=27 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the fake agent's statusline options, the reader's property and seeds) — `hook_stdin` seeds counted by rule (`ls fuzz/corpus/hook_stdin | wc -l`) in §6 and the `run --fuzz-replay` key. A StopFailure seed added to the corpus moves that count.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check on its event and dialog paths) — the hook's event path runs the instance check before any config read; `hook_fail_open.rs` cases that expect lines run in a viola-made instance dir (`Env::Made`); `hook_refused_by_the_instance_check_fails_open_and_writes_nothing` is one rstest over named events. A new `hook stop-failure` event joins these fail-open rows and their `Env::Made` rule.

@history_item n=29 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook fail-open matrices' impostor rows) — the matrices are named by function (`hook_fails_open_silently_within_the_spine_bound`, `hook_dialog_tier_fails_open_silently`), each case under the 1.0 s `Instant` bound; a master states no case count. A new hook event extends these matrices (`decision/hooks-fail-open`, `contract/viola-hook/*`).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a bounded witness through `tests/support/cli.rs` `Running` (an unbounded witness hung 27 minutes). This binds the directive's per-test `gate.py control` under a kill deadline.

@history_item n=31 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract rows) — `WITHIN` is now 12 s and the mutants default kill 15 s (`decision/root-wait-bound`, `decision/within-7s-wait-bound`, `decision/mutants-kill-below-cargo-mutants-timeout`); `decision/record-home-names-no-home` added. These are the bounds the chunk's new waits and controls sit under, and a `--record` home goes through `record_home()`.

@history_item n=32 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate; mutation runs only at the epoch boundary or on demand. With the directive's «no mutation run», each new guard's strength is shown by its one-off remove-the-guard control instead.

@history_item n=33 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `contract/event-log/start-wheel-record` amended (a human-input wheel record may precede session-start) and `decision/start-lines-read-from-start-offset` added. A test reading the wheel and turn records around an API-error end must read from its start's own offset, not line 1.

@history_item n=34 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (the classifier's tables; the real CLI's two local routes) — a route entry's live readings of a real session are that chunk's own local measurement, not a test layer and never in CI (`decision/live-readings-not-a-test-layer`). This frames any live StopFailure reading the operator may grant.

@history_item n=35 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the board's witnesses) — `list_status_of_a_joined_wrapper_is_idle_then_busy` (`tests/cli_list.rs`) pins the board's idle/busy reading. The chunk's «instance reads idle after the API-error end» is asserted beside it, and the defect is the board reading `idle` while turn-running stands.

@history_item n=36 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5 as landed: `tests/tui_wheel.rs` and `tests/cli_wheel.rs` (`pause` / `release`, `human-typing` refusals, hints without `release`). The wheel's turn-running state that a StopFailure must clear lives with these cases (`decision/wheel-returns-only-via-cli-release`).
