# Apply — a11y-plan

## Verdicts
- fanout-a11y-plan--cascade-new-a11y-scope-summary.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:24+#1
- fanout-a11y-plan--cascade.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:8#1
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:19#1
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26#1
- fanout-a11y-plan--cascade.md 4 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:26+#1
- fanout-a11y-plan--cascade.md 5 · applied · .andromeda/a11y-plan-detail/cognitive-accessibility.md:35#1

## Edits
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:24
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** Claude Code plugin against an endpoint whose serving process is not the recorded wrapper: the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`), off macOS (no peer pid is read there and the check reads as passed: the recorded gap)»
  add: «  - **Source:** `src/cmd/hook.rs` (`connect`, `NotSent`) and `crates/viola-mcp/src/tools.rs` (`server_verified`); `tests/hook_fail_open.rs` (the impostor rows) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`).»
  add: «  - **Assertability:** not-assertable»
  add: «  - **Reason:** a hook event or dialog call that meets such a server sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s; an MCP frame tool returns `isError` with the code `instance-unreachable` only (text `refused: instance-unreachable`) and sends no frame. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  old: «`viola plugin` alone is a clap usage error (exit 2)»
  new: «`viola plugin` alone is a clap usage error (exit 2). `send`, `wait`, `last`, `answer`, `pause` and `release` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:19
  old: «its kill and pump join end at once.»
  new: «its kill and pump join end at once. CLI `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint.»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
  old: «chunk 2026-10-17-optional-plugin-install), and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs.»
  new: «chunk 2026-10-17-optional-plugin-install; the six frame verbs' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint, beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`), and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only.»
- after: .andromeda/a11y-plan-detail/cognitive-accessibility.md:26
  anchor: «  - **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal (`viola list`'s exit-21 state pairs — the hint last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty — are pinned by literal in the `cfg(unix)` root cases `list_refuses_*` and the unit case `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list; `viola plugin install`'s exit-21 refusal, the same home pair, and its `--json` document, by `plugin_install_refuses_a_widened_home`, chunk 2026-10-17-optional-plugin-install), and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs.»
  add: «  - **cli, macOS:** the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".»
- line: .andromeda/a11y-plan-detail/cognitive-accessibility.md:35
  old: «`write_plugin_installed_is_the_planning_form_in_one_write` (chunk 2026-10-17-optional-plugin-install).»
  new: «`write_plugin_installed_is_the_planning_form_in_one_write` (chunk 2026-10-17-optional-plugin-install), and the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`.»

## Patterns
[[pattern]]
id = "a11y-01"
fixed = 'on all three OS legs'

[[pattern]]
id = "a11y-02"
fixed = 'every fixed CLI string'
