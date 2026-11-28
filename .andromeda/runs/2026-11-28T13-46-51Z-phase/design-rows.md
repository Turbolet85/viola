# design-system · the rows that bind 2026-11-28-console-actions — picked 245 of 371 (184 by every draw) · rule any · pin 5384

## brand-identity · ## Brand Identity
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/dialog-pending-cocked-strip · Dialog-pending shown as a cocked strip — A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/pause-release-transfer-control · Pause and release as control transfer — `viola pause` is the human's "I have control" and `viola release` is "you have control".
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.
decision/stable-session-slots · Stable session slots — Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
decision/strip-board-density · Calm strip-board density — The density is a strip board's, not a monitoring dashboard's: fixed fields, tabular numbers, one attention colour and one handoff colour, everything else paper and ink.
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/flight-progress-strip/name · Flight-progress strip · NAME — NAME is the first of the six fixed fields, set at callsign weight.
contract/flight-progress-strip/no-reading · Flight-progress strip · field with no reading — A field with no reading prints `unknown` in its box, never a shimmer or a guess.
contract/handoff/transfer-marker · Handoff display · transfer marker — Transfer of control is shown as a `driver -> driven` transfer marker.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/clear · Send readback · /clear — A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
contract/send-readback/compact · Send readback · /compact — On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
contract/send-readback/context · Send readback · /context — On a verified CLI a `/context` send counts, and reads back, only once one new local-command stdout tag appears in the session transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads `unconfirmable`, exit 0.
contract/send-readback/mcp · Send readback · /mcp — On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
contract/send-readback/unconfirmable · Send readback · unconfirmable — A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.
decision/expression-level · Expression level 0.2 — The base expression level `0.2` is the single source of truth for animation intensity, layout freedom, interaction complexity and motion across the whole project.
decision/stale-strip-dimmed-not-deleted · Stale strip dimmed, never deleted — A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.
contract/flight-progress-strip/cli · Flight-progress strip · CLI — CLI is the sixth and last of the six fixed fields.
contract/flight-progress-strip/dialog · Flight-progress strip · DIALOG — DIALOG is the fifth of the six fixed fields.
contract/flight-progress-strip/live · Flight-progress strip · LIVE — LIVE is the second of the six fixed fields.
contract/flight-progress-strip/status · Flight-progress strip · STATUS — STATUS is the third of the six fixed fields.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.

## color-palette · ## Color Palette
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
decision/bare-strip-never-cocked · Bare strip never cocked — A bare (unwrapped) strip is never shown cocked.
decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.
contract/colour-palette/accent · Colour palette · Accent #D97706 — The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.

## typography · ## Typography
decision/bold-only-name-column · Bold only for the NAME column — In the cli, bold is used only for the NAME column.
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.

## iconography · ## Iconography
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
contract/cli-glyphs/arrow · CLI glyphs · -> — The arrow glyph is `->`, with no inbound form.
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
contract/cli-transfer-arrow/ascii-arrow · CLI transfer arrows · -> — The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`) and as the `<driver> -> <driven>` lines of `viola list`'s `-- LINKS --` block.
decision/list-six-fixed-columns · viola list has six fixed columns — `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.
contract/viola-list/columns · viola list output · columns — `viola list` carries no link column, so its six columns, their captions and their slot order stay fixed; a tree's indent stands inside the NAME cell and links print apart, in the board's `-- LINKS --` block.

