# a11y-plan · the rows that bind 2026-11-24-viola-focus — picked 92 of 238 (75 by every draw) · rule any · pin 4392

## 1-a11y-scope-summary · ## 1. A11y Scope Summary · twin a11y-scope-summary
decision/cli-not-keyboard-interactive · CLI not keyboard-interactive — The CLI has no prompt, TUI mode or pager; its input comes only from stdin or --file.
decision/colour-second-cue-only · Colour only a second cue — Colour is never the sole cue on the CLI: the word always sits beside any SGR.
decision/no-sgr-in-plain-modes · No SGR in plain modes — No SGR, glyph or cursor control is written under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run` or a passed `viola revive`.
decision/output-discipline-stands-in-for-sc · Output discipline stands in for WCAG SC — Output-discipline assertions stand in for SC 1.3.2, 1.4.1 and 3.3.1: meaningful linear order, colour never the sole cue, and the refusal reason and detail given in text. · why: WCAG does not apply directly to terminal output
decision/static-linear-output · Static linear output — viola's own output is static and linear, with no spinner or redraw.
decision/terminal-structural-equivalents · Terminal structural equivalents of ARIA roles — In place of ARIA roles, terminal structure is carried by the caption line, the unwrapped separator, the `-- LINKS --` separator of the related links, the plain-space NAME indent of tree depth and the --help group labels.
contract/cli-output/control-characters · CLI output · C0/C1 control characters — C0/C1 control characters are escaped except \n and \t, which list rows escape too.
contract/cli-refusal/format · CLI refusals · refusal format — A refusal is written to stderr as unable … hint:, giving the reason and detail in text, with a typed exit code.
decision/a11y-tier-standard · A11y tier Standard (1) — The a11y tier is Standard (1): cli and tui get output-discipline and boundary assertions plus manual screen-reader passes, with no WCAG conformance claim.
contract/tui-passthrough/viola-terminal-bytes · viola run TUI passthrough · viola terminal bytes — Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.

## a11y-scope-summary · ## 1. A11y Scope Summary · twin 1-a11y-scope-summary
decision/control-character-escaping · C0/C1 control characters escaped — CLI output escapes C0/C1 control characters except `\n` and `\t`; `wait` and `last` keep those two. · why: the list-row exception is ratified (T5) · except: in `viola list` table rows `\n` and `\t` are escaped too
decision/linux-only · Linux only — Every surface targets Linux only: the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland.
contract/cli-help/group-labels · viola --help · group labels — The `--help` verb table is grouped under the labels board / traffic / wheel / handoff / setup.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI failure under `--json` is one document, a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
contract/cli-usage-error/exit-2 · CLI exit codes · 2 — A clap usage error exits 2; `viola plugin` alone is such an error.
contract/frame-verb-refusal/socket-dir-refused · frame verb refusals · socket-dir-refused — Frame verbs refuse a failing socket directory before any frame: exit 21, socket-directory `hint:` last on stderr, plain ASCII, no path, pid or uid; under `--json` `error` `instance-unreachable`, `detail` `socket-dir-refused`; the directory is left as found.
contract/frame-verb-refusal/strict-modes-failed · frame verb refusals · strict-modes-failed — send, wait, last, answer, pause and release refuse an instance failing the strict-modes check before any frame or log: exit 21, stdout empty, `hint:` last on stderr, or under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the mode is left as found.
decision/manual-a11y-pass-gates-nothing · Manual screen-reader pass gates nothing — The manual Orca screen-reader pass (GNOME Terminal for the cli, foot on Hyprland for the tui) is founder-owned and supplemental; nothing is gated on it.
decision/output-discipline-a11y-fallback · Output-discipline assertions stand in for a11y tools — CLI a11y is checked by output-discipline assertions on the existing CLI drivers (process Command, literal-pinned strings, SGR-only checks through the portable-pty outer PTY), their results machine-parseable through the harness `run` JSON. · why: no automated a11y tool exists for terminal output; a11y tools assume a DOM or native widget tree
contract/revive-json-refusal/no-session · viola revive --json refusals · no-session — One of the closed set of `error` codes a `viola revive --json` exit-1 refusal document carries; `viola revive <name> --list --json` over an empty chain prints this document, exit 1.
contract/tui-passthrough/own-bytes · TUI passthrough · terminal bytes — viola writes zero bytes to the terminal except the child's output and, before it, a keyed start's automatic-verify lines and verify's own output; a passed `viola revive` writes no line of its own while the child holds the terminal.

## 2-a11y-strategy · ## 2. A11y Strategy
decision/no-sgr-without-tty · No SGR without a colour-capable terminal — The CLI emits no SGR of its own under a non-TTY, `NO_COLOR`, `TERM=dumb`, `--json`, `viola run` or a passed `viola revive`. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
decision/word-beside-every-sgr · Word beside every SGR span — Every SGR span on the CLI sits beside its word, so each state is still carried by the word without the colour. · guard: literal pins over std::process::Command and the portable-pty outer PTY on ubuntu-latest
contract/cli-refusal/hint-line · CLI refusal output · hint line — A refusal ends with `hint:` as the last stderr line, carrying a recovery instruction where recovery exists.
contract/cli-refusal/unable-line · CLI refusal output · unable line — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, the plain word followed by plain detail; `viola revive`'s exit-1 refusals, `viola run`'s socket-directory refusal and `viola verify`'s interrupted pair print one `unable: <text>` line instead.

## 4-aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin aria-patterns-roles
decision/c0-c1-escaped · C0 and C1 escaped in CLI output — C0 and C1 control characters in CLI output are escaped. · guard: literal pins over std::process::Command on ubuntu-latest
decision/cli-structural-landmark-equivalents · CLI structural equivalents of landmarks — The CLI has no landmark roles; its structure is carried by the BAY context line, the uppercase caption line, the `-- UNWRAPPED - READ-ONLY --` separator, the `-- LINKS --` separator, the plain-space NAME indent of tree depth and the `--help` group labels.
decision/linear-static-output · Linear static CLI output — CLI output is linear and static. · guard: literal pins over std::process::Command on ubuntu-latest
decision/cli-output-discipline-assertion-tools · CLI assertable as output discipline — The cli is assertable as output discipline, with no conformance claim, by `std::process::Command` with literal pins and portable-pty outer-PTY SGR checks on the `ubuntu-latest` CI leg.
decision/portable-pty-outer-pty-driver · portable-pty as the outer-PTY boundary driver — The tui boundary assertions run on the portable-pty `=0.9.0` outer-PTY driver, checking that viola emits zero terminal bytes of its own and that human keystrokes are never blocked or delayed. · except: The checks hold from the child's start: before it, a keyed start's automatic-verify lines and verify's own output reach the terminal, and a key waits in the terminal's own buffer.

## aria-patterns-roles · ## 4. ARIA Patterns & Roles · twin 4-aria-patterns-roles
decision/control-bytes-escaped · Control bytes printed escaped — Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line. · why: security control for C0/C1 escaping, ratified for list rows · guard: escape_row_shows_every_control_character_as_hex_text
decision/linear-static-cli-output · Linear static CLI output — CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
decision/no-sgr-off-terminal · No SGR off a terminal — viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive. · guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
decision/sgr-beside-its-word · SGR only beside its word — Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured. · guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
contract/cli-help/served-verbs-only · viola --help · served verbs only — The help group rows list the served verbs only: `plugin install` is one entry in `setup:` and a bare `plugin` is unserved.
contract/cli-help/usage-line · viola --help · usage line — `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the group rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:`, then `Options:`, plain ASCII.

## 5-keyboard-navigation · ## 5. Keyboard Navigation
decision/cli-no-interactive-input · No interactive CLI input — The CLI has no interactive prompt, pager or redraw, and takes input only from stdin or `--file`.
decision/tui-terminal-belongs-to-child · Tui terminal belongs to the child — On the tui, the terminal belongs to the wrapped child process, not to viola.

## 6-visual-design-verification · ## 6. Visual Design Verification · twin visual-design-verification
decision/unable-fail-never-coloured · Refusal and failure words never coloured — The words `unable` and `fail` are never coloured.

## visual-design-verification · ## 6. Visual Design Verification · twin 6-visual-design-verification
decision/colour-decision-inputs · Colour decision inputs — The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.

## 8-cognitive-accessibility · ## 8. Cognitive Accessibility · twin cognitive-accessibility
decision/fixed-cli-strings-pinned · Fixed CLI strings pinned by literal — Every fixed CLI string, hints and result lines included, is pinned by literal. · why: so any copy change fails CI until the pinned string is updated on purpose
decision/fixed-copy-from-design · Fixed CLI copy comes from design — The CLI's fixed copy set (`no wrapped sessions  start one: viola run <name> -- claude`, `unknown`, `n/a`) comes from design.
decision/no-bare-error-text · No stack trace or bare status as error text — The CLI never shows a stack trace or a bare status number as the only error text.
decision/reason-code-beside-unable · Reason codes always beside unable — Every kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) always sits beside the plain word `unable`.
decision/refusal-layout-literal-pins · Refusal layout held by literal pins — Each refusal's `unable <reason> <detail>` plus final `hint:` layout, its typed exit code and a hint-free `--json` are held by literal asserts in CI on ubuntu-latest. · guard: literal pins over std::process::Command on ubuntu-latest
decision/time-bounds-never-on-human · Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
contract/cli-refusal/exit-code · CLI refusal output · exit code — Every refusal ends with a typed exit code.
contract/cli-refusal/json · CLI refusal output · --json — Under `--json` a refusal gives one document with no hint, except the `viola answer`, `viola send` and `viola open` exit-2 paths, which print a fixed stderr line and no document.
contract/cli-refusal/unknown-refusal · CLI refusal output · the `unknown` refusal — The `unknown` refusal exits 14 and is the one refusal with no hint; its detail is a code word or nothing.
contract/cli-refusal/fault · CLI refusal output · fault — Faults are not refusals and carry no hint.

## cognitive-accessibility · ## 8. Cognitive Accessibility · twin 8-cognitive-accessibility
decision/error-bodies-have-writer-and-witness · Error bodies have a writer and a witness — Each error body names a writer and a witness that stand, and each product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/error-word-detail-hint · Errors as word, detail and hint — Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
decision/fixed-cli-strings-pinned-by-literal · Fixed CLI strings pinned by literal — Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
decision/json-failure-documents-are-clean · --json failure documents are clean — Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text. · guard: cli_error_documents_hold_no_path_or_upstream_text
decision/json-one-document-no-hint · One --json document with no hint — Under --json a failure gives one document with no hint. · except: the viola answer, viola send and viola open exit-2 paths write a fixed line to stderr whatever --json says, and no document
decision/no-hint-names-release · No hint names release — No hint names the release verb. · guard: no_hint_names_release
decision/one-hint-per-code · One hint per code — No two codes share a hint. · guard: no_two_codes_share_a_hint
decision/peer-replies-screened-to-code-words · Peer replies screened to code words — A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
decision/printed-names-are-parsed-arguments · Printed names are the parsed arguments — Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
decision/socket-dir-refusal-before-any-frame · Socket-directory refusal before any frame — A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
decision/strict-modes-refusal-before-any-write · Strict-modes refusal before any write — A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — No thiserror message interpolates a value. · guard: thiserror_messages_interpolate_nothing
decision/time-bounds-fall-on-the-driver · Time bounds fall on the driver only — Every time bound (MCP wait cap, MCP call deadline, dialog hold, readiness gate, paste write, confirm windows, agents read, the start's Claude config lock wait) falls on the driver, viola's own child or viola's own start, never blocking, refusing or delaying a human key and never moving the wheel.
decision/unknown-refusal-has-no-hint · unknown refusal carries no hint — The unknown refusal prints no hint. · why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess
decision/unlisted-code-words-stay-visible · Unlisted code words stay visible — A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14. · why: visible by design
decision/unstattable-home-is-strict-modes-refusal · Unstattable home is a strict-modes refusal — A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
contract/cli-exit-codes/14 · CLI exit codes · 14 — unknown: a refusal whose word is outside the closed refusal list.
contract/cli-exit-codes/2 · CLI exit codes · 2 — A usage error, or an unusable answer response or send text, or a viola open word after -- other than claude alone or a parent equal to the name, written as one fixed stderr line with no document that echoes none of the refused words.
contract/cli-exit-codes/21 · CLI exit codes · 21 — instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
contract/cli-refusal/stderr-layout · CLI refusal · stderr layout — unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
contract/code-word/format · code word · format — 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
contract/instance-unreachable-detail/instance-dead · instance-unreachable detail · instance-dead — The name has no live endpoint but its instance directory stands, or the instance is stale.
contract/instance-unreachable-detail/strict-modes-failed · instance-unreachable detail · strict-modes-failed — The instance failed the strict-modes check; refused before any frame or log, no log line written.
contract/instance-unreachable-detail/unwrapped-name · instance-unreachable detail · unwrapped-name — The name has no live endpoint and no instance directory (absent or a regular file).
contract/instance-unreachable-json/document · exit 21 under --json · document — One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
decision/faults-carry-no-hint · Faults carry no hint — Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was budget-paused) and never reused; an older wrapper's budget-paused refusal reads as unknown, exit 14.
contract/cli-exit-codes/20 · CLI exit codes · 20 — A fault, with no hint.
contract/instance-unreachable-detail/socket-dir-refused · instance-unreachable detail · socket-dir-refused — The socket directory was refused; the verb sent no frame.
contract/json-fault/internal-error · CLI fault under --json · internal-error — Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.

## 11-a11y-anti-patterns-never-do-these · ## 11. A11y Anti-Patterns (NEVER do these)
decision/codes-beside-plain-word · Kebab-case codes beside a plain word — A kebab-case reason or detail code (`not-delivered`, `human-typing`, `unverified-cli`) on a refusal line never appears without the plain word `unable`; the board's cells show status codes such as `unverified-cli` without it. · why: no jargon without a plain alternative; SC 3.1.5 AAA is not claimed
decision/no-colour-on-failure-words · Failure words are never coloured — The CLI words `unable` and `fail` are never coloured.
decision/no-information-by-colour-alone · No information by colour alone — Colour is only a second cue: every state indicator has a text or icon beside it (SC 1.4.1).
decision/no-spinners · No spinners — CLI output is static and linear, with no spinner or redraw. · why: design ban
decision/no-stack-trace-only-error · No stack trace as the only error text — The CLI never renders a stack trace as the only error text.
decision/no-unwarned-timeouts · No harsh timeouts without warning — No harsh timeout is set without a warn-extend pattern (SC 2.2.1).
decision/refusal-ends-with-hint · Every refusal ends with a hint — Every human-mode CLI refusal ends with a `hint:` recovery line (SC 3.3.3). · except: `unknown` (exit 14), whose detail is a code word or nothing
decision/machine-output-a11y-review · No human-only a11y review — No a11y review is human-only without machine output. · why: the canonical agent-runnable principle
decision/manual-checks-supplemental-only · Manual checks are supplemental only — Manual screen reader, keyboard or contrast checks are never the ONLY method; as a supplement to automated tools they are OK.

## a11y-testing-tool-pick · key of ## 3. A11y Assertion Harness Contract
decision/cli-a11y-by-literal-tests · CLI a11y checked by literal tests — With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.

## keyboard-test-harness · key of ## 3. A11y Assertion Harness Contract
decision/zero-viola-bytes-on-the-terminal · Zero viola bytes on the terminal — The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run. · except: A keyed start's automatic-verify lines and verify's own output, which precede the child's bytes.
decision/live-readings-never-gate · Live readings never gate — Live readings of the takeover clause on a real terminal are supplemental and never gating.

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
