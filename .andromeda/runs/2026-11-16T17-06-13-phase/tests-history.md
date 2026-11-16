@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (5. Integration Test Strategy) — `~ contract/event-data/turn-ended`: data keys moved 2 → 3 with `cause`, present on an API-error turn end only and absent on a Stop, its one word `api-error`. This is the row this chunk widens from one word to nine; its promise names the key, not the value set, so the closed set's growth lands on the tests/readers that match the word.

@history_item n=2 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (6. E2E Test Strategy) — `~ contract/viola-send/turn-running`: after `turn-ended`, an API-error turn end included, a send is accepted with no pause or release. Every one of the nine cause words must still clear the running turn the same way; the send-after-API-error case is the witness to keep green across the widening.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures, 2 rows) — `~ decision/hook-payloads-recorded-by-verify` except: StopFailure is registered with no recorded payload, no ledger row and no verify probe, its tests using a synthetic StopFailure payload; `+ decision/send-cases-copy-recorded-fixtures`: a send-after-turn case's fixtures root starts as a copy of the recorded set with its synthetic payloads added (a purely synthetic root reads input-not-ready). The nine-word cases ride synthetic StopFailure payloads (one per `error` value) and must follow the copy rule for any case that sends after the turn.

@history_item n=4 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures, 1 row) — `~ contract/fake-script/format`: the script's event enum covers ten registered hook events, StopFailure added to the fake-script v1 schema. A gated StopFailure step is available for E2E cases that need a scripted API-error end per cause word.

@history_item n=5 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (StopFailure no-ledger-row answer ratified, 6. E2E and 7. Test Data entries; `~ decision/hook-payloads-recorded-by-verify` except made standing) — the founder ratified StopFailure registered with no ledger row and no verify probe, its live reading owned by the route entry «The 2.1.325 recorded fixture set». Constrains this chunk: no recorded StopFailure payload exists to replay, and no live start is planned, so each `error` value is measured from source or a synthetic payload, not from a recording this chunk makes.

@history_item n=6 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — named the `turn_ended_background_tasks_*` unit cases (`hook.rs`), `state_events_background_tasks_and_option_details_are_known_keys` and `EventKind::TurnEnded`'s data keys in §5 On-disk, and Path 3's `hook_a_recorded_stop_carries_its_background_task_count`. The precedent and test sites for a `turn-ended` data field's unit, on-disk round-trip and wake-witness tiers, which the nine-word `cause` extends.

@history_item n=7 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — wake set by rule `EventKind::WAIT_WAKE` (six), `state_events.rs` round-tripping kinds by rule `EventKind::ALL`, and `viola-core` (no rstest dev-dependency) writing table-driven cases as labelled literal tables. `TurnEndCause` lives in `viola-core`: its nine-word table must be a literal table, not `#[case]`, and counts are stated by their rule (`TurnEndCause::ALL`), never a bare number.

@history_item n=8 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (the torn append as landed) — the reader's three counts (`three_counts`: torn, unknown kind, known kind with unknown keys) and that it never panics. Bears on the open question of how a reader treats a `cause` value outside the closed set (a known key with an unknown value is not one of the three counts today); the plan must say whether such a line is counted, refused or read as `api-error`.

@history_item n=9 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the reply screen and the fault screen) — `viola_core::is_code_word` (1–64 bytes of `[a-z0-9-]`, `code_word_literal_table`), and peer words that are no code word read `unknown` / `null`. The nine cause words must each be a code word; the screen is the existing pattern for a closed word that never carries upstream text.

