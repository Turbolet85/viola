# Task — test-plan · 2026-10-26-explicit-session-links

## The session
Built the explicit `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` verbs and served their channel methods `link` / `unlink` on the driven instance's wrapper; before, both methods answered `-32601` and the verbs were held for the founder's word on the boundary widening.
Each verb runs `viola_state::strict::check_instance` on the driven instance before any role file, log or snapshot read (refused: exit 21, the fixed instance-state pair, `--json` `instance-unreachable`), then the liveness pre-check, then the server identity check through `cmd::client::connect` (Linux and Windows; macOS the recorded gap) under the spans `cli.link` / `cli.unlink`.
The wrapper validates `driver` / `driven` through `ViolaName::try_new`, refuses a foreign `driven` and a self-pair with `-32602` and writes no line; `link` answers `{since, already_linked}` (a held pair appends nothing), `unlink` answers `{}` and is the first product path to append an `unlink` event line (`LinkSet::remove` gains its first caller).
The CLI frame verbs go from six to eight; `tests/security_negatives_server_identity.rs` goes from 19 to 25 cases; served channel methods go from seven plus `hook.event` to nine; `viola --help` gains the row `handoff:  link, unlink` between `wheel:` and `setup:`; three plain result lines print the outcome.
Retired: the hold on the explicit verbs, the `-32601` answer for `link` / `unlink`, `LinkSet::remove`'s dead-code allow, and `link` / `unlink` as the unserved-method example of `channel_wrapper_logs_each_call_with_corr_and_conn` (now `anything`, asserting the response line carries no `method` key).
Measured: CI green on three OSes (ubuntu 77, windows 76, macOS 69 passes); `LinkSet::line` stamps `since` with `Utc::now()` outside the clock seam, so root cases assert `since` by shape only.

## Reached files
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/run/links.rs
- src/cmd/run.rs
- src/cmd/mod.rs
- src/human/verbs.rs
- src/human.rs
- tests/channel_links.rs
- tests/channel_endpoint.rs
- tests/cli_output_plain.rs
- tests/security_negatives_server_identity.rs
- tests/cli_links.rs
- .config/nextest.toml
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/working-route.md

## Touches
- §6 Scenario E3 `link` / `unlink`: its "As landed: owed, built in part" note (the verbs not built, the wrapper answering `-32601`, held to the route entry "Explicit session links") is stale; E3 is now built in `tests/cli_links.rs`.
- §6 Scenario E3 steps: step 1 boots `overseer` and `builder`, but the landed test boots only the driven `builder`; the driver is a self-reported name, never booted.
- §6 Scenario E3 detail entity: its claim that no `viola link` verb exists is stale, and so is any line naming E3's kill-line or owner as pending.
- §6 Security control negatives: the raw-channel `link` / `unlink` name-validation negative is now built (`tests/channel_links.rs`), not owed.
- §6 Security control negatives and §1 Vector 1 trigger: the server-identity table now covers eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) in both modes plus the controls, 25 cases; any line naming the six verbs or 19 cases is stale.
- §1 Test scope summary, the identity-table line in its scope entities (the six frame verbs named): moves to eight.
- §6 Exit-cause matrix: the server-verification cause in `tests/security_negatives_server_identity.rs` now has `link` / `unlink` rows; the verbs' strict-modes and stopped-instance exit 21 causes are cases of `tests/cli_links.rs`.
- §1 Critical paths list "Covered by E2E suite expansion", the `link` / `unlink` item: built now.
- §1 Entity, CLI verbs and the `--json` / exit-code contract: gains the `link` / `unlink` verbs (exits 0, 1, 20, 21).
- §1 Vector 6 / Vector 7 triggers (group-writable instance dir refused): gains `link_verbs_refuse_a_widened_instance_before_any_log`, refusing before any log with the mode read back as found.
- §4 root bin unit coverage, "the server check refused inside each frame verb's span": now includes `cli.link` / `cli.unlink` (`request_refuses_an_unverified_server_inside_its_dispatch_span` in both modules).
- §2 unit pyramid and its macOS compile-out of the server check's captured-span cases: `link.rs` gates only that case off macOS, while `unlink.rs`'s module (one gated case) is gated whole.
- §5 Wrapper channel, "every method through the crate and root channel suites": the served set grows to nine methods, with `link` / `unlink` covered by `tests/channel_links.rs` and `tests/cli_links.rs`.
- §5 Wrapper channel / channel suites: `channel_wrapper_logs_each_call_with_corr_and_conn` in `tests/channel_endpoint.rs` uses `anything` as its unserved method and asserts that the conn-less response line has no `method` key.
- §5 Human-mode layouts, the `viola --help` `HELP` literal in `tests/cli_output_plain.rs`: it now holds the `handoff:  link, unlink` row (`help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`).
- §5 On-disk strict-modes refusal exits (CLI 21 for every other verb): `link` / `unlink` take exit 21, not `revive`'s exit 1. Re-read for agreement.
- §6 Scenario E5 step 1 ("run a wrapper through `link`"): an explicit `link` can now be driven by the verb. The As-landed note's CARRY is unchanged.
- §7 Time-sensitive data ("injected per test behind `viola_core::Clock`") and §8 Mocking Time ("`chrono::Utc::now()` is read only through the clock seam, except at the budget gate's three evaluation points"): `LinkSet::line` reads `Utc::now()` directly for `since`, which is a new exception these lines do not name.
- §9 / §5 nextest `ci` profile: the 20 s stamped-home kill-line filter in `.config/nextest.toml` gains `binary(channel_links)`. Update any line that enumerates the kill-line binaries.
- §6 E3 verification signal "after unlink: an `unlink` record exists": now held by a product path (`source: wrapper`, `{driver, driven}`). Before this session, no product path appended `unlink`.
- §1 Surface ipc-internal (wrapper channel): `link` / `unlink` refusal payloads are `-32602` `data: null`, and a `record` / `remove` error is `-32603`, under the same schema assertions.
- §6 Error sanitization: the verbs' `--json` refusal `{"v":1,"error":"instance-unreachable","detail":null}` and the result lines print only the verb's own parsed names, never a name read back from the reply or the log.

