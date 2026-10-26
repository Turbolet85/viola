# Apply — test-plan · part 1 of 2

## Verdicts
- fanout-test-plan.md 1 · applied · .andromeda/test-plan.md:570#1
- fanout-test-plan--cascade-g1.md 17 · folded into fanout-test-plan.md 1
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:606#1
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:705#1
- fanout-test-plan--cascade-g1.md 27 · folded into fanout-test-plan.md 3
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:653#1
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:12#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:35+#1
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:61+#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-strategy.md:10#1
- cascade-compare-test-plan.md 3 · applied · .andromeda/test-plan-detail/test-strategy.md:58#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:25#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:68#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:69+#1
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:69+#2
- fanout-test-plan--cascade-g1.md 9 · already stands · .andromeda/test-plan-detail/integration-test-strategy.md:42–42 · «`send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`»
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:52+#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:72#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:73+#1
- cascade-compare-test-plan.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:57+#1

## Edits
- line: .andromeda/test-plan.md:570
  old: «**As landed: owed, built in part.** `viola link` and `viola unlink` are not built (the wrapper answers both `-32601`) and are held to the route entry "Explicit session links"; the implicit link that a `send` or `answer` from another instance records is held by root cases of `tests/cli_links.rs`.»
  new: «**As landed: built.** The driven wrapper serves `link` / `unlink`, and the scenario runs as `tests/cli_links.rs` `e3_two_links_and_a_send_leave_one_link_record` from `stamped_home`: the driver `overseer` is a self-reported `VIOLA_NAME` and is not booted, so step 1 boots only `builder`. `links_an_unlink_survives_a_wrapper_restart` holds an `unlink` across a wrapper restart, and the implicit link that a `send` or `answer` from another instance records is held by root cases of the same file.»
- line: .andromeda/test-plan.md:606
  old: «raw-channel `link` / `unlink` name validation,»
  new: «raw-channel `link` / `unlink` name validation (`tests/channel_links.rs`),»
- line: .andromeda/test-plan.md:705
  old: «except at the budget gate's three evaluation points,»
  new: «except at the budget gate's three evaluation points and at `LinkSet::line`, which stamps a link line's `since` (root cases assert it by its shape only, `is_clock_time` in `tests/cli_links.rs`),»
- line: .andromeda/test-plan.md:653
  old: «injected per test behind `viola_core::Clock`;»
  new: «injected per test behind `viola_core::Clock`, except a link line's `since`, which the wrapper stamps from the wall clock and root cases assert by its shape only (§8 Time);»
- line: .andromeda/test-plan-detail/test-scope-summary.md:12
  old: «the six CLI frame verbs by»
  new: «the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by»
