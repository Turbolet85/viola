# Apply — test-plan · part 2 of 2

## Verdicts
- fanout-test-plan--cascade-g1.md 13 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:10–10 · «`tests/cli_list.rs`, `tests/cli_output_plain.rs` and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`)»
- fanout-test-plan--cascade-g1.md 14 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:226–226 · «`viola link` / `viola unlink` and the wrapper's `link` / `unlink` methods are served.»
- fanout-test-plan--cascade-g1.md 15 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:227–227 · «**E3 itself:** `e3_two_links_and_a_send_leave_one_link_record` (`tests/cli_links.rs`)»
- fanout-test-plan--cascade-g1.md 16 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:228–228 · «`links_an_unlink_survives_a_wrapper_restart` (`tests/cli_links.rs`)»
- fanout-test-plan--cascade-g1.md 18 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:236–236 · «the served `viola link` verb can drive step 1's `link`»
- fanout-test-plan--cascade-g1.md 19 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:271–271 · «`link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`)»
- fanout-test-plan--cascade-g1.md 20 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:273–273 · «its Linux and Windows case is the eight-verb server-identity negative»
- fanout-test-plan--cascade-g1.md 21 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:275–275 · «`link_verbs_against_a_stopped_instance_exit_21`»
- fanout-test-plan--cascade-g1.md 22 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:302–302 · «As landed: `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`»
- fanout-test-plan--cascade-g1.md 23 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:309–309 · «`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`»
- fanout-test-plan--cascade-g1.md 24 · already stands · .andromeda/test-plan-detail/e2e-test-strategy.md:310–310 · «runs each of the eight verbs»
- fanout-test-plan--cascade-g1.md 25 · already stands · .andromeda/test-plan-detail/test-data-fixtures.md:21–21 · «The link line's `since` is stamped outside the clock seam and asserted by shape only (§8).»
- cascade-compare-test-plan.md 2 · already stands · .andromeda/test-plan-detail/test-data-fixtures.md:17–17 · «`link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone`»
- fanout-test-plan--cascade-g1.md 26 · already stands · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:8–8 · «`LinkSet::line` stamps the link line's `since` with it directly (`src/run/links.rs:92`)»
- fanout-test-plan.md 5 · applied · .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#1

## Edits
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «as `.config/nextest.toml` reads at chunk 2026-10-26-explicit-session-links;»
  new: «as `.config/nextest.toml` reads;»

## Patterns
(none)
