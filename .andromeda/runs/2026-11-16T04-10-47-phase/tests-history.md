@history_item n=1 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (Harness prefixes: four, the cross-session tag included) — widened the harness-origin prefixes to four, classified on the raw untrimmed start (a tag after a leading space stays `human`); a trim-tolerant match must not loosen this untrimmed origin rule (contract/prompt-submitted/origin-harness, decision/harness-turn-keeps-the-wheel).

@history_item n=2 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (The fake-agent drift contract, byte for byte; change-driven size; matchers and S8 moved) — made `contract_fake_agent_drift` compare hook stdin byte for byte with the recorded fixtures and kept UserPromptSubmit's trailing newline; any fake-agent change to submit a trimmed prompt must keep the drift contract green (decision/fake-agent-matches-verify-fixtures, decision/fixture-drift-byte-compare).

@history_item n=3 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** (the named spine bound, the vt100 property and fuzz target, the fixture-repo leak's cause) — the readiness-gate chunk landed the vt100 feed property (512 cases) and `vt100_feed` fuzz target and planned `--vt100-panic-bytes`; the gate this chunk races sits on that feed (decision/parser-property-fuzz-coverage, decision/vt100-panic-contained).

@history_item n=4 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (Path 2 as landed; paste_text joins the fuzz targets) — landed Path 2 (`path2_send_confirms_with_cl1_events`), the `paste_text` fuzz target and the 512-case `validate_paste_text` property against a per-char oracle; "validated as received, never stripped" extends exactly these pins (decision/paste-control-characters-rejected, contract/viola-send/confirmed).

@history_item n=5 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** (the second test-data carve-out: the chaos home (F4)) — `tests/chaos_feed_panic.rs` alone boots outside G2/G4/secret scan; a new readiness-gate race test must not join that carve-out (decision/fresh-home-per-test except, decision/vt100-panic-contained).

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (Path 5 as landed; the human-wheel controls row) — Path 5 landed the human-typing refusals (exit 10, detail null | manual-pause), hints without `release`, and the human-wheel control negative; the misread send moved the wheel to the human and refused later sends, so these are the tests the trim fix must keep green (contract/viola-send/human-typing, decision/wheel-returns-only-via-cli-release).

@history_item n=7 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (Path 5 and Path 2 span the running turn) — a running turn refuses `send` 13 `turn-running` with nothing typed and only a `send-refused` record; every test that sends twice must end the first turn with a scripted Stop (the interactive submit fires none) — the precedent for the gate-race test (contract/viola-send/turn-running).

@history_item n=8 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the turn-running control negative) — the controls table holds `send` during a running harness turn → 13 under every setting; the race fix must keep turn-running a non-disableable control.

@history_item n=9 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the fake agent quiesces its hooks before it exits) — fake-agent exit waits for a running hook (truncated-profile cause); a fake-agent change to start a turn inside the gate window must keep that quiescence.

