## 2026-10-26-explicit-session-links — `channel_links` on the 20 s kill line
**Section:** §3 → Bootstrap phases (derive for route / setup-project) (bootstrap-phases-derive-for-route-setup-project.md: test-runner-install, the `[profile.ci]` kill line)
**Change:**
- The `ci` profile's 20 s kill line holds twenty-two verify-driven binaries (the rule: the `binary(…)` names on that filter line; was twenty-one, per 2026-10-17-mcp-server-for-drivers — harness keys and the time mechanism): `channel_links` (`tests/channel_links.rs`) joined, each of its cases booting a wrapper (`booted_wrapper`).
- The list's reading is no longer stamped "at chunk 2026-10-17-mcp-server-for-drivers"; that chunk now dates `mcp_tools`' joining.
**Why:** the chunk added `binary(channel_links)` to the kill line's filter. The report's claim that no master lists the kill line's binaries was false: this key file enumerates them, so its count moves by its rule.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' result lines and outputs pinned
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the surface table's cli row; the Exit-cause matrix, exit 21 no instance running) · §5 Integration Test Strategy (integration-test-strategy.md: CLI) · §7 Test Data & Fixtures (test-data-fixtures.md: Human-mode expected output) · §4 Unit Test Strategy (unit-test-strategy.md: root bin, the link verbs)
**Change:**
- §6 cli row and §7: the three `viola link` / `viola unlink` result lines join `viola --help` and the `viola list` board as human layouts pinned by literal in their test files: `tests/cli_links.rs` `link_verbs_print_the_design_lines` (the printed time checked by shape), and the units `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`).
- §5 CLI: `help_lists_only_served_verbs` reads no `hook`, `help` or bare `plugin` in the table (was "no `hook`, `help`, `link`, `unlink` or bare `plugin`", per 2026-10-17-cli-output-discipline — viola --help pinned by literal; cli_output_plain's new cases; the colour variables in Vector 6). New: `tests/cli_links.rs` pins `link_verbs_print_the_design_lines` (the three lines byte for byte, `{"v":1,"ok":…}` under `--json`), `link_verbs_against_a_stopped_instance_exit_21` and `link_verbs_log_the_channel_pair_and_no_names` (the wrapper's `channel-request` / `channel-response` pair carrying `method` `link` / `unlink`, and no driver or driven name in its process log).
- §6 exit 21, no instance running: for the two verbs a `tests/cli_links.rs` case, `link_verbs_against_a_stopped_instance_exit_21` (both verbs in both modes: the not-running hint last, or the `instance-unreachable` document with stderr empty), not a `cross_exit_causes.rs` row.
- §4: `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` (`src/cmd/link.rs`) beside the two `verbs.rs` cases; the line's time is the reply's `since` as UTC clock time, and a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through `escape_message`.
**Why:** the chunk built both verbs with three plain result lines and their help row; the report reads each of these outcomes met by the cases named.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — E3 built: the explicit verbs' hold retired
**Section:** §6 E2E Test Strategy (test-plan.md: Scenario E3, As landed; e2e-test-strategy.md: the Scenario E3 and Scenario E5 procedures)
**Change:**
- E3 As landed: was "owed, built in part" — steps 1 and 3 could not run, `viola link` / `viola unlink` not built (the wrapper answering both methods `-32601`) and held, their widening not ratified, no explicit verb before Epoch 6, the hold on the route entry "Explicit session links"; now "built": the verbs and the wrapper's `link` / `unlink` methods are served. The driver `overseer` is a self-reported `VIOLA_NAME` and is not booted, so step 1 boots only `builder`.
- E3 itself: `e3_two_links_and_a_send_leave_one_link_record` (`tests/cli_links.rs`, from `stamped_home`): two `link` calls leave one `link` record and one `since`, the second answering `already_linked: true`; an issued `send` from the self-reported driver adds no `link` record; `unlink` appends one `unlink` record (`source: wrapper`, `{driver, driven}`) and empties the snapshot's `links`, a second `unlink` appending nothing; a later `link` is a new pair with a later `since`.
- `links_an_unlink_survives_a_wrapper_restart`: a wrapper started over a log whose pair the methods linked and then unlinked restores an empty set (its first snapshot's `links` empty, no line appended).
- Was "no case reads an `unlink` record a product path wrote, because none writes one" (per 2026-10-10-session-links — Path 6's four verbs, E3 and E5 as landed); now a product path writes it: the `unlink` method, `LinkSet::remove`'s one product caller. Step 2's four root cases stand.
- E5's link leg: was "no `viola link` verb exists; the explicit verbs are not built"; now the served verb can drive step 1's `link`, and steps 1–3 stay unbuilt for the `read_snapshot` reason.
**Why:** the chunk served both methods and built both verbs in the form the founder's own answer to its P4 fork named (inputs#I3, live by dialog, relayed verbatim by the operator), ratifying the widening the hold waited on; the report retires the hold and the `-32601` answer. The two E3 procedure items, read at first as contradicting the line they sit under, were applied with their companion on the overseer's founder-delegated answer at the wrap's halt (inputs#I6, PROVISIONAL).
**Kept:** E3's body steps (boot `overseer` and `builder`) stand as the spec; the deviation is stated in the As landed lines.
**Supersedes:** 2026-10-13-t06-38-35-wrap — E3's explicit steps held on the Epoch 6 entry
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link verbs' instance check before any log
**Section:** §1 Test Scope Summary (test-scope-summary.md: the CLI verbs entity) · §4 Unit Test Strategy (unit-test-strategy.md: the instance check) · §5 Integration Test Strategy (integration-test-strategy.md: On-disk) · §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix, exit 21 the home fails strict-modes)
**Change:**
- §1 As landed (`link` / `unlink`): of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens; a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
- §5 On-disk: their refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found.
- §6: a `tests/cli_links.rs` case, not a `cross_exit_causes.rs` row: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) makes the driven instance's directory group-writable while its wrapper runs; both verbs in both modes exit 21, the verb's `cli` log gains no line, the wrapper gets no frame, and the mode reads back as found. Remove-the-guard control A, the check moved, read red 1 of 1.
- §4: the instance check's root refusal cases gain that case beside the three in `tests/cli_revive.rs`.
**Why:** the founder's own answer to the chunk's P4 fork (inputs#I3, live by dialog, relayed verbatim by the operator) put the instance check first, its refusal exit 21. The order trap the case holds: opening a role file narrows an existing home to 0700, so a check placed after it passes a 0770 home. The wrapper-only self-pair refusal is the operator's answer of chunk 2026-10-10-session-links.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — eight CLI frame verbs under the server check
**Section:** §1 Test Scope Summary (test-scope-summary.md: As landed (server verification)) · §2 Test Strategy (test-strategy.md: Unit, Tier-specific depth) · §4 Unit Test Strategy (unit-test-strategy.md: the server check before any frame) · §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's server-verification cause; Security control negatives, Server identity)
**Change:**
- The CLI frame verbs read eight, `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink` (was six, per the 2026-10-26-server-verification-before-any-frame entries on the exit-cause matrix, the server-identity security negative and where the server verification is asserted): §1 names them as the verbs `tests/security_negatives_server_identity.rs` asserts; §6's server-verification exit-21 cause is the eight-verb negative, which runs the eight under `--json` and in human mode, and its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the eight in both modes.
- §4: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` (was the six, per 2026-10-26-server-verification-before-any-frame — the root bin's server-check unit cases).
- §2: the `last.rs`, `pause.rs` and `unlink.rs` test modules are gated whole off macOS (was `last.rs` and `pause.rs`, per 2026-10-26-server-verification-before-any-frame — the server check's captured-span cases compiled out on macOS); a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS.
**Why:** the two verbs take the liveness pre-check and the server check through `cmd::client::connect` like the other six, and the identity binary grew to eight verbs in two modes, green on CI's Linux and Windows legs. The module-gate rule is the chunk's implement deviation: a whole-module gate fits only a module whose every case is gated.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the served `link` / `unlink` methods and the raw-channel name negative
**Section:** §1 Test Scope Summary (test-scope-summary.md: the ipc-internal wrapper channel surface) · §2 Test Strategy (test-strategy.md: the trigger table, IPC vector (V1)) · §4 Unit Test Strategy (unit-test-strategy.md: root bin, the served link methods) · §6 E2E Test Strategy (test-plan.md: Non-path suites, Security control negatives; e2e-test-strategy.md: Security control negatives, Raw-channel `link` / `unlink` name validation)
**Change:**
- §1 As landed: both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written; a failed `record` / `remove` is `-32603`; `link` answers `ok` `{since, already_linked}` and `unlink` an empty `ok` object.
- §6 the raw-channel negative, as landed: `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`, `binary(channel_links)` on the 20 s kill line) writes its frames through `viola_channel::Client::request`, past every CLI check and the server check. For `link` and `unlink` each, a `driver` or `driven` of `../x`, `BUILDER`, `a/b`, an empty string, a 33-character name or a number, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` each get `-32602` with `data: null`, no `link` / `unlink` record and the snapshot's `links` empty; its control, a valid `link`, answers `already_linked: false` and appends one `link` record. The master's suite list names the file.
- §2 the V1 row gains root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written.
- §4: `links_set_method_link_twice_writes_one_line_and_answers_the_same_since`, `links_set_method_unlink_then_link_takes_a_later_since`, `links_set_method_unlink_of_an_absent_pair_writes_nothing`, `links_set_method_refuses_a_bad_pair_with_invalid_params`, `links_set_method_a_from_changes_nothing`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged` (`src/run/links.rs`) and `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`).
**Why:** the chunk served both methods (before, both answered `-32601`), neither reading `from`; the raw-channel negative is its new binary, green with the served methods' units.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — an unserved method's response line carries no `method` key
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Wrapper channel)
**Change:**
- New: an unserved method's conn-less response line carries no `method` key, because the channel's `method_label` names only the closed method list: `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) sends the unserved `anything` and asserts the key absent; a test asserting a `method` value on such a line for a name outside that list is wrong by construction.
**Why:** measured this chunk, disproving the plan's assumption that such a line logs its `method`: the test's unserved example was `unlink`, now served, and its assertion became the key's absence.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the link line's `since` outside the clock seam
**Section:** §8 Mocking & Stubbing Discipline (test-plan.md: Time; mocking-stubbing-discipline.md: Time) · §7 Test Data & Fixtures (test-plan.md: Time-sensitive data; test-data-fixtures.md: Time-sensitive data)
**Change:**
- §8: `chrono::Utc::now()` is read only through the clock seam with two exceptions as landed (was one, the budget gate's three evaluation points): `LinkSet::line` stamps the link line's `since` with it directly, so root cases assert `since` by shape only, through `is_clock_time` in `tests/cli_links.rs`, itself held by `is_clock_time_reads_the_shape_only`. The master's Time line names `LinkSet::line` beside the budget gate.
- §7: time-sensitive data is injected behind `viola_core::Clock` except a link line's `since`, which the wrapper stamps from the wall clock and root cases assert by its shape only (§8); the detail's row states the same.
**Why:** the report's Decisions: `LinkSet::line` stamps `since` with `Utc::now()` directly, outside the clock seam. The detail's clause, read at first as contradicting the run it rewrites (one exception to two), was applied on the overseer's founder-delegated answer at the wrap's halt (inputs#I6, PROVISIONAL).
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/
