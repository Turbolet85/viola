@history_item n=1 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** (the drift contract, byte for byte) — moved the S8 `annotations` assertion to "Dialog answers by dialog_id" and made `contract_fake_agent_drift` compare each hook's `stdin_hex` byte for byte with its recorded fixture, insta kept for decision bodies only; any new PermissionRequest question variant this chunk records is replayed under this contract (rows decision/fake-agent-contract-suite, decision/fixture-drift-byte-compare, decision/dialog-bodies-pinned-by-insta).

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (relayed dialog fixtures, Paths 3 and 4) — first dialog tier: `PreToolUse.*` / `PermissionRequest.*` fixtures for `ask-user-question` / `exit-plan-mode`, the fake agent's matchers evaluated against `tool_name`, Path 4 `tests/cli_answer.rs` over `path4.json` (question answered with annotations, plan revise via the PermissionRequest repeat, "the question's repeat silent", second concurrent dialog empty, `unknown-dialog` exit 13); this is the question/PermissionRequest path the chunk changes, and its "repeat silent" cases are the ones to re-read.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** (five perf rows on an unstamped session) — the `pre-tool-use` perf row times the unverified-CLI dialog path (null at once) on an unstamped session; a PermissionRequest question path must keep that unverified null-at-once behaviour (row contract/perf-budgets/pre-tool-use).

@history_item n=4 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — Path 5's human-wheel refusals and the controls table; `answer` while the human holds the wheel exits 10 with an empty hook stdout, the "human never blocked" rule this chunk's body must keep (rows contract/viola-answer/human-wheel, decision/human-takes-wheel-on-edit, decision/decision-body-gate).

@history_item n=5 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the turn-running control negative) — the controls table's negatives include `answer` on an unstamped home → 12; any new answer path stays under that unstamped refusal and the no-switch-disables-a-control rule (rows decision/controls-not-disableable, contract/cli-exit-codes/12).

