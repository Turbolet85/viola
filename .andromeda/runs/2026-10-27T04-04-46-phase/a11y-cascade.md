# a11y · cascade read

## Carried
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) Every CLI verb the chunk re-verifies or bounds (`send`'s client-side paste refusal, `revive`'s `--id` closed-shape check, `list`'s closed-type board) must keep its human, plain and `--json` output discipline, `revive`'s refusals staying the closed document under `--json` and the `unable:` / `hint:` pair without it.
  > CLI verbs (`list`, `send`, `wait`, `last`, `answer`, `verify`, `pause`, `release`, `link`, `unlink`, `revive`, `plugin install`, `--help`) in human TTY, plain (non-TTY / `NO_COLOR` / `TERM=dumb`) and `--json` modes. `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract): under it each exit-1 refusal of its strict-modes check or its preflight is one document `{"v":1,"error":"<code>","detail":null}` on stdout with empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8)
  > each exit-1 refusal is one `unable:` line and one `hint:` line last on stderr, byte-identical
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:10)
  > assertable. This covers output-discipline assertions on text and streams only. No automated a11y tool reaches a terminal.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:24) The cap the chunk puts on `send`'s `--file` / stdin text read must sit ahead of the strict-modes refusal without displacing it: `send` reads its text first, then refuses a widened instance with exit 21 and the fixed pair before any frame or log, and its witness must keep passing over `send` in both modes.
  > each refuses before any frame or log (`send` after its text read) with exit 21, stdout empty and `link`'s instance-state pair on stderr, `hint:` last, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:31)
  > an instance that fails the strict-modes check is refused with exit 21 before any frame or log (`send` after its text read), no log line written
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:81)
  > `frame_verb_refuses_a_widened_instance_before_any_log` (`tests/security_negatives_strict_modes.rs`) reads, over `send`, `wait`, `last`, `answer`, `pause` and `release` in both modes against a live wrapper whose instance directory another user may write, exit 21 with the `INSTANCE_PAIR` literal on stderr and stdout empty, or under `--json` the exact `instance-unreachable` document with stderr empty, no line added and no frame sent, the widening left as found;
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:43) The re-verified screen-model panic degrade and the new cap on the child's PTY output into the vt100 feed must leave the passthrough whole: viola writes no byte of its own to the terminal (no panic text, no escape sequence), the cap bounds only the screen-model feed, and no keystroke is blocked, refused or delayed past the current atomic paste.
  > The rendered content belongs to the vendor (`claude`) and is not traced. The assertion boundary is viola's wrapper:
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:44)
  > viola writes zero bytes to the terminal except the child's output;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:45)
  > human keystrokes are never blocked, refused or delayed past the current atomic paste;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:46)
  > focus, mouse and resize sequences and terminal replies (F-W2's closed list) pass through and do not count as human editing; every other byte does.
  (.andromeda/a11y-plan.md:33)
  > viola writes zero bytes to the terminal except the child's output, human keystrokes are never blocked, refused or delayed past the current atomic paste, and focus, mouse and resize sequences and terminal replies pass through and do not count as human editing.
  (.andromeda/a11y-plan.md:340)
  > NEVER colour CLI `unable` / `fail`, and never emit SGR under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json` / `viola run` / a passed `viola revive`.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:64) The cap on the hook's stdin must stay behind the instance check, so a refused instance still reads no stdin, and every newly bounded or refused hook, MCP or channel input must stay fail-open with no human-facing byte, its caps proven by unit witnesses rather than output-discipline assertions, as for the internal crates' validators and readers.
  > a refused hook event or dialog call returns exit 0 at once with empty stdout and stderr, under 1.0 s — no `config.json` read, no stdin read, no frame, no line, no SessionEnd append — and a `VIOLA_DIR` naming no existing instance directory writes nothing;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:64)
  > Neither writes a human-facing byte, so the entity stays not-assertable and no human keystroke is touched.
  (.andromeda/a11y-plan.md:37)
  > **Entity:** Claude Code plugin: hooks (`viola hook`, `hook statusline`) and the stdio MCP server; wrapper IPC channel
  (.andromeda/a11y-plan.md:40)
  > ipc-internal with no UI surface. Machine-to-machine traffic only.
  (.andromeda/a11y-plan.md:42)
  > **Entity:** Internal crates (modular monolith)
  (.andromeda/a11y-plan.md:45)
  > Library-only with no UI surface.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:81) The capped, closed-type `claude agents --json` read must leave the board as before on a failed, oversize or malformed read (and on an unknown status value it skips), and every row built from it must keep its C0/C1 escaping, `\n` / `\t` escaped in `list` rows.
  > then, only when the one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped read-only rows (NAME the row's `sessionId` through `escape_row`, `n/a` in WHEEL, DIALOG and CLI); no separator without an unwrapped row, and a failed, oversize or malformed read leaves the board as before;
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:89)
  > C0/C1 control characters are escaped except `\n` and `\t` [resolved: in `list` table rows `\n` / `\t` are escaped too (ratified T5, design cli); `wait` / `last` keep them; Section 4 P6].
  (.andromeda/a11y-plan.md:53)
  > C0/C1 control characters are escaped except `\n` and `\t`, which `list` rows escape too.
