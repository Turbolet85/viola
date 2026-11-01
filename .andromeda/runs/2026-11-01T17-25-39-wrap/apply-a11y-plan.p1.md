# Apply — a11y-plan · part 1 of 2

## Verdicts
- fanout-a11y-plan--cascade.md 18 · applied · .andromeda/a11y-plan.md:257#1
- fanout-a11y-plan--cascade.md 19 · applied · .andromeda/a11y-plan.md:261#1
- fanout-a11y-plan--cascade.md 20 · applied · .andromeda/a11y-plan.md:273#1
- fanout-a11y-plan--cascade.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:8#1
- fanout-a11y-plan--cascade.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:8#2
- fanout-a11y-plan--cascade.md 3 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:12#1
- fanout-a11y-plan--cascade.md 4 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:50+#1
- fanout-a11y-plan--cascade.md 5 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:71#1
- fanout-a11y-plan--cascade.md 6 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:85+#1
- fanout-a11y-plan--cascade.md 7 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:85+#2
- cascade-compare-a11y-plan.md 1 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:31+#1
- cascade-compare-a11y-plan.md 2 · applied · .andromeda/a11y-plan-detail/a11y-scope-summary.md:39+#1

## Edits
- line: .andromeda/a11y-plan.md:257
  old: «`viola revive`'s exit-1 refusals and `viola verify`'s interrupted pair»
  new: «`viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair»
- line: .andromeda/a11y-plan.md:261
  old: «the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid»
  new: «the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest»
- line: .andromeda/a11y-plan.md:273
  old: «the server-verify, `unwrapped-name` and `endpoint-lost` hints»
  new: «the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  old: «`squatted-name` · `internal-error`»
  new: «`squatted-name` · `socket-dir-refused` · `internal-error`»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:8
  old: «on macOS no peer pid is read and the refusal cannot occur (the recorded gap)»
  new: «on macOS no peer pid is read (the recorded gap, a named carve-out no route entry owns), so there the refusal means an euid mismatch alone»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:12
  old: «`batch-script-child`, `pinned-hash-mismatch` and `squatted-name` each print one exit-1 document in place of the human pair, before the child spawns;»
  new: «`batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused` each print one exit-1 document in place of the human pair, before the child spawns (`socket-dir-refused` before any snapshot or bind too);»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:50
  anchor: «    - under `viola revive --json` a `pump_child` error prints no document: the child has held the terminal, and viola writes nothing past it.»
  add: «    - a peer the wrapper refuses is dropped in `accept_loop`, before its worker thread and any frame: it writes nothing to the outer terminal and moves no wheel (`serve_drops_a_refused_peer_before_any_frame`, `channel_endpoint_answers_protocol_faults` the control).»
- line: .andromeda/a11y-plan-detail/a11y-scope-summary.md:71
  old: «, off macOS (no peer pid is read there and the check reads as passed: the recorded gap)»
  new: «, on every OS (on macOS no peer pid is read, the named carve-out of the CLI entity above, so there such a server is one of another euid alone)»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:85
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** the `viola hook` event and dialog calls against a socket directory that fails its check, on Unix»
  add: «  - **Source:** `src/cmd/hook.rs` (`connect`, `NotSent`, `Asked`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_hook_fails_open_on_the_socket_directory`).»
  add: «  - **Assertability:** not-assertable»
  add: «  - **Reason:** such a call sends no frame and fails open: exit 0, empty stdout and stderr, its one record a `hook-decision` at WARN with `detail` `socket-dir-refused`. It writes no human-facing byte, so the entity stays not-assertable and no human keystroke is touched.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:85
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`) against a socket directory that fails its check, on Unix»
  add: «  - **Source:** `crates/viola-mcp/src/tools.rs` (`frame`); `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_loosened_socket_directory`).»
  add: «  - **Assertability:** not-assertable»
  add: «  - **Reason:** a frame tool past its live endpoint returns `isError` with the code `instance-unreachable` and `detail` `socket-dir-refused`, sends no frame and writes no line, `isError` the record. It writes no human-facing byte, so the entity stays not-assertable and no human keystroke is touched.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:31
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` over a socket directory that fails its check (mode open to another user, a symlink, not a directory, another owner), on Unix, in the human TTY, plain and `--json` modes: each refuses before any frame with exit 21 and the socket-directory `hint:` line last on stderr, plain ASCII, no SGR, naming no path, pid or uid, or under `--json` one document with `error` `instance-unreachable` and `detail` `socket-dir-refused`; the directory is left as found; off Unix the check reads nothing (the refusal's text: Cognitive Accessibility › Error recovery)»
  add: «  - **Source:** `src/cmd/client.rs` (`connect`), `src/human/hints.rs` (`send_hint`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame`), `tests/cross_exit_causes.rs` (`a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused`), `tests/cli_error_bodies.rs` (`cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`).»
  add: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/a11y-plan-detail/a11y-scope-summary.md:39
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · scope-entities -->»
  add: «»
  add: «- **Entity:** `viola run` and a `viola revive` start over a socket directory that fails its check, on Unix: each refuses with exit 1 before any snapshot, bind or child, a failed create reading the same, with the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name, and revive under `--json` gives its one `socket-dir-refused` document (the CLI entity above); the directory is left as found (the pair's text: Cognitive Accessibility › Error recovery)»
  add: «  - **Source:** `src/cmd/run.rs` (`socket_dir_ready`, `refuse_socket_dir`), `src/human/hints.rs` (`send_hint_is_the_design_string`); `tests/security_negatives_socket_dir.rs` (`security_negative_a_start_refuses_the_socket_directory_before_any_bind`), `tests/cross_exit_causes.rs` (`a_loosened_socket_directory_on_a_start_is_socket_dir_refused`).»
  add: «  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "a11y1-01"
fixed = 'exit-1 refusals and `viola verify`'

[[pattern]]
id = "a11y1-02"
fixed = 'server-verify refusal on windows-2025 and ubuntu-latest only'

[[pattern]]
id = "a11y1-03"
fixed = 'the server-verify, `unwrapped-name` and `endpoint-lost` hints'

[[pattern]]
id = "a11y1-04"
fixed = '`squatted-name` · `internal-error`'

[[pattern]]
id = "a11y1-05"
fixed = 'the refusal cannot occur'

[[pattern]]
id = "a11y1-06"
fixed = 'and `squatted-name` each print one exit-1 document'

[[pattern]]
id = "a11y1-07"
fixed = 'the check reads as passed'
