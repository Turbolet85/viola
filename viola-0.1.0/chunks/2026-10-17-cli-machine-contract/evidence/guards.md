# Remove-the-guard readings (testing.md 2026-09-25)

Each new guard neutralised in place, its witness run, the guard restored and its restore read back before the next
step. Part A's guard (`count_of`'s non-empty capture) is recorded in `ci-red.md` §5.

| guard | neutralised as | witness | reading with the guard off | restore read back |
|---|---|---|---|---|
| the completeness table (`tests/cli_controls_not_disableable.rs` `SETTINGS`) | the `VIOLA_BIN` row renamed `VIOLA_BIN_GONE` (line 399, re-read after rustfmt reflowed the tuple; a first `sed` anchored on the one-line form matched nothing and its green run proved nothing) | `every_flag_variable_and_config_key_is_a_row_of_the_table` | **red** — `panicked at tests/cli_controls_not_disableable.rs:491:9: VIOLA_BIN is in no row` | line 399 reads `"VIOLA_BIN",` |
| the `VIOLA_DIR` step (`src/cmd/mod.rs` `resolve_home`) | `instance_of(name, dir)` → `instance_of(name, dir).filter(\|_\| false)` (line 233) | `resolve_home_` units; `cli_machine_contract::home_resolution_takes_the_flag_then_the_session_then_the_default` | **red** — unit `resolve_home_takes_the_flag_then_the_session_then_the_default::case_3_a_wrapped_session_beats_the_default` (1 of 10); integration `panicked at tests/cli_machine_contract.rs:488:5: assertion left == right failed: the session's home` | `filter(\|_\| false)` count 0 |
| canonicalisation (`src/cmd/mod.rs` `canonical_home`) | the canonical path dropped for `std::path::absolute(home)` (`.and_then(\|_\| std::path::absolute(home))`) | `cli_machine_contract::two_homes_are_isolated`, `::a_home_reached_through_a_link_is_canonicalised`, `cli_instance_state::run_rewrites_the_settings_override_each_start`, `::run_in_the_default_home_reads_the_user_settings` | **red, 4 of 4** — `cli_instance_state.rs:337:5`, `cli_instance_state.rs:420:5`, `cli_machine_contract.rs:575:5`, `cli_machine_contract.rs:524:5` (each a `pinned_bin` or endpoint under the `target/e2e-home` link spelling, not its target) | `and_then(\|_\| std::path::absolute(home))` count 0 |

The cases that assert an absence (a refused check writes no line, `--list` with no name opens no log, a default
home not made) are not guards removed here: they hold on the tree before the chunk and after it.

Note on the host: the canonical readings are red here because this host's `target/e2e-home` is a link to an
owner-only tmpfs directory. On a CI runner `target/e2e-home` is a plain directory, so only the link case
(`a_home_reached_through_a_link_is_canonicalised`, Unix) separates the guard there.