## surface-cli · ## Surface: cli · twin surface
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/fleet-console-not-verb-output · Fleet console governed by no verb-output rule — The terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule.
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify`, `plugin install` and `console` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
decision/permission-request-question-held · PermissionRequest question held for an answer — A question first raised by a PermissionRequest is held as a pending dialog and answered by `viola answer` like any dialog; a human key, an expired hold or an unverified CLI leave it to the human.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or file only — Prompt text comes only from stdin or `--file`.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
decision/truncate-only-unwrapped-name · Only an unwrapped NAME is truncated — Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
contract/cli-colour-tokens/attention · CLI colour tokens · attention — Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
contract/cli-colour-tokens/bold · CLI colour tokens · bold — NAME values are bold.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up`, `verify` and `console` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run`, `revive` or `adopt` prints a fixed-message `unable:` line and one hint per cause; `revive`'s and `adopt`'s causes are read in a fixed order that stops at the first.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/cli-strict-modes-refusal/instance-state-pair · CLI strict-modes refusal · instance-state pair — The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
contract/cli-verb-output/answer · CLI verb output · viola answer — `viola answer <name> <id>` prints `answered  <name>  dialog <id>`, or `unable  <name>  not-delivered  unknown-dialog`.
contract/cli-verb-output/focus · CLI verb output · viola focus — `viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace is ever printed or logged.
contract/cli-verb-output/pause · CLI verb output · viola pause — `viola pause <name>` prints `<name>  wheel human  manual-pause  I have control`.
contract/cli-verb-output/release · CLI verb output · viola release — `viola release <name>` prints `<name>  wheel driver  you have control`.
contract/exit-21-cause/endpoint-lost · exit 21 cause · endpoint-lost — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/instance-dead · exit 21 cause · instance-dead — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/server-verify-failed · exit 21 cause · server-verify-failed — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/socket-dir-refused · exit 21 cause · socket-dir-refused — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/strict-modes-failed · exit 21 cause · strict-modes-failed — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/unwrapped-name · exit 21 cause · unwrapped-name — One of the closed set of causes named in exit 21's `detail`.
contract/placement-word/malformed · placement word · malformed — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/oversize · placement word · oversize — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/placed · placement word · placed — One of the eight closed placement words `viola open` prints and the window record carries: the window was moved to the asked workspace.
contract/placement-word/refused · placement word · refused — One of the eight closed placement words `viola open` prints and the window record carries: the move was refused.
contract/placement-word/unplaced · placement word · unplaced — One of the eight closed placement words `viola open` prints and the window record carries: no workspace was asked.
contract/placement-word/unreachable · placement word · unreachable — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/unsupported · placement word · unsupported — One of the eight closed placement words `viola open` prints and the window record carries: off Hyprland, the open still live.
contract/placement-word/window-unseen · placement word · window-unseen — One of the eight closed placement words `viola open` prints and the window record carries: no window of the wrapper's own terminal was seen.
contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch`, is made before any frame, exits 21, and writes no frame and no line.
contract/strict-modes-refusal/human-output · Instance strict-modes refusal · unable: / hint: pair — The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
contract/viola-console-refusals/home-refused · viola console refusals · home failing its read rule — A home failing its read rule refuses `viola console` with exit 1: `unable: viola console could not use its home` then `hint: make the viola home readable by you alone, then run viola console again`; no line is written and the mode is left as found.
contract/viola-console-refusals/no-terminal · viola console refusals · no terminal — With stdin or stdout not a terminal, `viola console` exits 1 before any home read with `unable: viola console needs a terminal` then `hint: run viola console in a terminal window`.
contract/viola-focus-exit-1-codes/malformed · viola focus exit-1 codes · malformed — `malformed` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"malformed","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
contract/viola-focus-exit-1-codes/oversize · viola focus exit-1 codes · oversize — `oversize` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"oversize","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
contract/viola-focus-exit-1-codes/refused · viola focus exit-1 codes · refused — `refused` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"refused","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
contract/viola-focus-exit-1-codes/unreachable · viola focus exit-1 codes · unreachable — `unreachable` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"unreachable","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
contract/viola-focus-exit-1-codes/unsupported · viola focus exit-1 codes · unsupported — `unsupported` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"unsupported","detail":null}`, in human mode its fixed `unable:` / `hint:` pair.
contract/viola-focus-exit-1-codes/window-not-found · viola focus exit-1 codes · window-not-found — `window-not-found` is one of the six closed exit-1 codes of `viola focus`: under `--json` `{"v":1,"error":"window-not-found","detail":null}`, in human mode its fixed `unable:` / `hint:` pair, nothing dispatched.
contract/viola-focus-refusals/exit-21 · viola focus refusals · exit 21 — `viola focus` exits 21 with `instance-dead` or `unwrapped-name` for a name not live, and with `strict-modes-failed` for a home or instance failing strict modes, each before any compositor read.
contract/viola-last-output/header · viola last output · header — `last` prints `last  <name>  <kind> <time>` on stderr, TTY only.
contract/viola-last-output/message · viola last output · message — `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
contract/viola-list-table/row-order · viola list table · row order — Rows keep the board's name order, each wrapped child drawn directly under its parent.
contract/viola-run-output/child-started · viola run and revive output · child started — `run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/auto-verify-line/did-not-verify · viola run and revive automatic-verify line · did not verify — A start whose automatic verify ran and left a stamp that is not green prints the fixed did-not-verify line naming the parsed claude version.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/auto-verify-line/line-form · viola run and revive automatic-verify line · line form — A start prints exactly one of the five automatic-verify lines, each one stderr write, ASCII, with no SGR, holding only the parsed version and a closed word; none prints under `--json`.
contract/auto-verify-line/no-stamp · viola run and revive automatic-verify line · no stamp — A start whose automatic verify left no stamp, a verify refused for an untrusted folder included, prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`; the version stays unattempted.
contract/auto-verify-line/verifying · viola run and revive automatic-verify line · verifying — A start that runs its automatic verify first prints the fixed verifying line naming the parsed claude version.
contract/auto-verify-line/verifying-elsewhere · viola run and revive automatic-verify line · verifying elsewhere — A start that finds another start holding the verify attempt lock prints the fixed verifying-elsewhere line naming the parsed version, runs no verify and starts unverified.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
contract/cli-exit-codes/20 · CLI exit codes · 20 — `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/origin-word/agent · origin word · agent — One of the two closed origin words: an open made from inside a wrapped session.
contract/origin-word/cli · origin word · cli — One of the two closed origin words: an open made from a human shell.
contract/revive-json-refusal-code/already-live · revive --json refusal code · already-live — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/cwd-missing · revive --json refusal code · cwd-missing — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/internal-error · revive --json refusal code · internal-error — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/no-session · revive --json refusal code · no-session — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/pinned-hash-mismatch · revive --json refusal code · pinned-hash-mismatch — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/session-live · revive --json refusal code · session-live — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/socket-dir-refused · revive --json refusal code · socket-dir-refused — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/squatted-name · revive --json refusal code · squatted-name — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/stale-heartbeat · revive --json refusal code · stale-heartbeat — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/strict-modes-failed · revive --json refusal code · strict-modes-failed — One of the closed set of codes a `revive --json` start refusal document carries.
contract/viola-list-table/columns · viola list table · columns — Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.

