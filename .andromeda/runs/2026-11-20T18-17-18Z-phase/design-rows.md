# design-system · the rows that bind 2026-11-20-harness-prefix-and-identity-floor-ledger-rows — picked 55 of 324 (40 by every draw) · rule any · pin 3504

## brand-identity · ## Brand Identity
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.

## iconography · ## Iconography
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.

## surface-cli · ## Surface: cli · twin surface
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/auto-verify-line/did-not-verify · viola run and revive automatic-verify line · did not verify — A start whose automatic verify ran and did not leave a green stamp prints the fixed did-not-verify line naming the parsed claude version.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/auto-verify-line/line-form · viola run and revive automatic-verify line · line form — A start prints exactly one of the five automatic-verify lines, each one stderr write, ASCII, with no SGR, holding only the parsed version and a closed word; none prints under `--json`.
contract/auto-verify-line/no-stamp · viola run and revive automatic-verify line · no stamp — A start whose automatic verify left no stamp prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`.
contract/auto-verify-line/verifying · viola run and revive automatic-verify line · verifying — A start that runs its automatic verify first prints the fixed verifying line naming the parsed claude version.
contract/cli-colour-decision-order/viola-hook · CLI colour decision order · viola hook — Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` refusal of `run`, `revive` or `verify` with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`, stderr empty.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/hook-verb/exit-code · The hook verb · exit code — `hook` always exits 0.
contract/hook-verb/stderr · The hook verb · stderr — `hook` writes nothing to stderr.
contract/viola-verify-output/stamps-refused · viola verify output · stamps.json update refused — A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/stopped-or-refused · viola verify output · stopped or refused run — A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.

## surface · ## Surface: cli · twin surface-cli
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`; a failing row prints no stderr word.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · CLI not found — `unable: the claude CLI was not found`, followed by its own hint, exit 1.
contract/viola-verify-refusals/home-tree-check · viola verify refusals · home-tree check — Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`, each that exists; a refusal prints no step line and no summary, only the home-state pair, and exits 1.
contract/viola-verify-refusals/interrupted · viola verify refusals · interrupted run — A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
contract/viola-verify-refusals/stamps-update · viola verify refusals · stamps.json update — A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
contract/viola-verify-refusals/unclean-fixture · viola verify refusals · unclean fixture — `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · version unreadable — `unable: the CLI version could not be read`, followed by its own hint, exit 1.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.

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
