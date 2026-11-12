@row_none n=1 rec=1 type=S
@row_none n=2 rec=2 type=8
@row_none n=3 rec=3 type=0
what: a note on when the document may be edited by hand
@row_none n=4 rec=4 type=S
@row_add n=1 row=absence/web-surface area=surface-web-spa rec=5
capability: A web surface: viola renders no page
@row_none n=5 rec=6 type=S
@row_add n=2 row=decision/clap-derive-cli-parsing area=surface-cli rec=7
name: clap derive for CLI parsing
rule: The viola command line is parsed with clap 4 in its derive form.
@row_add n=3 row=decision/std-isterminal-tty-detection area=surface-cli rec=7
name: TTY detection through std IsTerminal
rule: Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
@row_add n=4 row=decision/hand-written-sgr-styling area=surface-cli rec=7
name: Hand-written SGR styling module
rule: Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
@row_none n=6 rec=8 type=S
@row_none n=7 rec=9 type=S
@row_none n=8 rec=10 type=S
@row_add n=5 row=contract/cli-token-rendering/color-accent area=surface-cli rec=11,9
boundary: CLI token rendering
item: color-accent
promise: The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
@row_add n=6 row=contract/cli-token-rendering/color-text-muted area=surface-cli rec=12,9
boundary: CLI token rendering
item: color-text-muted
promise: The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
@row_add n=7 row=contract/cli-token-rendering/callsign-role area=surface-cli rec=13,9
boundary: CLI token rendering
item: Callsign role
promise: The callsign token: SGR bold on NAME values only.
@row_add n=8 row=contract/cli-token-rendering/color-text-primary-secondary area=surface-cli rec=14,9
boundary: CLI token rendering
item: color-text-primary / color-text-secondary
promise: The terminal's default foreground; the terminal owns ink.
@row_add n=9 row=contract/cli-token-rendering/color-role-handoff area=surface-cli rec=15,9
boundary: CLI token rendering
item: color-role-handoff
promise: Not used; link output is plain.
@row_add n=10 row=contract/cli-token-rendering/color-surface area=surface-cli rec=16,9
boundary: CLI token rendering
item: color-surface-*
promise: Never set; the terminal background belongs to the user.
@row_add n=11 row=contract/cli-token-rendering/label-role area=surface-cli rec=17,9
boundary: CLI token rendering
item: Label role
promise: Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
@row_add n=12 row=contract/cli-token-rendering/data-role area=surface-cli rec=18,9
boundary: CLI token rendering
item: Data role
promise: Tabular figures and timestamps.
@row_add n=13 row=contract/cli-token-rendering/spacing area=surface-cli rec=19,9
boundary: CLI token rendering
item: Spacing
promise: The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
@row_add n=14 row=contract/cli-token-rendering/radius area=surface-cli rec=20,9
boundary: CLI token rendering
item: radius-*
promise: No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
@row_add n=15 row=contract/cli-token-rendering/motion-tokens area=surface-cli rec=21,9
boundary: CLI token rendering
item: Motion tokens
promise: None; nothing redraws in place.
@row_add n=16 row=decision/one-token-vocabulary-on-cli area=surface-cli rec=21
name: One token vocabulary resolved on the cli
rule: The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
@row_add n=17 row=decision/human-tty-expression-level area=surface-cli rec=22
name: Human TTY expression level
rule: On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
@row_add n=18 row=decision/plain-output-modes area=surface-cli rec=22
name: Plain output modes
rule: Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
@row_add n=19 row=decision/readback-mirror-signature area=surface-cli rec=23
name: Readback mirror as the send signature
rule: The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
@row_add n=20 row=contract/viola-send-output/issue-line area=surface-cli rec=24,23
boundary: viola send output
item: issue line
promise: `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
@row_add n=21 row=contract/viola-send-output/read-back area=surface-cli rec=25,23
boundary: viola send output
item: read back
promise: `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by the matching prompt-submitted.
@row_add n=22 row=contract/viola-send-output/read-back-clear area=surface-cli rec=25,23
boundary: viola send output
item: read back for /clear
promise: On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
@row_add n=23 row=contract/viola-send-output/read-back-compact area=surface-cli rec=25,23
boundary: viola send output
item: read back for /compact
promise: On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
@row_add n=24 row=contract/viola-send-output/unable area=surface-cli rec=25,23
boundary: viola send output
item: unable
promise: `[/ ] unable …` prints on stderr with exit 13 (or 10/11/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
@row_add n=25 row=contract/viola-send-output/unconfirmable area=surface-cli rec=25,23
boundary: viola send output
item: unconfirmable
promise: `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.
@row_add n=26 row=decision/shared-refusal-word-column area=surface-cli rec=26
name: Shared refusal word column
rule: The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
@row_add n=27 row=contract/viola-send-output/json-mode area=surface-cli rec=26
boundary: viola send output
item: --json
promise: Under --json the mirror is replaced by the typed ok / refusal document.
@row_add n=28 row=decision/dialog-attention-only-on-board area=surface-cli rec=26
name: DIALOG attention word only on the board
rule: The amber DIALOG attention word appears only in viola list.
@row_add n=29 row=contract/cli-verbs/list area=surface-cli rec=27
boundary: CLI verbs
item: viola list
promise: The board: every session as one strip row, with the BAY context header.
@row_add n=30 row=contract/cli-verbs/send area=surface-cli rec=28
boundary: CLI verbs
item: viola send <target>
promise: Types at a turn boundary and reports read back, unable or unconfirmable.
@row_add n=31 row=contract/cli-verbs/wait area=surface-cli rec=29
boundary: CLI verbs
item: viola wait <target>
promise: Returns one turn-boundary result, with no spinner.
@row_add n=32 row=contract/cli-verbs/last area=surface-cli rec=29
boundary: CLI verbs
item: viola last <target>
promise: Returns the last assistant message, with no spinner.
@row_add n=33 row=decision/spoken-phraseology-verb-output area=surface-cli rec=30
name: Spoken phraseology for wheel, handoff and dialog verbs
rule: The output of viola pause, release, link, unlink and answer is spoken phraseology.
@row_add n=34 row=decision/single-progress-pattern area=surface-cli rec=31
name: Single progress pattern
rule: The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
@row_add n=35 row=contract/cli-verbs/run area=surface-cli rec=32
boundary: CLI verbs
item: viola run <name> -- claude
promise: Passthrough; prints nothing while the child runs.
@row_add n=36 row=decision/mcp-server-without-human-surface area=surface-cli rec=33
name: MCP server with no human surface
rule: viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
@row_add n=37 row=contract/mcp-tools/send area=surface-cli rec=33
boundary: viola mcp tools
item: send
promise: A driver's machine view of the viola send verb.
@row_add n=38 row=contract/mcp-tools/wait area=surface-cli rec=33
boundary: viola mcp tools
item: wait
promise: A driver's machine view of the viola wait verb.
@row_add n=39 row=contract/mcp-tools/last area=surface-cli rec=33
boundary: viola mcp tools
item: last
promise: A driver's machine view of the viola last verb.
@row_add n=40 row=contract/mcp-tools/answer area=surface-cli rec=33
boundary: viola mcp tools
item: answer
promise: A driver's machine view of the viola answer verb.
@row_add n=41 row=contract/mcp-tools/list area=surface-cli rec=33
boundary: viola mcp tools
item: list
promise: A driver's machine view of the viola list verb.
@row_add n=42 row=contract/mcp-tool-results/success area=surface-cli rec=33
boundary: viola mcp tool results
item: success
promise: Each success is the CLI verb's --json document less `v`.
@row_add n=43 row=contract/mcp-tool-results/refusal area=surface-cli rec=33
boundary: viola mcp tool results
item: refusal
promise: Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
@row_add n=44 row=contract/mcp-tool-results/invalid-params area=surface-cli rec=33
boundary: viola mcp tool results
item: invalid-params
promise: Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
@row_add n=45 row=decision/mcp-hides-wheel-and-handoff-verbs area=surface-cli rec=33
name: MCP hides wheel and handoff verbs
rule: No viola mcp tool, description or result names release, pause, link or unlink.
@row_add n=46 row=contract/cli-verbs/revive area=surface-cli rec=34
boundary: CLI verbs
item: viola revive <name>
promise: Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
@row_add n=47 row=contract/cli-verbs/revive-list area=surface-cli rec=34
boundary: CLI verbs
item: viola revive <name> --list
promise: Prints the instance's logged sessions, one static line each.
@row_add n=48 row=contract/cli-verbs/plugin-install area=surface-cli rec=35
boundary: CLI verbs
item: viola plugin install
promise: Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude.
@row_add n=49 row=contract/cli-verbs/help area=surface-cli rec=36
boundary: CLI verbs
item: viola --help
promise: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding plugin install.
@row_none n=9 rec=37 type=0
what: a sample output wireframe of viola list
@row_add n=50 row=decision/board-ordered-by-name area=surface-cli rec=38
name: Board ordered by name
rule: viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold.
@row_add n=51 row=contract/viola-list-board/gate-closed area=surface-cli rec=39
boundary: viola list board
item: closed budget gate
promise: A closed budget gate prints `budget-paused` in place of `gate open`.
@row_add n=52 row=contract/viola-list-board/window-expired area=surface-cli rec=39
boundary: viola list board
item: passed window
promise: A passed budget window prints with `expired` after its figure, as `5H 62 % expired`.
@row_add n=53 row=contract/viola-list-board/stale-row-with-dialog area=surface-cli rec=40,44
boundary: viola list board
item: stale row with a dialog
promise: On a cocked and stale row, the DIALOG word stays amber and not dim, while the kind word and the rest of the row stay dim.
@row_add n=54 row=contract/viola-list-board/empty-board area=surface-cli rec=41
boundary: viola list board
item: empty board
promise: An empty board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_add n=55 row=decision/words-carry-every-state area=surface-cli rec=43,42
name: Words carry every state
rule: Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
@row_add n=56 row=contract/viola-list-board/unknown-width area=surface-cli rec=43,44
boundary: viola list board
item: unknown terminal width
promise: Nothing is truncated when the terminal width is unknown.
@row_add n=57 row=contract/viola-list-board/single-write area=surface-cli rec=44
boundary: viola list board
item: write order
promise: viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one.
@row_add n=58 row=contract/viola-list-board/uncoloured-modes area=surface-cli rec=44
boundary: viola list board
item: uncoloured modes
promise: The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
@row_add n=59 row=contract/viola-list-board/colour-depth area=surface-cli rec=44
boundary: viola list board
item: colour depth
promise: On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM.
@row_add n=60 row=contract/viola-list-board/stale-row-dim area=surface-cli rec=44
boundary: viola list board
item: stale row
promise: A wrapped stale row is dim from its first byte to its last, NAME dim and bold.
@row_add n=61 row=contract/viola-list-board/plain-lines area=surface-cli rec=44
boundary: viola list board
item: plain lines
promise: The BAY line, caption, empty line and separator print plain.
@row_add n=62 row=contract/viola-list-board/status-join area=surface-cli rec=44
boundary: viola list board
item: STATUS
promise: One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
@row_add n=63 row=contract/viola-list-board/agents-read-failure area=surface-cli rec=44
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
@row_add n=64 row=contract/viola-list-board/unwrapped-rack area=surface-cli rec=44
boundary: viola list board
item: unwrapped rack
promise: Unjoined agent rows print after the wrapped rows under `-- UNWRAPPED - READ-ONLY --`, sorted by name: NAME the escaped sessionId, LIVE live, its STATUS, n/a in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints.
@row_add n=65 row=contract/viola-list-board/column-widths area=surface-cli rec=44
boundary: viola list board
item: column widths
promise: NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
@row_add n=66 row=contract/viola-list-board/name-truncation area=surface-cli rec=44
boundary: viola list board
item: NAME truncation
promise: With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
@row_add n=67 row=contract/viola-list-board/wheel-unknown area=surface-cli rec=44
boundary: viola list board
item: WHEEL unknown
promise: A replayed row with no logged holder reads WHEEL `unknown`.
@row_add n=68 row=contract/viola-list-board/cli-unknown area=surface-cli rec=44
boundary: viola list board
item: CLI unknown
promise: A row with no recorded version reads CLI `unknown`.
@row_add n=69 row=contract/viola-list-board/dialog-unknown area=surface-cli rec=44
boundary: viola list board
item: DIALOG unknown
promise: A pending dialog of unknown kind reads `DIALOG unknown`.
@row_add n=70 row=decision/board-planning-forms-wording area=surface-cli rec=44
name: Board planning forms as the ratified wording
rule: The board's planning forms (a figure rounded down, the whole-minute age and `read unknown`, bare skip counts, `human - budget-paused`, the empty board's three lines) are its wording.
why: confirmed as built by the founder, superseding the operator's word
@row_none n=10 rec=45 type=0
what: a sample output of viola list --json
@row_add n=71 row=contract/viola-list-json/document area=surface-cli rec=46
boundary: viola list --json
item: document
promise: One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract.
@row_add n=72 row=contract/viola-list-json/strict-modes-failed area=surface-cli rec=46
boundary: viola list --json
item: strict-modes-failed
promise: A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
@row_add n=73 row=contract/viola-send-output/input-not-ready-hint area=surface-cli rec=47
boundary: viola send output
item: input-not-ready hint
promise: An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
@row_add n=74 row=contract/viola-send-output/json-ok area=surface-cli rec=47
boundary: viola send output
item: --json ok
promise: Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
@row_add n=75 row=contract/viola-send-output/word-column area=surface-cli rec=48
boundary: viola send output
item: word column
promise: The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
@row_add n=76 row=decision/send-lines-appended-never-redrawn area=surface-cli rec=48
name: Send lines appended, never redrawn
rule: viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
@row_add n=77 row=contract/viola-send-output/piped-stdout area=surface-cli rec=48
boundary: viola send output
item: piped stdout
promise: When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
@row_add n=78 row=contract/viola-wait-output/turn-ended area=surface-cli rec=49
boundary: viola wait output
item: turn-ended
promise: `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
@row_add n=79 row=contract/viola-wait-output/dialog area=surface-cli rec=49
boundary: viola wait output
item: dialog
promise: A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
@row_add n=80 row=contract/viola-wait-output/timed-out area=surface-cli rec=49
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_add n=81 row=contract/viola-last-output/message area=surface-cli rec=49
boundary: viola last output
item: message
promise: The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
@row_add n=82 row=contract/cli-refusals/instance-unreachable area=surface-cli rec=49
boundary: CLI refusals
item: instance-unreachable
promise: With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
@row_add n=83 row=decision/no-spinner area=surface-cli rec=50
name: No spinner
rule: The cli shows no spinner and no elapsed-time counter.
@row_add n=84 row=contract/viola-wait-output/waiting-line area=surface-cli rec=50,49
boundary: viola wait output
item: waiting line
promise: `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set.
@row_add n=85 row=contract/viola-wait-output/dialog-unknown area=surface-cli rec=50
boundary: viola wait output
item: dialog unknown
promise: A dialog line whose event carries no dialog_id reads `dialog unknown`.
@row_add n=86 row=contract/viola-wait-output/dialog-expired area=surface-cli rec=50,49
boundary: viola wait output
item: dialog-expired
promise: A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
@row_add n=87 row=contract/cli-json-mode/one-document area=surface-cli rec=50
boundary: CLI --json mode
item: one document
promise: Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
@row_add n=88 row=contract/cli-internal-error/human area=surface-cli rec=50
boundary: CLI internal error
item: human output
promise: A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
@row_add n=89 row=contract/cli-internal-error/json area=surface-cli rec=50
boundary: CLI internal error
item: --json output
promise: Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
@row_add n=90 row=contract/wheel-verb-output/pause area=surface-cli rec=51
boundary: wheel verb output
item: viola pause
promise: Prints `<name>  wheel human  manual-pause  I have control`.
@row_add n=91 row=contract/wheel-verb-output/release area=surface-cli rec=51
boundary: wheel verb output
item: viola release
promise: Prints `<name>  wheel driver  you have control`.
@row_add n=92 row=contract/viola-answer-output/answered area=surface-cli rec=51
boundary: viola answer output
item: answered
promise: Prints `answered  <name>  dialog <id>`.
@row_add n=93 row=contract/viola-answer-output/unknown-dialog area=surface-cli rec=51
boundary: viola answer output
item: unknown-dialog
promise: An answer to a dialog that is not pending prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exit 13, then `hint: that dialog is not pending; viola list shows the current DIALOG`.
@row_add n=94 row=contract/release-budget-output/gate-lifted area=surface-cli rec=52
boundary: viola release --budget output
item: gate lifted
promise: When a pause stood, prints `<name>  budget gate lifted until <end>  wheel <holder>` on stdout, exit 0, wheel left where it is; `<end>` is `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` otherwise, a non-RFC 3339 value escaped.
@row_add n=95 row=contract/release-budget-output/gate-open area=surface-cli rec=52
boundary: viola release --budget output
item: gate open
promise: When no pause stood, prints `<name>  budget gate open  wheel <holder>` on stdout, exit 0, with the wheel left where it is.
@row_add n=96 row=contract/link-output/link area=surface-cli rec=53
boundary: viola link / unlink output
item: link
promise: Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped.
@row_add n=97 row=contract/link-output/already-linked area=surface-cli rec=53
boundary: viola link / unlink output
item: already linked
promise: A pair already held prints `link    <driver> -> <driven>  already linked`, exit 0.
@row_add n=98 row=contract/link-output/unlink area=surface-cli rec=53
boundary: viola link / unlink output
item: unlink
promise: Prints `unlink  <driver> -> <driven>` in one write, exit 0, whether or not a pair was removed.
@row_add n=99 row=contract/link-output/printed-names area=surface-cli rec=53
boundary: viola link / unlink output
item: printed names
promise: The names printed are the verb's own arguments, never names read back from the reply.
@row_add n=100 row=contract/link-output/json-link area=surface-cli rec=53
boundary: viola link / unlink output
item: link --json
promise: Prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`.
@row_add n=101 row=contract/link-output/json-unlink area=surface-cli rec=53
boundary: viola link / unlink output
item: unlink --json
promise: Prints `{"v":1,"ok":{}}`.
@row_add n=102 row=contract/link-output/wrapper-error area=surface-cli rec=53
boundary: viola link / unlink output
item: wrapper error
promise: A wrapper error, a driver equal to driven (-32602) included, is the wrapper-fault line, exit 20.
@row_add n=103 row=contract/viola-verify-output/failing-row area=surface-cli rec=54,55
boundary: viola verify output
item: failing row
promise: A run with a failing row exits 1 and still writes the stamp.
@row_add n=104 row=contract/viola-verify-output/usage area=surface-cli rec=55
boundary: viola verify output
item: usage
promise: `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
@row_add n=105 row=contract/viola-verify-output/step-line area=surface-cli rec=55
boundary: viola verify output
item: step line
promise: One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
@row_add n=106 row=contract/viola-verify-output/summary area=surface-cli rec=55,54
boundary: viola verify output
item: summary
promise: `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
@row_add n=107 row=contract/viola-verify-output/exit-0 area=surface-cli rec=55
boundary: viola verify output
item: exit 0
promise: Exit 0 when every relied row passes.
@row_add n=108 row=contract/viola-verify-output/exit-1 area=surface-cli rec=55
boundary: viola verify output
item: exit 1
promise: Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
@row_add n=109 row=contract/viola-verify-output/stop area=surface-cli rec=55,54
boundary: viola verify output
item: stop
promise: A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
@row_add n=110 row=contract/viola-verify-output/record area=surface-cli rec=55
boundary: viola verify output
item: --record
promise: --record adds no human line.
@row_add n=111 row=contract/viola-verify-output/plain-ascii area=surface-cli rec=55
boundary: viola verify output
item: plain ASCII
promise: All viola verify output is plain ASCII with no colour.
@row_add n=112 row=contract/viola-verify-output/help-paragraph area=surface-cli rec=55
boundary: viola verify output
item: --help
promise: viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
