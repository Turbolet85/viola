# design-system · the rows that bind 2026-11-24-viola-up — picked 174 of 344 (136 by every draw) · rule any · pin 4474

## brand-identity · ## Brand Identity
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight.
decision/stale-strip-dimmed-not-deleted · Stale strip dimmed, never deleted — A strip of `stale` liveness is dimmed to lamp-off, never deleted, and keeps its slot.

## color-palette · ## Color Palette
decision/amber-only-for-dialog-pending · Arrival amber for dialog-pending only — Arrival-holder amber #D97706 is the only attention colour, used for `dialog_pending` only. · except: never for errors, warnings, budget or hover
decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
contract/colour-palette/accent · Colour palette · Accent #D97706 — The only attention colour: the `DIALOG` word of a pending dialog in `viola list` human output, as truecolor or its 256- and 16-colour fallbacks; never used for errors, warnings or budget.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
decision/bold-only-name-column · Bold only for the NAME column — In the cli, bold is used only for the NAME column.
decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/flat-cli-verbs · Flat CLI verbs — CLI verbs are flat, each one lower-case word, with no nested subcommand groups. · except: `plugin install`
decision/hand-written-sgr-module · Hand-written SGR styling module — Styling is a small hand-written SGR module in the `viola` bin. · why: adds no dependency
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, and every other verb a person runs acts on one strip.
decision/list-refuses-board-on-strict-modes-failure · Strict-modes failure refuses the whole board — A failed strict-modes check of the home or any instance refuses the whole list board with exit 21, in the fixed unable and hint pair or, under --json, the instance-unreachable document with detail strict-modes-failed. · why: The MCP list tool reads the same refusal document through the shared board reader, and the list tests pin exit 21 and the document byte for byte.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/mcp-hidden-verb · mcp is a hidden verb — `mcp` and `hook` have no human design surface, and `mcp` appears in no `viola --help` row. · why: `mcp` is run by the plugin, never by a person.
decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/no-interactive-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager.
decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
decision/tty-detection-std-isterminal · TTY detection by std IsTerminal — TTY detection uses std `IsTerminal`.
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-callers/llm-driver · CLI callers · LLM driver sessions — LLM driver sessions call the cli via `--json` or MCP.
contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
contract/cli-exit-codes/20 · CLI exit codes · 20 — `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `open`, `focus` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/exit-21-cause/socket-dir-refused · exit 21 cause · socket-dir-refused — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/strict-modes-failed · exit 21 cause · strict-modes-failed — One of the closed set of causes named in exit 21's `detail`.
contract/help-output/verb-groups · viola --help output · verb groups — `viola --help` lists the served verbs as `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
contract/origin-word/cli · origin word · cli — One of the two closed origin words: an open made from a human shell.
contract/placement-word/malformed · placement word · malformed — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/oversize · placement word · oversize — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/placed · placement word · placed — One of the eight closed placement words `viola open` prints and the window record carries: the window was moved to the asked workspace.
contract/placement-word/refused · placement word · refused — One of the eight closed placement words `viola open` prints and the window record carries: the move was refused.
contract/placement-word/unplaced · placement word · unplaced — One of the eight closed placement words `viola open` prints and the window record carries: no workspace was asked.
contract/placement-word/unreachable · placement word · unreachable — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/unsupported · placement word · unsupported — One of the eight closed placement words `viola open` prints and the window record carries: off Hyprland, the open still live.
contract/placement-word/window-unseen · placement word · window-unseen — One of the eight closed placement words `viola open` prints and the window record carries: no window of the wrapper's own terminal was seen.
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
contract/strict-modes-refusal/human-output · Instance strict-modes refusal · unable: / hint: pair — The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-run-output/child-started · viola run and revive output · child started — `run` and `revive` print nothing once the child starts.
decision/clap-without-colour-feature · clap built without its colour feature — Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable. · why: every styled byte of human output is viola's own SGR module's
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
decision/truncate-only-unwrapped-name · Only an unwrapped NAME is truncated — Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
contract/auto-verify-line/did-not-verify · viola run and revive automatic-verify line · did not verify — A start whose automatic verify ran and did not leave a green stamp prints the fixed did-not-verify line naming the parsed claude version.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/auto-verify-line/line-form · viola run and revive automatic-verify line · line form — A start prints exactly one of the five automatic-verify lines, each one stderr write, ASCII, with no SGR, holding only the parsed version and a closed word; none prints under `--json`.
contract/auto-verify-line/no-stamp · viola run and revive automatic-verify line · no stamp — A start whose automatic verify left no stamp prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`.
contract/auto-verify-line/verifying · viola run and revive automatic-verify line · verifying — A start that runs its automatic verify first prints the fixed verifying line naming the parsed claude version.
contract/auto-verify-line/verifying-elsewhere · viola run and revive automatic-verify line · verifying elsewhere — A start that finds another start holding the verify attempt lock prints the fixed verifying-elsewhere line naming the parsed version, runs no verify and starts unverified.
contract/cli-colour-tokens/attention · CLI colour tokens · attention — Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
contract/cli-message-escaping/json · CLI message-mode escaping · --json — `--json` output stays serde-escaped.
contract/cli-strict-modes-refusal/instance-state-pair · CLI strict-modes refusal · instance-state pair — The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
contract/cli-strict-modes-refusal/plugin-install · CLI strict-modes refusal · viola plugin install — A refused strict-modes check of `viola plugin install` exits 21 with the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.
contract/cli-verb-output/focus · CLI verb output · viola focus — `viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace is ever printed or logged.
contract/exit-21-cause/endpoint-lost · exit 21 cause · endpoint-lost — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/instance-dead · exit 21 cause · instance-dead — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/server-verify-failed · exit 21 cause · server-verify-failed — One of the closed set of causes named in exit 21's `detail`.
contract/exit-21-cause/unwrapped-name · exit 21 cause · unwrapped-name — One of the closed set of causes named in exit 21's `detail`.
contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause` and `release`, is made before any frame, exits 21, and writes no frame and no line.
contract/strict-modes-refusal/json-document · Instance strict-modes refusal · --json document — Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.

## surface · ## Surface: cli · twin surface-cli
decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` prints no document. · why: The child has held the terminal and viola writes nothing past it.
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
contract/cli-catch-site/json-flag-scope · CLI catch site · --json flag scope — The catch site counts `--json` only among the arguments before a `--`, read from argv.
contract/cli-catch-site/json-internal-error · CLI catch site · --json internal error — Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `open`, `focus` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/exit-21-json-cause/socket-dir-refused · exit 21 --json causes · socket-dir-refused — `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/socket-dir · refusal hints · socket-dir — The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-help/verb-groups · viola --help · verb groups — `viola --help` lists the served verbs only, grouped `board: list, focus` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: open, run, revive, verify, plugin install`, then `Options:`.
contract/viola-list-board/strict-modes-refusal · viola list board · strict-modes refusal — A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-revive-json-codes/already-live · viola revive --json exit-1 codes · already-live — `already-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/cwd-missing · viola revive --json exit-1 codes · cwd-missing — `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
contract/viola-revive-json-codes/no-session · viola revive --json exit-1 codes · no-session — `no-session` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/pinned-hash-mismatch · viola revive --json exit-1 codes · pinned-hash-mismatch — `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/session-live · viola revive --json exit-1 codes · session-live — `session-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
contract/viola-revive-json-codes/squatted-name · viola revive --json exit-1 codes · squatted-name — `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/stale-heartbeat · viola revive --json exit-1 codes · stale-heartbeat — `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/strict-modes-failed · viola revive --json exit-1 codes · strict-modes-failed — `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
contract/viola-revive-refusals/home-tree-check · viola revive refusals · home-tree check — The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
contract/viola-revive-refusals/no-session · viola revive refusals · no-session — `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/order · viola revive refusals · order — `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
contract/viola-revive-refusals/session-live · viola revive refusals · session-live — When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive-refusals/socket-dir · viola revive refusals · socket directory — A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
contract/viola-revive-refusals/strict-modes-no-log · viola revive refusals · strict-modes — `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/home-tree-check · viola run start refusals · home-tree check — At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-run-start-refusals/squatted-name · viola run start refusals · squatted-name — `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
contract/viola-run-start-refusals/stale-heartbeat · viola run start refusals · stale-heartbeat — A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
decision/instance-unreachable-hint-per-cause · One instance-unreachable hint per cause — The `instance-unreachable` refusal (exit 21) prints one hint per cause. · why: The exit code is shared by several causes.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
contract/cli-colour-tokens/callsign · CLI colour tokens · callsign — Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/refusal-hints/unwrapped-name · refusal hints · unwrapped-name — When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
contract/viola-list-board/failed-agents-read · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
contract/wrapper-fault-detail/message · wrapper fault --json detail · message — Exit 20's `--json` `detail` is closed for the five known codes, and `message` is each code's fixed text.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-shimmer-or-glow-theme · No skeleton shimmer or glow theme — No skeleton shimmer and no terminal-green or neon-glow theme; missing values print `unknown` in their box at once. · why: Nothing glows in a tower cab unless the lamp is on it.
decision/no-traffic-light-status-dots · No traffic-light status dots — Status is never a green/yellow/red dot; `stale` is a coasting lamp-off track, not red, and the only colour state is the cocked strip's amber beside its `DIALOG` word. · why: A strip prints its status word in a fixed column.

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
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
