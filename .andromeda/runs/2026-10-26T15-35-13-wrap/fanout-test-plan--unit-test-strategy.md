proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 68
    section: "## 4. Unit Test Strategy"
    change: "  - The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` (the eight CLI frame verbs; `link` and `unlink` since chunk 2026-10-26-explicit-session-links) and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit: in `src/cmd/link.rs` the case alone, its module's reply-shape and line cases running there, in `src/cmd/unlink.rs` the whole module, whose one case is gated); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, the server cause `during` `connect`)."
    sidecar: "§4 root bin server-check unit cases: the six frame verbs' list becomes eight (`link`, `unlink` added), with the macOS gate's case-versus-module split (chunk 2026-10-26-explicit-session-links)."
    rationale: "The report's Counts move CLI frame verbs six → eight and its Coverage names two new `request_refuses_an_unverified_server_inside_its_dispatch_span` cases (src/cmd/link.rs, src/cmd/unlink.rs); line 68 still enumerates the six verbs' files; implement deviation 1 gates only link.rs's case and unlink.rs's module whole."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:94
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: "## 4. Unit Test Strategy"
    change: "  - The served `link` / `unlink` channel methods (chunk 2026-10-26-explicit-session-links, `src/run/links.rs`): `LinkSet::link` and `LinkSet::unlink` share `driver_of`, which reads `driver` and `driven` as strings through `ViolaName::try_new`; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` (`data: null`) with no line written, and neither method reads `from`. Unit cases: `links_set_method_link_twice_writes_one_line_and_answers_the_same_since` (a second `link` of a held pair appends nothing and answers the same `since` with `already_linked: true`), `links_set_method_unlink_then_link_takes_a_later_since`, `links_set_method_unlink_of_an_absent_pair_writes_nothing`, `links_set_method_refuses_a_bad_pair_with_invalid_params`, `links_set_method_a_from_changes_nothing` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged` (a `record` / `remove` error is `-32603`); `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`) holds `dispatch_call` routing both names to the link set. `LinkSet::line` stamps `since` with `Utc::now()` directly, outside the clock seam."
    sidecar: "§4 root bin gains the served link/unlink methods' unit cases (`links_set_method_*`, `methods_route_link_and_unlink_to_the_link_set`), chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Symbols / APIs serve `link` / `unlink` through `LinkSet::link` / `unlink` / `driver_of` and its Coverage of new surfaces names the `links_set_method_*` functions and `methods_route_link_and_unlink_to_the_link_set` as the unit tier; the root bin group names no link-method case; the `since` stamp outside the clock seam is the report's Decisions entry."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:243
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: "## 4. Unit Test Strategy"
    change: "  - The `viola link` / `viola unlink` verbs (chunk 2026-10-26-explicit-session-links): `link_reply_is_read_by_its_verbs_shape` (`src/cmd/link.rs`) reads the reply by its verb's shape, a `link` `ok` with a string `since` and a boolean `already_linked`, an `unlink` `ok` that is an object, and any other `ok` shape an internal error; `link_line_is_built_from_the_names_and_the_reply` builds the result line from the verb's own parsed `ViolaName` arguments, never a name read back from the reply; `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`) pin `write_linked`, `write_already_linked` and `write_unlinked`, each one `write_all`, the time the reply's `since` as UTC clock time in whole seconds with `Z`, and a `since` that is no RFC 3339 instant printed through `escape_message`."
    sidecar: "§4 root bin gains the link/unlink verbs' reply-shape and result-line unit cases, chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Symbols / APIs add the verbs' reply read (`client::answer_of` by verb shape) and the three result lines, and its Coverage of new surfaces names `link_reply_is_read_by_its_verbs_shape`, `link_line_is_built_from_the_names_and_the_reply`, `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` as the unit tier; no root bin item names them."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:238
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: "## 4. Unit Test Strategy"
    change: "Append to the instance-check item, after the three `tests/cli_revive.rs` root cases: since chunk 2026-10-26-explicit-session-links `check_instance` is also run by `viola link` / `viola unlink` on the driven instance before any role file or log opens, its refusal held by the `cfg(unix)` root case `link_verbs_refuse_a_widened_instance_before_any_log` (`tests/cli_links.rs`: a widened driven instance refused exit 21 for both verbs, the mode read after the refusal and left as found)."
    sidecar: "§4 viola-state instance check: the root cases holding its refusal gain the link/unlink verbs' case (chunk 2026-10-26-explicit-session-links)."
    rationale: "The report moves `check_instance(` product call sites 2 → 3 (+ src/cmd/link.rs, shared by both verbs) and asks a detector to extend a list of refusing verbs where it finds one; line 25 lists only `viola revive`'s root refusal cases; the Outcome names `link_verbs_refuse_a_widened_instance_before_any_log` (ubuntu, macOS)."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:102
note: "The body's §4 root bin summary (.andromeda/test-plan.md:386) names no link/unlink method or verb case, and §6 E3's As-landed (test-plan.md:570) still states the verbs unbuilt — both outside this category file, for the body and E2E detectors."