@history_item n=6 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — ten ledger rows: `[NN/10]` step lines, `LEDGER_ROWS` literal list, `contract_ledger_probes` stamped vs drift-only sets as literals, boot step 4 flags; the template every row-count change since (and this chunk's new row) moves together (rows decision/ledger-probe-contract-suite, contract/verify-output/step-line).

@history_item n=7 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — fourteen rows; the dialog tier recorded by verify's Runs C / D as `<Event>.<stem>-<n>.json` (12 per version), the S7 plan approve fixed as `allow` + `updatedInput` echoing the tool's input (founder ruling), the permission kind split out, and the `verify_window_` class's kill sized for verify's dialog runs; the direct precedent for a new dialog ledger row, its Run C/D probe and recorded PermissionRequest variants (rows decision/dialog-answer-mapping, decision/hook-payloads-recorded-by-verify).

@history_item n=8 marker=2026-10-05-permission-end-to-end
**2026-10-05-permission-end-to-end** — Path 4's permission kind end to end over `fixtures/fake-scripts/path4-permission.json`: `path4_permission_is_logged_once_woken_and_answered_by_id` asserts "a `question` first raised by PermissionRequest logged once, its hook silent, a late `answer` exit 13 `unknown-dialog`" — the exact behaviour this chunk replaces, so that assertion must be rewritten, not left red; also `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (exit 12, no body) and allow suggestion fields never reaching the CLI.

@history_item n=9 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — seventeen rows: `[NN/17]`, `LEDGER_ROWS: [&str; 17]`, one stamped set and the drift-only sets, and "a stamp of the fourteen older ids reads unverified"; precedent for the older-stamp oracle a new relied row extends.

@history_item n=10 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — eighteen rows (`LedgerRow::ALL`, the new row last), the `cli_verify` no-replay summaries re-counted, and the trap that a live home stamped at the older count reads `row-missing` until its next re-stamp; the same re-stamp consequence this chunk's grown relied set brings.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines become `[NN/MM]` with MM = `LedgerRow::relied()`'s length; `cli_verify`'s per-run summary literals, `contract_ledger_probes`' `ROW_IDS` and `LEDGER_ROWS` all key off the relied count, so a new relied row moves each literal.

@history_item n=12 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E added: relied 22, `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` grown, older-stamp oracle widened, new stop case for the run, and the no-screen `verify_window_` case's time re-measured against its kill; a precedent if the new probe adds a run or settle time.

@history_item n=13 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (agents-join at 23 / 20 rows) — relied 23, the no-replay and modal summaries re-counted, verify's spawn-pair count grown; another instance of a new row moving every count literal.

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** (twenty-four rows) — relied 24, the older-stamp oracle at the twenty-three older ids, no-dialog / tag-turn summaries re-literaled; the last row-count shape before the Linux-only fold.

@history_item n=15 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the configurable dialog hold: expiry writes one `dialog-expired` and one `dialog-answered{deadline_hit:true}`, a late answer is `unknown-dialog`, `pending_dialog` set before the line, hook bound and timeout margin outlasting the hold; the scope says an expired hold keeps today's fall-back, so the PermissionRequest question path must keep these cases (rows decision/dialog-hold-expiry, decision/dialog-timeouts-outlast-the-hold, decision/pending-dialog-before-its-line).

@history_item n=16 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — named the question's `option_details` cases (`dialog.rs`), `hook_a_recorded_question_carries_its_option_details`, and the live reading of a question answered by a product `viola answer` with `dialog-answered{deadline_hit:false}`; the question data shape a PermissionRequest-raised question must carry too (rows contract/dialog-data/null-details, contract/event-data/question, contract/question-event/option-details).

@history_item n=17 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — `contract_ledger_probes`' stamped sets became `2.1.287` and `2.1.302`, dialog variants 12 under each of `2.1.288`, `2.1.287`, `2.1.302`; any new recorded PermissionRequest question variant joins every stamped set's walk and replay.

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` compares MCP `answer` with the CLI's `--json` less `v`; if the answer's shape moves, the MCP parity case moves with it (rows contract/mcp-tools/result-body, contract/mcp-cli-parity/refusal).

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the hook fail-open matrices' impostor rows) — `hook_dialog_tier_fails_open_silently` gained `question_impostor` and `permission_impostor` driving `pre-tool-use` / `permission-request` against an impostor: exit 0, no body, one `hook-decision` with `server-verify-failed`; a new PermissionRequest decision path stays inside this matrix (rows decision/hook-fails-open, contract/hook-decision/server-verify-failed).

@history_item n=20 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the root bin's server-check unit cases) — `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper`: the dialog path writes no frame and emits no decision to an impostor; the guard of a picked row the new body must not bypass (row decision/hook-dialog-no-frame-to-impostor).

@history_item n=21 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the hook's instance check on its event and dialog paths) — the dialog path runs the instance check before any `config.json` read; `hook_refused_by_the_instance_check_fails_open_and_writes_nothing` covers `pre-tool-use` and `permission-request` (rows decision/hook-instance-check-first, contract/strict-modes-refusal/hook).

@history_item n=22 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six CLI frame verbs and the MCP frame tools under the instance check) — `answer` (CLI and MCP) runs `check_instance` before any role file, log or frame, exit 21 / `instance-unreachable`; any change to `answer` keeps that order (rows decision/instance-check-before-role-file, decision/mcp-frame-tools-check-instance-first).

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the tree, diagnostics, stamps and pin checks at the unit tier) — `update_stamps_refuses_a_widened_ledger_and_leaves_it` (checked before the create and under the lock); re-stamping writes through this path (row decision/stamps-refuse-widened-ledger).

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the exit-2 paths that write no document) — oversize `viola answer` stdin exits 2 before connecting with the fixed `UNUSABLE_RESPONSE` line whatever `--json` says; an answers-carrying `answer` stays under that rule (rows contract/viola-answer/oversize-stdin, decision/cli-json-one-document).

@history_item n=25 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the reply screen and the fault screen) — `of_reply` / `reply_of`: a refusal that is no code word reads `unknown`, a non-code-word detail `null`; MCP `answer`'s reply goes through this screen (row contract/mcp-reply/screened-words).

@history_item n=26 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `instance-unreachable` body names its cause) — MCP `{error, detail}` equals the CLI exit-21 document less `v` per cause; the parity row covering `answer` (row contract/mcp-cli-parity/exit-21).

@history_item n=27 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code on `process-exit` (`version-unreadable`, `unclean-fixture`, …), a failing row is no refusal and carries no code; a new probe failing reads as a fail line, not a code (rows contract/verify-exit-detail/unclean-fixture, contract/verify-exit-detail/version-unreadable, decision/verify-failing-row-no-refusal).

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the hook's two new fail-open causes) — a refused socket directory on the hook's dialog path logs one `hook-decision` with `socket-dir-refused`, `decision_emitted:false`, exit 0 empty streams (row contract/hook-decision/socket-dir-refused).

@history_item n=29 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` is one set on every build, `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` ungated `[&str; 24]`, older-stamp oracle restated; the base the new row grows from (no per-OS tallies left to move).

@history_item n=30 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the planted annotation drift witness) — `dialog_drift_names_a_changed_annotation` plants a change to a forwarded `annotations` byte of `synthetic_dialog_set` and reads `dialog_drift` red; the question answers' annotations forwarding this chunk reuses on PermissionRequest stays held by it.

@history_item n=31 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy) — `decision/version-gate-needs-all-relied-rows` gained "a retired row it also holds changing nothing" with the statusline-rate-limits row retired; the verified rule a grown relied set must satisfy, older stamps reading unverified.