- (.andromeda/a11y-plan-detail/a11y-scope-summary.md:105) The re-verified panic degrade must be witnessed on the outer-PTY driver as zero terminal bytes of viola's own with keystrokes unblocked, its diagnostics going to `diagnostics/` only, and the witness must read passthrough and readiness, never the rendered screen.
  > Boundary assertions run on the tests' portable-pty `=0.8.1` outer-PTY driver (tests excerpt, E2E driver → tui). They check that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed.
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:104)
  > The rendered `claude` TUI is vendor content and parsing it is banned (R7).
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:109)
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal (arch excerpt, A11y-Relevant Conventions → terminal output discipline).
  (.andromeda/a11y-plan.md:55)
  > viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; a boundary guarantee only, no conformance claim about `claude`'s TUI.
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19) The P4 outer-PTY cases must keep passing with the vt100 feed capped and the input path bounded: zero viola-originated bytes, keystrokes delivered past the current atomic paste, and focus, mouse and resize sequences leaving the wheel unmoved.
  > three portable-pty outer-PTY nextest cases on all three OS legs, one per Section 1 boundary clause (Section 3 → Keyboard test harness → Tooling): (1) zero viola-originated bytes on the outer-PTY stream (byte for byte against an unwrapped fake-agent run on Linux/macOS; viola's own literals absent on Windows ConPTY); (2) human keystrokes delivered unblocked past the current atomic paste; (3) `\x1b[I`/`\x1b[O`, mouse and resize sequences do not move the wheel
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:20)
  > 2.1.1 (boundary stand-in on tui).
  (.andromeda/a11y-plan.md:196)
  > on the tui, outer-PTY nextest cases on all three OS legs — zero viola-originated bytes, keystrokes delivered unblocked past the current atomic paste, and focus, mouse and resize sequences not moving the wheel on Linux and macOS
