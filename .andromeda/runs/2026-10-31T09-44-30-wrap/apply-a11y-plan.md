# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan.md:257#1
- fanout-a11y-plan--cascade.md 4 · applied · .andromeda/a11y-plan.md:257#2
- fanout-a11y-plan--cascade.md 5 · applied · .andromeda/a11y-plan.md:345#1
- fanout-a11y-plan--cascade.md 1 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#1
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#2
- fanout-a11y-plan--cascade.md 6 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:63+#1
- fanout-a11y-plan--cascade.md 7 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:63+#2
- fanout-a11y-plan--cascade.md 8 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:63+#3
- fanout-a11y-plan--cascade.md 9 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:63+#4
- fanout-a11y-plan--cascade.md 10 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:83+#1
- fanout-a11y-plan--cascade.md 11 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:83+#2
- fanout-a11y-plan--cascade.md 12 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:83+#3
- fanout-a11y-plan--cascade.md 13 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:90+#1
- fanout-a11y-plan--cascade.md 14 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:10+#1
- fanout-a11y-plan--cascade.md 15 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:102#1

## Edits
- line: .andromeda/a11y-plan.md:257
  old: «whose detail is opaque»
  new: «whose detail is a code word or nothing, opaque to a reader»
- line: .andromeda/a11y-plan.md:257
  old: «`--json` gives one document with no hint.»
  new: «`--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document).»
- line: .andromeda/a11y-plan.md:345
  old: «`unknown` (exit 14, opaque detail, design cli)»
  new: «`unknown` (exit 14, its detail a code word or nothing, design cli)»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «its detail is opaque, so design prints no hint rather than a guess»
  new: «its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «`--json` gives one document with no hint.»
  new: «`--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (below).»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:63
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a peer's refusal:** `reply_of` and `of_reply` screen a peer's reply with `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads nothing (null under `--json` and on MCP).»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:63
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a word outside the list:** a code word outside the closed refusal list is visible by design, in the document and in `refused: <word>`, and takes `unknown`'s exit 14.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:63
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, a fault's body:** under `--json` and on MCP a fault's body is read through `viola_channel::sanitised_fault`: a code outside the five known reads the fixed `message` `wrapper fault` with null `data`. The human line is `error: wrapper fault  <code>`.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:63
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · cognitive-errors -->»
  add: «»
  add: «- **CLI, the exception to one document:** the `viola answer` and `viola send` exit-2 paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document; `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:83
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, `--json` failure documents:** `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds each `--json` failure it drives to one document, a literal exit, empty stderr, no ESC byte and a clean body.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:83
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, the census of error writers:** in `tests/contract_error_bodies.rs`, `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each error body to a writer and a witness that stand, `product_error_writers_are_each_named` holds each product error writer to a name, and `thiserror_messages_interpolate_nothing` holds each `thiserror` message to no interpolation.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:83
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: procedure · cognitive-errors -->»
  add: «»
  add: «- **Verification, the body check:** `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:90
  anchor: «  - trycmd pins every fixed CLI string, except `viola list`'s, which literals pin (the board, the empty line and the refusal pairs: `tests/cli_list.rs` and the `src/human/board/` units `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list), and `viola --help`'s, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` with the served set held by `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`, chunk 2026-10-17-cli-output-discipline), and `viola plugin install`'s planning-form lines, pinned by literal in `plugin_install_prints_the_two_commands` (`tests/cli_plugin_install.rs`) and the unit `write_plugin_installed_is_the_planning_form_in_one_write` (chunk 2026-10-17-optional-plugin-install), and the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`, and the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`). Any copy change therefore fails CI until the pinned string is updated on purpose.»
  add: «  - The `message` literal `wrapper fault` under `--json` and on MCP, for a code outside the five known, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`).»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:10
  anchor: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «  - **Failures under `--json`:** each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC byte and a clean body; the `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document (the text: Cognitive Accessibility › Error recovery).»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:102
  old: «the revive refusal pairs carry no ESC byte;»
  new: «the revive refusal pairs carry no ESC byte, and each `--json` failure document `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives carries none;»

## Patterns
[[pattern]]
id = "a11y-01"
fixed = 'detail is opaque'

[[pattern]]
id = "a11y-02"
fixed = 'opaque detail'

[[pattern]]
id = "a11y-03"
fixed = 'gives one document with no hint'