## New facts
- F1 · e2e-test-strategy · E3 landed as `e3_two_links_and_a_send_leave_one_link_record` in `tests/cli_links.rs`: two `link` calls leave one link record with the same `since`, and the second answers `already_linked: true`; a `send` from the self-reported driver records no further link; `unlink` writes an `unlink` record, and a later re-link takes a later `since`.
- F2 · e2e-test-strategy · `links_an_unlink_survives_a_wrapper_restart` (`tests/cli_links.rs`): an `unlink` stays in force after the wrapper restarts and restores its links from the log.
- F3 · e2e-test-strategy · `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`, 14 cases) sends raw frames through `viola_channel::Client::request`, past every CLI check. A missing, non-string or invalid name, a `driven` that is not the serving wrapper, or a `driver` equal to `driven` each gets `-32602` with `data: null` and no link line or snapshot change, beside a delivering control.
- F4 · e2e-test-strategy · `link_verbs_refuse_a_widened_instance_before_any_log` (Unix): a group-writable driven instance is refused exit 21 before any role file or log opens, and the instance dir's mode is read back as found, because a check placed after `client::start` would pass a 0770 home. A remove-the-guard control that moves the check went red 1 of 1.
- F5 · integration-test-strategy · `tests/cli_links.rs` covers the verbs' outputs and refusals. `link_verbs_print_the_design_lines` pins the three lines byte for byte and the `--json` `{"v":1,"ok":…}`. `link_verbs_against_a_stopped_instance_exit_21` checks the not-running hint comes last. `link_verbs_log_the_channel_pair_and_no_names` checks the wrapper's `channel-request` / `channel-response` pair carries `method` `link` / `unlink` and no driver or driven name.
- F6 · unit-test-strategy · New root-bin unit cases for links: six `links_set_method_*` functions in `src/run/links.rs`, covering link twice → one line and the same `since`, unlink then link → a later `since`, unlink of an absent pair → nothing written, bad pair → invalid params, a `from` that changes nothing, and a failed append → internal error with the set unchanged. Also `methods_route_link_and_unlink_to_the_link_set` in `src/cmd/run.rs`.
- F7 · unit-test-strategy · New root-bin unit cases for the verb side: `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` in `src/cmd/link.rs`, plus `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` in `src/human/verbs.rs`. A `since` that is not RFC 3339, such as the restore's `unknown`, prints through `escape_message`.
- F8 · mocking-stubbing-discipline · `LinkSet::line` stamps the link line's `since` with `Utc::now()` directly (`src/run/links.rs:92`), outside the clock seam. Root cases assert `since` by shape only, through `is_clock_time` in `tests/cli_links.rs`, which has its own `is_clock_time_reads_the_shape_only`.
- F9 · test-strategy · macOS gating rule: a `cfg(not(target_os = "macos"))` whole-module gate fits only a module whose every case is gated. Otherwise gate the cases, as `src/cmd/link.rs` does while `src/cmd/unlink.rs` is gated whole.
- F10 · integration-test-strategy · An unserved method's conn-less response line carries no `method` key, because the channel's `method_label` names only the closed method list. A test that asserts a `method` value on such a line for a name outside that list is wrong by construction.
- F11 · test-scope-summary · Only `link` / `unlink` among the eight CLI frame verbs run `check_instance` before `client::start`, and their strict-modes refusal is exit 21 (as `list` / `plugin install`), not `revive`'s exit 1. A self-pair is refused only by the wrapper (`-32602`, CLI exit 20), never client-side.

## Untouched
- §1 MCP surface, "`tools/list` is exactly the five tools, never `release` / `pause` / `link` / `unlink`": still true, because no MCP tool or handler changed.
- §7 Fake agent contract (`viola-fake-agent` modes, receipts, scripts, the SIGHUP exit disposition): no fake-agent change.
- §7 Fixture hygiene and recorded fixtures (`fixtures/claude/<cli-version>/`, `Mcp.` exchanges, the scrub codes): no fixture was recorded or changed.
- §3 Harness contract (the five commands, session record, harness exit codes, `gate` suites): no agent-run verb, flag or status shape changed.
- §10 Coverage thresholds, the mutation gate and the perf rows: no threshold, mutation rule or hyperfine row changed (`run --perf` still times the same five hooks).
- §9 CI workflows, matrix, deny / zizmor / release-check / ConPTY vendor gates and the uploads: no workflow or script changed.
- §6 Paths 1–7 and E1 / E2 scenarios: their surfaces and signals are not reached by the link verbs.
- §6 Property, chaos and contract suites and the `list` row-integrity suite: no parser, fault seam or board change.
- §4 viola-state / viola-channel / viola-core / viola-agent-claude / viola-mcp unit coverage: no crate was changed. `check_instance` and `ViolaName` are only called, not modified.
- §6 Exit-cause matrix `cross_exit_causes.rs`: still not built. Only its pointer rows into existing test files gain the link verbs.
- §6 Scenario E5 As-landed CARRY (snapshot readers on `read_snapshot`, owed to "Home and code-bearing file integrity"): unchanged.