- (.andromeda/a11y-plan-detail/aria-patterns-roles.md:32) `list` rows built from the agents read parsed as closed types must still print every injected control only in escaped form, each row one fixed-width line, held by the existing assert_cmd and unit witnesses on all three OS legs.
  > C0/C1 escaped (security excerpt, C0/C1 escaping; ratified T5 for `list` rows), checked with assert_cmd on all three OS legs:
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:33)
  > As built (chunk 2026-10-16-unwrapped-sessions-on-the-board) `viola list` makes one `claude agents --json` read and prints every unwrapped NAME (the row's `sessionId`, never a `ViolaName`) through `escape_row` on one line
  (.andromeda/a11y-plan-detail/aria-patterns-roles.md:34)
  > In `list` table rows `\n` / `\t` are escaped too (`\x0A`, `\x09`), so every row stays one fixed-width line.
- (.andromeda/a11y-plan-detail/visual-design-verification.md:8) An unwrapped row from the closed-type agents parse must keep its NAME cut only on a terminal too narrow and never inside a `\xHH` escape, and its STATUS word carried in text, as the scope names.
  > each state is still carried by its printed word (`stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`, the STATUS words `idle` / `busy`, the unwrapped rack's `n/a` cells)
  (.andromeda/a11y-plan-detail/visual-design-verification.md:8)
  > On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus `...`, never inside a `\xHH` escape (the width from `viola_pty::host_size()`); a wrapped NAME is never cut, and piped output and `--json` keep every name whole.
  (.andromeda/a11y-plan-detail/visual-design-verification.md:9)
  > `board_row_width_*` units and `cross_list_rows_style_each_span_beside_its_word_on_a_terminal`, `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json` and `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it`
  (.andromeda/a11y-plan.md:221)
  > an unwrapped NAME is cut with `...` only on a terminal too narrow, never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` never cut.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9) The agents read the chunk re-asserts must keep its `take(MAX_FRAME + 1)` form and its fail-open on a failed, oversize, malformed or timed-out read, and the config values it re-verifies as validated newtypes must keep taking the default with one `parse-rejected` line, no value switching a bound off.
  > `viola list` and `viola revive` each make one `claude agents --json` read (a direct spawn, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; chunk 2026-10-16-unwrapped-sessions-on-the-board): a failed, oversize, malformed or timed-out read leaves `list`'s board as `board::read` gave it and passes `revive`'s preflight (fail open). That bound falls on viola's own child only, never on a keystroke or the human: no key is blocked, refused or delayed by it.
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:8)
  > `config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, read once at the wrapper's start into `DialogHold`, its bounds `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX`, each PROVISIONAL; any other value gives 60 and one `parse-rejected`
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9)
  > (`config.json` `five_hour`, default 90, and `seven_day`, default 85; no value of either switches the gate off)
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19) The re-verified `send` paste refusal (`not-delivered` / `control-character`, `not-delivered` / `empty-text`, exit 13), any refusal a newly capped CLI input prints, and `revive`'s `--id` refusals must take this layout, a fault carrying no hint, with every kebab-case code beside the plain word `unable`.
  > `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code. The one refusal with no hint is `unknown` (exit 14): its detail is opaque, so design prints no hint rather than a guess. Faults (`error: wrapper fault`, `error: internal error`) are not refusals and carry no hint. `--json` gives one document with no hint. Without `--json`, the exit-1 refusals of `viola revive` take the fixed-message form of design-system cli pattern 2 (one `unable: <text>` line, then one `hint:` line last; a clap usage error exits 2), pinned by the root cases in `tests/cli_revive.rs`;
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:18)
  > every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
  (.andromeda/a11y-plan.md:270)
  > Every kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
  (.andromeda/a11y-plan.md:271)
  > The CLI never shows a stack trace or bare status number as the only error text.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:61) A wrapper-side name check on the channel `driver` / `driven` the scope names must refuse as invalid params, reaching `link` / `unlink` as a fault (exit 20, no hint), never turned into a client-side refusal of that pair.
  > a pair the wrapper refuses as invalid params (`-32602`, e.g. `driver` equal to `driven`) reaches the verb as a fault, exit 20, no hint; the verbs never refuse it client-side.
- (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68) The re-verified `send` paste refusal must be witnessed by these checks: trycmd for its `unable` + final `hint:` layout and assert_cmd for exit 13 and a hint-free `--json`, on all three OS legs.
  > trycmd cases pin the `unable <reason> <detail>` + final `hint:` layout per refusal
  (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68)
  > and assert_cmd checks the typed exit codes (`10`–`14`, `20`, `21`) and that `--json` carries no hint text. These run on all three OS legs
  (.andromeda/a11y-plan.md:261)
  > trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs
- (.andromeda/a11y-plan.md:334) No new size cap, closed-type check or paste validation may block, refuse or delay a human keystroke past the current atomic paste, nor count focus, mouse or resize sequences or terminal replies as editing: the paste rule and its refusals fall on automation (`send`) only.
  > NEVER block, refuse or delay a human keystroke in `viola run`, or in a start a passed `viola revive` made, past the current atomic paste, and never count focus / mouse / resize sequences or terminal replies as editing (P4)
  (.andromeda/a11y-plan.md:209)
  > **tui:** the terminal belongs to the child. Human keystrokes are never blocked or delayed past the atomic paste.
- (.andromeda/a11y-plan.md:346) Any code a newly bounded reader's refusal prints on the CLI must sit beside the plain word `unable`, never as the only error text.
  > a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
  (.andromeda/a11y-plan.md:347)
  > NEVER render a stack trace as the only error text in the CLI.
- (.andromeda/a11y-plan.md:374) The panic-degrade witness and the vt100 feed cap's witness must read readiness and the passthrough bytes only, never the rendered `claude` screen's content.
  > NEVER parse the rendered `claude` TUI screen for content or take screenshots as a11y evidence (R7; tests "no screen-content parsing").

## Bindings
- **scope-entities (TUI passthrough) ↔ path-p4-wheel**: the boundary the panic degrade and the vt100 feed cap must keep (zero viola bytes, keystrokes unblocked, focus / mouse / resize off the wheel) is witnessed by the three P4 outer-PTY cases, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:44) and (.andromeda/a11y-plan-detail/aria-patterns-roles.md:19), with the keyboard ban beside them (.andromeda/a11y-plan.md:334).
- **scope-surfaces (tui) ↔ the body's screen-parsing ban**: the outer-PTY witness of the degrade reads bytes and readiness, never screen content, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:105) and (.andromeda/a11y-plan.md:374).
- **scope-entities (CLI verbs) ↔ cognitive-errors**: `send`'s re-verified paste refusal and `revive`'s `--id` refusals print the `unable` / `hint:` layout and the closed `--json` document, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:8) and (.andromeda/a11y-plan-detail/cognitive-accessibility.md:19), witnessed by trycmd / assert_cmd (.andromeda/a11y-plan-detail/cognitive-accessibility.md:68).
- **scope-entities (strict-modes frame verbs) ↔ cognitive-errors**: the order `send` reads its text before the strict-modes refusal, which the new text cap must keep, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:24) and (.andromeda/a11y-plan-detail/cognitive-accessibility.md:31), its witness (.andromeda/a11y-plan-detail/cognitive-accessibility.md:81).
- **scope-surfaces (cli board) ↔ path-p6-board ↔ visual-sgr ↔ cognitive-timeouts**: the one capped, closed-type `claude agents --json` read feeds the board, which stays as before on a bad read (.andromeda/a11y-plan-detail/a11y-scope-summary.md:81) and (.andromeda/a11y-plan-detail/cognitive-accessibility.md:9), escapes every control in its rows (.andromeda/a11y-plan-detail/aria-patterns-roles.md:34), and cuts an unwrapped NAME outside any `\xHH` escape with the STATUS word in text (.andromeda/a11y-plan-detail/visual-design-verification.md:8).
- **scope-entities (plugin, instance check) ↔ the body's not-assertable surfaces**: the hook's stdin cap stays behind the instance check and the hook stays fail-open, its bounds proven by unit witnesses only, (.andromeda/a11y-plan-detail/a11y-scope-summary.md:64) and (.andromeda/a11y-plan.md:40).