## surface · ## Surface: cli · twin surface-cli
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/instance-unreachable-hint-per-cause · One instance-unreachable hint per cause — The `instance-unreachable` refusal (exit 21) prints one hint per cause. · why: The exit code is shared by several causes.
decision/no-hint-for-unknown · No hint for unknown — The `unknown` refusal (exit 14) prints no hint line. · why: Its detail is a code word or nothing, and a hint would have to guess.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document. · why: The child has held the terminal and viola writes nothing past it. · except: `viola adopt`'s not-resumed end writes its fixed pair or `--json` document after the child released the terminal; `run` and `revive` still write nothing past the child.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-colour-tokens/callsign · CLI colour tokens · callsign — Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
contract/cli-colour-tokens/stale · CLI colour tokens · stale — SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text`, `paste-cut`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; under `--json` the refusal object.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/exit-21-json-cause/endpoint-lost · exit 21 --json causes · endpoint-lost — A wrapper lost with the call open reads `endpoint-lost`.
contract/exit-21-json-cause/instance-dead · exit 21 --json causes · instance-dead — With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
contract/exit-21-json-cause/server-verify-failed · exit 21 --json causes · server-verify-failed — A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the nine frame-writing verbs read `server-verify-failed`.
contract/exit-21-json-cause/socket-dir-refused · exit 21 --json causes · socket-dir-refused — `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the seven frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/exit-21-json-cause/unwrapped-name · exit 21 --json causes · unwrapped-name — With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent or a regular file reads `unwrapped-name`; a symbolic link in its place is refused by the strict-modes check and reads `strict-modes-failed`.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release`, `switch` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/human-tables/name-truncation · human tables · NAME truncation — Where the width is detectable, human tables truncate only the NAME of unwrapped rows, ending it in `...`.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/peer-reply-screening/code-word · peer reply screening · code word — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
contract/peer-reply-screening/non-string-detail · peer reply screening · non-string detail — A peer reply with a non-string `detail` is no known reply.
contract/peer-reply-screening/refusal-not-code-word · peer reply screening · refusal that is no code word — A peer's `refusal` that is no code word reads `unknown`.
contract/peer-reply-screening/string-detail-not-code-word · peer reply screening · string detail that is no code word — A peer's string `detail` that is no code word reads as no detail, null under `--json`.
contract/peer-reply-screening/unlisted-code-word · peer reply screening · code word outside the refusal list — A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
contract/refusal-hints/human-typing · refusal hints · human-typing — `the human has the wheel; send again after the human hands it back`
contract/refusal-hints/not-delivered-control-character · refusal hints · not-delivered · control-character — `the text contains a control character (only LF, CR, TAB are allowed)`
contract/refusal-hints/not-delivered-empty-text · refusal hints · not-delivered · empty-text — `the text is empty once its trailing newlines are removed; send a text with content`
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-no-prompt-submitted · refusal hints · not-delivered · no-prompt-submitted — `<name> did not submit the prompt; check it, then send again`
contract/refusal-hints/not-delivered-paste-cut · refusal hints · not-delivered · paste-cut — `<name> did not take the whole text in time; a human must look at the session before the next send`, printed in human mode only.
contract/refusal-hints/not-delivered-turn-running · refusal hints · not-delivered · turn-running — `a turn is running; viola wait <name> first`
contract/refusal-hints/not-delivered-unknown-dialog · refusal hints · not-delivered · unknown-dialog — `that dialog is not pending; viola list shows the current DIALOG`
contract/refusal-hints/not-running · refusal hints · not-running — When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
contract/refusal-hints/socket-dir · refusal hints · socket-dir — The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/refusal-hints/unwrapped-name · refusal hints · unwrapped-name — When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
contract/viola-revive-refusals/home-tree-check · viola revive refusals · home-tree check — The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
contract/viola-revive-refusals/no-session · viola revive refusals · no-session — `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/no-such-id · viola revive refusals · no logged session with that id — An `--id` the log does not hold prints `unable: <name> has no logged session with that id` then `hint: viola revive <name> --list`.
contract/viola-revive-refusals/order · viola revive refusals · order — `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
contract/viola-revive-refusals/session-live · viola revive refusals · session-live — When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive-refusals/socket-dir · viola revive refusals · socket directory — A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
contract/viola-revive-refusals/strict-modes-no-log · viola revive refusals · strict-modes — `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`) and never reused; a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14.
contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/viola-list-board/colour-spans · viola list board · colour spans — Colour spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
contract/viola-list-board/failed-agents-read · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
contract/viola-list-board/name-cut · viola list board · NAME cut — An unwrapped NAME longer than its column is cut to the column less 3 characters and ends in `...`, never splitting an escape; a wrapped NAME is never cut; a piped board or `--json` keeps every name whole.
contract/viola-list-board/status · viola list board · STATUS — A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-list-board/unwrapped-row · viola list board · unwrapped row — One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-run-start-refusals/stale-heartbeat · viola run start refusals · stale-heartbeat — A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last`, `list` and the fleet console escapes C0/C1 control characters.
decision/no-auto-sorting-strips · No auto-sorting of strips — Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place. · why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-convergent-safe-choices · No convergence on safe design choices — The design never converges on common safe choices across generations; the readback box and the cocked strip stand in their place. · why: Agent dashboards converge on dots, toasts and graphs.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.

