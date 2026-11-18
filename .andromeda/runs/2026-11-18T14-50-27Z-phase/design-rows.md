# design-system · the rows that bind 2026-11-18-browser-tools-prompt-modal — picked 82 of 316 (68 by every draw) · rule any · pin 2742

## brand-identity · ## Brand Identity
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — An unstamped CLI build runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version stamped by `viola verify` prints `verified`.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
contract/send-readback/unconfirmable · Send readback · unconfirmable — A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so does a `/compact` send on an unverified CLI, and a `/mcp` send on an unverified CLI or in a session that declared no transcript.
contract/send-readback/compact · Send readback · /compact — On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
contract/send-readback/mcp · Send readback · /mcp — On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.

## color-palette · ## Color Palette
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).

## surface-cli · ## Surface: cli · twin surface
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-message-escaping/json · CLI message-mode escaping · --json — `--json` output stays serde-escaped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
contract/viola-run-output/child-started · viola run and revive output · child started — `run` and `revive` print nothing once the child starts.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
contract/viola-verify-output/stamps-refused · viola verify output · stamps.json update refused — A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
contract/viola-verify-output/stopped-or-refused · viola verify output · stopped or refused run — A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail` like every relied row, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.

## surface · ## Surface: cli · twin surface-cli
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`) and never reused; a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-unknown-dialog · refusal hints · not-delivered · unknown-dialog — `that dialog is not pending; viola list shows the current DIALOG`
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
contract/viola-verify-refusals/interrupted · viola verify refusals · interrupted run — A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
contract/viola-verify-refusals/stamps-update · viola verify refusals · stamps.json update — A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
contract/viola-verify-refusals/unclean-fixture · viola verify refusals · unclean fixture — `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-catch-site/json-internal-error · CLI catch site · --json internal error — Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.

## self-validation-protocol · ## Self-Validation Protocol
decision/readback-box-signature-check · Readback box signature check — Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.

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