@history_item n=32 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** (7. Test Data & Fixtures) — `decision/hook-payloads-recorded-by-verify` gained the StopFailure except (no recorded payload, no ledger row, no verify probe); contrast case: this chunk's PermissionRequest question body is the opposite choice, a relied row with a probe and recorded payload.

@history_item n=33 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled on the live reading) — a contained live reading on claude 2.1.328 settled the no-ledger-row exception on the founder's word, claiming nothing for other versions; precedent for how a live reading on 2.1.328 is recorded and scoped.

@history_item n=34 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (3. Test Harness Contract) — + `decision/stamping-verify-passes-transcript`: every stamping verify passes `--transcript` "for its stamp to hold the 22nd row"; trap — a new probe needing a fake-agent option must be passed by every stamping helper and the harness boot, or the stamp misses the row.

@history_item n=35 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (3. Test Harness Contract) — + `decision/one-verify-stamp-per-build`: stamped homes copy one verify stamp per build keyed by binary hashes; a grown relied set re-takes that stamp, and `decision/verify-driven-tests-20s-kill`'s why now rests on it.

@history_item n=36 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** (7. Test Data & Fixtures) — `contract/fake-agent-cli/script` lists the turn-script step kinds (PermissionRequest among them); the fake agent's PermissionRequest question script the scope names is built on this contract and the fake-script schema.

@history_item n=37 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (6. E2E Test Strategy) — `decision/live-readings-not-a-test-layer` replaced by `decision/standing-live-rig`: live rounds reuse the committed `viola-live-rig`, rehearsals in CI, live starts by hand under the founder's start cap; the scope's re-stamp and any live start are shaped through this rig.

@history_item n=38 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (1. Test Scope Summary: 2 rows) — `decision/real-claude-local-only` reads the real CLI reached only through viola verify and the rig's live starts, never in CI (row decision/real-claude-local-only).

@history_item n=39 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** (11. Test Anti-Patterns) — `decision/no-blanket-approval-tests` gained its one except (the rig passes a read trust dialog under its key grant); an allow body for a PermissionRequest question must stay a driver's answer, never an auto-approval (rows decision/no-blanket-approval, decision/no-blanket-approval-tests).

@history_item n=40 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — + `decision/no-untied-viola-spawn`: a census over every root test source refuses an untied viola spawn; any new answer / Path 4 test must spawn through the tied builder.

@history_item n=41 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — the last added ledger row before this one: verify's stop removes its own trust record (`contract/verify-stop/signal`), + `decision/no-verify-start-outside-test-home`, + `decision/verify-under-test-owned-user-home`, + `decision/live-failing-reading-not-verification` (a failing or confounded live reading is recorded as read, never verification); the rules a new probe and its live re-stamp follow.

@history_item n=42 marker=2026-11-18-trust-record-row-read-live
**2026-11-18-trust-record-row-read-live** — `decision/verify-window-60s-kill` → `decision/verify-window-80s-kill` (both profiles), and the rig-only first-screen except of `decision/record-only-at-zero-failing-rows`; a new probe adding verify time is measured against the 80 s kill, recording stays at zero failing rows.

@history_item n=43 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — new ledger row with an `absent` verdict: `contract/verify-output/step-line`, `contract/verify-output/summary-line`, `contract/local-live-suite/pass`, `decision/version-gate-needs-all-relied-rows` and `decision/ledger-row-verify-probe` all re-worded; the most recent shape of adding a relied row and its verified rule (absent only on browser-tools-prompt).

@history_item n=44 marker=2026-11-18-the-2-1-325-recorded-fixture-set
**2026-11-18-the-2-1-325-recorded-fixture-set** — `decision/record-home-names-no-home` extended to the live rig's record arm (refuses `record-home`, exit 3, before any credentials read); a re-recording of fixtures for the new PermissionRequest question variant goes through it.

@history_item n=45 marker=2026-11-19-a-fired-scheduled-wake-keeps-the-wheel
**2026-11-19-a-fired-scheduled-wake-keeps-the-wheel** (6. E2E Test Strategy) — + `decision/fired-wake-keeps-wheel`, guarded by `path5_a_fired_wake_keeps_the_driver_wheel_its_next_send_and_its_dialog`: the driver keeps the wheel through a fired wake and answers its dialog; the dialog-on-driver-wheel precondition the new body relies on.

@history_item n=46 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** (3. Test Harness Contract) — `decision/root-wait-bound` and `decision/within-7s-wait-bound` moved to 12 s, the mutants default kill to 15 s; new Path 4 / answer waits stay under these bounds.

@history_item n=47 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (2. Test Strategy) — + `decision/os-gated-cases-in-binary-root`: the `cli_verify` split kept its OS-gated cases in the root file; new `cli_verify` cases for the row follow the split layout.
