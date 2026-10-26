proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 72
    section: '## 5. Integration Test Strategy'
    change: 'In the `help_lists_only_served_verbs` clause replace "no `hook`, `help`, `link`, `unlink` or bare `plugin` in the table" with "no `hook`, `help` or bare `plugin` in the table; since chunk 2026-10-26-explicit-session-links the `HELP` literal holds the row `handoff:  link, unlink` between `wheel:` and `setup:`, and `link` / `unlink` are served verb paths read by both `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`"'
    sidecar: 'integration-test-strategy CLI: the help pin no longer reads `link` / `unlink` as absent from the table; the `handoff:` row is built (chunk 2026-10-26-explicit-session-links)'
    rationale: 'Report Retired: the hold on `viola link` / `viola unlink` and every master line stating it; Symbols / APIs: `viola --help` gains `handoff:  link, unlink` between `wheel:` and `setup:`; Outcome (layouts): met by `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 52
    section: '## 5. Integration Test Strategy'
    change: '  - served `link` / `unlink` (chunk 2026-10-26-explicit-session-links; before it both answered `-32601`): root `tests/channel_links.rs` `channel_link_names_are_validated_by_the_wrapper` sends raw frames through `viola_channel::Client::request` (past every CLI check) with a missing, non-string or invalid name, a `driven` that is not the serving wrapper own name, and a `driver` equal to `driven`, each answered `-32602` / `data: null` with no line appended, beside a control that links; the `src/run/links.rs` units `links_set_method_*` hold a second `link` of a held pair appending nothing and answering the same `since` with `already_linked: true`, an `unlink` after which a new `link` takes a later `since`, an `unlink` of an absent pair appending nothing and answering `{}`, a `from` of any shape changing nothing, and a failed append answered `-32603` with the set unchanged; `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`) holds the routing; `binary(channel_links)` is on the nextest `ci` profile 20 s stamped-home kill line (`.config/nextest.toml`)'
    sidecar: 'integration-test-strategy Wrapper channel: the served `link` / `unlink` methods, their `-32602` causes and their tests, `channel_links` on the kill line (chunk 2026-10-26-explicit-session-links)'
    rationale: 'Report Symbols / APIs (channel methods `link` and `unlink` are served; `driver_of` refusals; `{since, already_linked}` / `{}` answers); Coverage of new surfaces (channel methods row); Schema / config and Harness (`binary(channel_links)` joins the `ci` profile kill list); expected amendment test-plan §5 Wrapper channel'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 52
    section: '## 5. Integration Test Strategy'
    change: '  - the unserved-method example of `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) is `anything` (`-32601`), no longer `link` / `unlink`; the conn-less response line of an unserved method carries no `method` key, because the channel names only its closed method list (`method_label`), so the case asserts the key absent and a test asserting a method value on that line for a name outside the list is wrong by construction (chunk 2026-10-26-explicit-session-links)'
    sidecar: 'integration-test-strategy Wrapper channel: the unserved-method example moves to `anything` and its response line holds no `method` key'
    rationale: 'Report Retired (`link` / `unlink` as the unserved example of `channel_wrapper_logs_each_call_with_corr_and_conn`, now `anything`); Spec claims disproved by measurement (the conn-less response line logs no `method`); Deviations implement 2; Decisions & corrections hazard'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 47
    section: '## 5. Integration Test Strategy'
    change: '  - since chunk 2026-10-26-explicit-session-links the CLI half of the server-identity negative holds every CLI frame verb, `viola link` and `viola unlink` among them, in both modes on Linux and Windows (`tests/security_negatives_server_identity.rs`, beside its control reaching the recorded wrapper): no frame, exit 21 `instance-unreachable` with the server-verify hint and `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`; macOS the recorded gap'
    sidecar: 'integration-test-strategy Wrapper channel: the identity negative reaches the two new frame verbs'
    rationale: 'Report Symbols / APIs (new verbs step 5; the CLI frame verbs now include `link` / `unlink`); Counts (`idtable`: the table gains the two verbs in both modes); Outcome (security impostor server, Linux and Windows)'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: '## 5. Integration Test Strategy'
    change: '  - `viola link` / `viola unlink` run `check_instance` on the driven instance before `client::start` (the founder form, chunk 2026-10-26-explicit-session-links), so a refusal is exit 21 with the fixed instance-state pair (or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, stderr empty), no role file opened, no line in any log and the directory mode as found: `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home'
    sidecar: 'integration-test-strategy On-disk: the link verbs strict-modes refusal before any log and its test'
    rationale: 'Report Symbols / APIs (`link_call` step 1); Decisions & corrections (the order trap held by a test; exit 21 as `list` / `plugin install`); Outcome (security group-writable driven instance refused before any log, mode as found)'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 72
    section: '## 5. Integration Test Strategy'
    change: '  - `tests/cli_links.rs` pins the link verbs (chunk 2026-10-26-explicit-session-links): `link_verbs_print_the_design_lines` (the three result lines byte for byte, `link    <driver> -> <driven>  since <HH:MM:SSZ>` / `...  already linked` / `unlink  <driver> -> <driven>`, and `{"v":1,"ok":<ok>}` under `--json`), `link_verbs_against_a_stopped_instance_exit_21` (the not-running hint last), `link_verbs_log_the_channel_pair_and_no_names` (the wrapper `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no name); the E3 and restart cases (`e3_two_links_and_a_send_leave_one_link_record`, `links_an_unlink_survives_a_wrapper_restart`) are §6; `since` is asserted by shape only (`is_clock_time`, self-checked by `is_clock_time_reads_the_shape_only`) because `LinkSet::line` stamps it outside the clock seam; backed by the `src/human/verbs.rs` units `link_verb_lines_are_the_design_lines_in_one_write`, `link_since_is_the_utc_time_alone` and the `src/cmd/link.rs` units `link_reply_is_read_by_its_verbs_shape`, `link_line_is_built_from_the_names_and_the_reply`'
    sidecar: 'integration-test-strategy CLI: the `viola link` / `viola unlink` cases of `tests/cli_links.rs` and their units'
    rationale: 'Report Symbols / APIs (three result lines; new verbs step 6); Coverage of new surfaces (CLI verbs and result-line rows); Decisions & corrections (`since` read outside the clock seam, asserted by shape); Outcome (design, obs rows); expected amendment test-plan §5 CLI'
note: 'Body test-plan.md §5 (lines 435-445) reads true as worded (every method; CLI 21 for every other verb); the stale E3 "As landed" at test-plan.md:570 is the body / e2e category, not this file.'