@history_item n=10 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (ten ledger rows, verify's interactive runs and the screen fixture class) — introduced the `send_window_` mutants override class, screen signatures feeding the full gate, and fake-agent `--screens`/`--trusted-root` (argv, no env); new send-window tests inherit that class and the argv-only rule (decision/screen-signatures-only, decision/fake-agent-argv-only).

@history_item n=11 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** (seventeen rows, the framing tier, the fake agent's two options, Path 2 step 3 as built) — framing variants `UserPromptSubmit.paste-<n>.json` recorded by Run B, `--framing` replay keeping the recorded `prompt`, test-only `--paste-hint-ms`; these recorded payloads are where P3 reads the CLI's submitted (possibly trimmed) prompt (decision/hook-payloads-recorded-by-verify).

@history_item n=12 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (Path 2's local verdicts as landed) — landed local-command `unconfirmable`, `/clear` confirmed by a new session, off-list slash text exit 13, and the forced-window pair `send_under_the_paste_hint_on_a_verified_cli`; the local-command list stays untouched but reads the exact typed text (contract/viola-send/unconfirmable-local-command, contract/viola-send/off-list-slash-text).

@history_item n=13 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (the unwrap's frame and case list, the seventeen row ids, and the floor's Linux reading) — `prompt_text_live_shape_normalises_as_measured` pins texts "whose last newline the CLI dropped" — a measured CLI trim already absorbed in normalisation; the literal-oracle rule says pins state what the measured prompt normalises to (decision/send-confirmation-exact-match, decision/literal-test-oracles).

@history_item n=14 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (kills, hold and cap follow the 8.5 s gate; Path 2 delivered under the hint) — gate verdict ready/wait/input-not-ready up to `GATE_MAX_WAIT` 8 500 ms; `send` reads human-typing then turn-running a second time after the gate returns ready (four clock-driven units); `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is only a floor; a refusal at the bound is unit-tier only — the race this chunk closes sits exactly in this second read (decision/send-refusal-order, contract/viola-send/human-key-during-gate).

@history_item n=15 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** (the matcher reads the typed text; the trailing-newline cases; a measured corrupt-profile mechanism and its witness) — the direct precedent: the matcher compares normalised `prompt-submitted.text` with the typed text, "validated as received, then typed without its trailing LF", `trailing_newline` tests incl. a prompt that keeps the newline claims nothing; trap: a test binary can never be entered as the wrapper's child (decision/send-confirmation-exact-match, decision/sent-text-typed-form).

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (`--local-live` alone runs no other suite; the live readings of Path 2 are measured) — recorded a text ending in one CR filed `human`, moving the wheel and ending exit 13 — the same failure shape as the trailing-space instance; trap: such an outcome had no pin at any tier (contract/viola-send/not-delivered).

@history_item n=17 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (Path 5 records the classifier's tables; the real CLI has two local routes) — a route entry's live readings are that chunk's own local measurement, not a test layer, never in CI; governs how any live trim reading this chunk asks for is stated (decision/live-readings-not-a-test-layer, decision/terminal-replies-not-editing).

@history_item n=18 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the typed text, the `empty-text` refusal and the Path 2 readings) — typed text drops every trailing CR and LF; order control-character → empty-text → human-typing; `send_empty_text_straight_to_the_wrapper_is_refused` and `send_empty_text_is_refused_at_once`; a whitespace-only (non-empty) text is the scope's open question beside this rung (decision/empty-text-refused-before-the-wheel, contract/viola-send/empty-text).

@history_item n=19 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — first-match override order pinned by test; a kill line moves only for a floor that is by design, with planted-hang controls both ways — binding on any new gate-race test's bound.

@history_item n=20 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** (the inner CR outcome is pinned at two tiers and read live) — typed text: every CR LF and lone CR as one LF, no CR; tables `typed_text_drops_every_trailing_newline` / `typed_text_every_inner_cr_is_one_lf`, the 512-case property, `send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing`, `channel_paste_accepts_lf_cr_tab_and_multibyte`; a fake-agent green proves only the typed bytes — the guards a trim tolerance must not break (decision/sent-text-typed-form, contract/wrapper-channel/accepted-text).

@history_item n=21 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** (the shared root helpers named, the first whole-member viola-e2e score, the waits counted by pattern) — `tests/support/{events.rs,cli.rs,…}` hold the one events reader/wait and child runner; no root test file defines its own copy (decision/one-shared-test-helper, decision/shared-e2e-helpers).

@history_item n=22 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (moved unit tests re-cited after the send and human split) — `trailing_newline` / `inner_cr_is_one_lf` tests live in `src/run/send/classify.rs` and `src/run/send/relabel.rs`, `send_empty_text_*` stay in `src/run/send.rs` — the files this chunk edits.

@history_item n=23 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** (eighteen ledger rows in the harness and the suites, `paste-3`, Run B's four added pastes) — precedent for standing an undocumented paste behaviour on a ledger row with a Run B paste, recorded variant and literal tallies in `cli_verify` / `contract_ledger_probes`; the scope leaves open whether the trim needs such a row (decision/ledger-row-verify-probe, decision/ledger-probe-contract-suite).

@history_item n=24 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows, Run B's five added pastes, the compact variant) — the last ledger-row addition: the cost of a row (relied count, `[NN/24]`, `ROW_IDS`, no-framing summaries, older-stamp oracle) if the trim tolerance takes a row (decision/version-gate-needs-all-relied-rows).

@history_item n=25 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (/compact's same-session confirmation and its own window at the unit and E2E tiers) — `/compact` claimed only by a same-id session-start in `COMPACT_CONFIRM_WINDOW`, never by a prompt-submitted; the trim match must not let a prompt-submitted claim a local command (decision/compact-confirmed-by-same-session, decision/compact-confirm-window).

@history_item n=26 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — MCP send landed in `crates/viola-e2e/tests/mcp_tools.rs`, each result compared with the CLI's `--json` less `v`; the send outcomes this chunk changes must stay equal there (decision/cli-mcp-identical-payloads, contract/mcp-send/success).

@history_item n=27 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6) — `setting_does_not_disable_the_send_paste_control` covers the colour variables; the paste control stays non-disableable under every setting.

@history_item n=28 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (tests: revive's machine form, the completeness case and the home-resolution cases landed) — added `viola_variable_does_not_disable_the_send_paste_control` and the rule that a test-started viola drops `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` (decision/test-started-viola-drops-env, decision/test-viola-drops-session-vars).

@history_item n=29 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data: the fake agent's agents files, the test-scoped PATH, the Agents fixture class, the agents property and fuzz target) — the fake agent is configured by files beside its exe or argv, never a new env seam, under a test-scoped PATH; a new fake-agent submit/turn option follows this (decision/fake-agent-on-test-path, decision/tests-never-read-host-sessions).

@history_item n=30 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the long-turn screen, the second literal and the 2.1.302 set in the suite) — two input-box literals and the long-turn screen; `2.1.287` and `2.1.302` stamped sets — the recorded sets P3 reads for the submitted prompt (scope names `2.1.325` and older) and the gate's settled-input-box predicate (decision/screen-signatures-only).

@history_item n=31 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill at 60 s) — `send_under_the_paste_hint_` cases take a 30 s mutants kill, pinned by `mutants_paste_hint_override_holds_the_class_above_the_default_kill`; a gate-race test that waits out the gate must join a by-design class, never raise the default (decision/send-window-paste-hint-30s-kill).

@history_item n=32 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named, the scratch control made, E4 retitled) — every command the fake agent spawns runs in its own process group; keep it when changing the fake agent (decision/fake-agent-own-process-group).

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs and the MCP frame tools under the instance check) — `send` runs `check_instance` before any log or frame (exit 21, no line); new send tests start from viola-made homes (decision/strict-modes-before-any-log).

@history_item n=34 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — trap: a remove-the-guard witness that lets a product process start hung 27 min; bound it through `tests/support/cli.rs` `Running` (decision/bounded-remove-the-guard-witness; scope: no test may hang past a failure).

@history_item n=35 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the feed-panic passthrough witnessed on the outer terminal) — `chaos_feed_panic_refuses_send_and_keeps_passthrough`: send 13 `input-not-ready`, the outer terminal byte-searched only for this case; the gate change must keep it (decision/vt100-panic-keeps-passthrough, decision/never-parse-rendered-child-screen).

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the exit-2 paths that write no document) — `viola send`'s exit-2 path writes the fixed `UNUSABLE_TEXT` line to stderr with no document, the exception to one-document-per-verb (decision/cli-json-one-document, contract/cli-json/one-document-per-verb).

@history_item n=37 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table is built) — `tests/cross_exit_causes.rs` with self-checks `no_hint_names_release` and `every_code_is_a_code_word`; send's refusal hints stay unique and never name release (decision/no-release-in-driver-hints, decision/hints-never-suggest-release).

@history_item n=38 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the CLI tests over std::process::Command, the E2E tier) — CLI E2E runs `std::process::Command` with literals for `--json` documents and Path 2's human lines; no trycmd/assert_cmd (decision/cli-tests-as-subprocess, decision/cli-fixed-strings-pinned-by-literal).

@history_item n=39 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit Test Strategy: 1 row(s)) — `~ contract/readiness-gate/gate-max-wait`: the input box reads up on its own rule and prompt rows with no footer literal — the settled-input-box predicate the turn race runs against.

@history_item n=40 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (7. Test Data & Fixtures: 1 row(s)) — `+ decision/screen-fixture-box-rows-as-literals`: screen fixtures keep box rows as literals alone, the walk refusing more; a fake-agent screen for the race follows it.

@history_item n=41 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy: 5 row(s)) — `~ decision/send-refusal-order` dropped the budget rung (control-character, empty-text, human-typing, turn-running, input-not-ready, no-prompt-submitted, second read after the gate); `~ decision/version-gate-needs-all-relied-rows` gained the retired-row clause — the current order this chunk must keep or extend.

@history_item n=42 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 6 row(s)) — `~ contract/viola-send/exit-11`: exit 11 retired, never reused; send's codes 10–14 stay as they are (contract/cli-exit-codes/10-14).

@history_item n=43 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 9 row(s) (1 of 2)) — `− decision/driver-hints-never-name-release → decision/hints-never-suggest-release`; the hint rule binding send's refusal hints.

@history_item n=44 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 1 row(s), root-wait-bound) — `~ decision/root-wait-bound`: root waits 12 s below the 15 s mutants kill; a new race test's waits sit under it.

@history_item n=45 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 2 row(s)) — `~ decision/send-window-paste-hint-30s-kill` (why) and `~ decision/within-7s-wait-bound` (WITHIN = 12 s): the bounds a gate-window test inherits.

@history_item n=46 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract: 1 row(s), mutants kill) — `~ decision/mutants-kill-below-cargo-mutants-timeout`: 15 s default kill, a stated departure from "never raise the default"; no further raise for this chunk.

@history_item n=47 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (6. E2E Test Strategy: 1 row(s)) — `~ contract/viola-send/turn-running`: after turn-ended, an API-error end included, a send is accepted with no pause or release — the turn rung the race fix touches.

@history_item n=48 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures: 2 row(s)) — `+ decision/send-cases-copy-recorded-fixtures` (a send after the first turn needs a copy of the recorded set, else input-not-ready) and the StopFailure synthetic-payload exception to `decision/hook-payloads-recorded-by-verify` — the precedent for a trimmed-prompt or race fixture.

@history_item n=49 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures: 1 row(s)) — `~ contract/fake-script/format`: fake scripts over ten registered hook events (StopFailure added).

@history_item n=50 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy: 1 row(s), start-wheel-record) — `~ contract/event-log/start-wheel-record`: trust keys through the wrapper write a `wheel{human, human-input}` before session-start; a test asserting the wheel after a send must read from its own cursor, not assume driver from start.

@history_item n=51 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (6. E2E Test Strategy: 1 row(s), start offset) — `+ decision/start-lines-read-from-start-offset`: a start's lines are read from its own offset (trap: a reused name read earlier lines) — applies to records asserted after a send's cursor.

@history_item n=52 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (6. E2E Test Strategy: 1 row(s)) — `~ contract/viola-wait/non-waking-records`: a harness-origin prompt-submitted never wakes `wait`; a relabelled driver prompt-submitted must keep wait's wake rule (untouched per scope).

@history_item n=53 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness Contract: 2 row(s)) — `+ decision/one-verify-stamp-per-build`: stamped homes copy one verify stamp per build (guard `stamped_home_reuses_one_stamp_for_every_home`); check-guarded send tests start stamped from it (decision/guarded-paths-start-stamped).

@history_item n=54 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (7. Test Data & Fixtures: 1 row(s)) — `~ contract/fake-agent-cli/script`: fake scripts drive Notification and UserPromptSubmit steps — the existing means to start a turn inside the readiness gate's window for the race test.
