# a11y-plan · Cognitive Accessibility
Read whole. The master's broad view is .andromeda/a11y-plan.md; this file holds its Cognitive Accessibility detail.

## 8. Cognitive Accessibility › ### Timeout extensions

<!-- detail: values · cognitive-timeouts -->

- **CLI:** `viola wait` returns `{timed_out:true}` only for a timeout the caller passes in. The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP` 50 000 ms, which is also its default when the caller passes none (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s; that bound falls on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person): no key is blocked, refused or delayed by it, and the wheel is untouched. A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, read once at the wrapper's start into `DialogHold`, its bounds `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX`, each PROVISIONAL; any other value gives 60 and one `parse-rejected`; under the `hooks.json` dialog `timeout` of 3615 s and the hook's own bound `DIALOG_HOLD_MAX + 5 s`, 3605 s; chunk 2026-10-19-dialog-hold-for-the-driver); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`) that wakes a parked `wait`, the hook exits 0 with no body, the dialog renders for the human in the `claude` TUI, and a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13) — the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. A `viola send` on a verified CLI waits in the readiness gate for the input box for at most `GATE_MAX_WAIT` (8.5 s, architecture [Screen Model]) and is then refused `not-delivered` / `input-not-ready` with nothing typed. That bound falls on the driver only as well: a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is then refused `human-typing`.
 Once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL, architecture [Delivery Confirmation]); a send not confirmed inside its window is `not-delivered` / `no-prompt-submitted`. That bound falls on the driver only too: the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. `viola hook statusline` runs the user's own status line command under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`) and exits 0 with empty stderr in every case. That bound falls on the user's own command, never on a keystroke and never on the human: no key is blocked, refused or delayed by it, and the wheel is untouched. `viola list` and `viola revive` each make one `claude agents --json` read (a direct spawn, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; chunk 2026-10-16-unwrapped-sessions-on-the-board): a failed, oversize, malformed or timed-out read leaves `list`'s board as `board::read` gave it and passes `revive`'s preflight (fail open). That bound falls on viola's own child only, never on a keystroke or the human: no key is blocked, refused or delayed by it. A budget pause is a bound on the driver only as well (chunk 2026-10-10-budget-governor): while a five-hour or seven-day window stands at or over its threshold (`config.json` `five_hour`, default 90, and `seven_day`, default 85; no value of either switches the gate off) with a known reset still ahead, an automated `viola send` is refused `budget-paused` with detail `five-hour` or `seven-day` (exit 11) and nothing is typed, until that window's reset or a `viola release <name> --budget` for that instance.
 No key is blocked, refused or delayed by it: a key written while the gate is closed reaches the child, and the outer PTY carries none of viola's own literals (`path6_a_reading_over_the_threshold_refuses_the_send`, `tests/cli_budget.rs`). `answer`, `wait`, `last` and `pause` do not read the gate; that clause is read in the diff (the gate's five product call sites) and, since chunk 2026-10-10-session-links, held by one case: `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) closes the gate, reads the automated `send` refused exit 11 `budget-paused` / `five-hour` as its control, and then reads from `wait`, `last`, `answer` and `pause` the exit code and the document each gives under an open gate, with one `send-refused` record in the log (green on the three CI runners, run 38098238731). There is no conformance claim for the terminal.

<!-- /detail -->

## 8. Cognitive Accessibility › ### Error recovery

<!-- detail: values · cognitive-errors -->

- **Pattern:** every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
  - **CLI:** `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is a code word or nothing, opaque to a reader, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (below). Without `--json`, the exit-1 refusals of `viola revive` take the fixed-message form of design-system cli pattern 2 (one `unable: <text>` line, then one `hint:` line last; a clap usage error exits 2), pinned by the root cases in `tests/cli_revive.rs`; under `--json` its strict-modes and preflight refusals give one `{"v":1,"error":"<code>","detail":null}` document with no hint and empty stderr (`revive_json_*` in `tests/cli_revive.rs`; chunk 2026-10-17-cli-machine-contract). So does `viola verify`'s interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`), pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound, since its waits end at the stop and its kill and pump join end at once. CLI `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` refuse an endpoint whose serving process is not the recorded wrapper with exit 21: `unable  <name>  instance-unreachable` on stderr, then `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` as the last stderr line; under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document with stderr empty and no hint.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `viola link` / `viola unlink`, strict-modes:** a driven instance that fails the strict-modes check is refused with exit 21 before any log opens, no log line written: the fixed instance-state `unable` / `hint:` pair on stderr, `hint:` last; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `send` / `wait` / `last` / `answer` / `pause` / `release`, strict-modes:** an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no log line written: the same fixed instance-state `unable` / `hint:` pair as `viola link` / `viola unlink` on stderr, `hint:` last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty and no hint.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `viola run` / `viola revive` start, strict-modes:** a home tree that fails the strict-modes check (the home, the instance's files, `ledger/`, `bin/`, `plugin/`, the pinned copy and its plugin folders) is refused with exit 1 before any log opens, no line written and no child started: the fixed home-state `unable` / `hint:` pair for a home-level refusal, the instance-state pair for an instance-level one, `hint:` last on stderr, stdout empty; under `--json`, `revive` gives its `strict-modes-failed` document (above).

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `viola verify`, strict-modes:** a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check is refused with exit 1 before its log opens, with no step line, no summary and no line written: the fixed home-state `unable` / `hint:` pair on stderr, `hint:` last; a `ledger/` that `update_stamps`'s own check refuses gives the same pair.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, an unstattable home:** a home path under a regular file is a strict-modes refusal with its hint on Unix (`verify`: the home-state pair, exit 1; the six frame verbs: the instance-state pair, exit 21); on Windows such a path stats `NotFound`, the check skips it as absent, and the `cli` catch site's fault line stands, with no hint.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `viola link` / `viola unlink`, stopped instance:** a driven instance with no live endpoint is refused with exit 21, the not-running `hint:` as the last stderr line.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, `viola link` / `viola unlink`, wrapper refusal:** a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, a peer's refusal:** `reply_of` and `of_reply` screen a peer's reply with `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first): a `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads nothing (null under `--json` and on MCP).

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, a word outside the list:** a code word outside the closed refusal list is visible by design, in the document and in `refused: <word>`, and takes `unknown`'s exit 14.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, a fault's body:** under `--json` and on MCP a fault's body is read through `viola_channel::sanitised_fault`: a code outside the five known reads the fixed `message` `wrapper fault` with null `data`. The human line is `error: wrapper fault  <code>`.

