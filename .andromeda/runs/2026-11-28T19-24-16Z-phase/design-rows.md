# design-system · the rows that bind 2026-11-28-the-fleet-on-the-founder-s-host — picked 192 of 381 (124 by every draw) · rule any · pin 5476

## brand-identity · ## Brand Identity
decision/bare-strips-never-target · Unwrapped sessions are never a target — Wrapped instances sit in a holder; unwrapped sessions are bare strips, visible but never a `target`.
decision/fixed-phraseology-vocabulary · Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.
decision/turn-ended-never-fills-readback · Turn-ended never fills the readback box — A `turn-ended` event never fills a readback box.
contract/readback-box/cli-mirror · Readback box · CLI mirror — The CLI mirrors the box states as `[RB]` / `[  ]` / `[/ ]`.
contract/readback-box/unconfirmable · Readback box · unconfirmable — The printed word `unconfirmable` reuses the open drawing and is never filled.
contract/send-readback/prompt · Send readback · prompt send — A prompt send counts, and its readback box fills, only once the matching `prompt-submitted` reads it back: the submitted prompt equals the sent text or the sent text with its trailing run of ASCII space, TAB, CR or LF removed; a prompt trimmed at the start does not match.
contract/send-readback/unable · Send readback · unable — A send not read back is reported as `unable` plus a typed reason.
decision/pause-release-transfer-control · Pause and release as control transfer — `viola pause` is the human's "I have control" and `viola release` is "you have control".
decision/readback-box · The readback box as signature element — The readback box is a fixed-width, 1px-ruled square cell whose one place is the CLI mirror, with three drawn states each also printed as a word.

## color-palette · ## Color Palette
decision/refused-send-folds-into-send-line · Refused send folds into its send line — A refused send folds into its send line, never a separate line.

## typography · ## Typography
decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density. · except: the fleet console's editing line echoes the typed text as typed, UTF-8 included, controls escaped

## spacing · ## Spacing
decision/cli-two-space-columns · Two spaces between CLI columns — CLI spacing follows the fixed-width terminal grid, with two spaces between columns.

