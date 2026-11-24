# design-system · the rows that bind 2026-11-24-switching-a-session-s-model-and-effort — picked 131 of 360 (106 by every draw) · rule any · pin 4716

## brand-identity · ## Brand Identity
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.

## iconography · ## Iconography
decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.
decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).

## surface-cli · ## Surface: cli · twin surface
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/flat-cli-verbs · Flat CLI verbs — CLI verbs are flat, each one lower-case word, with no nested subcommand groups. · except: `plugin install`
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/list-refuses-board-on-strict-modes-failure · Strict-modes failure refuses the whole board — A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
contract/cli-exit-codes/20 · CLI exit codes · 20 — `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-message-escaping/json · CLI message-mode escaping · --json — `--json` output stays serde-escaped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/cli-strict-modes-refusal/instance-state-pair · CLI strict-modes refusal · instance-state pair — The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/exit-21-cause/endpoint-lost · exit 21 cause · endpoint-lost — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/instance-dead · exit 21 cause · instance-dead — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/server-verify-failed · exit 21 cause · server-verify-failed — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/socket-dir-refused · exit 21 cause · socket-dir-refused — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/strict-modes-failed · exit 21 cause · strict-modes-failed — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/unwrapped-name · exit 21 cause · unwrapped-name — One of the closed set of causes named in exit 21's `detail`.
contract/help-output/verb-groups · viola --help output · verb groups — `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, verify, plugin install`, then `Options:`.
contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
contract/strict-modes-refusal/human-output · Instance strict-modes refusal · unable: / hint: pair — The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
contract/strict-modes-refusal/json-document · Instance strict-modes refusal · --json document — Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.
decision/clap-without-colour-feature · clap built without its colour feature — Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable. · why: every styled byte of human output is viola's own SGR module's
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
contract/cli-exit-codes/1-adopt-start · CLI exit codes · 1 of viola adopt — A start refusal of `viola adopt` exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty.
contract/viola-focus-refusals/exit-21 · viola focus refusals · exit 21 — `viola focus` exits 21 with `instance-dead` or `unwrapped-name` for a name not live, and with `strict-modes-failed` for a home or instance failing strict modes, each before any compositor read.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.

## surface · ## Surface: cli · twin surface-cli
decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/instance-unreachable-hint-per-cause · One instance-unreachable hint per cause — The `instance-unreachable` refusal (exit 21) prints one hint per cause. · why: The exit code is shared by several causes.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-hint-for-unknown · No hint for unknown — The `unknown` refusal (exit 14) prints no hint line. · why: Its detail is a code word or nothing, and a hint would have to guess.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/exit-21-json-cause/endpoint-lost · exit 21 --json causes · endpoint-lost — A wrapper lost with the call open reads `endpoint-lost`.
contract/exit-21-json-cause/instance-dead · exit 21 --json causes · instance-dead — With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
contract/exit-21-json-cause/server-verify-failed · exit 21 --json causes · server-verify-failed — A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the eight frame-writing verbs read `server-verify-failed`.
contract/exit-21-json-cause/socket-dir-refused · exit 21 --json causes · socket-dir-refused — `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/exit-21-json-cause/unwrapped-name · exit 21 --json causes · unwrapped-name — With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent or a regular file reads `unwrapped-name`; a symbolic link in its place is refused by the strict-modes check and reads `strict-modes-failed`.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/log-detail-codes/server-verify-failed · log detail codes · server-verify-failed — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
contract/peer-reply-screening/code-word · peer reply screening · code word — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
contract/peer-reply-screening/non-string-detail · peer reply screening · non-string detail — A peer reply with a non-string `detail` is no known reply.
contract/peer-reply-screening/refusal-not-code-word · peer reply screening · refusal that is no code word — A peer's `refusal` that is no code word reads `unknown`.
contract/peer-reply-screening/string-detail-not-code-word · peer reply screening · string detail that is no code word — A peer's string `detail` that is no code word reads as no detail, null under `--json`.
contract/peer-reply-screening/unlisted-code-word · peer reply screening · code word outside the refusal list — A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
contract/refusal-hints/not-running · refusal hints · not-running — When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
contract/refusal-hints/socket-dir · refusal hints · socket-dir — The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/refusal-hints/unwrapped-name · refusal hints · unwrapped-name — When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-help/verb-groups · viola --help · verb groups — `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, adopt, up, verify, plugin install`, then `Options:`.
contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/viola-list-board/strict-modes-refusal · viola list board · strict-modes refusal — A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-revive-refusals/home-tree-check · viola revive refusals · home-tree check — The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
contract/viola-revive-refusals/socket-dir · viola revive refusals · socket directory — A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
contract/viola-run-start-refusals/home-tree-check · viola run start refusals · home-tree check — At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/cli-catch-site/json-flag-scope · CLI catch site · --json flag scope — The catch site counts `--json` only among the arguments before a `--`, read from argv.
contract/cli-catch-site/json-internal-error · CLI catch site · --json internal error — Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
contract/viola-link-output/printed-names · viola link and unlink output · printed names — The names printed are the verb's own parsed arguments, never names read back from the reply or the log.
contract/viola-link-output/since · viola link and unlink output · since — The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
contract/viola-revive-json-codes/strict-modes-failed · viola revive --json exit-1 codes · strict-modes-failed — `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-refusals/strict-modes-no-log · viola revive refusals · strict-modes — `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
contract/wrapper-socket/refused-peer · wrapper socket · refused peer — A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
decision/wheel-as-words · Wheel shown as words — The WHEEL field prints `human` / `driver`, never icons, emoji or a toggle; taking control is spoken with `viola pause`. · why: Taking control is never implied by a switch.

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
