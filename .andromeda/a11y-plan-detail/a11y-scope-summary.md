# a11y-plan · A11y Scope Summary
Read whole. The master's broad view is .andromeda/a11y-plan.md; this file holds its A11y Scope Summary detail.

## 1. A11y Scope Summary

<!-- detail: values · scope-entities -->

- **Entity:** CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes. `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract): under it each exit-1 refusal of its strict-modes check or its preflight is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`, and `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order (an empty chain the `no-session` document, exit 1); without `--json` `viola revive <name> --list` prints static ASCII rows (`<ts>  <cause>  <id>`, no header, no colour, no ESC byte) and each exit-1 refusal is one `unable:` line and one `hint:` line last on stderr, byte-identical; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to "Exit-cause code catalogue"); a clap usage error exits 2, and a passed start is the TUI passthrough entity below. A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only one `unable: verify was stopped before it finished` line and one `hint: run viola verify again to stamp this CLI` line last on stderr (plain ASCII), and exits 1 (chunk 2026-10-13-interrupted-verify-cleanup). `viola plugin install` (chunk 2026-10-17-optional-plugin-install) prints its four planning-form lines as one plain-ASCII write, no SGR, no ESC or non-ASCII byte piped, under `NO_COLOR` or under `TERM=dumb`; under `--json` exactly `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; its exit-21 refusal is the home pair with `hint:` last and stdout empty, or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; `viola plugin` alone is a clap usage error (exit 2). `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` each refuse, before any frame, an endpoint whose serving process is not the recorded wrapper: exit 21, stdout empty and the server-verify `hint:` line last on stderr (plain ASCII, no SGR), or under `--json` `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty (the refusal's text: Cognitive Accessibility › Error recovery); on macOS no peer pid is read and the refusal cannot occur (the recorded gap); their `--json` codes and exit codes are unchanged
  - **Source:** Arch excerpt, Stack → clap. Design excerpt, Surfaces → cli. Layout excerpt, Layout Types → cli. Creator brief excerpt, Overseer Direction 5.
  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
  - **Failures under `--json`:** each failure `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives is one document, a literal exit, empty stderr, no ESC byte and a clean body; the `viola answer` and `viola send` exit-2 paths write a fixed stderr line and no document (the text: Cognitive Accessibility › Error recovery).

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** `viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]`, the verb's instance the driven one, in the human TTY, plain and `--json` modes: each prints one plain-ASCII result line in one write with no SGR in any mode (the lines: Cognitive Accessibility › Plain language), or under `--json` one `{"v":1,"ok":<ok>}` document; each refuses with exit 21 a driven instance that fails the strict-modes check, before any log opens, a stopped driven instance and a server that is not the recorded wrapper (the entity above), and a pair the wrapper refuses as invalid params reaches it as a fault, exit 20, no hint (the refusals' text: Cognitive Accessibility › Error recovery)
  - **Source:** `src/cmd/link.rs` (`link_call`), `src/cmd/unlink.rs`, `src/human/verbs.rs` (`write_linked`, `write_already_linked`, `write_unlinked`); `tests/cli_links.rs`.
  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** `viola send`, `wait`, `last`, `answer`, `pause` and `release` against an instance that fails the strict-modes check, in the human TTY, plain and `--json` modes: each refuses before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`'s instance-state pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty; no line is written and the mode or ACL is left as found, on all three OSes (the refusal's text: Cognitive Accessibility › Error recovery)
  - **Source:** `src/cmd/client.rs` (`refuse_untrusted`), `src/cmd/{send,wait,last,answer,pause,release}.rs`; `tests/security_negatives_strict_modes.rs` (`frame_verb_refuses_a_widened_instance_before_any_log`).
  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** `viola run`, a `viola revive` start and `viola verify` over a home tree that fails the strict-modes check: each refuses before any log opens with exit 1, stdout empty and the home-state pair on stderr, `hint:` last, and starts no child — `run` and revive's start print the instance-state pair for an instance-level refusal, and revive under `--json` gives its `strict-modes-failed` document (the CLI entity above); `verify` prints no step line and no summary, and its `update_stamps` refusal gives the same home-state pair; on Unix a home under a regular file is this refusal, on Windows (such a path stats `NotFound`) the `cli` catch-site fault line stands (the refusals' text: Cognitive Accessibility › Error recovery)
  - **Source:** `src/cmd/run.rs` (`checked_exe`, `refuse_tree`), `src/cmd/revive.rs`, `src/cmd/verify.rs` (`refuse_home`); `tests/run_cli.rs` (`run_refuses_a_widened_home_tree_before_its_log`), `tests/cli_verify.rs` (`verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`), `tests/security_negatives_strict_modes.rs` (`a_home_another_user_may_read_refuses_run`).
  - **Assertability:** assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** `viola run` TUI passthrough: the unmodified `claude` TUI hosted in ConPTY/openpty, entered by `viola run` and by a `viola revive` whose preflight passed (the same start; the boundary below binds both verbs, and under revive the clause read by a case of its own is that a passed revive writes no line of its own while the child holds the terminal, `tests/chaos_revive.rs`)
  - **Source:** Arch excerpt, Stack → portable-pty + vt100 ("viola does not own the rendered TUI content"). Tests excerpt, E2E driver → tui ("Tests never parse the rendered child screen for content"). Creator brief excerpt, A11y Anti-Patterns ("Viola parses no screen for content (R7)").
  - **Assertability:** boundary-only
  - **Reason:** The rendered content belongs to the vendor (`claude`) and is not traced. The assertion boundary is viola's wrapper:
    - viola writes zero bytes to the terminal except the child's output;
    - human keystrokes are never blocked, refused or delayed past the current atomic paste;
    - focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as human editing; every other byte does. On Windows the platform decides what arrives (F-W3): the ConPTY host swallows focus reports, and under win32-input-mode an injected mouse report reaches viola as typed keys and takes the wheel.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** `viola run` TUI passthrough after a contained vt100 feed panic (the boundary of the entity above)
  - **Source:** `tests/chaos_feed_panic.rs` (`chaos_feed_panic_refuses_send_and_keeps_passthrough`).
  - **Assertability:** boundary-only
  - **Reason:** the driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, on all three OSes; on Unix the child's own bytes (`e4 b8 ad`) reach it.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Owed:** the Windows outer-terminal reading of the child's own bytes (`e4 b8 ad`) after a contained vt100 feed panic — ConPTY re-renders a wide character on a 1-column screen, so windows-2025 reads only viola's literal absence; owed to "Windows-only live measurements".

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** Claude Code plugin against an endpoint whose serving process is not the recorded wrapper: the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`), off macOS (no peer pid is read there and the check reads as passed: the recorded gap)
  - **Source:** `src/cmd/hook.rs` (`connect`, `NotSent`) and `crates/viola-mcp/src/tools.rs` (`server_verified`); `tests/hook_fail_open.rs` (the impostor rows) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`).
  - **Assertability:** not-assertable
  - **Reason:** a hook event or dialog call that meets such a server sends no frame and fails open: exit 0, empty stdout and stderr, under 1.0 s; an MCP frame tool returns `isError` with the code `instance-unreachable` only (text `refused: instance-unreachable`) and sends no frame. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.

<!-- /detail -->

<!-- detail: values · scope-entities -->

- **Entity:** Claude Code plugin against an instance that fails the strict-modes check: the `viola hook` event and dialog calls and the stdio MCP server's frame tools (`send`, `wait`, `last`, `answer`)
  - **Source:** `src/cmd/hook.rs` (`canonical`, `trusted`) and `crates/viola-mcp/src/tools.rs` (`frame`); `tests/hook_fail_open.rs` (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`) and `crates/viola-mcp/tests/handlers.rs` (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).
  - **Assertability:** not-assertable
  - **Reason:** a refused hook event or dialog call returns exit 0 at once with empty stdout and stderr, under 1.0 s — no `config.json` read, no stdin read, no frame, no line, no SessionEnd append — and a `VIOLA_DIR` naming no existing instance directory writes nothing; an MCP frame tool returns `isError` with the code `instance-unreachable` only (`detail: null`) and sends no frame and writes no line. Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Surface:** cli (`viola` verbs)
  - **Automated tool reach:**
    - No automated a11y verification tool exists for terminal output; a11y tools assume a DOM or native widget tree.
    - The fallback is output-discipline assertions on the tests' existing CLI drivers: `assert_cmd` 2.2.2 with `predicates` 3.1.4, `trycmd` 1.2.1, and SGR-only checks through the portable-pty outer PTY (tests excerpt, E2E driver → cli). Their pass/fail results are machine-parseable through the harness `run` JSON.
    - These are not a11y tools. They produce no a11y-tool JSON, so no WCAG conformance claim may rest on them.
  - **Manual verification (supplemental):**
    - NVDA with Windows Terminal/conhost (live target).
    - VoiceOver with Terminal.app.
    - Orca with GNOME Terminal.
    - Overseer Direction 5: "keyboard and screen-reader discipline, no fake automation".
  - **ARIA roles inventory:** N/A (terminal output). The structural equivalents are:
    - the BAY context line, then uppercase column captions, then the `-- UNWRAPPED - READ-ONLY --` separator (heading-equivalent); as built (chunk 2026-10-11-the-board-viola-list; the rack, chunk 2026-10-16-unwrapped-sessions-on-the-board) the board is the BAY line, the caption row and the wrapped rows (or the BAY line, the caption row and `no wrapped sessions  start one: viola run <name> -- claude`), then, only when the one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped read-only rows (NAME the row's `sessionId` through `escape_row`, `n/a` in WHEEL, DIALOG and CLI); no separator without an unwrapped row, and a failed, oversize or malformed read leaves the board as before;
    - `--help` group labels: board / traffic / wheel / handoff / setup (layout excerpt, Heading Hierarchy Anchors → cli).
  - **Service identity tagging:** `service_name:"viola"` and `version`. Obs uses `process:"cli"` for short-lived verbs that have a resolved instance (obs excerpt, Log Format additions). The CLI a11y checks run on all three OSes, so `os` is tagged per matrix leg.
  - **Notes:**
    - Required behaviour (design excerpt, Surfaces → cli; Loading/Error/Empty → CLI waiting; security excerpt, C0/C1 escaping):
      - colour is only a second cue (amber `DIALOG`, dim `stale`, always beside the word); as built (chunk 2026-10-17-cli-output-discipline) `viola list` on a terminal writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`;
      - there is no SGR, glyph or cursor control under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`; `viola revive --list` rows and the revive refusal pairs carry no ESC byte, and each `--json` failure document `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) drives carries none;
      - output is static and linear, with no spinner or redraw;
      - C0/C1 control characters are escaped except `\n` and `\t` [resolved: in `list` table rows `\n` / `\t` are escaped too (ratified T5, design cli); `wait` / `last` keep them; Section 4 P6].
    - This must hold on `[windows-2025, macos-latest, ubuntu-latest]` (tests excerpt, Coverage Triggers → multi-platform).

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Surface:** cli, `viola link`'s result line — a `since` that is no RFC 3339 instant (the restore's `unknown`, or a value from a hand-altered log) prints through `escape_message`, which keeps `\n`, `\t` and non-ASCII; the wrapper's own `since` is a link line's `ts` or `unknown`, so such a byte reaches the line only from a hand-altered log.

<!-- /detail -->

<!-- detail: values · scope-surfaces -->

- **Surface:** tui (`viola run` passthrough, entered by a passed `viola revive` too; boundary-only)
  - **Automated tool reach:**
    - No automated a11y verification tool applies. The rendered `claude` TUI is vendor content and parsing it is banned (R7).
    - Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
  - **Manual verification (supplemental):** NVDA with Windows Terminal (Windows is the live target), confirming the wrapped TUI reads the same as unwrapped. Creator brief excerpt, Must-Work → O1: "all rendered as without the wrapper".
  - **ARIA roles inventory:** N/A (terminal).
  - **Service identity tagging:** `service_name:"viola"`, `process:"run"`, `instance:<ViolaName>`.
  - **Notes:** viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline). This part of the scope is a boundary guarantee only, not a conformance claim about `claude`'s TUI.

<!-- /detail -->

<!-- detail: values · path-p6-board -->

- **Required ARIA roles:** N/A (terminal). The structural equivalents are the caption line, the `-- UNWRAPPED - READ-ONLY --` separator and the `--help` group labels. As built the caption line and the `-- UNWRAPPED - READ-ONLY --` separator stand (the separator since chunk 2026-10-16-unwrapped-sessions-on-the-board, printed only when unwrapped rows exist); the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / handoff / setup over the served verbs, the `handoff:` row `handoff:  link, unlink` between `wheel:` and `setup:`).

<!-- /detail -->