<!-- /detail -->

<!-- detail: values · cognitive-errors -->

- **CLI, the exception to one document:** the `viola answer` and `viola send` exit-2 paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document; `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification:**
  - **cli:** trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal (`viola list`'s exit-21 state pairs — the hint last, stdout empty; under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty — are pinned by literal in the `cfg(unix)` root cases `list_refuses_*` and the unit case `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list; `viola plugin install`'s exit-21 refusal, the same home pair, and its `--json` document, by `plugin_install_refuses_a_widened_home`, chunk 2026-10-17-optional-plugin-install; the eight frame verbs' exit-21 server-verify refusal, by the `HINT` literal in `tests/security_negatives_server_identity.rs`, whose `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` reads, over `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` in both modes, the hint as the last stderr line with no ESC byte and stdout empty, or under `--json` the exact document with stderr empty, and no output holding the recorded pid, the home's path or the endpoint, beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`), and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only.
  - **cli, macOS:** the server-verify refusal cannot occur: no peer pid is read and the check reads as passed (the recorded gap), so its cases compile out there; its macOS witness is owed to "Unix endpoint and home hardening".

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification, `viola link` / `viola unlink`:** in `tests/cli_links.rs`, `link_verbs_against_a_stopped_instance_exit_21` reads the stopped-instance refusal (exit 21, the not-running hint last), and `link_verbs_refuse_a_widened_instance_before_any_log` the strict-modes refusal (a group-writable driven instance refused before any log, its mode read back as found; `cfg(unix)`, so on ubuntu-latest and macos-latest only).

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification, strict-modes refusals:** `frame_verb_refuses_a_widened_instance_before_any_log` (`tests/security_negatives_strict_modes.rs`) reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes against a live wrapper whose instance directory another user may write, exit 21 with the `INSTANCE_PAIR` literal on stderr and stdout empty, or under `--json` the exact `instance-unreachable` document with stderr empty, no line added and no frame sent, the widening left as found; `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`) reads `run`'s exit 1 with the `HOME_PAIR` or `INSTANCE_PAIR` literal, stdout empty, no line and no child; `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`, the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows) read `verify`'s home-state pair with no step line or summary. Unlike the server-verify cases, these run on all three OS legs.

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification, `--json` failure documents:** `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds each `--json` failure it drives to one document, a literal exit, empty stderr, no ESC byte and a clean body.

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification, the census of error writers:** in `tests/contract_error_bodies.rs`, `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each error body to a writer and a witness that stand, `product_error_writers_are_each_named` holds each product error writer to a name, and `thiserror_messages_interpolate_nothing` holds each `thiserror` message to no interpolation.

<!-- /detail -->

<!-- detail: procedure · cognitive-errors -->

- **Verification, the body check:** `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.

<!-- /detail -->

## 8. Cognitive Accessibility › ### Plain language

<!-- detail: procedure · cognitive-plain-language -->

- **Verification:**
  - trycmd pins every fixed CLI string, except `viola list`'s, which literals pin (the board, the empty line and the refusal pairs: `tests/cli_list.rs` and the `src/human/board/` units `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts`, chunk 2026-10-11-the-board-viola-list), and `viola --help`'s, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` with the served set held by `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`, chunk 2026-10-17-cli-output-discipline), and `viola plugin install`'s planning-form lines, pinned by literal in `plugin_install_prints_the_two_commands` (`tests/cli_plugin_install.rs`) and the unit `write_plugin_installed_is_the_planning_form_in_one_write` (chunk 2026-10-17-optional-plugin-install), and the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`, and the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`). Any copy change therefore fails CI until the pinned string is updated on purpose.
  - The `message` literal `wrapper fault` under `--json` and on MCP, for a code outside the five known, is pinned by the unit case `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` (`crates/viola-channel/src/lib.rs`).

<!-- /detail -->

<!-- detail: procedure · cognitive-plain-language -->

- **Verification, `viola link` / `viola unlink`:** their three result lines (`write_linked`, `write_already_linked`, `write_unlinked`; the lines' text: design-system §Surface: cli), each one write with no SGR in any mode, are pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`).

<!-- /detail -->

<!-- detail: procedure · cognitive-plain-language -->

- **`viola link`'s time and names:** the `link` line's time is the reply's `since` as UTC clock time in whole seconds with `Z`, pinned by the unit `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`); the names each line prints are the verb's own parsed arguments, never names read back from the reply or the log.

<!-- /detail -->

## 8. Cognitive Accessibility › ### Consistency and predictability

<!-- detail: procedure · cognitive-consistency -->

- **Consistent identification (SC 3.2.4, AA):**
  - The `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) pins the caption order `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI` for `viola list` (chunk 2026-10-11-the-board-viola-list).

<!-- /detail -->
