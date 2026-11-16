@history_item n=1 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (Path 2 as landed) a local command was exit 13 `not-delivered`/`no-prompt-submitted` while no local-command row was compiled, "never presumed delivered"; `unconfirmable` owed onward. The origin of today's off-list reading this chunk replaces: an unlisted command that runs still ends `no-prompt-submitted`.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (ten ledger rows, verify's interactive runs, the screen fixture class) set the ledger-row/verify-probe/screen-fixture machinery (`--screens`, `--turn-stop`, `Screen.*.json` scrub-as-detector, `contract_ledger_probes` literal lists) and moved local-command `unconfirmable` to the local-command entries. Any echo row this chunk adds rides this machinery and its literal-list rules.

@history_item n=3 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — (seventeen rows, the framing tier) the `/clear` post-condition row and its recorded `SessionEnd.clear-<n>` / `SessionStart.clear-<n>` variants, replayed byte for byte by the fake agent under `--framing`; `[NN/MM]` counts, `LEDGER_ROWS` and the drift contract all move together when a row is added. Precedent for how a measured local-command echo becomes a row, a recorded variant and a fake-agent replay.

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — (Path 2's local verdicts as landed) the current local-command outcomes this chunk re-measures: `unconfirmable` exit 0 `{confirmed:false, detail:"unconfirmable", cursor}` with `send-issued` then `send-confirmed{confirmed:false}`, no window, the command's text in no role log; `/clear` confirmed by a new-session `session-start`; off-list slash text exit 13 (`send_window_slash_text_off_the_list_is_not_delivered`); the human `[  ] unconfirmable` line; the `send_local_command_` units and `--tag-turn-screen`. These guards and rows (`contract/viola-send/off-list-slash-text`, `contract/viola-send/unconfirmable-local-command`, `contract/viola-send-human-mode/unconfirmable`) are exactly what a new echo confirmation changes.

@history_item n=5 marker=2026-10-07-a-send-ending-in-a-newline-is-confirmed
**2026-10-07-a-send-ending-in-a-newline-is-confirmed** — the list is consumed by exact equality of the typed text (trailing newline dropped); three local-command `trailing_newline` cases. Basis of `decision/local-command-exact-match`: the echo's match must classify the typed form, never a leading-slash test or a trim beyond it. Trap noted there: the wrapper puts its plugin flag first in a child's argv.

@history_item n=6 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — `LEDGER_ROWS` had gone stale, missing `local-command-clear` among others, until this wrap named all ids. Trap: adding an echo row must update every literal row list (`LEDGER_ROWS`, `STEPS_PASS`, `ROW_IDS`) in the same chunk.

@history_item n=7 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (`--local-live`; Path 2 live readings; the real CLI has two local routes) `/clear`'s live proof was taken by a live run; a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, never in CI (`decision/live-readings-not-a-test-layer`). Bears on the scope's boundary of no live start: any echo shape needing live proof is a local measurement, not a suite.

@history_item n=8 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — (the typed text, the `empty-text` refusal) the typed text drops every trailing CR and LF; the send refusal order gained `empty-text`; the `send_local_command_` count was deliberately left alone. Also (the `mutants` override order) the `verify_window_` class and its kills: a verify-driven case added for an echo probe lands in that class.

@history_item n=9 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — the typed text holds no CR, every inner CR/CRLF one LF (`typed_text_every_inner_cr_is_one_lf`, `typed_text_drops_every_trailing_newline`). The typed form is what the local-command list matches, so an echo classification must take this normalised text (`decision/sent-text-typed-form`).

@history_item n=10 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's 8.5 s `GATE_MAX_WAIT`, the second wheel/turn read after ready, the `send_under_the_paste_hint_` cases and the kills sized for by-design floors. A local-command send waits the same gate; a kill line moves only for a floor by design, with a planted-hang control both ways.

@history_item n=11 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (moved unit tests re-cited) the `send_local_command_` functions live in `src/run/send/classify.rs`, the `trailing_newline` tests split across `classify.rs`, `relabel.rs` and `send.rs`. Where this chunk's classify cases go.

@history_item n=12 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — verify's framing waits became the named predicates `added_turn_ended` and `local_command_answered` in `src/cmd/verify/typed/`, each killed by a unit case. A verify probe for a local-command echo extends this code and its unit-tested predicates.

@history_item n=13 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — a precedent for adding one ledger row: `LedgerRow::ALL` grows, Run B gains a paste, a new recorded variant joins the hygiene walk and `--framing` replay, the `cli_verify` per-run summaries shift, and the live home re-stamps (`row-missing` until then). The checklist an echo row repeats.

@history_item n=14 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — `/compact` was `unconfirmable` at once on a verified CLI (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`); the master stopped stating a `send_local_command_` case count, the `#[case]` rows being the count. Write no case count into the plan.

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (agents-join at 23 / 20 rows) another row-add precedent: relied set grows, `[NN/MM]`, the no-replay and tag-turn summaries and the spawn-pair count all re-derived, the arm's `check_*` cases beside it.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (twenty-four rows; /compact's same-session confirmation and its own window) the direct precedent: a local command's post-condition measured, made the `local-command-compact` row with a Run B paste and a recorded `compact-<n>` variant replayed by the fake agent, then consumed by `send` (`COMPACT_CONFIRM_WINDOW` 40 s, `send_compact_on_a_verified_cli_is_confirmed_by_its_same_session`, the unverified case still `unconfirmable`). Rows `decision/compact-confirmed-by-same-session`, `decision/compact-confirm-window`, `contract/fake-agent-input/compact`.

@history_item n=17 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — rows ten to fourteen with the `[profile.ci]` `verify_window_` and verify-driven kill overrides; a verify a test drives has no test-side bound (`decision/verify-driven-tests-20s-kill`). An echo probe's verify-driven cases sit under that rule.

@history_item n=18 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares the fake agent's hook stdin byte for byte with each recorded set, never an insta snapshot, an empty walk failing. An echo the fake agent emits must replay recorded bytes under this contract (`decision/fake-agent-matches-recordings`, `decision/fixture-drift-byte-compare`).

@history_item n=19 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — the fake agent's argv option count and the `--tag-turn-screen`/`--long-turn-screen` phase options; `2.1.302` joined `2.1.287` as a stamped set in `contract_ledger_probes`. A new row must pass on every stamped set or a set moves to drift-only, and a new fake-agent option is argv-only.

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the MCP tests as landed) MCP `send` is compared with the CLI's `--json` less `v` (`mcp_tools_path2_path3_send_wait_last_match_the_cli`). Rows `contract/mcp-send/success`, `contract/mcp-cli-parity/refusal`: a changed local-command outcome must hold on both surfaces.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the exit-2 paths that write no document; the reply screen) `viola send`'s exit-2 `UNUSABLE_TEXT` line writes no document whatever `--json` says (`decision/cli-json-one-document` except); a `detail` that is no code word reads null (`contract/reply-screen/detail`, `contract/mcp-reply-screen/detail`). A new detail word for an echo outcome must be a code word or it is screened out.

