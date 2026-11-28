# a11y-plan · the rows that bind 2026-11-28-console-actions — picked 130 of 244 (94 by every draw) · rule any · pin 5384

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/cli-not-keyboard-interactive · CLI not keyboard-interactive — The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file. · except: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR.
decision/critical-path-dialog-answer · Critical path: dialog pending, then answer — A pending dialog answered with viola answer (question, permission, plan, and a question first raised by PermissionRequest), including the unverified-cli refusal that withholds answers, is a must-be-accessible critical path.
decision/critical-path-human-takes-wheel · Critical path: human takes the wheel — The human taking the wheel in the tui, automation refused (send refused with human-typing) and release returning it, with pause and release on the cli, is a must-be-accessible critical path.
decision/critical-path-send-readback · Critical path: confirmed send and readback — Confirmed send and readback, or refusal, through the viola send readback mirror [RB] is a must-be-accessible critical path.
decision/manual-keyboard-pass · Manual keyboard pass as supplement — A manual keyboard pass over CLI and TUI supplements the automated keystroke boundary assertion.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw. · except: the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes or the fleet it shows changes
decision/terminal-structural-equivalents · Terminal structural equivalents of ARIA roles — In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator, the `-- LINKS --` separator of the related links, the plain-space NAME indent of tree depth and the --help group labels.
decision/tui-boundary-assertions-only · TUI boundary assertions only — The tui is asserted only at its boundary, through the portable-pty outer-PTY driver; the result is a boundary guarantee, never a conformance claim.
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
contract/tui-passthrough/focus-mouse-resize-sequences · viola run TUI passthrough · focus, mouse and resize sequences — Focus, mouse and resize sequences and terminal replies pass through and do not count as human editing for wheel purposes.
contract/tui-passthrough/human-keystrokes · viola run TUI passthrough · human keystrokes — Human keystrokes are never blocked, refused or dropped once the wrapped child holds the terminal: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker; the one exception is a start's automatic verify window, before the child starts.
contract/viola-list/unwrapped-separator · viola list output · -- UNWRAPPED - READ-ONLY -- — The separator appears only when unwrapped rows exist.
contract/viola-switch-output/result-line · viola switch output · result line — `viola switch` prints one line, `<name>  switched  <model> <effort>`, on stdout in plain ASCII with no SGR.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
decision/critical-path-cli-board-output · Critical path: CLI board, wait/last and verify output — The CLI board, wait/last and verify output, with viola list's fixed captions and slot order, is a must-be-accessible critical path.
decision/manual-orca-pass · Manual Orca screen-reader pass — A manual Orca screen-reader pass on Section 4 P4 and P6, founder-owned, supplements the automated assertions. · why: no blind or low-vision target users are named, so screen-reader priority is scoped to CLI/TUI linearity, not a full assistive-tech matrix
decision/no-a11y-tool-output-discipline-assertions · Output-discipline assertions instead of an a11y tool — cli and tui use no a11y tool: the existing process, literal-pin and portable-pty output-discipline assertions on text and streams stand in, with manual screen-reader passes as a supplement. · why: no automated a11y tool reaches a terminal
contract/tui-passthrough/entry · viola run TUI passthrough · entry — The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.
contract/tui-passthrough/run-diagnostics · viola run TUI passthrough · run diagnostics — viola's diagnostics for run go to diagnostics/, never to the terminal.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too
decision/manual-a11y-pass-gates-nothing · Manual screen-reader pass gates nothing — The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
contract/frame-verb-refusal/server-verify-failed · frame verb refusals · server-verify-failed — send, wait, last, answer, pause, release, link, unlink and switch refuse before any frame a server that is not the recorded wrapper: exit 21, stdout empty, the server-verify `hint:` last on stderr; under `--json` that `error` `instance-unreachable` document, stderr empty.
contract/frame-verb-refusal/socket-dir-refused · frame verb refusals · socket-dir-refused — Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
contract/frame-verb-refusal/strict-modes-failed · frame verb refusals · strict-modes-failed — send, wait, last, answer, pause, release and switch refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
contract/revive-human-refusal/pair · viola revive human refusals · unable/hint pair — Without `--json`, each exit-1 refusal of `viola revive` is one `unable:` line and one `hint:` line last on stderr, byte-identical.
contract/start-refusal/home-strict-modes · viola run, revive start and verify refusals · home strict-modes failure — Over a home tree failing the strict-modes check, `viola run`, a `viola revive` or `viola adopt` start and `viola verify` refuse before any log opens: exit 1, stdout empty, the home-state pair on stderr with `hint:` last, no child; `verify` prints no step line and no summary.
contract/start-refusal/socket-dir · viola run, revive start and verify refusals · socket directory failure — Over a failing socket directory, `viola run` and a `viola revive` or `viola adopt` start refuse with exit 1 before any snapshot, bind or child: the socket-directory pair on stderr, `hint:` last, plain ASCII, no SGR, naming only the parsed name; the directory is left as found.
contract/tui-passthrough/after-feed-panic · TUI passthrough · after a contained vt100 feed panic — The driver's `send` is refused `not-delivered` / `input-not-ready` (exit 13) while a typed human key still reaches the child; none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal, and the child's own bytes do.
contract/viola-list-board/unwrapped-separator · viola list board · -- UNWRAPPED - READ-ONLY -- — The separator and unwrapped rows print only when the one `claude agents --json` read lists a session no wrapped row joins; unwrapped rows show the escaped `sessionId` as NAME and `n/a` in WHEEL, DIALOG and CLI; a failed, oversize or malformed read adds no separator and no unwrapped row.
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
decision/run-diagnostics-off-terminal · Run diagnostics never reach the terminal — viola's diagnostics for `run` go to `diagnostics/`, never to the terminal; the start's trust step reports its word in one `trust-record` line there, never as a refusal, exit code or hint.
contract/revive-json-refusal/internal-error · viola revive --json refusals · internal-error — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; a `claude` not found at a start reads as this document and has no human pair.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.
contract/viola-list-board/colour · viola list board · colour — On a terminal `viola list` writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`.
contract/viola-list-board/order · viola list board · structure — The board is the BAY context line, the uppercase caption row and the wrapped rows drawn as a tree, then the unwrapped read-only rows behind their separator, then the links behind their `-- LINKS --` separator.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/human-keystrokes-never-blocked · Human keystrokes never blocked — A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
decision/word-beside-every-sgr · Word beside every SGR span — Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola adopt`'s three own refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed. · except: The checks hold from the child's start: before it, a keyed start's automatic-verify lines and verify's own output reach the terminal, and a key waits in the terminal's own buffer.
decision/send-refused-while-human-typing · Send refused while the human types — When the human takes the wheel, `send` and `switch` are refused with `human-typing` on stderr and a typed exit code. · guard: std::process::Command asserts
decision/wheel-ignores-terminal-events · Focus, mouse and resize never move the wheel — On the tui, focus, mouse and resize sequences do not move the wheel, and viola originates zero bytes on the human's terminal. · guard: outer-PTY nextest cases on ubuntu-latest · except: Before the child starts, a keyed start's automatic-verify lines and verify's own output reach the human's terminal; after the child ends, a not-resumed viola adopt's refusal pair or document does.
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
decision/cli-structural-landmark-equivalents · CLI structural equivalents of landmarks — The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator, the `-- LINKS --` separator, the plain-space NAME indent of tree depth and the `--help` group labels.
decision/tui-boundary-only · Tui accessibility is boundary-only — The tui is the vendor `claude` TUI, and viola's accessibility work on it is held only at its boundary, with no landmarks of its own.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
contract/cli-exit-codes/10 · CLI exit codes · 10 — human-typing: the human holds the wheel, so the automated send or switch is refused with nothing typed and no child replaced.

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/cli-no-interactive-input · No interactive CLI input — The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`. · except: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize or a change in the fleet it shows

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/name-truncation · NAME truncation only on a narrow terminal — An unwrapped NAME is cut with `...` only on a terminal too narrow and never inside a `\xHH` escape; a wrapped NAME, piped output and `--json` are never cut.
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/colour-decision-inputs · Colour decision inputs — The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.
decision/state-carried-by-printed-word · State carried by its printed word — Each board state is carried by its printed word (stale, DIALOG <kind>, expired, unverified-cli, idle, busy, n/a), never by colour alone.
decision/unwrapped-name-truncation · Unwrapped NAME truncation — On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus ..., never inside an escape; a wrapped NAME is never cut, and piped output and --json keep every name whole. · guard: cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
decision/time-bounds-never-on-human · Time bounds never fall on the human — Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by them.
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/claude-agents-read-fails-open · claude agents read fails open — A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's and adopt's preflight; in the fleet console it hands an empty list, so STATUS reads unknown.
decision/dialog-hold-bounds-the-driver · Dialog hold bounds the driver — A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
decision/hyprctl-read-deadline · Each compositor read is bounded at 5 s — Each compositor read through the Hyprland seam is bounded at 5 s a reply; it falls on viola's own start or verb (the window step, viola open, viola focus), never on a key typed into a session, and never moves the wheel.
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release
decision/open-live-wait-bound · viola open's live wait is bounded at 30 s — viola open waits at most 30 s for the instance it started to come live, then reads not-live; the wait holds only the verb's own return, never a key typed into a session, and never moves the wheel.
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
decision/respawn-reap-bound · A switch's reap wait is bounded at 10 s — A `viola switch` waits at most 10 s (RESPAWN_REAP_BOUND) for the old child to exit after its hang-up; a child not exited is not replaced, and the switch is refused not-delivered child-not-ended, exit 13. The bound falls on the wrapper, never on the human.
decision/start-verify-stop-grace-off-the-human · Start's verify stop grace never falls on the human — The start's verify stop grace, the wait before a stopped start's verify child is killed, falls on viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs, link/unlink and focus, exit 1 for run, revive's and adopt's start, verify and console. · except: the fleet console's mid-run refusal, after frames were drawn: the terminal is restored first, then exit 1 with the home pair and one process-exit strict-modes-failed line
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Eight time bounds (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) fall on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
decision/unknown-refusal-has-no-hint · unknown refusal carries no hint — The unknown refusal prints no hint. · why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design
decision/window-wait-bound · The window step's wait is bounded at 5 s — A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list; the wait falls on viola's own start, never on a key typed into a session, and never moves the wheel.
contract/cli-exit-codes/13 · CLI exit codes · 13 — An answer for an expired or absent dialog is refused not-delivered / unknown-dialog.
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list.
contract/cli-exit-codes/2 · CLI exit codes · 2 — A usage error, or an unusable answer response or send text, or a viola open word after -- other than claude alone or a parent equal to the name, written as one fixed stderr line with no document that echoes none of the refused words.
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint.
contract/cli-fixed-message/form · CLI fixed-message refusal · form — viola revive's exit-1 refusals, viola adopt's strict-modes-failed, session-live and not-resumed refusals, viola run's socket-directory refusal, viola verify's untrusted-folder refusal and viola console's two refusals print one unable: <text> line, then one hint: line last.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
contract/human-fault-line/wrapper-fault · human fault line · wrapper fault — error: wrapper fault  <code>.
contract/send-refusal/input-not-ready · viola send refusal · input-not-ready — A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
contract/send-refusal/no-prompt-submitted · viola send refusal · no-prompt-submitted — A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run, revive and adopt start.
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: a frame verb, viola focus before any compositor read, or viola up on the board's strict-modes refusal, was refused before reaching the instance, the cause in detail.
contract/event-line/dialog-expired · the event line · dialog-expired — Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
contract/instance-unreachable-detail/endpoint-lost · instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open.
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — The name has no live endpoint but its instance directory stands, or the instance is stale.
contract/instance-unreachable-detail/server-verify-failed · instance-unreachable detail · server-verify-failed — The process serving the endpoint is not the recorded wrapper and was not contacted.
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — The socket directory was refused; the verb sent no frame.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written.
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — The name has no live endpoint and no instance directory (absent or a regular file).
contract/start-holder-refusal/already-live · start against a holder, exit 1 · already-live — The holder is live, or a fresh replayed beat was read.
contract/start-holder-refusal/stale-heartbeat · start against a holder, exit 1 · stale-heartbeat — The holder is stale; stale heartbeat is exit 1's cause alone.

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
decision/no-screen-content-parsing · No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals, the compiled shapes over them), the prompt row only for its opening char, never the text after it; screenshots are never taken as a11y evidence (R7). · why: the rendered TUI is vendor content · except: the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/non-editing-sequences · Terminal sequences are not human editing — Focus, mouse and resize sequences and terminal replies (the F-W2 closed list) pass through and never count as human editing; every other byte does (P4).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing
decision/compiled-shapes-over-signatures · Compiled shapes over the screen signatures — The compiled shapes the rendered `claude` TUI screen is parsed for over its compiled signatures are a modal label row's option shape and the box's rows below it; no row text is kept.
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/tui-boundary-driver-pin · TUI boundary driver — The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/boundary-check-never-inspects-claude-content · Boundary check never inspects claude content — The keyboard boundary check never inspects claude content.
decision/boundary-tests-sequenced-on-records · Boundary tests sequenced on records — The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
decision/human-key-takes-the-wheel · Human key takes the wheel — A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed. · guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.
decision/stdin-classifier-non-editing-list · Closed non-editing stdin list — Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel. · why: ratified, the exact grammars chosen after a narrower option was shown

# a11y-plan · its absences, names and overviews, whole

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
absence/internationalization — Internationalization: no i18n, RTL or lang conventions
absence/touch-surface — A touch surface
absence/web-or-graphical-frontend — A web or graphical frontend

## 5-keyboard-navigation · ## 5. Keyboard Navigation
N/A — viola has no web or graphical frontend; the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
N/A — viola has no web or graphical frontend, so no page token pair or layout to verify.

## 7-screen-reader-support · ## 7. Screen Reader Support
N/A — viola has no web or graphical frontend.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
absence/user-time-limit — A time limit on the user, and any timeout-extension component

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. A11y Assertion Harness Contract
absence/rendered-page — A rendered page, and with it any browser a11y tooling

## contrast-verification-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## focus-management-test-harness · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## structured-violation-json-schema · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.

## wcag-criteria-mapping · key of ## 3. A11y Assertion Harness Contract
N/A — viola has no web or graphical frontend.
