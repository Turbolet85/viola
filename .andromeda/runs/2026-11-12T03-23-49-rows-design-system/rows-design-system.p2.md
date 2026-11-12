@row_add n=1 row=decision/clap-without-colour-feature area=surface-cli rec=1
name: clap built without its colour feature
rule: Argument parsing uses clap (derive) built without its `color` feature, so clap's help and usage output is plain in every mode and no dependency reads a colour or terminal variable.
why: every styled byte of human output is viola's own SGR module's
@row_add n=2 row=decision/tty-detection-std-isterminal area=surface-cli rec=2
name: TTY detection by std IsTerminal
rule: TTY detection uses std `IsTerminal`.
@row_add n=3 row=decision/hand-written-sgr-module area=surface-cli rec=3
name: Hand-written SGR styling module
rule: Styling is a small hand-written SGR module in the `viola` bin.
why: adds no dependency
@row_add n=4 row=contract/cli-colour-tokens/attention area=surface-cli rec=4,26
boundary: CLI colour tokens
item: attention
promise: Attention (arrival amber) colours the `DIALOG` word only, always followed by the kind word, at truecolor, 256-colour and 16-colour depth.
@row_add n=5 row=contract/cli-colour-tokens/dim area=surface-cli rec=4,26
boundary: CLI colour tokens
item: dim
promise: SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
@row_add n=6 row=contract/cli-colour-tokens/bold area=surface-cli rec=4,26
boundary: CLI colour tokens
item: bold
promise: NAME values are bold.
@row_add n=7 row=contract/cli-colour-tokens/reset area=surface-cli rec=4
boundary: CLI colour tokens
item: reset
promise: A reset follows every styled span.
@row_add n=8 row=decision/no-handoff-blue-in-cli area=surface-cli rec=4
name: No handoff blue in the CLI
rule: The CLI never uses the handoff blue colour.
@row_add n=9 row=decision/no-terminal-background area=surface-cli rec=4
name: No background colours
rule: CLI output never sets a background colour.
why: the terminal background belongs to the user
@row_add n=10 row=decision/ascii-mirror-words-always-print area=surface-cli rec=5
name: ASCII mirror marks are words, not glyphs
rule: A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
@row_add n=11 row=contract/cli-colour-decision-order/json area=surface-cli rec=6,5
boundary: CLI colour decision order
item: --json
promise: First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
@row_add n=12 row=contract/cli-colour-decision-order/viola-run area=surface-cli rec=7,5
boundary: CLI colour decision order
item: viola run
promise: Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
@row_add n=13 row=contract/cli-colour-decision-order/viola-hook area=surface-cli rec=8,5
boundary: CLI colour decision order
item: viola hook
promise: Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
@row_add n=14 row=contract/cli-colour-decision-order/viola-mcp area=surface-cli rec=8,5
boundary: CLI colour decision order
item: viola mcp
promise: Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
@row_add n=15 row=contract/cli-colour-decision-order/no-color area=surface-cli rec=9,5
boundary: CLI colour decision order
item: NO_COLOR
promise: Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
@row_add n=16 row=contract/cli-colour-decision-order/term-dumb area=surface-cli rec=10,5
boundary: CLI colour decision order
item: TERM=dumb
promise: Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
@row_add n=17 row=contract/cli-colour-decision-order/stdout-not-terminal area=surface-cli rec=11,5
boundary: CLI colour decision order
item: stdout is not a terminal
promise: Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
@row_add n=18 row=contract/cli-colour-depth/truecolor area=surface-cli rec=13,12
boundary: CLI colour depth
item: truecolor
promise: Where colour applies, the depth is chosen once per process, first match wins: truecolor if `COLORTERM` is `truecolor` or `24bit`.
@row_add n=19 row=contract/cli-colour-depth/256-colour area=surface-cli rec=14,12
boundary: CLI colour depth
item: 256-colour
promise: Where colour applies and truecolor did not match, the depth is 256-colour if `TERM` contains `256color`, chosen once per process.
@row_add n=20 row=contract/cli-colour-depth/16-colour area=surface-cli rec=15,12
boundary: CLI colour depth
item: 16-colour
promise: Where colour applies and neither truecolor nor 256-colour matched, the depth is the 16-colour fallback column, chosen once per process.
@row_none n=1 rec=16 type=S
@row_add n=21 row=contract/cli-streams/stdout area=surface-cli rec=17
boundary: CLI streams
item: stdout
promise: stdout carries results and data: tables, send/wait results, `last` text.
@row_add n=22 row=contract/cli-streams/stderr area=surface-cli rec=18
boundary: CLI streams
item: stderr
promise: stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
@row_add n=23 row=contract/cli-refusal-line/two-space-form area=surface-cli rec=18,34
boundary: CLI refusal line
item: two-space form
promise: A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
@row_add n=24 row=contract/cli-refusal-line/fixed-unable-form area=surface-cli rec=18
boundary: CLI refusal line
item: fixed unable form
promise: The exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed `unable: <text>` form.
@row_add n=25 row=contract/cli-refusal-line/hint area=surface-cli rec=34,18
boundary: CLI refusal line
item: hint
promise: A refusal is followed directly by one `hint:` line per reason, detail or cause.
@row_add n=26 row=decision/json-one-document-on-stdout area=surface-cli rec=19
name: --json result is one stdout document
rule: Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code.
except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
@row_add n=27 row=decision/silent-catch-sites area=surface-cli rec=19
name: Silent catch sites
rule: At the catch site, `run`, `revive`'s start, `mcp` and `hook` stay silent.
@row_add n=28 row=decision/no-hint-under-json area=surface-cli rec=19,34
name: No hint line under --json
rule: No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
@row_none n=2 rec=20 type=S
@row_add n=29 row=decision/truncate-only-unwrapped-name area=surface-cli rec=21,22
name: Only an unwrapped NAME is truncated
rule: Where stdout's own terminal width is known, human tables truncate only the NAME of unwrapped rows (`...`); otherwise nothing is truncated.
@row_add n=30 row=decision/columns-never-read area=surface-cli rec=21
name: COLUMNS is never read
rule: Terminal width comes only from stdout's own terminal; `COLUMNS` is never read and a piped stdout has no width.
@row_add n=31 row=absence/cli-tape area=surface-cli rec=21,28
capability: A tape in the CLI
@row_add n=32 row=decision/lines-never-wrapped area=surface-cli rec=22
name: Lines never wrapped
rule: CLI output lines are never wrapped.
@row_add n=33 row=decision/ascii-only-human-output area=surface-cli rec=23
name: ASCII-only human output
rule: Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
@row_none n=3 rec=24 type=S
@row_add n=34 row=contract/viola-list-table/columns area=surface-cli rec=25
boundary: viola list table
item: columns
promise: Under a BAY line the table has the columns NAME, LIVE, STATUS, WHEEL, DIALOG and CLI; unwrapped sessions follow a `-- UNWRAPPED - READ-ONLY --` line.
@row_add n=35 row=contract/viola-list-table/row-order area=surface-cli rec=27
boundary: viola list table
item: row order
promise: Rows are ordered by name.
@row_add n=36 row=contract/viola-list-table/bay-line area=surface-cli rec=28
boundary: viola list table
item: BAY line
promise: The BAY line is the ATIS without `resets` times (printed by `viola release <name> --budget`) and without a TAPE cell.
@row_add n=37 row=decision/expired-window-shown-expired area=surface-cli rec=28
name: Expired budget window shown as expired
rule: A budget window whose `resets_at` has passed still prints its figure followed by `expired`.
why: old information is shown as old
@row_add n=38 row=decision/list-rows-escape-all-controls area=surface-cli rec=29
name: list rows escape every control character
rule: Every field of `claude agents --json` origin has C0/C1 controls escaped as hex text, and in `list` table rows `\n` and `\t` are escaped too, so every row stays one fixed-width line.
why: stricter than the security plan's floor of keeping `\n` and `\t`; ratified
except: `wait` / `last` message text keeps `\n` and `\t`
@row_add n=39 row=contract/viola-list-table/empty area=surface-cli rec=30
boundary: viola list table
item: empty
promise: With no wrapped sessions the board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_none n=4 rec=31 type=4
relied: A failed strict-modes check refuses the whole board with exit 21, in the fixed `unable:` / `hint:` form or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`.
@row_none n=5 rec=32 type=S
@row_add n=40 row=contract/viola-send-mirror/open area=surface-cli rec=33
boundary: viola send readback mirror
item: open
promise: `[  ] open  <name>  issued <time>` prints on stderr, TTY only.
@row_add n=41 row=contract/viola-send-mirror/read-back area=surface-cli rec=33
boundary: viola send readback mirror
item: read back
promise: `[RB] read back  <name>  <time>  cursor <n>` prints on stdout with exit 0.
@row_add n=42 row=contract/viola-send-mirror/unable area=surface-cli rec=33
boundary: viola send readback mirror
item: unable
promise: `[/ ] unable  <name>  <reason>  <detail>` prints on stderr with exit 13, followed by its `hint:` line.
@row_add n=43 row=contract/viola-send-mirror/unconfirmable area=surface-cli rec=33
boundary: viola send readback mirror
item: unconfirmable
promise: `[  ] unconfirmable  <name>  local command, no measured post-condition` prints on stdout with exit 0.
@row_add n=44 row=contract/cli-strict-modes-refusal/instance-state-pair area=surface-cli rec=34
boundary: CLI strict-modes refusal
item: instance-state pair
promise: The exit-21 strict-modes refusal of `send`, `wait`, `last`, `answer`, `pause` and `release` prints the fixed instance-state `unable:` / `hint:` pair, not the `[/ ]` mirror.
@row_add n=45 row=contract/cli-start-refusal/fixed-message area=surface-cli rec=34,51
boundary: run and revive start refusals
item: fixed message
promise: A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
@row_add n=46 row=decision/no-hint-names-release area=surface-cli rec=34
name: No hint names viola release
rule: No hint names `viola release`.
why: a driver's `release` is refused
@row_add n=47 row=decision/no-hint-for-faults area=surface-cli rec=34
name: No hint for faults and unknown
rule: Faults and `unknown` get no hint line.
@row_add n=48 row=decision/hint-never-quotes-text area=surface-cli rec=35
name: Hints never quote text
rule: A hint never quotes the sent text or any upstream text.
@row_none n=6 rec=36 type=S
@row_add n=49 row=contract/viola-wait-output/waiting-line area=surface-cli rec=37
boundary: viola wait output
item: waiting line
promise: `wait` prints one static `waiting: <name>` line on stderr, TTY only, before its one stdout result line.
@row_add n=50 row=contract/viola-wait-output/event-form area=surface-cli rec=37
boundary: viola wait output
item: event form
promise: `turn-ended`, `session-end` and every other kind without a `dialog_id` print `<kind>  <name>  <time>  cursor <n>` on stdout.
@row_add n=51 row=contract/viola-wait-output/dialog-form area=surface-cli rec=37
boundary: viola wait output
item: dialog form
promise: `question`, `permission`, `plan` and `dialog-expired` print `<kind>  <name>  dialog <id>  cursor <n>` on stdout, uncoloured, with `dialog unknown` when the event carries no `dialog_id`.
@row_add n=52 row=contract/viola-wait-output/timed-out area=surface-cli rec=37
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_add n=53 row=decision/no-spinner area=surface-cli rec=38
name: No spinner or elapsed-time counter
rule: The CLI shows no spinner and no elapsed-time counter.
@row_add n=54 row=contract/viola-last-output/header area=surface-cli rec=39
boundary: viola last output
item: header
promise: `last` prints `last  <name>  <kind> <time>` on stderr, TTY only.
@row_add n=55 row=contract/viola-last-output/message area=surface-cli rec=39
boundary: viola last output
item: message
promise: `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
@row_add n=56 row=contract/cli-message-escaping/control-characters area=surface-cli rec=40
boundary: CLI message-mode escaping
item: control characters
promise: In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
@row_add n=57 row=contract/cli-message-escaping/json area=surface-cli rec=40
boundary: CLI message-mode escaping
item: --json
promise: `--json` output stays serde-escaped.
@row_none n=7 rec=41 type=S
@row_add n=58 row=contract/cli-verb-output/pause area=surface-cli rec=42
boundary: CLI verb output
item: viola pause
promise: `viola pause <name>` prints `<name>  wheel human  manual-pause  I have control`.
@row_add n=59 row=contract/cli-verb-output/release area=surface-cli rec=43
boundary: CLI verb output
item: viola release
promise: `viola release <name>` prints `<name>  wheel driver  you have control`.
@row_add n=60 row=contract/cli-verb-output/release-budget area=surface-cli rec=44
boundary: CLI verb output
item: viola release --budget
promise: Prints one unstyled stdout line, `<name>  budget gate lifted until <time>  wheel human` when a pause stood, else `<name>  budget gate open  wheel human`.
@row_add n=61 row=contract/cli-verb-output/link area=surface-cli rec=45
boundary: CLI verb output
item: viola link
promise: `viola link <a> <b>` prints `link    <a> -> <b>  since <time>`, or `link    <a> -> <b>  already linked` for an existing pair.
@row_add n=62 row=contract/cli-verb-output/unlink area=surface-cli rec=46
boundary: CLI verb output
item: viola unlink
promise: `viola unlink <a> <b>` prints `unlink  <a> -> <b>`.
@row_add n=63 row=contract/cli-verb-output/answer area=surface-cli rec=47
boundary: CLI verb output
item: viola answer
promise: `viola answer <name> <id>` prints `answered  <name>  dialog <id>`, or `unable  <name>  not-delivered  unknown-dialog`.
@row_add n=64 row=contract/cli-verb-output/plugin-install area=surface-cli rec=48
boundary: CLI verb output
item: viola plugin install
promise: Prints four plain ASCII stdout lines in one write, no SGR: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`; `<dir>` is quoted only where the host's shell would split it.
@row_none n=8 rec=49 type=S
@row_add n=65 row=decision/step-counter-only-progress-pattern area=surface-cli rec=50
name: Step counter as the one progress pattern
rule: The step counter of `verify` is the project's one progress pattern.
@row_add n=66 row=contract/viola-verify-output/step-line area=surface-cli rec=50
boundary: viola verify output
item: step line
promise: One static uncoloured stdout line, `pass` or `fail`, per relied ledger row.
@row_add n=67 row=contract/viola-verify-output/summary area=surface-cli rec=50
boundary: viola verify output
item: summary
promise: A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count.
@row_add n=68 row=contract/viola-verify-output/stopped-or-refused area=surface-cli rec=50
boundary: viola verify output
item: stopped or refused run
promise: A run stopped by Ctrl-C or TERM before its rows are checked, or refused by its home tree's strict-modes check before its log, prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1.
@row_add n=69 row=contract/viola-verify-output/stamps-refused area=surface-cli rec=50
boundary: viola verify output
item: stamps.json update refused
promise: A `stamps.json` update refused by its strict-modes check prints the home-state `unable:` / `hint:` pair.
@row_add n=70 row=contract/viola-run-output/child-started area=surface-cli rec=51,52
boundary: viola run and revive output
item: child started
promise: `run` and `revive` print nothing once the child starts.
@row_add n=71 row=decision/start-refusals-show-no-path-or-pid area=surface-cli rec=51
name: Start refusals show no path or pid
rule: No start refusal line or hint of `run` shows a path or a pid.
@row_add n=72 row=contract/viola-revive-output/list area=surface-cli rec=52
boundary: viola revive output
item: --list
promise: `revive --list` prints one plain stdout line per logged session.
@row_add n=73 row=contract/viola-revive-output/json-refusal area=surface-cli rec=52
boundary: viola revive output
item: --json start refusal
promise: Under `--json` every start refusal `revive` shares with `run` prints one stdout document and no hint, stderr empty; a `pump_child` error prints none.
@row_add n=74 row=contract/revive-json-refusal-code/strict-modes-failed area=surface-cli rec=52
boundary: revive --json refusal code
item: strict-modes-failed
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=75 row=contract/revive-json-refusal-code/already-live area=surface-cli rec=52
boundary: revive --json refusal code
item: already-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=76 row=contract/revive-json-refusal-code/no-session area=surface-cli rec=52
boundary: revive --json refusal code
item: no-session
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=77 row=contract/revive-json-refusal-code/session-live area=surface-cli rec=52
boundary: revive --json refusal code
item: session-live
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=78 row=contract/revive-json-refusal-code/cwd-missing area=surface-cli rec=52
boundary: revive --json refusal code
item: cwd-missing
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=79 row=contract/revive-json-refusal-code/stale-heartbeat area=surface-cli rec=52
boundary: revive --json refusal code
item: stale-heartbeat
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=80 row=contract/revive-json-refusal-code/pinned-hash-mismatch area=surface-cli rec=52
boundary: revive --json refusal code
item: pinned-hash-mismatch
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=81 row=contract/revive-json-refusal-code/squatted-name area=surface-cli rec=52
boundary: revive --json refusal code
item: squatted-name
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=82 row=contract/revive-json-refusal-code/socket-dir-refused area=surface-cli rec=52
boundary: revive --json refusal code
item: socket-dir-refused
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_add n=83 row=contract/revive-json-refusal-code/internal-error area=surface-cli rec=52
boundary: revive --json refusal code
item: internal-error
promise: One of the closed set of codes a `revive --json` start refusal document carries.
@row_none n=9 rec=53 type=S
@row_add n=84 row=contract/cli-exit-codes/0 area=surface-cli rec=54
boundary: CLI exit codes
item: 0
promise: The result on stdout.
@row_add n=85 row=contract/cli-exit-codes/1 area=surface-cli rec=54
boundary: CLI exit codes
item: 1
promise: `error: internal error` or a fixed-message `unable:` refusal with its hint; under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, and every refusal of `viola revive`'s start prints a document.
@row_add n=86 row=contract/cli-exit-codes/2 area=surface-cli rec=54
boundary: CLI exit codes
item: 2
promise: clap usage, or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of an oversize `viola answer` / an unusable `viola send`, with no document under `--json`.
@row_add n=87 row=contract/cli-exit-codes/10-14 area=surface-cli rec=54
boundary: CLI exit codes
item: 10 to 14
promise: An `unable` refusal with its reason; a refusal object under `--json`.
@row_add n=88 row=contract/cli-exit-codes/20 area=surface-cli rec=54
boundary: CLI exit codes
item: 20
promise: `error: wrapper fault`.
@row_add n=89 row=contract/cli-exit-codes/21 area=surface-cli rec=54
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state `unable:` / `hint:` pair; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with stderr empty.
@row_add n=90 row=contract/exit-21-cause/strict-modes-failed area=surface-cli rec=54
boundary: exit 21 cause
item: strict-modes-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=91 row=contract/exit-21-cause/instance-dead area=surface-cli rec=54
boundary: exit 21 cause
item: instance-dead
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=92 row=contract/exit-21-cause/unwrapped-name area=surface-cli rec=54
boundary: exit 21 cause
item: unwrapped-name
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=93 row=contract/exit-21-cause/endpoint-lost area=surface-cli rec=54
boundary: exit 21 cause
item: endpoint-lost
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=94 row=contract/exit-21-cause/server-verify-failed area=surface-cli rec=54
boundary: exit 21 cause
item: server-verify-failed
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=95 row=contract/exit-21-cause/socket-dir-refused area=surface-cli rec=54
boundary: exit 21 cause
item: socket-dir-refused
promise: One of the closed set of causes named in exit 21's `detail`.
@row_add n=96 row=contract/cli-strict-modes-refusal/plugin-install area=surface-cli rec=55
boundary: CLI strict-modes refusal
item: viola plugin install
promise: A refused strict-modes check of `viola plugin install` exits 21 with the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document with `detail` `strict-modes-failed`, stderr empty.
