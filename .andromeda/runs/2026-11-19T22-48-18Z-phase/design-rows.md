# design-system · the rows that bind 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel — picked 110 of 318 (68 by every draw) · rule any · pin 3197

## brand-identity · ## Brand Identity
decision/dialog-pending-cocked-strip · Dialog-pending shown as a cocked strip — A `dialog_pending` session keeps its slot and shows the word `DIALOG` plus the dialog kind, the word in arrival-holder amber on the CLI.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/pause-release-transfer-control · Pause and release as control transfer — `viola pause` is the human's "I have control" and `viola release` is "you have control".
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.
contract/flight-progress-strip/dialog · Flight-progress strip · DIALOG — DIALOG is the fifth of the six fixed fields.
contract/flight-progress-strip/no-reading · Flight-progress strip · field with no reading — A field with no reading prints `unknown` in its box, never a shimmer or a guess.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.
contract/handoff/transfer-marker · Handoff display · transfer marker — Transfer of control is shown as a `driver -> driven` transfer marker.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
contract/flight-progress-strip/cli · Flight-progress strip · CLI — CLI is the sixth and last of the six fixed fields.
contract/send-readback/unconfirmable · Send readback · unconfirmable — A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.

## color-palette · ## Color Palette
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
contract/colour-palette/accent · Colour palette · Accent #D97706 — The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.

## typography · ## Typography
decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.

## iconography · ## Iconography
contract/cli-transfer-arrow/ascii-arrow · CLI transfer arrows · -> — The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`).
contract/cli-glyphs/arrow · CLI glyphs · -> — The arrow glyph is `->`, with no inbound form.
contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-verb-output/pause · CLI verb output · viola pause — `viola pause <name>` prints `<name>  wheel human  manual-pause  I have control`.
contract/cli-verb-output/release · CLI verb output · viola release — `viola release <name>` prints `<name>  wheel driver  you have control`.
contract/viola-list-table/columns · viola list table · columns — Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
contract/viola-run-output/child-started · viola run and revive output · child started — `run` and `revive` print nothing once the child starts.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
contract/viola-wait-output/dialog-form · viola wait output · dialog form — `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/cli-colour-decision-order/viola-hook · CLI colour decision order · viola hook — Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
contract/cli-colour-tokens/attention · CLI colour tokens · attention — Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
contract/cli-message-escaping/json · CLI message-mode escaping · --json — `--json` output stays serde-escaped.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/hook-verb/exit-code · The hook verb · exit code — `hook` always exits 0.
contract/hook-verb/stderr · The hook verb · stderr — `hook` writes nothing to stderr.
contract/mcp-verb/stdout · The mcp verb · stdout — `mcp` writes only MCP frames on stdout.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-verify-output/browser-tools-line · viola verify output · browser-tools line — After a successful `stamps.json` update and before the summary, a run that met the browser-tools prompt prints one plain ASCII stdout line, no SGR: `the browser-tools prompt stood at start; viola never answers it: answer it once in claude, then run viola verify again`.
contract/viola-verify-output/stopped-or-refused · viola verify output · stopped or refused run — A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.

## surface · ## Surface: cli · twin surface-cli
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/wheel-column-width-23 · WHEEL column padded to 23 — The board's WHEEL column is padded to 23 characters. · why: Ratified.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/refusal-hints/human-typing · refusal hints · human-typing — `the human has the wheel; send again after the human hands it back`
contract/refusal-hints/not-delivered-no-prompt-submitted · refusal hints · not-delivered · no-prompt-submitted — `<name> did not submit the prompt; check it, then send again`
contract/refusal-hints/not-delivered-unknown-dialog · refusal hints · not-delivered · unknown-dialog — `that dialog is not pending; viola list shows the current DIALOG`
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-list-board/unwrapped-row · viola list board · unwrapped row — One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`; a failing row prints no stderr word.
contract/wrapper-socket/refused-peer · wrapper socket · refused peer — A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-colour-tokens/stale · CLI colour tokens · stale — SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-turn-running · refusal hints · not-delivered · turn-running — `a turn is running; viola wait <name> first`
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-verify-refusals/unclean-fixture · viola verify refusals · unclean fixture — `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.
decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
decision/no-auto-sorting-strips · No auto-sorting of strips — Strips needing input or with recent sessions are never re-ranked to the top; a strip is cocked in place. · why: Re-ranking breaks the founder's memory of where each session lives, and the `DIALOG` word already says "needs you".
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.

## self-validation-protocol · ## Self-Validation Protocol
decision/readback-box-signature-check · Readback box signature check — Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.
decision/domain-not-statistical-design · Design from the product's domain — Before UI output is presented, an interface another AI would produce from a similar prompt fails; the interface must emerge from this product's domain exploration.
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
