@row_add n=1 row=contract/cli-colour-tokens/attention area=surface rec=1,12
boundary: CLI colour tokens
item: attention
promise: Arrival amber #D97706: `\x1b[38;2;217;119;6m` in truecolor (`COLORTERM=truecolor|24bit`), `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16; used on the `DIALOG` word only, always followed by its kind word, which stays plain.
@row_add n=2 row=contract/cli-colour-tokens/stale area=surface rec=2,12
boundary: CLI colour tokens
item: stale
promise: SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
@row_add n=3 row=contract/cli-colour-tokens/callsign area=surface rec=3,12
boundary: CLI colour tokens
item: callsign
promise: Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
@row_add n=4 row=contract/cli-colour-tokens/reset area=surface rec=4
boundary: CLI colour tokens
item: reset
promise: `\x1b[0m` at every colour depth, after every styled span.
@row_add n=5 row=contract/cli-colour-tokens/handoff-blue area=surface rec=5
boundary: CLI colour tokens
item: handoff blue
promise: Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
@row_add n=6 row=decision/cli-colour-amber-and-dim-only area=surface rec=5
name: CLI colour limited to amber and dim
rule: CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted.
why: The quiz limits CLI colour to amber and dim.
@row_add n=7 row=decision/terminal-background-never-set area=surface rec=6
name: Terminal background never set
rule: The CLI never sets a background colour, at any colour depth.
why: The terminal background belongs to the user.
@row_add n=8 row=contract/cli-stderr/context-lines area=surface rec=7
boundary: CLI stderr
item: context lines
promise: Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
@row_add n=9 row=contract/cli-stderr/refusal-line area=surface rec=7
boundary: CLI stderr
item: refusal line
promise: A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
@row_add n=10 row=contract/cli-stderr/fixed-message-refusal area=surface rec=7
boundary: CLI stderr
item: fixed message form
promise: The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
@row_add n=11 row=contract/cli-stderr/hint-line area=surface rec=7
boundary: CLI stderr
item: hint line
promise: Each refusal's `hint:` line follows it directly on stderr.
@row_add n=12 row=contract/cli-stderr/error-line area=surface rec=7
boundary: CLI stderr
item: error line
promise: Errors print on stderr as `error: …`.
@row_add n=13 row=contract/peer-reply-screening/code-word area=surface rec=8
boundary: peer reply screening
item: code word
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first; both clients, the CLI and the MCP server, screen a peer's `refusal` and `detail` through it.
@row_add n=14 row=contract/peer-reply-screening/refusal-not-code-word area=surface rec=9
boundary: peer reply screening
item: refusal that is no code word
promise: A peer's `refusal` that is no code word reads `unknown`.
@row_add n=15 row=contract/peer-reply-screening/string-detail-not-code-word area=surface rec=9
boundary: peer reply screening
item: string detail that is no code word
promise: A peer's string `detail` that is no code word reads as no detail, null under `--json`.
@row_add n=16 row=contract/peer-reply-screening/non-string-detail area=surface rec=9
boundary: peer reply screening
item: non-string detail
promise: A peer reply with a non-string `detail` is no known reply.
@row_add n=17 row=contract/peer-reply-screening/unlisted-code-word area=surface rec=10
boundary: peer reply screening
item: code word outside the refusal list
promise: A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does.
@row_add n=18 row=contract/human-tables/name-truncation area=surface rec=11
boundary: human tables
item: NAME truncation
promise: Where the width is detectable, human tables truncate only the NAME of unwrapped rows, ending it in `...`.
@row_add n=19 row=contract/human-tables/width-source area=surface rec=11
boundary: human tables
item: width source
promise: The table width is stdout's own terminal size; `COLUMNS` is not read, and a piped stdout has no width.
@row_add n=20 row=absence/cli-tape area=surface rec=11
capability: A tape in the CLI
@row_add n=21 row=contract/viola-list-board/status area=surface rec=12
boundary: viola list board
item: STATUS
promise: A wrapped row joined to a `claude agents --json` row, by child pid or agent session id, reads STATUS `idle` or `busy`; any other wrapped row reads `unknown`.
@row_add n=22 row=contract/viola-list-board/unwrapped-separator area=surface rec=12
boundary: viola list board
item: unwrapped separator
promise: When the agents read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows, or the empty-board line when there are none; with no unwrapped row no separator prints.
@row_add n=23 row=contract/viola-list-board/unwrapped-row area=surface rec=12
boundary: viola list board
item: unwrapped row
promise: One row per unwrapped session, sorted by name: NAME the session's `sessionId`, LIVE `live`, its STATUS, and `n/a` in WHEEL, DIALOG and CLI.
@row_add n=24 row=contract/viola-list-board/failed-agents-read area=surface rec=12
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no `claude`.
@row_add n=25 row=contract/viola-list-board/name-column-width area=surface rec=12
boundary: viola list board
item: NAME column width
promise: On a terminal of known width NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes.
@row_add n=26 row=contract/viola-list-board/name-cut area=surface rec=12
boundary: viola list board
item: NAME cut
promise: An unwrapped NAME longer than its column is cut to the column less 3 characters and ends in `...`, never splitting an escape; a wrapped NAME is never cut; a piped board or `--json` keeps every name whole.
@row_add n=27 row=contract/viola-list-board/uncoloured-lines area=surface rec=12
boundary: viola list board
item: uncoloured lines
promise: The BAY line, the caption, the empty-board line and the separator carry no SGR.
@row_add n=28 row=contract/viola-list-board/colour-spans area=surface rec=12
boundary: viola list board
item: colour spans
promise: Colour spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
@row_add n=29 row=contract/viola-list-board/row-escaping area=surface rec=12
boundary: viola list board
item: row escaping
promise: The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
@row_add n=30 row=decision/wheel-column-width-23 area=surface rec=12
name: WHEEL column padded to 23
rule: The board's WHEEL column is padded to 23 characters, the width of its widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder).
why: Ratified.
@row_add n=31 row=contract/viola-list-board/window-figure area=surface rec=12
boundary: viola list board
item: window figure
promise: A window's figure is rounded down to a whole number (`89.6` prints `89 %`).
@row_add n=32 row=contract/viola-list-board/reading-age area=surface rec=12
boundary: viola list board
item: reading age
promise: The reading's age prints in whole minutes rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now.
@row_add n=33 row=contract/viola-list-board/skip-counts area=surface rec=12
boundary: viola list board
item: skip counts
promise: Nonzero skip counts print as bare numbers in key order (`skipped 2 - 0 - 1`).
@row_add n=34 row=contract/viola-list-board/human-holder-budget-paused area=surface rec=12
boundary: viola list board
item: human holder under a budget pause
promise: A `human` holder under a budget pause prints `human - budget-paused`.
@row_add n=35 row=contract/viola-list-board/empty-board area=surface rec=12
boundary: viola list board
item: empty board
promise: The empty board prints the BAY line, then the caption row, then its one line.
@row_add n=36 row=contract/viola-list-board/strict-modes-refusal area=surface rec=13
boundary: viola list board
item: strict-modes refusal
promise: A failed strict-modes check of the home, `instances/` or any one instance refuses the whole board with exit 21, nothing from state printed, in the fixed `unable:` / `hint:` form.
@row_add n=37 row=contract/fixed-refusal-pairs/home-state area=surface rec=13,16,24
boundary: fixed refusal pairs
item: home-state pair
promise: `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
@row_add n=38 row=contract/fixed-refusal-pairs/instance-state area=surface rec=13,16
boundary: fixed refusal pairs
item: instance-state pair
promise: `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
@row_add n=39 row=contract/viola-list-board/json-refusal area=surface rec=13
boundary: viola list board
item: --json refusal
promise: Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_add n=40 row=contract/viola-list-board/strict-modes-scan area=surface rec=14
boundary: viola list board
item: strict-modes scan
promise: The scan holds the home to the read rule and `instances/` to the write rule, then each instance's set; a home another user may read refuses the whole board with the home-state pair, with or without an instance.
@row_none n=1 rec=15 type=5
@row_add n=41 row=contract/refusal-hints/refusal-line area=surface rec=16
boundary: refusal hints
item: refusal line
promise: A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed.
@row_add n=42 row=contract/refusal-hints/human-typing area=surface rec=16
boundary: refusal hints
item: human-typing
promise: `the human has the wheel; send again after the human hands it back`
@row_add n=43 row=contract/refusal-hints/budget-paused area=surface rec=16
boundary: refusal hints
item: budget-paused
promise: `budget gate is closed; it lifts at the window reset, or when the human lifts it`
@row_add n=44 row=decision/no-hint-names-viola-release area=surface rec=16
name: No hint names viola release
rule: No refusal hint ever names `viola release`; `release` stays a human verb.
why: The human-typing and budget-paused refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
@row_add n=45 row=contract/refusal-hints/unverified-cli area=surface rec=16
boundary: refusal hints
item: unverified-cli
promise: `run viola verify for this CLI version`
@row_add n=46 row=contract/refusal-hints/not-delivered-turn-running area=surface rec=16
boundary: refusal hints
item: not-delivered · turn-running
promise: `a turn is running; viola wait <name> first`
@row_add n=47 row=contract/refusal-hints/not-delivered-control-character area=surface rec=16
boundary: refusal hints
item: not-delivered · control-character
promise: `the text contains a control character (only LF, CR, TAB are allowed)`
@row_add n=48 row=contract/refusal-hints/not-delivered-empty-text area=surface rec=16
boundary: refusal hints
item: not-delivered · empty-text
promise: `the text is empty once its trailing newlines are removed; send a text with content`
@row_add n=49 row=contract/refusal-hints/not-delivered-input-not-ready area=surface rec=16
boundary: refusal hints
item: not-delivered · input-not-ready
promise: `<name> was not ready for input; send again, and if it repeats a human must look at the session`
@row_add n=50 row=contract/refusal-hints/not-delivered-no-prompt-submitted area=surface rec=16
boundary: refusal hints
item: not-delivered · no-prompt-submitted
promise: `<name> did not submit the prompt; check it, then send again`
@row_add n=51 row=contract/refusal-hints/not-delivered-unknown-dialog area=surface rec=16
boundary: refusal hints
item: not-delivered · unknown-dialog
promise: `that dialog is not pending; viola list shows the current DIALOG`
@row_add n=52 row=decision/instance-unreachable-hint-per-cause area=surface rec=16
name: One instance-unreachable hint per cause
rule: The `instance-unreachable` refusal (exit 21) prints one hint per cause.
why: The exit code is shared by several causes.
@row_add n=53 row=contract/refusal-hints/not-running area=surface rec=16,17
boundary: refusal hints
item: not-running
promise: When no wrapper is running for the name: `<name> is not running; viola list shows the live instances`; `not-running` is the hint key of the log's `instance-dead`.
@row_add n=54 row=contract/refusal-hints/unwrapped-name area=surface rec=16,17
boundary: refusal hints
item: unwrapped-name
promise: When the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
@row_add n=55 row=contract/refusal-hints/endpoint-lost area=surface rec=16,17
boundary: refusal hints
item: endpoint-lost
promise: When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
@row_add n=56 row=contract/frame-verb-refusals/strict-modes area=surface rec=16
boundary: frame verb refusals
item: strict-modes
promise: A failed strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause`, `release` first; each before any frame) prints the instance-state pair, no `unable  instance-unreachable` line or mirror; `--json`: the `strict-modes-failed` document.
@row_add n=57 row=contract/refusal-hints/server-verify area=surface rec=16,17
boundary: refusal hints
item: server-verify
promise: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
@row_add n=58 row=contract/refusal-hints/socket-dir area=surface rec=16,17
boundary: refusal hints
item: socket-dir
promise: The socket-directory hint is `run`'s: `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=59 row=decision/start-refusals-show-no-path-or-pid area=surface rec=16
name: Start refusals show no path or pid
rule: The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
@row_add n=60 row=contract/viola-run-start-refusals/already-live area=surface rec=16
boundary: viola run start refusals
item: already-live
promise: `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
@row_add n=61 row=contract/viola-run-start-refusals/stale-heartbeat area=surface rec=16
boundary: viola run start refusals
item: stale-heartbeat
promise: A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.
@row_add n=62 row=contract/viola-run-start-refusals/squatted-name area=surface rec=16
boundary: viola run start refusals
item: squatted-name
promise: `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
@row_add n=63 row=contract/viola-run-start-refusals/pinned-hash-mismatch area=surface rec=16
boundary: viola run start refusals
item: pinned-hash-mismatch
promise: A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
@row_add n=64 row=contract/viola-run-start-refusals/home-tree-check area=surface rec=16
boundary: viola run start refusals
item: home-tree check
promise: At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
@row_add n=65 row=contract/viola-run-start-refusals/socket-dir area=surface rec=16
boundary: viola run start refusals
item: socket directory
promise: Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=66 row=contract/viola-revive-refusals/order area=surface rec=16
boundary: viola revive refusals
item: order
promise: `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
@row_add n=67 row=contract/viola-revive-refusals/no-session area=surface rec=16
boundary: viola revive refusals
item: no-session
promise: `unable: <name> has no logged session to resume` then `hint: start one: viola run <name> -- claude`.
@row_add n=68 row=contract/viola-revive-refusals/no-such-id area=surface rec=16
boundary: viola revive refusals
item: no logged session with that id
promise: An `--id` the log does not hold prints `unable: <name> has no logged session with that id` then `hint: viola revive <name> --list`.
@row_add n=69 row=contract/viola-revive-refusals/session-live area=surface rec=16
boundary: viola revive refusals
item: session-live
promise: When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_add n=70 row=contract/viola-revive-refusals/cwd-missing area=surface rec=16
boundary: viola revive refusals
item: cwd-missing
promise: `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
@row_add n=71 row=contract/viola-revive-refusals/home-tree-check area=surface rec=16
boundary: viola revive refusals
item: home-tree check
promise: The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
@row_add n=72 row=contract/viola-revive-refusals/socket-dir area=surface rec=16
boundary: viola revive refusals
item: socket directory
promise: A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
@row_add n=73 row=decision/hints-human-mode-only area=surface rec=16
name: Hints in human mode only
rule: Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
@row_add n=74 row=contract/log-detail-codes/server-verify-failed area=surface rec=16
boundary: log detail codes
item: server-verify-failed
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document reads the same word.
@row_add n=75 row=contract/log-detail-codes/socket-dir-refused area=surface rec=16
boundary: log detail codes
item: socket-dir-refused
promise: Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
@row_add n=76 row=contract/viola-revive-refusals/strict-modes-no-log area=surface rec=16
boundary: viola revive refusals
item: strict-modes
promise: `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
@row_add n=77 row=decision/no-hint-for-unknown area=surface rec=16
name: No hint for unknown
rule: The `unknown` refusal (exit 14) prints no hint line.
why: Its detail is a code word or nothing, and a hint would have to guess.
@row_add n=78 row=decision/no-hint-for-faults area=surface rec=16
name: No hint for faults
rule: `error: wrapper fault` (exit 20) and `error: internal error` (exit 1) print no hint; the human line holds the code only and further detail goes to `diagnostics/`.
why: They are faults, not refusals.
@row_add n=79 row=contract/frame-verb-refusals/socket-dir area=surface rec=18
boundary: frame verb refusals
item: socket directory
promise: A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
@row_add n=80 row=contract/viola-release-budget/lifted-line area=surface rec=19
boundary: viola release --budget
item: lifted line
promise: When a pause stood, `viola release <name> --budget` prints `<name>  budget gate lifted until <time>  wheel human`, one unstyled stdout line with the two-space gutter of the other verb lines.
@row_add n=81 row=contract/viola-release-budget/override-time area=surface rec=19
boundary: viola release --budget
item: override time
promise: The time is the reply's `override_until`: `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date; a value that is no RFC 3339 instant prints escaped.
@row_add n=82 row=contract/viola-release-budget/open-line area=surface rec=19
boundary: viola release --budget
item: open line
promise: When no pause stood the line is `<name>  budget gate open  wheel human`, one unstyled stdout line.
@row_add n=83 row=contract/viola-link-output/result-line area=surface rec=20
boundary: viola link and unlink output
item: result line
promise: `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
@row_add n=84 row=contract/viola-link-output/printed-names area=surface rec=20
boundary: viola link and unlink output
item: printed names
promise: The names printed are the verb's own parsed arguments, never names read back from the reply or the log.
@row_add n=85 row=contract/viola-link-output/since area=surface rec=21
boundary: viola link and unlink output
item: since
promise: The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
@row_add n=86 row=contract/viola-link-output/repeat-link area=surface rec=22
boundary: viola link and unlink output
item: repeat link
promise: A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`, and exits 0.
@row_add n=87 row=contract/viola-link-output/unlink-not-held area=surface rec=22
boundary: viola link and unlink output
item: unlink of a pair not held
promise: `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line and exits 0.
@row_add n=88 row=decision/one-progress-pattern area=surface rec=23
name: One progress pattern
rule: A step counter with static appended stdout lines is the project's one progress pattern.
@row_add n=89 row=contract/viola-verify-output/step-line area=surface rec=23
boundary: viola verify output
item: step line
promise: One static appended stdout line per relied ledger row: `[NN/MM] <row> <description>  pass` or `fail`, `MM` the relied row count.
@row_add n=90 row=contract/viola-verify-output/summary-line area=surface rec=23,33
boundary: viola verify output
item: summary line
promise: The last stdout line is a summary whose counts add up to `MM`: `stamped <version>  <n> pass  <m> fail`; a failing row prints no stderr word.
@row_add n=91 row=contract/viola-verify-output/fail-word area=surface rec=23
boundary: viola verify output
item: fail word
promise: The word `fail` is uncoloured.
@row_add n=92 row=contract/viola-verify-output/help area=surface rec=23
boundary: viola verify output
item: --help
promise: `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
@row_add n=93 row=contract/viola-verify-refusals/interrupted area=surface rec=23,33
boundary: viola verify refusals
item: interrupted run
promise: A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
@row_add n=94 row=contract/viola-verify-refusals/home-tree-check area=surface rec=24,33
boundary: viola verify refusals
item: home-tree check
promise: Before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`, each that exists; a refusal prints no step line and no summary, only the home-state pair, and exits 1.
@row_add n=95 row=contract/viola-verify-refusals/stamps-update area=surface rec=24,33
boundary: viola verify refusals
item: stamps.json update
promise: A `stamps.json` update refused by its strict-modes check prints the home-state pair and exits 1.
@row_add n=96 row=contract/strict-modes/home-under-regular-file area=surface rec=25
boundary: strict-modes refusals
item: home under a regular file
promise: A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
@row_add n=97 row=contract/viola-revive-output/after-child-start area=surface rec=26
boundary: viola revive output
item: after the child starts
promise: `viola revive`, as `run`, prints nothing once the child starts.
@row_add n=98 row=contract/viola-revive-output/list-line area=surface rec=26
boundary: viola revive output
item: --list line
promise: `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
@row_add n=99 row=contract/viola-revive-output/json-refusal area=surface rec=26,27,33
boundary: viola revive output
item: --json refusal
promise: Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents.
@row_add n=100 row=contract/viola-revive-output/list-json area=surface rec=26
boundary: viola revive output
item: --list --json
promise: `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.
@row_add n=101 row=contract/viola-revive-json-codes/strict-modes-failed area=surface rec=27
boundary: viola revive --json exit-1 codes
item: strict-modes-failed
promise: `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=102 row=contract/viola-revive-json-codes/already-live area=surface rec=27
boundary: viola revive --json exit-1 codes
item: already-live
promise: `already-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=103 row=contract/viola-revive-json-codes/no-session area=surface rec=27
boundary: viola revive --json exit-1 codes
item: no-session
promise: `no-session` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=104 row=contract/viola-revive-json-codes/session-live area=surface rec=27
boundary: viola revive --json exit-1 codes
item: session-live
promise: `session-live` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=105 row=contract/viola-revive-json-codes/cwd-missing area=surface rec=27
boundary: viola revive --json exit-1 codes
item: cwd-missing
promise: `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=106 row=contract/viola-revive-json-codes/stale-heartbeat area=surface rec=27
boundary: viola revive --json exit-1 codes
item: stale-heartbeat
promise: `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=107 row=contract/viola-revive-json-codes/pinned-hash-mismatch area=surface rec=27
boundary: viola revive --json exit-1 codes
item: pinned-hash-mismatch
promise: `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=108 row=contract/viola-revive-json-codes/squatted-name area=surface rec=27
boundary: viola revive --json exit-1 codes
item: squatted-name
promise: `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
@row_add n=109 row=contract/viola-revive-json-codes/socket-dir-refused area=surface rec=27,28
boundary: viola revive --json exit-1 codes
item: socket-dir-refused
promise: A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
@row_add n=110 row=contract/viola-revive-json-codes/internal-error area=surface rec=27,29
boundary: viola revive --json exit-1 codes
item: internal-error
promise: A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
@row_add n=111 row=decision/no-json-after-child-held-terminal area=surface rec=30,33
name: No document once the child held the terminal
rule: An error while pumping the child under `viola revive --json` prints no document.
why: The child has held the terminal and viola writes nothing past it.
@row_add n=112 row=contract/wrapper-socket/refused-peer area=surface rec=31
boundary: wrapper socket
item: refused peer
promise: A peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
@row_add n=113 row=contract/cli-exit-codes/0 area=surface rec=32
boundary: CLI exit codes
item: 0
promise: No stderr first word (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …); under `--json` the document `{"v":1,"ok":{…}}`.
@row_add n=114 row=contract/cli-exit-codes/1 area=surface rec=33
boundary: CLI exit codes
item: 1
promise: `error: internal error` (fixed; no chain, paths or hint), or a fixed `unable:` start refusal of `run` or `revive` or a `verify` refusal, each with its hint; `--json` only for `revive`'s shared-start refusals and the catch site's `internal-error`.
@row_add n=115 row=contract/viola-verify-refusals/cli-not-found area=surface rec=33
boundary: viola verify refusals
item: CLI not found
promise: `unable: the claude CLI was not found`, followed by its own hint, exit 1.
@row_add n=116 row=contract/viola-verify-refusals/version-unreadable area=surface rec=33
boundary: viola verify refusals
item: version unreadable
promise: `unable: the CLI version could not be read`, followed by its own hint, exit 1.
@row_add n=117 row=contract/viola-verify-refusals/unclean-fixture area=surface rec=33
boundary: viola verify refusals
item: unclean fixture
promise: `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), codes `home-path` · `absolute-path` · `username` · `email`, never the content, followed by its hint, exit 1.
@row_add n=118 row=contract/cli-exit-codes/2 area=surface rec=34
boundary: CLI exit codes
item: 2
promise: Clap usage text (never from `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (oversize `viola answer`) or `UNUSABLE_TEXT` line (unusable `viola send`) on stderr, whatever `--json` says; no document.
@row_add n=119 row=contract/cli-exit-codes/10 area=surface rec=35
boundary: CLI exit codes
item: 10
promise: `unable  human-typing` (+ `manual-pause`); under `--json` `{"v":1,"refusal":"human-typing","detail":…}`.
@row_add n=120 row=contract/cli-exit-codes/11 area=surface rec=36
boundary: CLI exit codes
item: 11
promise: `unable  budget-paused  five-hour` or `seven-day`; under `--json` the refusal object.
@row_add n=121 row=contract/cli-exit-codes/12 area=surface rec=37
boundary: CLI exit codes
item: 12
promise: `unable  unverified-cli`; under `--json` the refusal object.
@row_add n=122 row=contract/cli-exit-codes/13 area=surface rec=38
boundary: CLI exit codes
item: 13
promise: `unable  not-delivered` with `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character` or `empty-text`; under `--json` the refusal object.
@row_add n=123 row=contract/cli-exit-codes/14 area=surface rec=39
boundary: CLI exit codes
item: 14
promise: `unable  unknown  <detail>`, the detail a code word or nothing; under `--json` the refusal object.
@row_add n=124 row=contract/cli-exit-codes/20 area=surface rec=40
boundary: CLI exit codes
item: 20
promise: `error: wrapper fault  <code>`; under `--json` `{"v":1,"error":"wrapper-fault","detail":{…}}`.
@row_add n=125 row=contract/cli-exit-codes/21 area=surface rec=41
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
@row_add n=126 row=contract/exit-21-json-cause/strict-modes-failed area=surface rec=41
boundary: exit 21 --json causes
item: strict-modes-failed
promise: The `list`, `link`, `unlink` and `plugin install` refusals and the six frame verbs' strict-modes refusals read `strict-modes-failed`.
@row_add n=127 row=contract/exit-21-json-cause/instance-dead area=surface rec=41,51,52,53
boundary: exit 21 --json causes
item: instance-dead
promise: With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
@row_add n=128 row=contract/exit-21-json-cause/unwrapped-name area=surface rec=41,51
boundary: exit 21 --json causes
item: unwrapped-name
promise: With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent, a regular file or a symlink reads `unwrapped-name`.
@row_add n=129 row=contract/exit-21-json-cause/endpoint-lost area=surface rec=41,52
boundary: exit 21 --json causes
item: endpoint-lost
promise: A wrapper lost with the call open reads `endpoint-lost`.
@row_add n=130 row=contract/exit-21-json-cause/server-verify-failed area=surface rec=41,50
boundary: exit 21 --json causes
item: server-verify-failed
promise: A process on the endpoint that is not the recorded wrapper (pid or start time differs, the check cannot be made, or the peer's euid is not this user's or cannot be read) gets no frame; the eight frame-writing verbs read `server-verify-failed`.
@row_add n=131 row=contract/exit-21-json-cause/socket-dir-refused area=surface rec=41
boundary: exit 21 --json causes
item: socket-dir-refused
promise: `socket-dir-refused` is one of the six closed causes of exit 21's `--json` document.
@row_add n=132 row=contract/viola-link-json/ok area=surface rec=42
boundary: viola link --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link.
@row_add n=133 row=contract/viola-unlink-json/ok area=surface rec=42
boundary: viola unlink --json
item: ok
promise: Exit 0 prints `{"v":1,"ok":{}}`.
@row_add n=134 row=contract/viola-link-output/refused-pair area=surface rec=43
boundary: viola link and unlink output
item: refused pair
promise: A pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself) exits 20: `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
@row_add n=135 row=contract/wrapper-fault-detail/message area=surface rec=44
boundary: wrapper fault --json detail
item: message
promise: Exit 20's `--json` `detail` is closed for the five known codes, and `message` is each code's fixed text.
@row_add n=136 row=contract/wrapper-fault-detail/unsupported-version area=surface rec=44
boundary: wrapper fault --json detail
item: -32602 unsupported version
promise: `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values.
@row_add n=137 row=contract/wrapper-fault-detail/release-from-driver area=surface rec=44
boundary: wrapper fault --json detail
item: -32602 release-from-driver
promise: `-32602` with `data.reason` exactly `release-from-driver` reads that body.
@row_add n=138 row=contract/wrapper-fault-detail/other-invalid-params area=surface rec=44
boundary: wrapper fault --json detail
item: other -32602
promise: Any other `-32602` reads `invalid params` with null `data`.
@row_add n=139 row=contract/wrapper-fault-detail/other-known-codes area=surface rec=44
boundary: wrapper fault --json detail
item: other known codes
promise: Every known code other than `-32602` has null `data`.
@row_add n=140 row=contract/wrapper-fault-detail/unknown-code area=surface rec=45
boundary: wrapper fault --json detail
item: code outside the five
promise: A code outside the five known prints `error: wrapper fault  <code>`, and under `--json` `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
@row_add n=141 row=decision/json-failure-documents-clean area=surface rec=46
name: Clean --json failure documents
rule: Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
guard: tests/cli_error_bodies.rs
@row_add n=142 row=contract/cli-catch-site/json-internal-error area=surface rec=47
boundary: CLI catch site
item: --json internal error
promise: Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
@row_add n=143 row=contract/cli-catch-site/json-flag-scope area=surface rec=48
boundary: CLI catch site
item: --json flag scope
promise: The catch site counts `--json` only among the arguments before a `--`, read from argv.
@row_add n=144 row=contract/cli-catch-site/silent-verbs area=surface rec=49
boundary: CLI catch site
item: silent verbs
promise: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
@row_add n=145 row=contract/viola-help/verb-groups area=surface rec=54
boundary: viola --help
item: verb groups
promise: `viola --help` lists the served verbs only, grouped `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.
@row_add n=146 row=absence/rendered-page area=typography rec=55
capability: A rendered page or a typography of its own; the CLI's type is the terminal's
