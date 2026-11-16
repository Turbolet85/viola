@history_item n=1 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (Path 5 as landed; the human-wheel controls row) Path 5's wheel tests landed as `tests/tui_wheel.rs` (outer-PTY steps, the harness-turn case, the focus/mouse/resize case, the `^Z` case) and `tests/cli_wheel.rs` (`pause`/`release`, `human-typing` refusals with detail `null` | `manual-pause`, `release-from-driver` exit 20); the human-wheel negative (`send` exit 10) joined `tests/cli_controls_not_disableable.rs`. These are the files and cases a survey-key case extends, and the controls-table row a non-editing-list widening must not weaken (decision/human-takes-wheel-on-edit, decision/focus-mouse-resize-keep-wheel, contract/viola-send/human-typing).

@history_item n=2 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (Path 5 records the classifier's tables; the real CLI has two local routes) Path 5 names the unit pins of the closed non-editing list as extended (rstest `classifier_terminal_reply_is_not_editing`, 8 cases; the whole-and-split test over 23 `REPLIES`; 27 negative controls) and states that the list's reading on a real terminal is a local live measurement, not a suite (foot 1.28.0; keys `n`, `t`, `m` still take the wheel); live readings of a real session are the chunk's own local measurement, never a test layer or CI. This is the direct precedent for joining the survey grammar to the closed list: unit pins plus negative controls in CI, the live grammar reading kept as chunk evidence (decision/terminal-replies-not-editing, decision/terminal-reply-reading-live-measured, decision/live-readings-not-a-test-layer). Trap: the defect was invisible at every CI tier because no tier writes a terminal reply.

@history_item n=3 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — contract/event-log/start-wheel-record now says folder-trust keys passing through the wrapper write `wheel{holder:"human", cause:"human-input"}` before `session-start`, and a later human key adds none; + decision/start-lines-read-from-start-offset (a reused name's log is read from the start's own offset). The closest prior case of a non-prompt CLI key (a trust prompt) taking the wheel. A survey-key case must keep this record order true and read its lines from the start offset.

@history_item n=4 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (6. E2E Test Strategy, 9 rows) contract/event-log/start-wheel-record reworded (two start records, the budget-gate one gone); decision/driver-hints-never-name-release retired into decision/hints-never-suggest-release; decision/send-refusal-order lost the budget rung (now control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted); decision/version-gate-needs-all-relied-rows gained «a retired row it also holds changing nothing». These are the current forms of the wheel-start, refusal-order and version-gate rows this chunk relies on when it adds a ledger row.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send` reads human-typing and then turn-running a second time after the gate returns ready (four clock-driven unit cases); new `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (exit 10 `human-typing`, nothing typed, the key reaches the child; its green is a floor because it cannot tell the first wheel read from the second); `GATE_MAX_WAIT` 8 500 ms; kill lines moved only for a by-design floor with a planted-hang control both ways. Source of contract/viola-send/human-key-during-gate and decision/send-refusal-order, which a survey key that keeps the wheel must not disturb.

@history_item n=6 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — (Path 5 and Path 2 span the running turn) Path 5 step 8 boots over gated `fixtures/fake-scripts/path3.json`; a test that sends twice must end the first turn with a scripted Stop (the fake agent's interactive submit fires none). Trap for any wheel case that sends after the survey key.

@history_item n=7 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (Harness prefixes: four, the cross-session tag included) the harness-injected turn classification (four raw-start prefixes) is what makes «a harness-injected turn does not flip the wheel» hold (decision/human-takes-wheel-on-edit); and (the drift contract) `contract_fake_agent_drift` compares receipt `stdin_hex` byte for byte per recorded set, never insta (decision/fixture-drift-byte-compare, decision/fake-agent-matches-recordings). A new recorded survey screen or fake-agent survey mode must stay inside the byte-compare and walk rules.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — ten ledger rows, verify's interactive runs and the screen fixture class: fake agent `--screens`, `--trusted-root`, `--turn-stop` (argv, no env); screens walked apart against `schemas/claude-screen.v1.json`, signature-rows-only and checked scrub-as-detector, refused whole with a named code; §4 `SIGNATURES` feed the gate on a verified CLI. The founding precedent for a screen-signature ledger row with a verify probe and a recorded screen fixture (decision/screen-fixture-scrub-as-detector, decision/screen-signatures-only, decision/wrapper-boots-pass-trusted-screens).

@history_item n=9 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the long-turn screen, the second literal and the 2.1.302 set) the `input-box-signature` arm reads three Run B screens with `check_input_box_signature_*` cases; `SIGNATURES` gained a second input-box literal pinned by `resting_footer` cases; fake agent `--long-turn-screen <phase>` beside `--tag-turn-screen` with `opts_long_turn_screen_*` parse cases; `cli_verify` modal case at per-OS literals; hygiene walk and Recorded payloads cover `2.1.302`. Closest precedent for adding a new screen literal, its ledger arm, a fake-agent screen option and a recorded screen.

@history_item n=10 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the readiness stall report in wait_ready) `wait_ready` appends the stalled wrapper's last event and process state to the watch report before the panic; signals nothing on a passing boot (decision/readiness-stall-report).