## iconography · ## Iconography
contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/api-error-turn-end-shown-as-turn-end · API-error turn end shown as any turn end — A turn the CLI ends on an API error shows like any turn end: STATUS reads `idle`, `wait` reads `turn-ended`, no distinct status, colour or hint marks it, the only trace is one closed cause word of nine in `--json` and MCP `wait` event data, and the CLI's error text is shown nowhere. · guard: list_status_reads_idle_after_an_api_error_turn_end
decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
decision/columns-never-read · COLUMNS is never read — Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
decision/fleet-console-not-verb-output · Fleet console governed by no verb-output rule — The terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule.
decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `console` acts on its selected strip only through the CLI verbs it runs as children, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/list-rows-escape-all-controls · list rows escape every control character — Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line. · why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified · except: `wait` / `last` message text keeps `\n` and `\t`
decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or file only — Prompt text comes only from stdin or `--file`.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
decision/truncate-only-unwrapped-name · Only an unwrapped NAME is truncated — Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/2 · CLI exit codes · 2 — Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`), `UNUSABLE_TEXT` line (unusable `viola send`) or `PAIR_AFTER_DASHES` line (a pair after `--` of `run` or `revive`) on stderr, whatever `--json` says; no document.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up`, `verify` and `console` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run`, `revive` or `adopt` prints a fixed-message `unable:` line and one hint per cause; `revive`'s and `adopt`'s causes are read in a fixed order that stops at the first.
contract/cli-strict-modes-refusal/instance-state-pair · CLI strict-modes refusal · instance-state pair — The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
contract/cli-verb-output/up · CLI verb output · viola up — `viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
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
contract/strict-modes-refusal/exit-21 · Instance strict-modes refusal · exit 21 — A refused strict-modes check of the driven instance by `viola link` / `unlink`, or of the instance of `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch`, is made before any frame, exits 21, and writes no frame and no line.
contract/viola-last-output/header · viola last output · header — `last` prints `last  <name>  <kind> <time>` on stderr, TTY only.
contract/viola-last-output/message · viola last output · message — `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-run-output/child-started · viola run and revive output · child started — `run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.
contract/viola-send-mirror/open · viola send readback mirror · open — `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
contract/viola-send-mirror/read-back · viola send readback mirror · read back — `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
contract/viola-send-mirror/unable · viola send readback mirror · unable — `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
contract/viola-send-mirror/unconfirmable · viola send readback mirror · unconfirmable — `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
contract/viola-up-outcome/already-live · viola up outcome words · already-live — `already-live` is one of the nine closed outcome words of `viola up`: the instance is live and is not re-opened.
contract/viola-up-outcome/cwd-missing · viola up outcome words · cwd-missing — `cwd-missing` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/live · viola up outcome words · live — `live` is one of the nine closed outcome words of `viola up`: the spawned instance came up live, and its line carries the placement word.
contract/viola-up-outcome/no-session · viola up outcome words · no-session — `no-session` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/not-live · viola up outcome words · not-live — `not-live` is one of the nine closed outcome words of `viola up`: a spawned instance that did not come up live; a line reading it makes `up` exit 1.
contract/viola-up-outcome/not-taken · viola up outcome words · not-taken — `not-taken` is one of the nine closed outcome words of `viola up`: a name given that the scan did not find, or a named instance whose heartbeat is not older than the boot; a line reading it makes `up` exit 1.
contract/viola-up-outcome/session-live · viola up outcome words · session-live — `session-live` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/stale-heartbeat · viola up outcome words · stale-heartbeat — `stale-heartbeat` is one of the nine closed outcome words of `viola up`: the instance reads stale and is not re-opened.
contract/viola-up-outcome/would-open · viola up outcome words · would-open — `would-open` is one of the nine closed outcome words of `viola up`: a `--dry-run` plan line for an instance that would be spawned.
contract/viola-up-output/dry-run · viola up output · --dry-run — `viola up --dry-run` prints the plan as `would-open` lines and spawns nothing.
contract/viola-up-output/line-order · viola up output · line order — Every `viola up` line that is not a spawn comes first, in name order, then the spawned or `would-open` lines in spawn order, parents before children.
contract/viola-wait-output/dialog-form · viola wait output · dialog form — `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
contract/viola-wait-output/event-form · viola wait output · event form — `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.
contract/viola-wait-output/waiting-line · viola wait output · waiting line — `wait` prints one static `waiting: <name>` line on stderr, TTY only, before its one stdout result line.
decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`). · except: the fleet console's editing line echoes the typed text as typed, UTF-8 included, controls escaped
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/linux-terminal-platform · Linux terminals as the CLI platform — The cli targets Linux terminals: the core on any current distribution, CI on `ubuntu-latest`.
decision/no-hint-for-faults · No hint for faults and unknown — `error: wrapper fault` (exit 20), `error: internal error` (exit 1) and `unknown` (exit 14) print no hint line; a fault's human line holds the code only and further detail goes to `diagnostics/`. · why: Faults are not refusals; the detail of `unknown` is a code word or nothing, so a hint would have to guess.
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
decision/permission-request-question-held · PermissionRequest question held for an answer — A question first raised by a PermissionRequest is held as a pending dialog and answered by `viola answer` like any dialog; a human key, an expired hold or an unverified CLI leave it to the human.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern. · except: the fleet console's static running line while an action child runs · provisional
contract/cli-callers/human · CLI callers · human on a TTY — The human calls the cli on a TTY.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/10-14 · CLI exit codes · 10 to 14 — An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/cli-surface/output-streams · CLI surface · stdout/stderr — The cli surface is the stdout and stderr of the `viola` binary.
contract/origin-word/agent · origin word · agent — One of the two closed origin words: an open made from inside a wrapped session.
contract/origin-word/cli · origin word · cli — One of the two closed origin words: an open made from a human shell.
contract/strict-modes-refusal/human-output · Instance strict-modes refusal · unable: / hint: pair — The refusal prints the instance-state `unable:` / `hint:` pair naming that instance, as `viola list`'s does.
contract/strict-modes-refusal/json-document · Instance strict-modes refusal · --json document — Under `--json` the refusal prints the `instance-unreachable` document with `detail` `strict-modes-failed`, and stderr stays empty.

## surface · ## Surface: cli · twin surface-cli
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document. · why: The child has held the terminal and viola writes nothing past it. · except: `viola adopt`'s not-resumed end writes its fixed pair or `--json` document after the child released the terminal; `run` and `revive` still write nothing past the child.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`, `empty-text`, `paste-cut`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; under `--json` the refusal object.
contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the seven frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/exit-21-json-cause/unwrapped-name · exit 21 --json causes · unwrapped-name — With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent or a regular file reads `unwrapped-name`; a symbolic link in its place is refused by the strict-modes check and reads `strict-modes-failed`.
contract/fixed-refusal-pairs/boot-time-unread · fixed refusal pairs · boot-time-unread pair — When the host boot time cannot be read, `viola up` selects nothing and prints its own fixed `unable:` / `hint:` pair, or under `--json` an exit-1 document with the code `boot-time-unread`.
contract/human-tables/width-source · human tables · width source — The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
contract/refusal-hints/unwrapped-name · refusal hints · unwrapped-name — When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
contract/viola-revive-json-codes/already-live · viola revive --json exit-1 codes · already-live — `already-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/cwd-missing · viola revive --json exit-1 codes · cwd-missing — `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/no-session · viola revive --json exit-1 codes · no-session — `no-session` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/session-live · viola revive --json exit-1 codes · session-live — `session-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/stale-heartbeat · viola revive --json exit-1 codes · stale-heartbeat — `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
contract/viola-revive-refusals/home-tree-check · viola revive refusals · home-tree check — The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
contract/viola-revive-refusals/no-session · viola revive refusals · no-session — `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/no-such-id · viola revive refusals · no logged session with that id — An `--id` the log does not hold prints `unable: <name> has no logged session with that id` then `hint: viola revive <name> --list`.
contract/viola-revive-refusals/order · viola revive refusals · order — `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
contract/viola-revive-refusals/session-live · viola revive refusals · session-live — When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive-refusals/strict-modes-no-log · viola revive refusals · strict-modes — `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/home-tree-check · viola run start refusals · home-tree check — At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
contract/viola-run-start-refusals/stale-heartbeat · viola run start refusals · stale-heartbeat — A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
contract/viola-up-output/not-live-hint · viola up output · not-live hint — After its lines, `viola up` writes one fixed stderr `hint:` line when any line reads `not-live`, and none under `--json`.
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern. · except: the fleet console's static running line while an action child runs · provisional
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/exit-21-json-cause/endpoint-lost · exit 21 --json causes · endpoint-lost — A wrapper lost with the call open reads `endpoint-lost`.
contract/exit-21-json-cause/instance-dead · exit 21 --json causes · instance-dead — With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
contract/exit-21-json-cause/server-verify-failed · exit 21 --json causes · server-verify-failed — A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the nine frame-writing verbs read `server-verify-failed`.
contract/exit-21-json-cause/socket-dir-refused · exit 21 --json causes · socket-dir-refused — `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
contract/frame-verb-refusals/strict-modes · frame verb refusals · strict-modes — A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release`, `switch` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
contract/log-detail-codes/server-verify-failed · log detail codes · server-verify-failed — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
contract/refusal-hints/not-delivered-control-character · refusal hints · not-delivered · control-character — `the text contains a control character (only LF, CR, TAB are allowed)`
contract/refusal-hints/not-delivered-empty-text · refusal hints · not-delivered · empty-text — `the text is empty once its trailing newlines are removed; send a text with content`
contract/refusal-hints/not-delivered-input-not-ready · refusal hints · not-delivered · input-not-ready — `<name> was not ready for input; send again, and if it repeats a human must look at the session`
contract/refusal-hints/not-delivered-no-prompt-submitted · refusal hints · not-delivered · no-prompt-submitted — `<name> did not submit the prompt; check it, then send again`
contract/refusal-hints/not-delivered-paste-cut · refusal hints · not-delivered · paste-cut — `<name> did not take the whole text in time; a human must look at the session before the next send`, printed in human mode only.
contract/refusal-hints/not-delivered-turn-running · refusal hints · not-delivered · turn-running — `a turn is running; viola wait <name> first`
contract/refusal-hints/not-delivered-unknown-dialog · refusal hints · not-delivered · unknown-dialog — `that dialog is not pending; viola list shows the current DIALOG`
contract/refusal-hints/not-running · refusal hints · not-running — When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
contract/refusal-hints/socket-dir · refusal hints · socket-dir — The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-list-board/failed-agents-read · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
contract/viola-list-board/strict-modes-scan · viola list board · strict-modes scan — The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
contract/viola-revive-json-codes/pinned-hash-mismatch · viola revive --json exit-1 codes · pinned-hash-mismatch — `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
contract/viola-revive-json-codes/squatted-name · viola revive --json exit-1 codes · squatted-name — `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/strict-modes-failed · viola revive --json exit-1 codes · strict-modes-failed — `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-refusals/socket-dir · viola revive refusals · socket directory — A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-run-start-refusals/squatted-name · viola run start refusals · squatted-name — `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
decision/no-optimistic-send-confirmation · No optimistic send confirmation — The readback box stays `open` until the matching `prompt-submitted` or the command's post-condition, then fills or is struck with `unable` plus the typed reason; no spinner, green mark or toast. · why: A clearance does not count until it is read back.
decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.

## self-validation-protocol · ## Self-Validation Protocol
decision/readback-box-signature-check · Readback box signature check — Before UI output is presented, the readback box must be present and drawn the same in its one place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`.

