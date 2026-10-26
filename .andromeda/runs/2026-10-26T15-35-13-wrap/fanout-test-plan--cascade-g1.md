proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: '## 1. Test Scope Summary'
    change: 'In the **As landed (server verification)** line, "the six CLI frame verbs by `tests/security_negatives_server_identity.rs`" reads "the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `link` / `unlink` since chunk 2026-10-26-explicit-session-links) by `tests/security_negatives_server_identity.rs`"; the rest of the line unchanged.'
    sidecar: 'test-scope-summary: the server-verification As-landed line names eight CLI frame verbs, not six (link / unlink added, chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Counts / qualifiers moved: CLI frame verbs six to eight, pattern frameverbs stale at test-scope-summary.md:12; idtable (security_negatives_server_identity.rs, six verbs to eight) stale at the same line.'
    basis: '.andromeda/test-plan-detail/test-scope-summary.md:12'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 35
    section: '## 1. Test Scope Summary'
    change: 'Add a sub-bullet under the CLI verbs entity: "**As landed (`link` / `unlink`, chunk 2026-10-26-explicit-session-links):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance before `client::start`, and their strict-modes refusal is exit 21 (as `list` / `plugin install`), not the exit 1 of `revive`; a self-pair (`driver` equal to `driven`) is refused only by the wrapper (`-32602`, CLI exit 20), never client-side."'
    sidecar: 'test-scope-summary: the CLI verbs entity records the link verbs'' instance check first, their exit 21 strict-modes refusal and the wrapper-only self-pair refusal.'
    rationale: 'Report Symbols / APIs (link_call steps 1 and 6) and Decisions & corrections (exit 21 per the founder''s option text, card lean 5; self-pair refused by the wrapper only, card lean 2).'
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 62
    section: '## 1. Test Scope Summary'
    change: 'Add a sub-bullet under the ipc-internal surface: "**As landed (`link` / `unlink`, chunk 2026-10-26-explicit-session-links):** both methods are served by the driven wrapper (before: `-32601`); a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a self-pair is `-32602` with `data: null` and no line written, and a `record` / `remove` error is `-32603`; `link` answers `ok` `{since, already_linked}`, `unlink` an empty `ok` object, under the same schema assertions."'
    sidecar: 'test-scope-summary: the ipc-internal surface records the served link / unlink methods and their -32602 data-null and -32603 faults.'
    rationale: 'Report Symbols / APIs: channel methods link and unlink are served, driver_of refuses with ProtocolError::InvalidParams (-32602, data null, no line), record / remove error is ProtocolError::Internal (-32603); the surface signal names -32602 only with data {supported, wrapper}.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: '## 2. Test Strategy'
    change: 'In the Tier-specific depth line, "the `last.rs` and `pause.rs` test modules, gated whole" reads "the `last.rs`, `pause.rs` and `unlink.rs` test modules, gated whole (a `cfg(not(target_os = "macos"))` whole-module gate fits only a module whose every case is gated; otherwise the cases are gated, as `src/cmd/link.rs` gates only its server-check case and keeps its reply-shape table and line case running on macOS; chunk 2026-10-26-explicit-session-links)"; the rest unchanged.'
    sidecar: 'test-strategy: the macOS compile-out names unlink.rs gated whole and the rule that a whole-module gate fits only a module whose every case is gated.'
    rationale: 'Report Deviations from intent (implement 1) and Decisions & corrections (testing-rule refinement met: whole-module gate only where every case is gated).'
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'In the instance-check bullet, after the three `cfg(unix)` root cases of `tests/cli_revive.rs`, add: "and, since chunk 2026-10-26-explicit-session-links, `link_verbs_refuse_a_widened_instance_before_any_log` (`tests/cli_links.rs`, `cfg(unix)`): `viola link` / `viola unlink` run the check on the driven instance before `client::start` and refuse exit 21 before any role file or log opens"; the rest unchanged.'
    sidecar: 'unit-test-strategy: the instance check''s root refusal cases gain the link verbs'' widened-instance case.'
    rationale: 'Report Counts: check_instance( product call sites 2 to 3 (+ src/cmd/link.rs:98), pattern checkinst, a detector extends a list of refusing verbs where it finds one; Outcome (security) group-writable driven instance refused before any log.'
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 68
    section: '## 4. Unit Test Strategy'
    change: 'In the server-check bullet, "`src/cmd/{send,wait,last,answer,pause,release}.rs`" reads "`src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` (the spans `cli.link` / `cli.unlink` since chunk 2026-10-26-explicit-session-links)"; the rest unchanged.'
    sidecar: 'unit-test-strategy: the per-verb request_refuses_an_unverified_server_inside_its_dispatch_span list gains link.rs and unlink.rs.'
    rationale: 'Report Counts: CLI frame verbs six to eight; Coverage of new surfaces: two request_refuses_an_unverified_server_inside_its_dispatch_span cases under cli.link / cli.unlink; New text: src/cmd/link.rs 148-154, src/cmd/unlink.rs 29-36.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: '## 4. Unit Test Strategy'
    change: 'Add a bullet: "The served link methods (chunk 2026-10-26-explicit-session-links): six `links_set_method_*` functions in `src/run/links.rs` (link twice writes one line and answers the same `since`; unlink then link takes a later `since`; unlink of an absent pair writes nothing; a bad pair is invalid params; a `from` changes nothing; an append that fails is internal with the set unchanged) and `methods_route_link_and_unlink_to_the_link_set` in `src/cmd/run.rs`."'
    sidecar: 'unit-test-strategy: root-bin unit cases for the served link / unlink methods.'
    rationale: 'Report Coverage of new surfaces (channel methods link / unlink: links_set_method_* 6 functions, methods_route_link_and_unlink_to_the_link_set) and New text, by line (src/run/links.rs 388-526, src/cmd/run.rs 851-870).'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: '## 4. Unit Test Strategy'
    change: 'Add a bullet: "The link verbs (chunk 2026-10-26-explicit-session-links): `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` in `src/cmd/link.rs`, `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` in `src/human/verbs.rs`; a `since` that is no RFC 3339 instant (the restore''s `unknown`) prints through `escape_message`."'
    sidecar: 'unit-test-strategy: root-bin unit cases for the link verbs'' reply reading and result lines.'
    rationale: 'Report Coverage of new surfaces (viola link / unlink and the three result lines: unit cases named) and Symbols / APIs (link_since, escape_message for a since that is no RFC 3339 instant).'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 42
    section: '## 5. Integration Test Strategy'
    change: 'Append to the methods bullet: "; `link` / `unlink` served since chunk 2026-10-26-explicit-session-links (before: `-32601`), covered by `tests/channel_links.rs` and `tests/cli_links.rs`"'
    sidecar: 'integration-test-strategy: the wrapper-channel method list names the suites that cover the now-served link / unlink.'
    rationale: 'Report Symbols / APIs (channel methods link and unlink are served) and Files (new tests/channel_links.rs, modified tests/cli_links.rs); Retired: the -32601 answer for link / unlink.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 52
    section: '## 5. Integration Test Strategy'
    change: 'Add a bullet: "an unserved method''s conn-less response line carries no `method` key, because the channel''s `method_label` names only the closed method list: `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) takes `anything` as its unserved method and asserts the key absent (since chunk 2026-10-26-explicit-session-links, when `link` / `unlink` stopped being its example); a test asserting a `method` value on such a line for a name outside that list is wrong by construction."'
    sidecar: 'integration-test-strategy: an unserved method''s response line logs no method key; the endpoint suite''s unserved example is anything.'
    rationale: 'Report Deviations (implement 2), Spec claims disproved by measurement (method_label), Decisions & corrections (hazard) and Retired (link / unlink as the unserved example, now anything).'
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 72
    section: '## 5. Integration Test Strategy'
    change: 'In the `tests/cli_output_plain.rs` bullet, "no `hook`, `help`, `link`, `unlink` or bare `plugin` in the table" reads "no `hook`, `help` or bare `plugin` in the table; since chunk 2026-10-26-explicit-session-links the `HELP` literal holds the row `handoff:  link, unlink` between `wheel:` and `setup:`"; the rest unchanged.'
    sidecar: 'integration-test-strategy: the pinned help table holds the handoff: link, unlink row; link / unlink no longer stated absent.'
    rationale: 'Report Retired (the hold on the explicit verbs and every master line stating it); Symbols / APIs (viola --help gains the row handoff:  link, unlink between wheel: and setup:); Outcome (layouts) met by help_is_the_grouped_verb_table and help_lists_only_served_verbs.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 73
    section: '## 5. Integration Test Strategy'
    change: 'Add a bullet after the `tests/cli_plugin_install.rs` bullet: "`tests/cli_links.rs` covers the link verbs'' outputs and refusals (chunk 2026-10-26-explicit-session-links): `link_verbs_print_the_design_lines` pins the three result lines byte for byte and the `--json` `{"v":1,"ok":…}`; `link_verbs_against_a_stopped_instance_exit_21` reads exit 21 with the not-running hint last; `link_verbs_log_the_channel_pair_and_no_names` reads the wrapper''s `channel-request` / `channel-response` pair carrying `method` `link` / `unlink` and no driver or driven name."'
    sidecar: 'integration-test-strategy: the CLI integration list names the link verbs'' output, stopped-instance and log cases in tests/cli_links.rs.'
    rationale: 'Report Outcome (design: link_verbs_print_the_design_lines, link_verbs_against_a_stopped_instance_exit_21; obs: link_verbs_log_the_channel_pair_and_no_names) and Coverage of new surfaces.'
    fact: F5
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 10
    section: '## 6. E2E Test Strategy'
    change: 'In the cli row''s Driver cell, "trycmd 1.2.1 for human layouts other than the `viola list` board and `viola --help`, pinned by literal in `tests/cli_list.rs` and `tests/cli_output_plain.rs`" reads "trycmd 1.2.1 for human layouts other than the `viola list` board, `viola --help` and the `link` / `unlink` result lines, pinned by literal in `tests/cli_list.rs`, `tests/cli_output_plain.rs` and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`, chunk 2026-10-26-explicit-session-links)"; the rest of the row unchanged, no pipe added.'
    sidecar: 'e2e-test-strategy: the cli driver row names the link verbs'' result lines among the literal-pinned layouts.'
    rationale: 'Report Symbols / APIs (three result lines, plain in every mode) and Outcome (design: the three lines byte for byte, met by link_verbs_print_the_design_lines), no trycmd case added.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 226
    section: '## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`'
    change: 'The line reads: "- **As landed (chunks 2026-10-10-session-links and 2026-10-26-explicit-session-links): built.** `viola link` / `viola unlink` and the wrapper''s `link` / `unlink` methods are served (chunk 2026-10-26-explicit-session-links, in the instance-check form the founder named, its `inputs#I3`); the driver is a self-reported name and is not booted, so step 1 boots only the driven `builder`. What step 2 leaves stays held by root cases of `tests/cli_links.rs` (`binary(cli_links)`, each from a stamped home): `links_a_send_from_another_instance_links_once` (one `link{driver:"overseer",driven:"builder"}` record with `source: wrapper` directly before that send''s `send-issued`, and a second issued send adds none), `links_an_answer_from_another_instance_links`, `links_a_refused_send_and_an_own_name_send_record_none` and `links_survive_a_wrapper_restart`, green on the three CI runners (run 38098238731). A product path now writes the `unlink` record (`source: wrapper`, `{driver, driven}`): the `unlink` method, `LinkSet::remove`''s first caller."'
    sidecar: 'e2e-test-strategy: E3 As landed retires the hold on the explicit verbs; the verbs and methods are served, the driver is not booted, a product path writes unlink.'
    rationale: 'Report Retired (the hold and the -32601 answer), Counts (owner pattern title stale at e2e-test-strategy.md:226; LinkSet::remove callers 0 to 1), Outcome (arch: served semantics met), Decisions & corrections (inputs#I3).'
    basis: '.andromeda/test-plan-detail/e2e-test-strategy.md:226'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 226
    section: '## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`'
    change: 'Add a sub-bullet under the As landed entry: "**E3 itself:** `e3_two_links_and_a_send_leave_one_link_record` (`tests/cli_links.rs`): two `link` calls leave one link record with the same `since`, the second answering `already_linked: true`; a `send` from the self-reported driver records no further link; `unlink` writes an `unlink` record, and a later re-link takes a later `since`."'
    sidecar: 'e2e-test-strategy: E3 landed as e3_two_links_and_a_send_leave_one_link_record.'
    rationale: 'Report Outcome (arch: served link / unlink semantics met by e3_two_links_and_a_send_leave_one_link_record) and Symbols / APIs (link answers since and already_linked; a held pair appends nothing).'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 226
    section: '## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`'
    change: 'Add a sub-bullet under the As landed entry: "`links_an_unlink_survives_a_wrapper_restart` (`tests/cli_links.rs`): an `unlink` stays in force after the wrapper restarts and restores its links from the log."'
    sidecar: 'e2e-test-strategy: E3 names links_an_unlink_survives_a_wrapper_restart.'
    rationale: 'Report Outcome (arch: restart restores, met by links_an_unlink_survives_a_wrapper_restart).'
    fact: F2
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan.md
    near: 570
    section: '## 6. E2E Test Strategy › #### Scenario: E3 — `link` / `unlink`'
    change: 'The E3 As landed line reads: "- **As landed: built.** `viola link` / `viola unlink` and the wrapper''s `link` / `unlink` methods are served (chunk 2026-10-26-explicit-session-links), held by root cases of `tests/cli_links.rs`; the driver is a self-reported name and is not booted, so step 1 boots only the driven `builder`; the implicit link that a `send` or `answer` from another instance records stays held by root cases of the same file."'
    sidecar: 'test-plan §6 E3: the broad As landed line retires the hold to the route entry Explicit session links.'
    rationale: 'Report Retired (the hold, every master line stating it; the -32601 answer) and Counts (owner pattern title stale at test-plan.md:570).'
    basis: '.andromeda/test-plan.md:570'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 234
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'In the link-leg sentence, "on the implicit link alone (no `viola link` verb exists; the explicit verbs are not built)" reads "on the implicit link alone (the explicit `viola link` / `viola unlink` landed later, at chunk 2026-10-26-explicit-session-links, so step 1''s `link` can now be driven by the verb; steps 1–3 stay unbuilt for the `read_snapshot` reason below)"; the rest unchanged.'
    sidecar: 'e2e-test-strategy: E5 no longer states that no viola link verb exists.'
    rationale: 'Report Retired (the hold on the explicit verbs and every master line stating it); Counts (linkverbs tests: the no-viola-link-verb claim stale).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 268
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add a sub-bullet under "the home fails strict-modes": "as landed for `viola link` / `viola unlink` (chunk 2026-10-26-explicit-session-links), a `tests/cli_links.rs` case, not a `cross_exit_causes.rs` row: `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) refuses a group-writable driven instance exit 21 before any role file or log opens and reads the instance dir''s mode back as found, because a check placed after `client::start` would pass a 0770 home; a remove-the-guard control that moves the check went red 1 of 1."'
    sidecar: 'e2e-test-strategy: the exit-21 strict-modes cause gains the link verbs'' widened-instance case.'
    rationale: 'Report Decisions & corrections (the order trap held by a test; remove-the-guard control A 1 of 1) and Outcome (security: group-writable driven instance refused before any log).'
    fact: F4
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 270
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '"its Linux and Windows case is the six-verb server-identity negative" reads "its Linux and Windows case is the eight-verb server-identity negative (`link` / `unlink` added at chunk 2026-10-26-explicit-session-links)"; the rest unchanged.'
    sidecar: 'e2e-test-strategy: the exit-cause matrix names the eight-verb server-identity negative.'
    rationale: 'Report Counts: CLI frame verbs six to eight; tests/security_negatives_server_identity.rs six frame verbs to eight (pattern idtable).'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 272
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Append to the "no instance is running for the name" bullet: "; for `viola link` / `viola unlink`, as landed a `tests/cli_links.rs` case (`link_verbs_against_a_stopped_instance_exit_21`, the not-running hint last), not a `cross_exit_causes.rs` row"'
    sidecar: 'e2e-test-strategy: the not-running exit-21 cause points at the link verbs'' stopped-instance case.'
    rationale: 'Report Symbols / APIs (link_call step 3: no live endpoint is exit 21 with the not-running hint last) and Outcome (design: link_verbs_against_a_stopped_instance_exit_21 met).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 298
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after the raw-channel bullet''s text: "As landed (chunk 2026-10-26-explicit-session-links): `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`; `binary(channel_links)` on the `ci` profile''s 20 s stamped-home kill list) sends raw frames through `viola_channel::Client::request`, past every CLI check: a missing, non-string or invalid name, a `driven` that is not the serving wrapper, or a `driver` equal to `driven` each gets `-32602` with `data: null` and no link line or snapshot change, beside a delivering control."'
    sidecar: 'e2e-test-strategy: the raw-channel link / unlink name negative is built as channel_link_names_are_validated_by_the_wrapper.'
    rationale: 'Report Outcome (security: raw bad-name / foreign-driven / self-pair frames, met by channel_link_names_are_validated_by_the_wrapper), Deviations (implement 4) and Harness / gate surface (binary(channel_links) on the kill line).'
    fact: F3
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 305
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '"runs `send`, `wait`, `last`, `answer`, `pause` and `release`" reads "runs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (the last two since chunk 2026-10-26-explicit-session-links)"; the rest unchanged.'
    sidecar: 'e2e-test-strategy: the server-identity negative runs eight CLI frame verbs.'
    rationale: 'Report Counts: CLI frame verbs six to eight; tests/security_negatives_server_identity.rs eight verbs in two modes plus the controls (pattern idtable).'
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 306
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '"runs each of the six verbs" reads "runs each of the eight verbs"; the rest unchanged.'
    sidecar: 'e2e-test-strategy: the server-identity control runs the eight verbs.'
    rationale: 'Report Counts: CLI frame verbs six to eight; the control row covers each verb in both modes.'
    dependent-of: D-tests-count-literals
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 21
    section: '## 7. Test Data & Fixtures'
    change: 'In the Time-sensitive data row, after the budget gate clause add: "; `LinkSet::line` stamps the link line''s `since` with `Utc::now()` directly (`src/run/links.rs:92`), outside the clock seam, so root cases assert `since` by shape only (chunk 2026-10-26-explicit-session-links)"; no pipe added, the rest of the row unchanged.'
    sidecar: 'test-data-fixtures: the Time-sensitive row names the link line''s since, read outside the clock seam.'
    rationale: 'Report Decisions & corrections: LinkSet::line stamps since with Utc::now() directly (src/run/links.rs:92), outside the clock seam; root cases assert since by shape only (is_clock_time).'
    basis: 'src/run/links.rs:92'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 8
    section: '## 8. Mocking & Stubbing Discipline'
    change: '"`chrono::Utc::now()` is read only through the clock seam, with one exception as landed: the budget gate''s three evaluation points read it directly" reads "`chrono::Utc::now()` is read only through the clock seam, with two exceptions as landed: the budget gate''s three evaluation points read it directly"; after the closing parenthesis add: "; and `LinkSet::line` stamps the link line''s `since` with it directly (`src/run/links.rs:92`, chunk 2026-10-26-explicit-session-links), so root cases assert `since` by shape only, through `is_clock_time` in `tests/cli_links.rs`, itself held by `is_clock_time_reads_the_shape_only`."'
    sidecar: 'mocking-stubbing-discipline: Utc::now() gains a second landed exception, LinkSet::line''s since, asserted by shape only.'
    rationale: 'Report Decisions & corrections (LinkSet::line stamps since with Utc::now() directly, src/run/links.rs:92) and New text, by line (tests/cli_links.rs is_clock_time 311-319, is_clock_time_reads_the_shape_only 411-418).'
    basis: 'src/run/links.rs:92'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 703
    section: '## 8. Mocking & Stubbing Discipline'
    change: 'In the Time line, "`chrono::Utc::now()` is read only through the clock seam, except at the budget gate''s three evaluation points" reads "`chrono::Utc::now()` is read only through the clock seam, except at the budget gate''s three evaluation points and the link line''s `since` (`LinkSet::line`, asserted by shape only)"; the rest unchanged.'
    sidecar: 'test-plan §8: the broad Time line names the link line''s since as a second Utc::now() exception.'
    rationale: 'Report Decisions & corrections: LinkSet::line stamps since with Utc::now() directly (src/run/links.rs:92), outside the clock seam; the body''s broad line names only the budget gate.'
note: 'E3 body steps (test-plan.md:562, Boot overseer and builder) left as the spec, the deviation stated in both As landed lines; security / obs / architecture / design / a11y stale sites the report lists (frameverbs, checkinst, refuse, vrejected, handoff) are other masters'' detectors; §5 / §9 kill-line and §5 strict-modes exits belong to group 2.'