@history_item n=11 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — fake agent gained `--tag-turn-screen <phase>` (eight argv options) and `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`: precedent for a fake-agent screen phase option a verify guard reads, argv-only.

@history_item n=12 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — (4. Unit Test Strategy) ~ contract/readiness-gate/gate-max-wait: the input box reads up on its own rule and prompt rows with no footer literal; (7. Test Data & Fixtures) + decision/screen-fixture-box-rows-as-literals: box rows recorded as their literals alone, the hygiene walk refusing anything else. The latest change to how screen state and screen fixtures are read; a survey screen region and its fixture must fit these rules.

@history_item n=13 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — the vt100 feed property (`feed_prop_a_caught_panic_poisons_until_resize`, 512 cases) and the `vt100_feed` fuzz target landed; `--vt100-panic-bytes` measured, no new env seam. If the survey state reaches the wheel decision through the pump's vt100 model, its panic-safety rows bind (decision/vt100-feed-panic-safe).

@history_item n=14 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the feed-panic passthrough witnessed on the outer terminal) `chaos_feed_panic_refuses_send_and_keeps_passthrough`: after a caught feed panic `send` exits 13 `input-not-ready`, a typed key reaches the child, and the outer terminal is byte-searched for the child's bytes and no viola literal — the one carve-out from never parsing the rendered child screen (decision/vt100-panic-contained, decision/vt100-panic-keeps-passthrough, decision/never-parse-rendered-child-screen). A wheel decision fed by screen state must keep this case green and assert nothing by parsing the child screen.

@history_item n=15 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (the second test-data carve-out: the chaos home) `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, outside G2, G4 and the secret scan, by the founder's ruling (decision/chaos-home-outside-scan); `--vt100-panic-bytes` built.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (twenty-four rows) the full checklist of what moves when a ledger row is appended: `cli_verify` `[NN/MM]` and `STEPS_PASS`/`ROW_IDS`, `contract_ledger_probes` `ROW_IDS`, `LEDGER_ROWS`/`LIVE_PASS`, the arm's `check_*` cases, the older-stamp oracle, the no-framing and tag-turn summaries, the fake agent replay, no kill line moved. The template this chunk's ledger row follows (decision/ledger-row-verify-probe, decision/ledger-probe-contract-suite, contract/verify-output/step-line).

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set, all twenty-four rows on every build) `LedgerRow::relied()` is every row on every build, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` ungated; the older-stamp oracle reads the earlier id sets unverified. A new row extends these ungated literals (decision/version-gate-needs-all-relied-rows).

@history_item n=18 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the statusline-rate-limits arm's live red) a live verify read a row red that measured the account's plan state, not the CLI; the row's reading went to 0.2.0. Trap: a ledger row must measure CLI behaviour that holds on every build, not a schedule- or account-dependent state — directly relevant since the survey may appear only on the CLI's own schedule.

@history_item n=19 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (3. Test Harness Contract) + decision/stamping-verify-passes-transcript: every stamping verify passes the fake agent `--transcript`, else the 22nd row fails; (7. Test Data & Fixtures) + contract/fake-agent-cli/transcript and + contract/fake-agent-input/echo-command. The immediately prior chunk that added a ledger row with a probe and a new fake-agent option and input behaviour: a survey probe needing a fake-agent option must be passed by the harness boot and stamped-home helpers too, or the stamp fails its row.

@history_item n=20 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — (3. Test Harness Contract) + decision/one-verify-stamp-per-build and ~ decision/verify-driven-tests-20s-kill (one stamp per build keeps stamped-home tests under the 20 s kill); (6) ~ contract/viola-wait/non-waking-records: wheel records and a harness-origin prompt never wake a parked wait. A longer verify (a new probe) is paid once per build; a survey-key wheel record must not wake `wait`.

@history_item n=21 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the wake set is six (`EventKind::WAIT_WAKE`), wheel lines not among them (contract/wait/wake-kinds); the controls table `SETTINGS` row added for a new config key with each bad value logged `parse-rejected` (decision/controls-not-disableable).