## self-validation-protocol · ## Self-Validation Protocol
decision/readback-box-signature-check · Readback box signature check — Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
decision/domain-not-statistical-design · Design from the product's domain — Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
decision/squint-test · Squint test — Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly. · except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.
decision/swap-test · Swap test — Before UI output is presented, a design that would not feel meaningfully different with a default typeface, default colours and a standard layout is redone with intent.

## surface-console · ## Surface: console
contract/fleet-console-keys/ctrl-c · Fleet console keys · Ctrl-C — Ctrl-C ends the fleet console with exit 0.
contract/fleet-console-keys/q · Fleet console keys · q — `q` quits the fleet console with exit 0.
contract/fleet-console-keys/unbound · Fleet console keys · any other key — Every key other than `q` and Ctrl-C is unbound and ignored; the end of the console's input also ends it with exit 0.
contract/fleet-console-view/line-cut · Fleet console view · line cut — A console line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape; an unwrapped NAME is not cut separately in the console.
contract/fleet-console-view/more-line · Fleet console view · more line — A fleet taller than the console's body rows ends in a line of the form `... <n> more`.
contract/viola-console-refusals/mid-run-home-widened · viola console refusals · home widened mid-run — A home widened while the console runs ends it with exit 1 and the home-state `unable:` / `hint:` pair after the terminal is restored; a board directory that cannot be listed ends it as `error: internal error`.
contract/fleet-console-view/captions · Fleet console view · caption row — The console's caption row is the board's six captions, then a seventh, `LAST`.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
N/A — viola has no web or graphical frontend, so no frontend expression level.
overview/brand-identity — The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.

## color-palette · ## Color Palette
Surface Scale (elevation hierarchy) › N/A — viola renders no page, so no surface scale. ¶ Text Hierarchy › N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli). ¶ Semantic Colors › N/A — viola renders no page, so no colour states; the cli's status guards below stand.

## typography · ## Typography
N/A — viola renders no page; the cli's type is the terminal's (below).

## spacing · ## Spacing
N/A — viola renders no page; the cli's spacing is the terminal grid (below).

## depth-strategy · ## Depth Strategy
N/A — viola renders no page, so no depth tokens; the cli draws no depth.

## border-radius · ## Border Radius
N/A — viola renders no page, so no radius tokens.
absence/rendered-page — A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's

## surface-web-spa · ## Surface: web-spa
absence/web-page-surface — A web page or web surface of any kind

## surface-cli · ## Surface: cli · twin surface
absence/cli-tape — A tape in the CLI
name/command/answer — answer
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/up — up
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