## surface-console · ## Surface: console
decision/console-actions-run-cli-verbs · Fleet console actions run the CLI verbs — The fleet console's j, k and the arrows move the selection, and l last, p pause, r release, f focus, v revive (`viola up`), s send, m switch, a answer and o open each run on the selected row as its CLI verb in a child, one at a time. · provisional
decision/console-result-rows-are-verb-output · Console result rows are the verb's own output — An action child's output lines are the fleet console's result rows, each escaped; a refusal shows as the verb's own unable lines, never a console-made message. · provisional
contract/fleet-console-keys/answer · Fleet console keys · answer — `a` on a pending permission opens `answer <name> permission? y allow  n deny`, on a plan `answer <name> plan? y approve  n revise`; Escape leaves it answering nothing.
contract/fleet-console-keys/confirm · Fleet console keys · confirm — Release and revive ask a confirm naming the verb and the session; `y` runs it, `n` or Escape leaves it and runs nothing.
contract/fleet-console-keys/editing-line · Fleet console keys · editing line — `s`, `m` and `o` open one editing line with the prompt `send <name>: `, `switch <name>: ` or `open child of <name>: `; Enter hands the text to the verb, Escape cancels and runs nothing.
contract/fleet-console-keys/q · Fleet console keys · q — `q` quits the fleet console with exit 0.
contract/fleet-console-keys/question · Fleet console keys · answer on a question — `a` on a pending question draws the one line `<name>  question - answer it in the session (f)` and runs nothing; no question or option text is read or shown.
contract/fleet-console-keys/unbound · Fleet console keys · any other key — Every key the fleet console binds to nothing in its mode is unbound and ignored, `a` with nothing pending included; the end of the console's input also ends it with exit 0.
contract/fleet-console-view/editing-line-width · Fleet console view · editing line width — The editing line echoes the text as typed, UTF-8 included, controls escaped, each scalar 1, 2 or 0 columns by unicode-width; past the edge it draws the prompt, `...`, and the longest tail that fits, a straddling wide scalar left out.
contract/fleet-console-view/line-cut · Fleet console view · line cut — A console line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape; an unwrapped NAME is not cut separately in the console.
decision/console-one-action-at-a-time · One console action at a time — While a fleet console action child runs, a static running line stands and any other action key reads busy; no second child starts.
contract/fleet-console-view/selection · Fleet console view · selection — The selected row is marked by its gutter alone; the selection moves over wrapped rows only, and none stands on a board with no wrapped row.

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

## surface-console · ## Surface: console
absence/console-adopt — An adopt action in the fleet console
