# design-system · the rows that bind 2026-11-21-key-after-a-resize — picked 110 of 324 (71 by every draw) · rule any · pin 3781

## brand-identity · ## Brand Identity
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/dialog-pending-cocked-strip · Dialog-pending shown as a cocked strip — A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.
contract/flight-progress-strip/dialog · Flight-progress strip · DIALOG — DIALOG is the fifth of the six fixed fields.
contract/flight-progress-strip/no-reading · Flight-progress strip · field with no reading — A field with no reading prints `unknown` in its box, never a shimmer or a guess.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
contract/send-readback/unconfirmable · Send readback · unconfirmable — A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/pause-release-transfer-control · Pause and release as control transfer — `viola pause` is the human's "I have control" and `viola release` is "you have control".
contract/flight-progress-strip/cli · Flight-progress strip · CLI — CLI is the sixth and last of the six fixed fields.
contract/send-readback/clear · Send readback · /clear — A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
contract/send-readback/compact · Send readback · /compact — On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
contract/send-readback/context · Send readback · /context — On a verified CLI a `/context` send counts, and reads back, only once one new local-command stdout tag appears in the session transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads `unconfirmable`, exit 0.
contract/send-readback/mcp · Send readback · /mcp — On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.

## color-palette · ## Color Palette
decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
contract/colour-palette/accent · Colour palette · Accent #D97706 — The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.

## iconography · ## Iconography
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/viola-run-output/child-started · viola run and revive output · child started — `run` and `revive` print nothing once the child starts.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or file only — Prompt text comes only from stdin or `--file`.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/auto-verify-line/did-not-verify · viola run and revive automatic-verify line · did not verify — A start whose automatic verify ran and did not leave a green stamp prints the fixed did-not-verify line naming the parsed claude version.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-message-escaping/json · CLI message-mode escaping · --json — `--json` output stays serde-escaped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.

## surface · ## Surface: cli · twin surface-cli
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` prints no document. · why: The child has held the terminal and viola writes nothing past it.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/refusal-hints/human-typing · refusal hints · human-typing — `the human has the wheel; send again after the human hands it back`
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-no-prompt-submitted · refusal hints · not-delivered · no-prompt-submitted — `<name> did not submit the prompt; check it, then send again`
contract/refusal-hints/not-delivered-turn-running · refusal hints · not-delivered · turn-running — `a turn is running; viola wait <name> first`
contract/refusal-hints/not-delivered-unknown-dialog · refusal hints · not-delivered · unknown-dialog — `that dialog is not pending; viola list shows the current DIALOG`
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/wheel-column-width-23 · WHEEL column padded to 23 — The board's WHEEL column is padded to 23 characters. · why: Ratified.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/peer-reply-screening/code-word · peer reply screening · code word — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.
decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.

## self-validation-protocol · ## Self-Validation Protocol
decision/readback-box-signature-check · Readback box signature check — Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
decision/squint-test · Squint test — Before UI output is presented, hierarchy must still be perceivable with blurred eyes and nothing may jump out harshly. · except: `open` and `unconfirmable` readback boxes look identical under the squint, and only the word cell tells them apart, as intended.

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
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