@history_item n=10 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the after-every-test check replaced; the error-body enumeration) — `error_body_hit` / `assert_error_body_clean` (classes `absolute-path` · `caused-by` · `serde-path` · `planted`, place and class never bytes) and `tests/contract_error_bodies.rs` with `thiserror_messages_interpolate_nothing` and the census of writers. The witnesses for "the CLI's error text reaches no event, log line or reply": a planted `error_details` / message canary checked with these, never by printing what was read.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`channel-response`'s closed `detail`) — a log field's value set closed from any string to three words, held by a case and by `schemas/diag-line.v1.json`. Precedent for closing `cause` to nine words in the event / log schemas and holding it by schema plus a named case.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `instance-unreachable` body names its cause) — a closed cause list widened from one value (`null`) to five words carried on CLI `--json` and MCP alike, with a Cross-surface parity row (MCP body equals the CLI document less `v`). The same shape as this chunk's `wait --json` / MCP `wait` carrying nine words: parity per word is the precedent.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the closed causes at the unit tier; the exit-cause table is built) — `viola_core::UnreachableCause` with no serde path held by `unreachable_cause_is_each_its_own_code_word`, and the table's self-check `every_code_is_a_code_word`. Unit pattern for `TurnEndCause` growing from `ALL: [Self; 1]` to nine.

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (exit 21's sixth cause) — adding one word to a closed cause list touched the enum, its word test, the CLI/MCP cause lists, the table's self-checks and the hint, each named. Precedent for the consumer sweep this chunk's scope requires (every site that accepts exactly `api-error`).

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy, `~ contract/viola-send/exit-11`; `+ decision/retired-words-as-fixtures`) — an older wrapper's retired word reads `unknown` on a newer reader, and retired words live as committed fixtures. Forward/backward compatibility precedent: a newer build's cause word read by an older reader, and a value outside the map reading `api-error`, need a stated reading and a test.

@history_item n=16 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — the hook stdin property (512 cases) and the `hook_stdin` fuzz target with its seed count by rule. The StopFailure `error` field is parsed by the hook stdin surface; a new `error` value or field shape falls under `decision/property-tests-on-parsers` and the fuzz seeds.

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — MCP `wait` covered by `mcp_tools_path2_path3_send_wait_last_match_the_cli` (compared with the CLI's `--json` less `v`) and the clamp by `handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`; MCP tests own their `viola mcp` child. The site where MCP `wait`'s nine-word `cause` is witnessed equal to the CLI's.

@history_item n=18 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — Path 3 as landed: `tests/cli_wait_last.rs` over `fixtures/fake-scripts/path3.json`, cli and wrapper channel. The CLI `wait --json` path a cause-word case extends.

@history_item n=19 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `~ contract/viola-wait/non-waking-records` (activity, wheel and a harness-origin prompt-submitted never wake), `~ contract/fake-agent-cli/script` (the fake script drives Notification and UserPromptSubmit steps), `+ decision/one-verify-stamp-per-build`. The latest change to `wait`'s wake contract and the fake script; every API-error `turn-ended` must still wake `wait` whatever its word.

@history_item n=20 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board pinned by literals in `tests/cli_list.rs`; `wait` / `last` keep `\n` and `\t` while `list` rows escape every control character. If the board or `list` shows a cause word, its literal pins and the escaping rule bound how it is printed.

@history_item n=21 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables) — a route entry's live readings are that chunk's own local measurement, not a test layer, never in CI (`decision/live-readings-not-a-test-layer`). With no live start planned here, a mapping needing a live reading is named with its probe, not covered by a suite.

@history_item n=22 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound` and `~ decision/within-7s-wait-bound`: `WITHIN` = 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s floor. The bound every new root wait in this chunk's tests uses (the directive asks for bounded tests).

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded by a time limit (an unbounded one hung 27 minutes). Trap for this chunk's controls on the wrapper's appended-line check (named with `--timeout S`).

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the bare temp-dir home and the liveness-check case) — trap: an oracle moved by a shared body can hide a test that never reached its subject. When eight new words share the `api-error` path, each word's case must show it reached the classifier, not a shared fallback.

@history_item n=25 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration U35) — the test-plan Decisions Log left the body; with `decision/closed-enums-need-decisions-log` standing, a closed enum's new values are recorded by amendment in its place (as 2026-10-03-mutation-scoring-completion did). Growing the closed `cause` set is a closed-value change that needs that record.

@history_item n=26 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. Test Harness Contract) — `+ decision/stamping-verify-passes-transcript`: every stamping helper passes `--transcript` so its stamp holds the 22nd row. Trap if this chunk adds a stamped-home helper or harness boot path.