@history_item n=22 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` landed, walking `--help` options, `VIOLA_*` literals and the config schema; the guard of decision/controls-not-disableable. The scope's «no env var, flag or config key widens the list» is held by this table: any new knob would fail it.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (remove-the-guard witnesses bounded by `WITHIN`) a control whose neutralised guard lets a product process start needs a witness spawned through `tests/support/cli.rs` `Running`, bounded by `WITHIN`: an unbounded one hung 27 minutes under nextest's default profile. Trap for this chunk's remove-the-guard controls on the classifier (the directive asks bounded waits).

@history_item n=24 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — ~ decision/root-wait-bound and ~ decision/within-7s-wait-bound: `WITHIN` is 12 s, the mutants default kill 15 s, below cargo-mutants' 20 s floor (~ decision/mutants-kill-below-cargo-mutants-timeout); ~ decision/send-window-paste-hint-30s-kill's why; + decision/record-home-names-no-home. The current bounds every new wait in this chunk's tests sits under, and the rule for a home given to `viola verify --record` if a survey screen is recorded.

@history_item n=25 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the mutants profile's paste-hint class and the verify_window_ kill at 60 s) the `send_under_the_paste_hint_` class takes a 30 s mutants kill pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`; `verify_window_` at 60 s; kills move only with a planted-hang pair, never by raising the default. A new verify-driven or gate-wait case must fit an existing class, not move a bound; (the headroom floor) the fake agent's spawned commands each run in a process group of their own (decision/fake-agent-own-process-group).

@history_item n=26 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — no per-chunk, pre-push or CI mutation gate: `run --mutants` is named-only and mutation runs at the epoch-boundary audit (decision/mutation-testing-on-demand). Matches the directive's «no mutation run»; the chunk's kills are shown by hand.

@history_item n=27 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `tests/cli_verify.rs` pins verify's step lines and `stamped` summary by literal asserts; the fixture hygiene walk is a run-time directory walk (`claude_fixtures`), not rstest `#[files]`, because rstest refuses an empty glob at compile time (decision/fixture-hygiene-walk, decision/human-output-pinned-by-literals).

@history_item n=28 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the default CLI version moved with the recorded set (`DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home`, `Wrapper::boot`); relayed fixtures were allowed only by founder ruling until a re-probe recorded them. If the survey screen is recorded at a new version, the default-version and per-version-dir rules move together (decision/harness-cli-version-matches-fake, decision/fixtures-per-cli-version).

@history_item n=29 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — (7. Test Data & Fixtures) ~ decision/fixture-hygiene-walk (Hyprland keep-list), ~ contract/fake-agent-cli/argv-only-options and ~ decision/fake-agent-argv-only (the hyprctl personality's mode read from a file beside its exe), ~ decision/recorder-scrub; (5) ~ decision/fake-agent-as-claude; (8) ~ decision/fakes-for-external-services. The current wording of the fixture-recording and argv-only rows a survey-screen fixture and fake-agent survey mode must respect; any non-argv configuration is a named exception.

@history_item n=30 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (7. Test Data & Fixtures) ~ contract/fake-agent-receipt/kind (statusline left the closed kind list), ~ decision/fixture-hygiene-walk except emptied, ~ decision/recorder-scrub; (1) ~ decision/fake-agent-contract-suite variant list. A new fake-agent receipt kind or fixture family must be added to these closed lists.

@history_item n=31 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (test data) the fake agent reads modes from files beside its exe rather than a third env seam, the founder's answer; recorded `Agents.recorded-1.json` joined the walk with planted cases (decision/fake-agent-argv-only, decision/fixtures-walked-for-hygiene). Precedent: no env seam for a new fake-agent mode.

@history_item n=32 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (run --local-live specified) `--local-live` refuses `live-in-ci` under `CI`, builds then runs one verify against the real `claude`, each literal row id naming one pass line (contract/local-live-suite/pass, contract/run-suite/local-live, decision/live-verify-never-in-ci). Adding a ledger row adds its id to `LEDGER_ROWS`/`LIVE_PASS`; the live firing is the founder-capped live start.

@history_item n=33 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the switch's live readings on `claude` 2.1.296 are stated as a local measurement, no suite, beside the tests that hold the shape. Form for the survey's live grammar reading (observed on 2.1.296, V158).

@history_item n=34 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the E2E witnesses on ubuntu-latest) Path 5's windows-2025 F-W3 arm left; the focus case holds the full assertion on `ubuntu-latest` (guard of decision/focus-mouse-resize-keep-wheel).

@history_item n=35 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — Path 5's MCP steps landed in `crates/viola-e2e/tests/mcp_tools.rs` (`mcp_tools_path4_path5_answer_by_id_and_a_paused_send`), the MCP `send` refused `human-typing` compared with the CLI (contract/mcp-send/refused-human-typing). A survey-key case that keeps the wheel touches the same refusal surfaces.

@history_item n=36 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — the shared root helpers are `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`, no root test defining its own copy (decision/one-shared-test-helper, decision/shared-e2e-helpers); later `frozen.rs` joined (2026-11-04).

@history_item n=37 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (tests: run --e2e, the E2E regex widened) the E2E layer regex `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, integration its complement (decision/e2e-layer-by-binary-prefix, decision/test-layer-filtersets, decision/integration-binary-surface-prefix). A new survey test binary's name decides its layer.

@history_item n=38 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (`--local-live` alone runs no other suite) a text ending in one CR was filed `human`, moved the wheel and ended exit 13. Trap: a byte the classifier misreads moves the wheel with no test at any tier; the chunk's negative controls guard the opposite direction (every other key still takes the wheel).
