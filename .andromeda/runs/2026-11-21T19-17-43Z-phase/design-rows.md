# design-system · the rows that bind 2026-11-21-mcp-calls-bounded — picked 76 of 324 (38 by every draw) · rule any · pin 3707

## brand-identity · ## Brand Identity
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
contract/send-readback/unconfirmable · Send readback · unconfirmable — A local command with no post-condition (`/remote-control`, `/cost`, `/usage`) stays `unconfirmable`; so do a `/compact` send on an unverified CLI, a `/mcp` send on an unverified CLI or with no declared transcript, and a `/context` send on an unverified CLI or before its transcript exists.
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.
contract/send-readback/clear · Send readback · /clear — A `/clear` send counts only once its post-condition, a `session-start` with cause `clear` (a new session), is observed.
contract/send-readback/compact · Send readback · /compact — On a verified CLI a `/compact` send counts only once a `session-start` of cause `compact` carrying the session's own id is observed.
contract/send-readback/context · Send readback · /context — On a verified CLI a `/context` send counts, and reads back, only once one new local-command stdout tag appears in the session transcript with no prompt submitted; on an unverified CLI or before its transcript exists it reads `unconfirmable`, exit 0.
contract/send-readback/mcp · Send readback · /mcp — On a verified CLI a `/mcp` send counts, and reads back, only once a new local-command stdout tag appears in the session transcript after the send; it prints no new word.

## color-palette · ## Color Palette
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.

## surface-cli · ## Surface: cli · twin surface
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-colour-decision-order/viola-mcp · CLI colour decision order · viola mcp — Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
contract/cli-exit-codes/20 · CLI exit codes · 20 — `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
contract/mcp-verb/stdout · The mcp verb · stdout — `mcp` writes only MCP frames on stdout.
contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/list-refuses-board-on-strict-modes-failure · Strict-modes failure refuses the whole board — A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-strict-modes-refusal/instance-state-pair · CLI strict-modes refusal · instance-state pair — The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-wait-output/dialog-form · viola wait output · dialog form — `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
contract/viola-wait-output/event-form · viola wait output · event form — `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
contract/viola-wait-output/waiting-line · viola wait output · waiting line — `wait` prints one static `waiting: <name>` line on stderr, TTY only, before its one stdout result line.

## surface · ## Surface: cli · twin surface-cli
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/viola-link-output/refused-pair · viola link and unlink output · refused pair — A pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself) exits 20: `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
contract/viola-verify-output/summary-line · viola verify output · summary line — The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`; a failing row prints no stderr word.
contract/wrapper-fault-detail/message · wrapper fault --json detail · message — Exit 20's `--json` `detail` is closed for the five known codes, and `message` is each code's fixed text.
contract/wrapper-fault-detail/other-invalid-params · wrapper fault --json detail · other -32602 — Any other `-32602` reads `invalid params` with null `data`.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/viola-list-board/failed-agents-read · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.

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