- after: .andromeda/test-plan-detail/test-scope-summary.md:35
  anchor: «  - **Testability:** testable»
  add: «  - **As landed (`link` / `unlink`):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens (their refusal: §5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:61
  anchor: «  - **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.»
  add: «  - **As landed (`link` / `unlink`):** both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written, and a failed `record` / `remove` is `-32603`; `link` answers `ok` `{since, already_linked}` and `unlink` an empty `ok` object.»
- line: .andromeda/test-plan-detail/test-strategy.md:10
  old: «the `last.rs` and `pause.rs` test modules, gated whole»
  new: «the `last.rs`, `pause.rs` and `unlink.rs` test modules, gated whole (a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS)»
- line: .andromeda/test-plan-detail/test-strategy.md:58
  old: «a CLI frame verb against a server that is not the recorded wrapper)»
  new: «a CLI frame verb against a server that is not the recorded wrapper; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:25
  old: «and one on `viola revive --list`.»
  new: «and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk).»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:68
  old: «`src/cmd/{send,wait,last,answer,pause,release}.rs`»
  new: «`src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs`»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:69
  anchor: «  - The hook's server check (`src/cmd/hook.rs`): `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose check fails takes the direct append (`append_session_end`), held by a mismatch row of the hook's unit cases. The unit impostor varies the recorded start time (this process recorded a second early; an unparseable `started_at`) rather than recording another live pid, which is the integration rows' shape.»
  add: «  - The served link methods: `links_set_method_link_twice_writes_one_line_and_answers_the_same_since`, `links_set_method_unlink_then_link_takes_a_later_since`, `links_set_method_unlink_of_an_absent_pair_writes_nothing`, `links_set_method_refuses_a_bad_pair_with_invalid_params`, `links_set_method_a_from_changes_nothing` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged` (`src/run/links.rs`), and `methods_route_link_and_unlink_to_the_link_set` (`src/cmd/run.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:69
  anchor: «  - The hook's server check (`src/cmd/hook.rs`): `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose check fails takes the direct append (`append_session_end`), held by a mismatch row of the hook's unit cases. The unit impostor varies the recorded start time (this process recorded a second early; an unparseable `started_at`) rather than recording another live pid, which is the integration rows' shape.»
  add: «  - The link verbs: `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` (`src/cmd/link.rs`), `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`); the line's time is the reply's `since` as UTC clock time, and a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through `escape_message`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:52
  anchor: «  - linking an existing pair adds no log line»
  add: «  - an unserved method's conn-less response line carries no `method` key, because the channel's `method_label` names only the closed method list: `channel_wrapper_logs_each_call_with_corr_and_conn` (`tests/channel_endpoint.rs`) sends the unserved `anything` and asserts the key absent; a test asserting a `method` value on such a line for a name outside that list is wrong by construction»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:57
  anchor: «  - strict-modes refusals: exit 1 for `run`/`mcp` and for `viola revive` (the start arm and `--list` alike with the state-files `unable:` / `hint:` stderr pair, or under `--json` the `strict-modes-failed` document on stdout — `tests/cli_revive.rs` `revive_json_refuses_state_another_user_could_write_on_both_arms` — and no log line: since chunk 2026-10-10-budget-governor the start arm's check runs before its wrapper log opens, so there is no `process-start` or `process-exit` line, no child, and the home's mode is left as found), hook 0 with no output (the statusline arm reads and opens nothing under the home after a refused check and writes no line, however often it is invoked), CLI 21 for every other verb»
  add: «  - `viola link` / `viola unlink` run `check_instance` on the driven instance before any role file or log opens: a refusal is exit 21 with the fixed instance-state `unable:` / `hint:` pair (under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout, stderr empty), no line in any log and the directory's mode as found; `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:72
  old: «no `hook`, `help`, `link`, `unlink` or bare `plugin` in the table»
  new: «no `hook`, `help` or bare `plugin` in the table»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:73
  anchor: «  - `tests/cli_plugin_install.rs` pins `viola plugin install [--json]` against `CARGO_BIN_EXE_viola`, each case in a fresh home: `plugin_install_writes_the_mcp_only_marketplace` (the tree's dirs and three files, the `.mcp.json` registration `["--home", <home>, "mcp"]` on the exe's absolute path, no `hooks` anywhere), `plugin_install_rerun_rewrites_the_same_tree`, `plugin_install_writes_owner_only_modes` (0700 / 0600 under `umask 000`, Unix), `plugin_install_json_prints_the_ok_document` (no absolute path on either stream), `plugin_install_prints_the_two_commands` (the printed lines by literal), `plugin_install_writes_no_escape_byte` (piped, `NO_COLOR`, `TERM=dumb`), `plugin_install_refuses_a_widened_home` (exit 21 in both modes, the home left as found, no `diagnostics/`, Unix), `plugin_install_logs_the_self_pair_under_a_name` and `plugin_alone_is_a_usage_error` (exit 2). Its units: `marketplace_files_are_the_validated_shape`, `marketplace_mcp_json_escapes_the_paths`, `marketplace_files_hold_no_hooks` (`crates/viola-agent-claude/src/marketplace.rs`); `tree_dirs_are_the_home_and_the_four_marketplace_dirs`, `tree_passes_a_new_home_and_its_own_tree`, `tree_passes_refuses_any_dir_another_user_could_write`, `fwd_writes_every_backslash_as_a_slash` (`src/cmd/plugin.rs`); `write_plugin_installed_is_the_planning_form_in_one_write`, `shell_word_quotes_only_what_the_shell_would_split` (both shell families on every host), `command_word_takes_this_host_family` (`src/human/verbs.rs`).»
  add: «  - `tests/cli_links.rs` pins the link verbs' outputs and refusals: `link_verbs_print_the_design_lines` (the three result lines byte for byte, and `{"v":1,"ok":…}` under `--json`), `link_verbs_against_a_stopped_instance_exit_21` (exit 21, the not-running hint last) and `link_verbs_log_the_channel_pair_and_no_names` (the wrapper's `channel-request` / `channel-response` pair carrying `method` `link` / `unlink`, and no driver or driven name in the wrapper's process log).»

## Patterns
[[pattern]]
id = "tp1-01"
regex = 'link`[^.]{0,60}not built'
ignore_case = true

[[pattern]]
id = "tp1-02"
fixed = 'answers both `-32601`'

[[pattern]]
id = "tp1-03"
regex = 'only through the clock seam, except at the budget gate.s three evaluation points[,.;]'

[[pattern]]
id = "tp1-04"
regex = 'injected per test behind `viola_core::Clock`;'

[[pattern]]
id = "tp1-05"
regex = 'six (CLI )?frame verbs'
ignore_case = true

[[pattern]]
id = "tp1-06"
fixed = '`last.rs` and `pause.rs` test modules'

[[pattern]]
id = "tp1-07"
fixed = '{send,wait,last,answer,pause,release}.rs'

[[pattern]]
id = "tp1-08"
fixed = '`help`, `link`, `unlink` or bare `plugin`'