@history_item n=22 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (the leading-slash rule) a leading-slash argument is never taken as a prompt, `/clear` among `tests/cli_slash_args.rs`' cases (`decision/leading-slash-never-a-prompt`, `contract/cli-argv/leading-slash`); (the statusline-rate-limits arm's live red) a row that reads account state rather than CLI behaviour was disproved live. An echo row must read CLI behaviour.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set, all twenty-four rows on every build) relied is all rows on every build, `STEPS_PASS`/`ROW_IDS`/`LEDGER_ROWS` `[&str; 24]`; the older-stamp oracle reads the sets with and without `local-command-compact` as unverified. A new row extends these literals and the oracle (`decision/version-gate-needs-all-relied-rows`).

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (4. Unit Test Strategy) `~ decision/send-refusal-order` drops `budget-paused`; `~ decision/version-gate-needs-all-relied-rows` gains "a retired row it also holds changing nothing" when `statusline-rate-limits` retired. The current refusal order and the retired-row rule a new row must keep.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (7. Test Data & Fixtures) `+ contract/fake-agent-cli/settings`: the fake agent keeps `--settings` for verify's runs; the StatusLine fixtures and Run B's probe `--settings` left. Fake-agent options and the verify runs this chunk may extend for an echo probe.

@history_item n=26 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ contract/readiness-gate/gate-max-wait` (the box read up on its rule and prompt rows) and `+ decision/screen-fixture-box-rows-as-literals`; if the echo is a screen line, screen fixtures keep only compiled literals (`decision/screen-signatures-only`, `decision/screen-fixture-scrub-as-detector`) and assertions never parse the rendered screen.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the feed-panic passthrough witnessed on the outer terminal) `chaos_feed_panic_refuses_send_and_keeps_passthrough`, the one byte-search exception to never parsing the child screen (`decision/never-parse-rendered-child-screen`, `decision/vt100-feed-panic-safe`). An echo read from the screen cannot be asserted by parsing it.

@history_item n=28 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `~ contract/viola-send/turn-running` (after any turn end a send is accepted) and `+ decision/send-cases-copy-recorded-fixtures`: a synthetic-only fixtures root draws no input box, so a send after the first turn reads `input-not-ready`. Also `decision/hook-payloads-recorded-by-verify`'s StopFailure exception: no ledger row, a synthetic payload, later ratified. Precedent if the echo is a hook event with no recorded payload.

@history_item n=29 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — `~ decision/root-wait-bound` and `~ decision/within-7s-wait-bound` (`WITHIN` 12 s, mutants default kill 15 s) and `~ decision/send-window-paste-hint-30s-kill`; `+ decision/record-home-names-no-home`. The bounds each new wait in this chunk's tests must sit under.

@history_item n=30 marker=2026-11-16-wait-wake-condition
**2026-11-16-wait-wake-condition** — `+ decision/one-verify-stamp-per-build`, `~ decision/fresh-home-per-test` (a stamped home is a copy of the build's stamp) and `~ decision/verify-driven-tests-20s-kill`. A new ledger row changes the shared stamp every stamped-home test copies; `contract/fake-agent-cli/script` gained `UserPromptSubmit`/`Notification` steps.

@history_item n=31 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `− decision/send-confirmation-exact-match → + decision/send-confirmation-trailing-trim-match`, `~ contract/viola-send/records`, `~ contract/fake-agent-input/bracketed-paste` (`--trim-prompt-end`). The scope keeps this ordinary-send match untouched; a local-command echo path must not widen or move it, and the trim set stays an assumption, not a live reading.

@history_item n=32 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the trailing-only tolerance (`decision/send-confirmation-trailing-trim-match`) and StopFailure's no-ledger-row registration ratified by the founder; a leading trim waits for the live reading on «Standing live rig with a screen model». Both stand as fixed boundaries for this chunk.

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (remove-the-guard witnesses bounded by `WITHIN`) a control whose neutralised guard lets a product process start needs a time-bounded witness; an unbounded one hung 27 min (`decision/bounded-remove-the-guard-witness`). Binds the scope's per-control `--timeout` rule.

@history_item n=34 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — (scoped log captures through with_scoped_default) every root-bin scoped capture goes through the pinned-dispatch helper (`decision/scoped-capture-pinned-dispatch`). Applies to any unit case asserting the local-command path's log lines (e.g. no command text in a role log).

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a test's endpoint directory is made private) a test endpoint in a temp dir goes through `create_private_dir`; a bare `tempdir` reads 0755 and the socket-dir check refuses it (`decision/private-dir-for-test-endpoints`). Trap for any wrapper-side unit case this chunk adds.

@history_item n=36 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — mutation runs only at the epoch boundary and on demand; no per-chunk gate (`decision/mutation-at-epoch-boundary`, `decision/mutation-gate-cadence`). Matches the scope's no-mutation-run boundary: controls are shown by hand, filtered and time-bounded.
