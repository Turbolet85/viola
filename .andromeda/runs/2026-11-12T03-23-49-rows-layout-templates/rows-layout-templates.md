@row_none n=1 rec=fct-hc5anmlh@1 type=S
@row_none n=2 rec=fct-ph6oztbi@1 type=8
@row_none n=3 rec=fct-ltetwsoo@1 type=0
what: a note on when the document may be edited by hand
@row_none n=4 rec=fct-2hfcvv5g@1 type=S
@row_add n=1 row=absence/web-surface area=surface-web-spa rec=fct-jdnb6ocv@2
capability: A web surface: viola renders no page
@row_none n=5 rec=fct-xc3aytgy@1 type=S
@row_add n=2 row=decision/clap-derive-cli-parsing area=surface-cli rec=fct-vnzdqtob@2
name: clap derive for CLI parsing
rule: The viola command line is parsed with clap 4 in its derive form.
@row_add n=3 row=decision/std-isterminal-tty-detection area=surface-cli rec=fct-vnzdqtob@2
name: TTY detection through std IsTerminal
rule: Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
@row_add n=4 row=decision/hand-written-sgr-styling area=surface-cli rec=fct-vnzdqtob@2
name: Hand-written SGR styling module
rule: Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate.
@row_none n=6 rec=fct-nlcn5hiz@1 type=S
@row_none n=7 rec=fct-psc5isem@1 type=S
@row_add n=5 row=contract/cli-token-rendering/color-accent area=surface-cli rec=fct-bb67thmp@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-accent
promise: The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word.
@row_add n=6 row=contract/cli-token-rendering/color-text-muted area=surface-cli rec=fct-xw7xe4pu@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-text-muted
promise: The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE.
@row_add n=7 row=contract/cli-token-rendering/callsign-role area=surface-cli rec=fct-jyy7fp52@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Callsign role
promise: The callsign token: SGR bold on NAME values only.
@row_add n=8 row=contract/cli-token-rendering/color-text-primary-secondary area=surface-cli rec=fct-bzzlsqum@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-text-primary / color-text-secondary
promise: The terminal's default foreground; the terminal owns ink.
@row_add n=9 row=contract/cli-token-rendering/color-role-handoff area=surface-cli rec=fct-lrmne7xu@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-role-handoff
promise: Not used; link output is plain.
@row_add n=10 row=contract/cli-token-rendering/color-surface area=surface-cli rec=fct-rl3k2zcm@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: color-surface-*
promise: Never set; the terminal background belongs to the user.
@row_add n=11 row=contract/cli-token-rendering/label-role area=surface-cli rec=fct-42zyjbxs@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Label role
promise: Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
@row_add n=12 row=contract/cli-token-rendering/data-role area=surface-cli rec=fct-frgtrpdl@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Data role
promise: Tabular figures and timestamps.
@row_add n=13 row=contract/cli-token-rendering/spacing area=surface-cli rec=fct-lxjb27mz@2,fct-xuyt7znh@1
boundary: CLI token rendering
item: Spacing
promise: The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
@row_add n=14 row=contract/cli-token-rendering/radius area=surface-cli rec=fct-uyipavwi@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: radius-*
promise: No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
@row_add n=15 row=contract/cli-token-rendering/motion-tokens area=surface-cli rec=fct-ndkpwvcx@1,fct-xuyt7znh@1
boundary: CLI token rendering
item: Motion tokens
promise: None; nothing redraws in place.
@row_add n=16 row=decision/one-token-vocabulary-on-cli area=surface-cli rec=fct-ndkpwvcx@1
name: One token vocabulary resolved on the cli
rule: The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box.
@row_add n=17 row=decision/human-tty-expression-level area=surface-cli rec=fct-ekdqr4ye@1
name: Human TTY expression level
rule: On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
@row_add n=18 row=decision/plain-output-modes area=surface-cli rec=fct-ekdqr4ye@1
name: Plain output modes
rule: Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
@row_add n=19 row=decision/readback-mirror-signature area=surface-cli rec=fct-w4oklt2h@1
name: Readback mirror as the send signature
rule: The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
@row_add n=20 row=contract/viola-send-output/issue-line area=surface-cli rec=fct-3xzidqkr@1,fct-w4oklt2h@1
boundary: viola send output
item: issue line
promise: `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
@row_add n=21 row=contract/viola-send-output/read-back area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back
promise: `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by the matching prompt-submitted.
@row_add n=22 row=contract/viola-send-output/read-back-clear area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back for /clear
promise: On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
@row_add n=23 row=contract/viola-send-output/read-back-compact area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: read back for /compact
promise: On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
@row_add n=24 row=contract/viola-send-output/unable area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: unable
promise: `[/ ] unable …` prints on stderr with exit 13 (or 10/11/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
@row_add n=25 row=contract/viola-send-output/unconfirmable area=surface-cli rec=fct-yebd4xys@1,fct-w4oklt2h@1
boundary: viola send output
item: unconfirmable
promise: `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.
@row_add n=26 row=decision/shared-refusal-word-column area=surface-cli rec=fct-byp7w5z4@1
name: Shared refusal word column
rule: The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
@row_add n=27 row=contract/viola-send-output/json-mode area=surface-cli rec=fct-byp7w5z4@1
boundary: viola send output
item: --json
promise: Under --json the mirror is replaced by the typed ok / refusal document.
@row_add n=28 row=decision/dialog-attention-only-on-board area=surface-cli rec=fct-byp7w5z4@1
name: DIALOG attention word only on the board
rule: The amber DIALOG attention word appears only in viola list.
@row_add n=29 row=contract/cli-verbs/list area=surface-cli rec=fct-s2seqjyg@1
boundary: CLI verbs
item: viola list
promise: The board: every session as one strip row, with the BAY context header.
@row_add n=30 row=contract/cli-verbs/send area=surface-cli rec=fct-5vvtaix2@1
boundary: CLI verbs
item: viola send <target>
promise: Types at a turn boundary and reports read back, unable or unconfirmable.
@row_add n=31 row=contract/cli-verbs/wait area=surface-cli rec=fct-36lpgwnn@1
boundary: CLI verbs
item: viola wait <target>
promise: Returns one turn-boundary result, with no spinner.
@row_add n=32 row=contract/cli-verbs/last area=surface-cli rec=fct-36lpgwnn@1
boundary: CLI verbs
item: viola last <target>
promise: Returns the last assistant message, with no spinner.
@row_add n=33 row=decision/spoken-phraseology-verb-output area=surface-cli rec=fct-xicxxmup@1
name: Spoken phraseology for wheel, handoff and dialog verbs
rule: The output of viola pause, release, link, unlink and answer is spoken phraseology.
@row_add n=34 row=decision/single-progress-pattern area=surface-cli rec=fct-4kxdu6am@1
name: Single progress pattern
rule: The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
@row_add n=35 row=contract/cli-verbs/run area=surface-cli rec=fct-zbrdgwj5@1
boundary: CLI verbs
item: viola run <name> -- claude
promise: Passthrough; prints nothing while the child runs.
@row_add n=36 row=decision/mcp-server-without-human-surface area=surface-cli rec=fct-lrricjwq@1
name: MCP server with no human surface
rule: viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
@row_add n=37 row=contract/mcp-tools/send area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: send
promise: A driver's machine view of the viola send verb.
@row_add n=38 row=contract/mcp-tools/wait area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: wait
promise: A driver's machine view of the viola wait verb.
@row_add n=39 row=contract/mcp-tools/last area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: last
promise: A driver's machine view of the viola last verb.
@row_add n=40 row=contract/mcp-tools/answer area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: answer
promise: A driver's machine view of the viola answer verb.
@row_add n=41 row=contract/mcp-tools/list area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tools
item: list
promise: A driver's machine view of the viola list verb.
@row_add n=42 row=contract/mcp-tool-results/success area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tool results
item: success
promise: Each success is the CLI verb's --json document less `v`.
@row_add n=43 row=contract/mcp-tool-results/refusal area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tool results
item: refusal
promise: Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
@row_add n=44 row=contract/mcp-tool-results/invalid-params area=surface-cli rec=fct-lrricjwq@1
boundary: viola mcp tool results
item: invalid-params
promise: Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
@row_add n=45 row=decision/mcp-hides-wheel-and-handoff-verbs area=surface-cli rec=fct-lrricjwq@1
name: MCP hides wheel and handoff verbs
rule: No viola mcp tool, description or result names release, pause, link or unlink.
@row_add n=46 row=contract/cli-verbs/revive area=surface-cli rec=fct-k7kme3xv@1
boundary: CLI verbs
item: viola revive <name>
promise: Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
@row_add n=47 row=contract/cli-verbs/revive-list area=surface-cli rec=fct-k7kme3xv@1
boundary: CLI verbs
item: viola revive <name> --list
promise: Prints the instance's logged sessions, one static line each.
@row_add n=48 row=contract/cli-verbs/plugin-install area=surface-cli rec=fct-5lv3otbb@1
boundary: CLI verbs
item: viola plugin install
promise: Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude.
@row_add n=49 row=contract/cli-verbs/help area=surface-cli rec=fct-pgyxchhq@1
boundary: CLI verbs
item: viola --help
promise: The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding plugin install.
@row_none n=8 rec=fct-j2ytpooj@1 type=0
what: a sample output wireframe of viola list
@row_add n=50 row=decision/board-ordered-by-name area=surface-cli rec=fct-mzvyqqgp@1
name: Board ordered by name
rule: viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold.
@row_add n=51 row=contract/viola-list-board/gate-closed area=surface-cli rec=fct-g6mmpt3p@1
boundary: viola list board
item: closed budget gate
promise: A closed budget gate prints `budget-paused` in place of `gate open`.
@row_add n=52 row=contract/viola-list-board/window-expired area=surface-cli rec=fct-g6mmpt3p@1
boundary: viola list board
item: passed window
promise: A passed budget window prints with `expired` after its figure, as `5H 62 % expired`.
@row_add n=53 row=contract/viola-list-board/stale-row-with-dialog area=surface-cli rec=fct-3lloyyhg@1,fct-mdoqfj5o@2
boundary: viola list board
item: stale row with a dialog
promise: On a cocked and stale row, the DIALOG word stays amber and not dim, while the kind word and the rest of the row stay dim.
@row_add n=54 row=contract/viola-list-board/empty-board area=surface-cli rec=fct-lnegijti@1
boundary: viola list board
item: empty board
promise: An empty board prints `no wrapped sessions  start one: viola run <name> -- claude`.
@row_add n=55 row=decision/words-carry-every-state area=surface-cli rec=fct-i5xtjrqd@1,fct-5ius22jn@1
name: Words carry every state
rule: Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
@row_add n=56 row=contract/viola-list-board/unknown-width area=surface-cli rec=fct-i5xtjrqd@1,fct-mdoqfj5o@2
boundary: viola list board
item: unknown terminal width
promise: Nothing is truncated when the terminal width is unknown.
@row_add n=57 row=contract/viola-list-board/single-write area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: write order
promise: viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one.
@row_add n=58 row=contract/viola-list-board/uncoloured-modes area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: uncoloured modes
promise: The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
@row_add n=59 row=contract/viola-list-board/colour-depth area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: colour depth
promise: On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM.
@row_add n=60 row=contract/viola-list-board/stale-row-dim area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: stale row
promise: A wrapped stale row is dim from its first byte to its last, NAME dim and bold.
@row_add n=61 row=contract/viola-list-board/plain-lines area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: plain lines
promise: The BAY line, caption, empty line and separator print plain.
@row_add n=62 row=contract/viola-list-board/status-join area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: STATUS
promise: One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
@row_add n=63 row=contract/viola-list-board/agents-read-failure area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: failed agents read
promise: A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
@row_add n=64 row=contract/viola-list-board/unwrapped-rack area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: unwrapped rack
promise: Unjoined agent rows print after the wrapped rows under `-- UNWRAPPED - READ-ONLY --`, sorted by name: NAME the escaped sessionId, LIVE live, its STATUS, n/a in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints.
@row_add n=65 row=contract/viola-list-board/column-widths area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: column widths
promise: NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
@row_add n=66 row=contract/viola-list-board/name-truncation area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: NAME truncation
promise: With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
@row_add n=67 row=contract/viola-list-board/wheel-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: WHEEL unknown
promise: A replayed row with no logged holder reads WHEEL `unknown`.
@row_add n=68 row=contract/viola-list-board/cli-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: CLI unknown
promise: A row with no recorded version reads CLI `unknown`.
@row_add n=69 row=contract/viola-list-board/dialog-unknown area=surface-cli rec=fct-mdoqfj5o@2
boundary: viola list board
item: DIALOG unknown
promise: A pending dialog of unknown kind reads `DIALOG unknown`.
@row_add n=70 row=decision/board-planning-forms-wording area=surface-cli rec=fct-mdoqfj5o@2
name: Board planning forms as the ratified wording
rule: The board's planning forms (a figure rounded down, the whole-minute age and `read unknown`, bare skip counts, `human - budget-paused`, the empty board's three lines) are its wording.
why: confirmed as built by the founder, superseding the operator's word
@row_none n=9 rec=fct-d6ktoce7@1 type=0
what: a sample output of viola list --json
@row_add n=71 row=contract/viola-list-json/document area=surface-cli rec=fct-yt73cavp@1
boundary: viola list --json
item: document
promise: One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract.
@row_add n=72 row=contract/viola-list-json/strict-modes-failed area=surface-cli rec=fct-yt73cavp@1
boundary: viola list --json
item: strict-modes-failed
promise: A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
@row_add n=73 row=contract/viola-send-output/input-not-ready-hint area=surface-cli rec=fct-xzvoaizl@1
boundary: viola send output
item: input-not-ready hint
promise: An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
@row_add n=74 row=contract/viola-send-output/json-ok area=surface-cli rec=fct-xzvoaizl@1
boundary: viola send output
item: --json ok
promise: Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
@row_add n=75 row=contract/viola-send-output/word-column area=surface-cli rec=fct-resu4xix@1
boundary: viola send output
item: word column
promise: The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
@row_add n=76 row=decision/send-lines-appended-never-redrawn area=surface-cli rec=fct-resu4xix@1
name: Send lines appended, never redrawn
rule: viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
@row_add n=77 row=contract/viola-send-output/piped-stdout area=surface-cli rec=fct-resu4xix@1
boundary: viola send output
item: piped stdout
promise: When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
@row_add n=78 row=contract/viola-wait-output/turn-ended area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: turn-ended
promise: `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
@row_add n=79 row=contract/viola-wait-output/dialog area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: dialog
promise: A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
@row_add n=80 row=contract/viola-wait-output/timed-out area=surface-cli rec=fct-r2nmoonb@1
boundary: viola wait output
item: timed out
promise: A timeout prints `timed out  <name>  <ms> ms` on stdout.
@row_add n=81 row=contract/viola-last-output/message area=surface-cli rec=fct-r2nmoonb@1
boundary: viola last output
item: message
promise: The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
@row_add n=82 row=contract/cli-refusals/instance-unreachable area=surface-cli rec=fct-r2nmoonb@1
boundary: CLI refusals
item: instance-unreachable
promise: With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
@row_add n=83 row=decision/no-spinner area=surface-cli rec=fct-ujoiukvr@1
name: No spinner
rule: The cli shows no spinner and no elapsed-time counter.
@row_add n=84 row=contract/viola-wait-output/waiting-line area=surface-cli rec=fct-ujoiukvr@1,fct-r2nmoonb@1
boundary: viola wait output
item: waiting line
promise: `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set.
@row_add n=85 row=contract/viola-wait-output/dialog-unknown area=surface-cli rec=fct-ujoiukvr@1
boundary: viola wait output
item: dialog unknown
promise: A dialog line whose event carries no dialog_id reads `dialog unknown`.
@row_add n=86 row=contract/viola-wait-output/dialog-expired area=surface-cli rec=fct-ujoiukvr@1,fct-r2nmoonb@1
boundary: viola wait output
item: dialog-expired
promise: A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
@row_add n=87 row=contract/cli-json-mode/one-document area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI --json mode
item: one document
promise: Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
@row_add n=88 row=contract/cli-internal-error/human area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI internal error
item: human output
promise: A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
@row_add n=89 row=contract/cli-internal-error/json area=surface-cli rec=fct-ujoiukvr@1
boundary: CLI internal error
item: --json output
promise: Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
@row_add n=90 row=contract/wheel-verb-output/pause area=surface-cli rec=fct-dryeax7x@1
boundary: wheel verb output
item: viola pause
promise: Prints `<name>  wheel human  manual-pause  I have control`.
@row_add n=91 row=contract/wheel-verb-output/release area=surface-cli rec=fct-dryeax7x@1
boundary: wheel verb output
item: viola release
promise: Prints `<name>  wheel driver  you have control`.
@row_add n=92 row=contract/viola-answer-output/answered area=surface-cli rec=fct-dryeax7x@1
boundary: viola answer output
item: answered
promise: Prints `answered  <name>  dialog <id>`.
@row_add n=93 row=contract/viola-answer-output/unknown-dialog area=surface-cli rec=fct-dryeax7x@1
boundary: viola answer output
item: unknown-dialog
promise: An answer to a dialog that is not pending prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exit 13, then `hint: that dialog is not pending; viola list shows the current DIALOG`.
@row_add n=94 row=contract/release-budget-output/gate-lifted area=surface-cli rec=fct-fkpva2qn@1
boundary: viola release --budget output
item: gate lifted
promise: When a pause stood, prints `<name>  budget gate lifted until <end>  wheel <holder>` on stdout, exit 0, wheel left where it is; `<end>` is `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` otherwise, a non-RFC 3339 value escaped.
@row_add n=95 row=contract/release-budget-output/gate-open area=surface-cli rec=fct-fkpva2qn@1
boundary: viola release --budget output
item: gate open
promise: When no pause stood, prints `<name>  budget gate open  wheel <holder>` on stdout, exit 0, with the wheel left where it is.
@row_add n=96 row=contract/link-output/link area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: link
promise: Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped.
@row_add n=97 row=contract/link-output/already-linked area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: already linked
promise: A pair already held prints `link    <driver> -> <driven>  already linked`, exit 0.
@row_add n=98 row=contract/link-output/unlink area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: unlink
promise: Prints `unlink  <driver> -> <driven>` in one write, exit 0, whether or not a pair was removed.
@row_add n=99 row=contract/link-output/printed-names area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: printed names
promise: The names printed are the verb's own arguments, never names read back from the reply.
@row_add n=100 row=contract/link-output/json-link area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: link --json
promise: Prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`.
@row_add n=101 row=contract/link-output/json-unlink area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: unlink --json
promise: Prints `{"v":1,"ok":{}}`.
@row_add n=102 row=contract/link-output/wrapper-error area=surface-cli rec=fct-6m6poraj@1
boundary: viola link / unlink output
item: wrapper error
promise: A wrapper error, a driver equal to driven (-32602) included, is the wrapper-fault line, exit 20.
@row_add n=103 row=contract/viola-verify-output/failing-row area=surface-cli rec=fct-hk2gtsxu@2,fct-pf5uzbca@2
boundary: viola verify output
item: failing row
promise: A run with a failing row exits 1 and still writes the stamp.
@row_add n=104 row=contract/viola-verify-output/usage area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: usage
promise: `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
@row_add n=105 row=contract/viola-verify-output/step-line area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: step line
promise: One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
@row_add n=106 row=contract/viola-verify-output/summary area=surface-cli rec=fct-pf5uzbca@2,fct-hk2gtsxu@2
boundary: viola verify output
item: summary
promise: `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
@row_add n=107 row=contract/viola-verify-output/exit-0 area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: exit 0
promise: Exit 0 when every relied row passes.
@row_add n=108 row=contract/viola-verify-output/exit-1 area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: exit 1
promise: Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
@row_add n=109 row=contract/viola-verify-output/stop area=surface-cli rec=fct-pf5uzbca@2,fct-hk2gtsxu@2
boundary: viola verify output
item: stop
promise: A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
@row_add n=110 row=contract/viola-verify-output/record area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: --record
promise: --record adds no human line.
@row_add n=111 row=contract/viola-verify-output/plain-ascii area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: plain ASCII
promise: All viola verify output is plain ASCII with no colour.
@row_add n=112 row=contract/viola-verify-output/help-paragraph area=surface-cli rec=fct-pf5uzbca@2
boundary: viola verify output
item: --help
promise: viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
@row_add n=113 row=contract/viola-help/usage-line area=surface-cli rec=fct-dljdvvyu@1,fct-4rbi2th6@2
boundary: viola --help output
item: usage line
promise: `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
@row_add n=114 row=contract/viola-help/verb-groups area=surface-cli rec=fct-dljdvvyu@1,fct-4rbi2th6@2,fct-xpsr3izn@1
boundary: viola --help output
item: verb group rows
promise: Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
@row_add n=115 row=decision/mcp-verb-hidden area=surface-cli rec=fct-4rbi2th6@2
name: viola mcp is a hidden verb
rule: `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
@row_add n=116 row=contract/viola-verb-help/usage area=surface-cli rec=fct-33x2qanb@2
boundary: viola <verb> --help output
item: per-verb usage
promise: `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
@row_add n=117 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=fct-33x2qanb@2
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so.
@row_add n=118 row=contract/viola-run/start-output area=surface-cli rec=fct-5cewxahg@1,fct-igaudjuy@1
boundary: viola run start
item: passed start output
promise: `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
@row_add n=119 row=contract/viola-run/already-live area=surface-cli rec=fct-5cewxahg@1
boundary: viola run start refusals
item: already live
promise: Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
@row_add n=120 row=contract/viola-revive/usage area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive
item: usage
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_add n=121 row=contract/viola-revive/start-output area=surface-cli rec=fct-dahc6e7w@1,fct-n7x2i5oo@1
boundary: viola revive
item: passed start output
promise: A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
@row_add n=122 row=contract/viola-revive/list-row area=surface-cli rec=fct-dahc6e7w@1,fct-n7x2i5oo@1
boundary: viola revive
item: --list row
promise: `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
@row_add n=123 row=contract/viola-revive/list-cause area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive
item: --list cause field
promise: The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise.
@row_add n=124 row=contract/viola-revive/refusal-shape area=surface-cli rec=fct-dahc6e7w@1
boundary: viola revive refusals
item: refusal shape
promise: Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
@row_add n=125 row=contract/viola-revive-refusals/state-files-writable area=surface-cli rec=fct-lu5j3ng5@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: state files writable by another user
promise: `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
@row_add n=126 row=contract/viola-revive-refusals/already-live area=surface-cli rec=fct-snlrx2zh@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: already live
promise: `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
@row_add n=127 row=contract/viola-revive-refusals/no-logged-session area=surface-cli rec=fct-qz5yofop@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: no logged session
promise: `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
@row_add n=128 row=contract/viola-revive-refusals/no-session-with-id area=surface-cli rec=fct-j4jbg56f@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: no logged session with that id
promise: `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
@row_add n=129 row=contract/viola-revive-refusals/session-open-elsewhere area=surface-cli rec=fct-gkytyqxv@1,fct-dahc6e7w@1
boundary: viola revive refusals
item: session open in another claude
promise: `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_add n=130 row=contract/viola-revive-refusals/directory-missing area=surface-cli rec=fct-xrecgcsk@1,fct-n7x2i5oo@1
boundary: viola revive refusals
item: recorded directory missing
promise: `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
@row_add n=131 row=decision/no-hint-names-release area=surface-cli rec=fct-bwpa6bmo@1,fct-xpsr3izn@1
name: No hint names viola release
rule: Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`.
why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
@row_add n=132 row=contract/viola-revive/list-refusals area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive refusals
item: --list refusals
promise: `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair.
@row_add n=133 row=contract/run-revive-start/home-tree-refused area=surface-cli rec=fct-bwpa6bmo@1,fct-7zli33yx@3
boundary: viola run and viola revive starts
item: home tree refused
promise: Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
@row_add n=134 row=contract/viola-revive/preflight-order area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive start
item: preflight order
promise: The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
@row_add n=135 row=contract/viola-revive/session-live-check area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive start
item: session-live check
promise: One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
@row_add n=136 row=contract/viola-revive/malformed-id area=surface-cli rec=fct-bwpa6bmo@1
boundary: viola revive
item: malformed --id
promise: A malformed `--id` is clap's usage error, exit 2.
@row_add n=137 row=contract/viola-revive-json/refusal-document area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json
item: refusal document
promise: Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
@row_add n=138 row=contract/viola-revive-json/strict-modes-failed area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: strict-modes-failed
promise: The code for a refused home tree or instance state files.
@row_add n=139 row=contract/viola-revive-json/already-live area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json error codes
item: already-live
promise: The code for a live holder or a fresh replayed beat.
@row_add n=140 row=contract/viola-revive-json/stale-heartbeat area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The code for a stale holder.
@row_add n=141 row=contract/viola-revive-json/no-session area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: no-session
promise: The code for both no-logged-session pairs: no logged session, and no logged session with that id.
@row_add n=142 row=contract/viola-revive-json/session-live area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: session-live
promise: The code for a session open in another claude.
@row_add n=143 row=contract/viola-revive-json/cwd-missing area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json error codes
item: cwd-missing
promise: The code for a missing recorded directory.
@row_add n=144 row=contract/viola-revive-json/list-document area=surface-cli rec=fct-i322ygxm@2,fct-un7m7e5w@1
boundary: viola revive --json
item: --list document
promise: `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
@row_add n=145 row=contract/run-revive-start-json/pinned-hash-mismatch area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: pinned-hash-mismatch
promise: After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
@row_add n=146 row=contract/run-revive-start-json/squatted-name area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: squatted-name
promise: After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
@row_add n=147 row=contract/run-revive-start-json/socket-dir-refused area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: socket-dir-refused
promise: After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
@row_add n=148 row=contract/run-revive-start-json/internal-error area=surface-cli rec=fct-i322ygxm@2
boundary: viola revive --json start refusals shared with run
item: internal-error
promise: A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
@row_add n=149 row=contract/viola-plugin-install/output area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2
boundary: viola plugin install
item: human output
promise: After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
@row_add n=150 row=contract/viola-plugin-install/dir-quoting area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: <dir> quoting
promise: `<dir>` is the canonical home joined with `marketplace`, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`, else printed bare.
@row_add n=151 row=contract/viola-plugin-install/json-ok area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2
boundary: viola plugin install
item: --json ok document
promise: Under `--json` it prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path.
@row_add n=152 row=contract/viola-plugin-install/strict-modes-refused area=surface-cli rec=fct-4an42gpj@1,fct-bievnfhg@2,fct-7zli33yx@3
boundary: viola plugin install
item: strict-modes refusal
promise: A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_add n=153 row=contract/viola-plugin-install/internal-error area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: internal error
promise: A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
@row_add n=154 row=contract/viola-plugin-install/bare-plugin area=surface-cli rec=fct-bievnfhg@2
boundary: viola plugin install
item: bare viola plugin
promise: `viola plugin` alone is a clap usage error, exit 2; clap serves its own `viola plugin help [install]`.
@row_add n=155 row=decision/only-header-bay-line area=surface-cli rec=fct-ngdz2kjd@1
name: Only header is the BAY line
rule: The only header on any verb is the one static BAY line at the top of `viola list`.
@row_add n=156 row=contract/bay-line/cell-order area=surface-cli rec=fct-bz2eawmt@1
boundary: the BAY line of viola list
item: cell order
promise: `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`, separated by the two-space gutter.
@row_add n=157 row=decision/bay-line-unstyled area=surface-cli rec=fct-hjvzsdrb@1
name: BAY line unstyled
rule: The BAY line's label words and data figures are all in the default foreground, with no bold, no colour, no box drawing and no rule line under it.
@row_add n=158 row=decision/no-banner area=surface-cli rec=fct-igaudjuy@1
name: No banner anywhere
rule: Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
@row_add n=159 row=decision/flat-verbs area=surface-cli rec=fct-ejymuula@2,fct-v22aegyt@1
name: Flat verbs plus a target
rule: Each verb is one lower-case word with no nested noun hierarchy or subcommand tree.
except: `plugin install` is the one two-word verb.
@row_add n=160 row=decision/no-prompt-or-pager area=surface-cli rec=fct-ejymuula@2
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
@row_add n=161 row=decision/argument-order-mirrors-channel area=surface-cli rec=fct-thhsjl2y@1
name: Argument order mirrors the channel params
rule: CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
@row_add n=162 row=decision/board-vs-strip area=surface-cli rec=fct-rgpifi6m@1,fct-v22aegyt@1
name: Board versus strip verbs
rule: `viola list` shows the whole bay, and every other verb acts on one strip named by its `ViolaName`.
@row_add n=163 row=contract/cli-global-flags/home area=surface-cli rec=fct-lp2knewg@1
boundary: CLI global flags
item: --home
promise: `--home` is the same on every verb.
@row_add n=164 row=decision/json-on-every-data-verb area=surface-cli rec=fct-lp2knewg@1
name: --json on every data verb
rule: `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
@row_add n=165 row=decision/hints-name-next-verb area=surface-cli rec=fct-xpsr3izn@1
name: Hints name the next verb
rule: `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
@row_add n=166 row=contract/readback-mirror/layout area=surface-cli rec=fct-3k27ozst@1
boundary: the readback mirror line
item: layout
promise: A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
@row_add n=167 row=contract/readback-mirror/issue-and-outcome area=surface-cli rec=fct-gyr4lim4@1
boundary: the readback mirror line
item: issue and outcome lines
promise: The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
@row_add n=168 row=decision/readback-mirror-no-colour area=surface-cli rec=fct-u27psd7j@1
name: Readback mirror has no colour
rule: The readback mirror is words and brackets only: `read back` is never green and `unable` is never red.
why: Colour on the CLI is reserved for `DIALOG` and `stale`.
@row_add n=169 row=decision/no-progress-glyphs area=surface-cli rec=fct-fce3gzpi@1
name: No send progress glyphs
rule: A send shows no `Sending…` spinner, no `✓` and no `done`.
@row_add n=170 row=decision/turn-ended-not-send-outcome area=surface-cli rec=fct-fce3gzpi@1
name: turn-ended is not a send outcome
rule: `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
@row_add n=171 row=contract/send-json/output area=surface-cli rec=fct-wi75uj5p@1
boundary: viola send --json
item: typed document
promise: `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
@row_add n=172 row=contract/viola-list/columns area=surface-cli rec=fct-xqbw4mbe@1
boundary: viola list
item: columns
promise: NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
@row_add n=173 row=decision/viola-list-styling area=surface-cli rec=fct-tgkwzcus@1,fct-33ocject@1
name: viola list styling
rule: On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off.
@row_add n=174 row=contract/viola-list/unwrapped-separator area=surface-cli rec=fct-hrcl6tma@1,fct-33ocject@1
boundary: viola list
item: rack separator
promise: `-- UNWRAPPED - READ-ONLY --` on its own line before the unwrapped rows, written only when unwrapped rows exist; the wrapped rack is always first, with no separator.
@row_add n=175 row=decision/truncate-only-unwrapped-names area=surface-cli rec=fct-zy7whx5p@1,fct-33ocject@1
name: Truncate only unwrapped names
rule: Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
@row_add n=176 row=decision/board-row-escaping area=surface-cli rec=fct-d7l52glo@1,fct-33ocject@1
name: Board row escaping
rule: Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width.
why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text.
@row_add n=177 row=decision/no-link-column area=surface-cli rec=fct-ehkdenpx@1
name: No link column on the board
rule: The six shared board columns stay fixed; links are visible only in `link` / `unlink` verb output.
@row_add n=178 row=contract/cli-refusal-line/form area=surface-cli rec=fct-7zli33yx@3
boundary: CLI refusal line
item: line form
promise: `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
@row_add n=179 row=contract/viola-verify-refusals/cli-not-found area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: claude CLI not found
promise: Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
@row_add n=180 row=contract/viola-verify-refusals/version-unreadable area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: CLI version unreadable
promise: Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
@row_add n=181 row=contract/viola-verify-refusals/fixture-not-clean area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: recorded fixture not clean
promise: Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
@row_add n=182 row=contract/viola-verify-refusals/stopped area=surface-cli rec=fct-7zli33yx@3,fct-g6zaewhr@1
boundary: viola verify refusals
item: verify stopped
promise: A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
@row_add n=183 row=contract/viola-verify-refusals/home-not-private area=surface-cli rec=fct-7zli33yx@3
boundary: viola verify refusals
item: viola home not private
promise: A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
@row_add n=184 row=contract/run-revive-start/socket-dir-refused area=surface-cli rec=fct-7zli33yx@3
boundary: viola run and viola revive starts
item: socket directory refused
promise: Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=185 row=contract/viola-list/strict-modes-refused area=surface-cli rec=fct-7zli33yx@3
boundary: viola list
item: strict-modes refusal
promise: Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
@row_add n=186 row=contract/frame-verbs/strict-modes-refused area=surface-cli rec=fct-7zli33yx@3
boundary: the frame verbs
item: strict-modes refusal
promise: For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_add n=187 row=contract/cli-catch-site/internal-error area=surface-cli rec=fct-7zli33yx@3
boundary: CLI verbs outside a revive start
item: internal error
promise: A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
@row_add n=188 row=contract/send-refusals/control-character area=surface-cli rec=fct-bffy2yb3@1
boundary: viola send refusals before any request
item: control-character
promise: A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
@row_add n=189 row=contract/send-refusals/empty-text area=surface-cli rec=fct-bffy2yb3@1
boundary: viola send refusals before any request
item: empty-text
promise: A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
@row_add n=190 row=decision/wrapper-repeats-send-refusals area=surface-cli rec=fct-bffy2yb3@1
name: Wrapper repeats the send text refusals
rule: The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
@row_add n=191 row=decision/hint-for-every-cause area=surface-cli rec=fct-qhojlk2s@3
name: A hint for every refusal cause
rule: Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint.
why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line.
except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
@row_add n=192 row=contract/frame-verb-hints/instance-not-running area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: instance not running
promise: `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
@row_add n=193 row=contract/frame-verb-hints/not-wrapped area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: name with no instance directory
promise: `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
@row_add n=194 row=contract/frame-verb-hints/wrapper-went-away area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: wrapper lost during the call
promise: `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line.
@row_add n=195 row=contract/frame-verb-hints/server-verify-failed area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: endpoint not the recorded wrapper
promise: No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
@row_add n=196 row=contract/frame-verb-hints/socket-dir-refused area=surface-cli rec=fct-qhojlk2s@3
boundary: frame verb exit-21 hints
item: socket directory refused
promise: No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty.
@row_add n=197 row=decision/hints-never-quote-or-locate area=surface-cli rec=fct-qhojlk2s@3
name: Hints never quote text or name paths
rule: A hint never quotes the sent text or any upstream text, and never names a path or a pid.
@row_add n=198 row=contract/cli-exit-codes/10 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 10
promise: `human-typing`.
@row_add n=199 row=contract/cli-exit-codes/11 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 11
promise: `budget-paused`.
@row_add n=200 row=contract/cli-exit-codes/12 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 12
promise: `unverified-cli`.
@row_add n=201 row=contract/cli-exit-codes/13 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 13
promise: `not-delivered`.
@row_add n=202 row=contract/cli-exit-codes/14 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 14
promise: `unknown`.
@row_add n=203 row=contract/cli-exit-codes/20 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 20
promise: Wrapper fault, `error: wrapper fault  <code>`.
@row_add n=204 row=contract/cli-exit-codes/21 area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
@row_add n=205 row=contract/cli-exit-codes/json-form area=surface-cli rec=fct-fe7hoqi5@1
boundary: CLI exit codes
item: --json form
promise: Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
@row_add n=206 row=contract/instance-unreachable-detail/strict-modes-failed area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: strict-modes-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=207 row=contract/instance-unreachable-detail/instance-dead area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: instance-dead
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=208 row=contract/instance-unreachable-detail/unwrapped-name area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: unwrapped-name
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=209 row=contract/instance-unreachable-detail/endpoint-lost area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: endpoint-lost
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=210 row=contract/instance-unreachable-detail/server-verify-failed area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: server-verify-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=211 row=contract/instance-unreachable-detail/socket-dir-refused area=surface-cli rec=fct-fe7hoqi5@1
boundary: exit-21 --json detail
item: socket-dir-refused
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=212 row=decision/stdout-results-stderr-rest area=surface-cli rec=fct-enuexet3@1
name: Results on stdout, the rest on stderr
rule: Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
@row_add n=213 row=decision/no-terminator-word area=surface-cli rec=fct-gnhj2wau@1
name: No terminator word
rule: No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
@row_add n=214 row=decision/hint-last-on-stderr area=surface-cli rec=fct-io65ypkg@1
name: Hint is the last stderr line
rule: On a refusal, the `hint:` line is the last line on stderr.
@row_add n=215 row=contract/viola-verify/summary area=surface-cli rec=fct-g6zaewhr@1
boundary: viola verify output
item: summary line
promise: `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
@row_add n=216 row=decision/no-persistent-footer area=surface-cli rec=fct-ilu5rgbe@1
name: No persistent footer
rule: The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
@row_add n=217 row=absence/cli-deep-links area=surface-cli rec=fct-6tq7nrxg@1
capability: Deep links or URL coupling in the CLI
@row_add n=218 row=decision/violaname-never-case-transformed area=surface-cli rec=fct-6tq7nrxg@1
name: ViolaName never case-transformed
rule: `ViolaName` is a shared reference and is never case-transformed.
@row_add n=219 row=contract/cursor/byte-offset area=surface-cli rec=fct-6tq7nrxg@1
boundary: cursor
item: cursor
promise: `cursor` is the byte offset `send` prints and `wait --after` consumes.
@row_add n=220 row=decision/human-output-is-contract area=surface-cli rec=fct-6hpmxl26@1
name: Human output is a contract
rule: The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice.
why: LLM drivers may read the human output.
@row_add n=221 row=decision/shared-captions-fixed area=surface-cli rec=fct-mpm2axrl@2
name: Shared captions fixed
rule: The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
@row_add n=222 row=contract/readback-vocabulary/words area=surface-cli rec=fct-mpm2axrl@2
boundary: readback vocabulary
item: state words
promise: The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
@row_add n=223 row=decision/cli-field-separators area=surface-cli rec=fct-mpm2axrl@2
name: CLI field separators
rule: ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form.
@row_add n=224 row=decision/amber-means-dialog area=surface-cli rec=fct-mpm2axrl@2
name: Amber means DIALOG only
rule: Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
@row_add n=225 row=decision/cli-holds-every-action area=surface-cli rec=fct-mpm2axrl@2
name: The CLI holds every action
rule: The CLI is where every action lives.
@row_none n=10 rec=fct-6pvzlllg@1 type=S
@row_none n=11 rec=fct-bamlmik6@1 type=S
@row_add n=226 row=decision/targets-are-wrapped-names area=surface-cli rec=fct-thhsjl2y@1
name: A CLI target is always a ViolaName
rule: A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
