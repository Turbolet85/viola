# design-system · the rows that bind 2026-11-15-live-round-on-the-trimmed-product — picked 69 of 313 (52 by every draw) · rule any · pin 1297

## brand-identity · ## Brand Identity
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — An unstamped CLI build runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version stamped by `viola verify` prints `verified`.

## surface-cli · ## Surface: cli · twin surface
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-colour-decision-order/viola-mcp · CLI colour decision order · viola mcp — Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
contract/mcp-verb/stdout · The mcp verb · stdout — `mcp` writes only MCP frames on stdout.
contract/revive-json-refusal-code/already-live · revive --json refusal code · already-live — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/stale-heartbeat · revive --json refusal code · stale-heartbeat — One of the closed set of codes a `revive --json` start refusal document carries.
contract/viola-run-output/child-started · viola run and revive output · child started — `run` and `revive` print nothing once the child starts.
contract/viola-verify-output/stamps-refused · viola verify output · stamps.json update refused — A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, ending `pass` or `fail`, counted against `MM`, the relied count.
contract/viola-verify-output/stopped-or-refused · viola verify output · stopped or refused run — A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/hook-verb/stderr · The hook verb · stderr — `hook` writes nothing to stderr.
contract/viola-wait-output/dialog-form · viola wait output · dialog form — `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
contract/viola-wait-output/event-form · viola wait output · event form — `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.

## surface · ## Surface: cli · twin surface-cli
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/stale-heartbeat · viola run start refusals · stale-heartbeat — A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · CLI not found — `unable: the claude CLI was not found`, followed by its own hint, exit 1.
contract/viola-verify-refusals/home-tree-check · viola verify refusals · home-tree check — Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`, each that exists; a refusal prints no step line and no summary, only the home-state pair, and exits 1.
contract/viola-verify-refusals/interrupted · viola verify refusals · interrupted run — A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
contract/viola-verify-refusals/stamps-update · viola verify refusals · stamps.json update — A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
contract/viola-verify-refusals/unclean-fixture · viola verify refusals · unclean fixture — `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · version unreadable — `unable: the CLI version could not be read`, followed by its own hint, exit 1.
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` prints no document. · why: The child has held the terminal and viola writes nothing past it.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`) and never reused; a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.

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
