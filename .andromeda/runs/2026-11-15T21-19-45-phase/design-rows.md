# design-system · the rows that bind 2026-11-15-api-error-turn-end — picked 67 of 313 (37 by every draw) · rule any · pin 1610

## brand-identity · ## Brand Identity
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/pause-release-transfer-control · Pause and release as control transfer — `viola pause` is the human's "I have control" and `viola release` is "you have control".
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — An unstamped CLI build runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version stamped by `viola verify` prints `verified`.
contract/flight-progress-strip/status · Flight-progress strip · STATUS — STATUS is the third of the six fixed fields.
contract/flight-progress-strip/wheel · Flight-progress strip · WHEEL — WHEEL is the fourth of the six fixed fields.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
decision/stable-session-slots · Stable session slots — Every session keeps a slot in the strip bay stable across state changes: wrapped sessions in name order, then unwrapped ones in name order; nothing is re-sorted by state, and a slot shifts only when a session arrives or leaves.
contract/flight-progress-strip/no-reading · Flight-progress strip · field with no reading — A field with no reading prints `unknown` in its box, never a shimmer or a guess.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.

## iconography · ## Iconography
decision/list-six-fixed-columns · viola list has six fixed columns — `viola list` carries no link column, so its six columns stay fixed.
contract/viola-list/columns · viola list output · columns — `viola list` carries no link column in v1, so its six columns stay fixed.

## surface-cli · ## Surface: cli · twin surface
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-colour-decision-order/viola-hook · CLI colour decision order · viola hook — Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/hook-verb/exit-code · The hook verb · exit code — `hook` always exits 0.
contract/hook-verb/stderr · The hook verb · stderr — `hook` writes nothing to stderr.
contract/viola-list-table/columns · viola list table · columns — Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
contract/viola-wait-output/event-form · viola wait output · event form — `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`) or `UNUSABLE_TEXT` line (unusable `viola send`) on stderr, whatever `--json` says; no document.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.

## surface · ## Surface: cli · twin surface-cli
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
decision/wheel-column-width-23 · WHEEL column padded to 23 — The board's WHEEL column is padded to 23 characters. · why: Ratified.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
contract/refusal-hints/not-delivered-turn-running · refusal hints · not-delivered · turn-running — `a turn is running; viola wait <name> first`
contract/viola-list-board/status · viola list board · STATUS — A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
contract/viola-verify-refusals/unclean-fixture · viola verify refusals · unclean fixture — `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-list-board/skip-counts · viola list board · skip counts — Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.

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
