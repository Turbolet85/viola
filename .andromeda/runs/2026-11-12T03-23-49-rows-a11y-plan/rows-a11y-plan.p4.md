@row_none n=1 rec=1 type=0
what: a table row marking ARIA landmarks as not applicable to the vendor claude TUI
@row_add n=1 row=contract/cli-exit-codes/10 area=aria-patterns-roles rec=2,5,55
boundary: CLI exit codes
item: 10
promise: human-typing: the human holds the wheel, so the automated send is refused with nothing typed.
@row_add n=2 row=decision/linear-static-cli-output area=aria-patterns-roles rec=3
name: Linear static CLI output
rule: CLI output is linear and static, with no spinner, carriage-return redraw, cursor movement or erase sequence; the waiting line appears once on stderr on a terminal and not at all under non-TTY.
@row_add n=3 row=contract/viola-list-board/caption area=aria-patterns-roles rec=3,38
boundary: viola list board
item: caption order
promise: The caption order is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, reused by the terminal fleet console.
@row_add n=4 row=decision/no-sgr-off-terminal area=aria-patterns-roles rec=3,39
name: No SGR off a terminal
rule: viola emits no SGR of its own under non-TTY, NO_COLOR, TERM=dumb, --json, a passthrough role, viola run or a passed viola revive.
guard: cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json
@row_add n=5 row=decision/sgr-beside-its-word area=aria-patterns-roles rec=3,39
name: SGR only beside its word
rule: Every SGR span sits beside its word: amber only on DIALOG, dim only on stale rows, bold only on NAME column values; unable and fail are never coloured.
guard: cross_list_rows_style_each_span_beside_its_word_on_a_terminal
@row_add n=6 row=decision/control-bytes-escaped area=aria-patterns-roles rec=3
name: Control bytes printed escaped
rule: Injected C0 and C1 control bytes appear only in escaped form; in wait and last text newline and tab stay raw, while in list rows they are escaped too so every row stays one fixed-width line.
why: security control for C0/C1 escaping, ratified for list rows
guard: escape_row_shows_every_control_character_as_hex_text
@row_add n=7 row=decision/no-terminal-conformance-claim area=aria-patterns-roles rec=3,5,39
name: No conformance claim for the terminal
rule: The CLI and TUI checks stand in for WCAG success criteria with no conformance claim for the terminal.
@row_none n=2 rec=4 type=5
@row_add n=8 row=decision/time-bounds-fall-on-the-driver area=cognitive-accessibility rec=5
name: Time bounds fall on the driver only
rule: Every time bound (MCP wait cap, dialog hold, readiness gate, confirm windows, budget pause, statusline deadline, agents read) falls on the driver or viola's own child, never blocking, refusing or delaying a human key and never moving the wheel.
@row_add n=9 row=contract/viola-wait/timed-out area=cognitive-accessibility rec=5
boundary: viola wait
item: timed_out
promise: viola wait returns timed_out true only for a timeout the caller passes in.
@row_add n=10 row=contract/mcp-wait-tool/timeout-ms area=cognitive-accessibility rec=5
boundary: MCP wait tool
item: timeout_ms
promise: Clamped to 50 000 ms, which is also the default when none is passed, so a wait with no timeout is answered timed-out after 50 s.
@row_add n=11 row=contract/config-json/dialog-hold-s area=cognitive-accessibility rec=5
boundary: config.json
item: dialog_hold_s
promise: Whole seconds 60 to 3600, default 60, read once at the wrapper's start; any other value gives 60 and one parse-rejected.
@row_add n=12 row=decision/dialog-hold-bounds-the-driver area=cognitive-accessibility rec=5
name: Dialog hold bounds the driver
rule: A raised dialog takes viola answer only within the dialog hold; at its end the dialog renders for the human in the claude TUI, and any human key hands a held dialog back at once whatever the hold.
state: provisional
@row_add n=13 row=contract/event-line/dialog-expired area=cognitive-accessibility rec=5
boundary: the event line
item: dialog-expired
promise: Appended once at the dialog hold's end with source wrapper and dialog_id, dialog_kind; wakes a parked wait.
@row_add n=14 row=contract/cli-exit-codes/13 area=cognitive-accessibility rec=5,2,55
boundary: CLI exit codes
item: 13
promise: An answer for an expired or absent dialog is refused not-delivered / unknown-dialog.
@row_add n=15 row=contract/send-refusal/input-not-ready area=cognitive-accessibility rec=5
boundary: viola send refusal
item: input-not-ready
promise: A send on a verified CLI whose input box is not ready within the readiness gate's wait is refused not-delivered / input-not-ready with nothing typed.
@row_add n=16 row=contract/send-refusal/no-prompt-submitted area=cognitive-accessibility rec=5
boundary: viola send refusal
item: no-prompt-submitted
promise: A typed send not confirmed inside its confirm window (10 s; 40 s for /compact on a verified CLI) is not-delivered / no-prompt-submitted.
@row_add n=17 row=contract/viola-hook-statusline/exit area=cognitive-accessibility rec=5
boundary: viola hook statusline
item: exit status
promise: Runs the user's own status line command under a 5 s deadline and exits 0 with empty stderr in every case.
@row_add n=18 row=decision/claude-agents-read-fails-open area=cognitive-accessibility rec=5
name: claude agents read fails open
rule: A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's preflight.
@row_add n=19 row=decision/budget-gate area=cognitive-accessibility rec=5
name: Budget gate on automated sends
rule: While a five-hour or seven-day window stands at or over its threshold with a known reset ahead, an automated viola send is refused budget-paused with nothing typed, until that reset or viola release --budget; no threshold value switches the gate off.
guard: path6_a_reading_over_the_threshold_refuses_the_send
@row_add n=20 row=decision/budget-gate-only-on-send area=cognitive-accessibility rec=5
name: Only send reads the budget gate
rule: answer, wait, last and pause do not read the budget gate and give under a closed gate what they give under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=21 row=contract/cli-exit-codes/11 area=cognitive-accessibility rec=5
boundary: CLI exit codes
item: 11
promise: budget-paused, with detail five-hour or seven-day.
@row_add n=22 row=contract/config-json/five-hour area=cognitive-accessibility rec=5
boundary: config.json
item: five_hour
promise: The five-hour window's budget threshold, default 90.
@row_add n=23 row=contract/config-json/seven-day area=cognitive-accessibility rec=5
boundary: config.json
item: seven_day
promise: The seven-day window's budget threshold, default 85.
@row_add n=24 row=decision/error-word-detail-hint area=cognitive-accessibility rec=6
name: Errors as word, detail and hint
rule: Every error is a printed word followed by plain detail, and a recovery instruction where recovery exists.
@row_add n=25 row=contract/cli-refusal/stderr-layout area=cognitive-accessibility rec=6
boundary: CLI refusal
item: stderr layout
promise: unable  <reason>  <detail> on stderr, then hint: as the last stderr line, with a typed exit code.
@row_add n=26 row=decision/unknown-refusal-has-no-hint area=cognitive-accessibility rec=6
name: unknown refusal carries no hint
rule: The unknown refusal prints no hint.
why: its detail is a code word or nothing, opaque to a reader, so no hint is printed rather than a guess
@row_add n=27 row=contract/cli-exit-codes/14 area=cognitive-accessibility rec=6,22
boundary: CLI exit codes
item: 14
promise: unknown: a refusal whose word is outside the closed refusal list.
@row_add n=28 row=decision/faults-carry-no-hint area=cognitive-accessibility rec=6
name: Faults carry no hint
rule: Faults (error: wrapper fault, error: internal error) are not refusals and carry no hint.
@row_add n=29 row=decision/json-one-document-no-hint area=cognitive-accessibility rec=6,25
name: One --json document with no hint
rule: Under --json a failure gives one document with no hint.
except: the viola answer and viola send exit-2 paths write a fixed line to stderr whatever --json says, and no document
@row_add n=30 row=contract/cli-fixed-message/form area=cognitive-accessibility rec=6
boundary: CLI fixed-message refusal
item: form
promise: viola revive's exit-1 refusals and viola run's socket-directory refusal print one unable: <text> line, then one hint: line last.
@row_add n=31 row=contract/cli-exit-codes/2 area=cognitive-accessibility rec=6,25
boundary: CLI exit codes
item: 2
promise: A usage error, or an unusable answer response or send text written as one fixed stderr line with no document.
@row_add n=32 row=contract/start-json-refusal/document area=cognitive-accessibility rec=6
boundary: run / revive start under --json
item: refusal document
promise: Every refusal of the start revive shares with run gives one {"v":1,"error":"<code>","detail":null} document with no hint and empty stderr.
@row_add n=33 row=contract/viola-verify/interrupted-pair area=cognitive-accessibility rec=6
boundary: viola verify
item: interrupted pair
promise: unable: verify was stopped before it finished, then hint: run viola verify again to stamp this CLI.
@row_add n=34 row=contract/cli-exit-codes/21 area=cognitive-accessibility rec=6,7
boundary: CLI exit codes
item: 21
promise: instance-unreachable: the frame verb was refused before reaching the instance, the cause in detail.
@row_add n=35 row=contract/instance-unreachable-json/document area=cognitive-accessibility rec=7
boundary: exit 21 under --json
item: document
promise: One {"v":1,"error":"instance-unreachable","detail":"<cause>"} document on stdout per cause, stderr empty, no hint and no ESC.
@row_add n=36 row=contract/instance-unreachable-detail/strict-modes-failed area=cognitive-accessibility rec=7,8,9
boundary: instance-unreachable detail
item: strict-modes-failed
promise: The instance failed the strict-modes check; refused before any frame or log, no log line written.
@row_add n=37 row=contract/instance-unreachable-detail/instance-dead area=cognitive-accessibility rec=7,11,17,18
boundary: instance-unreachable detail
item: instance-dead
promise: The name has no live endpoint but its instance directory stands, or the instance is stale.
@row_add n=38 row=contract/instance-unreachable-detail/unwrapped-name area=cognitive-accessibility rec=7,17,18
boundary: instance-unreachable detail
item: unwrapped-name
promise: The name has no live endpoint and no instance directory (absent, a regular file or a symlink).
@row_add n=39 row=contract/instance-unreachable-detail/endpoint-lost area=cognitive-accessibility rec=7,11
boundary: instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open.
@row_add n=40 row=contract/instance-unreachable-detail/server-verify-failed area=cognitive-accessibility rec=7,6
boundary: instance-unreachable detail
item: server-verify-failed
promise: The process serving the endpoint is not the recorded wrapper and was not contacted.
@row_add n=41 row=contract/instance-unreachable-detail/socket-dir-refused area=cognitive-accessibility rec=7,10
boundary: instance-unreachable detail
item: socket-dir-refused
promise: The socket directory was refused; the verb sent no frame.
@row_add n=42 row=decision/strict-modes-refusal-before-any-write area=cognitive-accessibility rec=8,9,12,15,18
name: Strict-modes refusal before any write
rule: A home or instance failing the strict-modes check is refused first, before any log opens, frame is sent or child starts, with no line written: exit 21 for frame verbs and link/unlink, exit 1 for run, revive's start and verify.
@row_add n=43 row=decision/socket-dir-refusal-before-any-frame area=cognitive-accessibility rec=10,13
name: Socket-directory refusal before any frame
rule: A refused socket directory stops the verb before any frame, snapshot, child or bind and leaves the directory as found: exit 21 for the frame verbs, exit 1 for the run and revive start.
@row_add n=44 row=contract/revive-json-refusal/socket-dir-refused area=cognitive-accessibility rec=13
boundary: viola revive --json
item: socket-dir-refused
promise: One document with code socket-dir-refused, holding no path, pid or uid, with stderr empty and no hint.
@row_add n=45 row=contract/start-holder-refusal/stale-heartbeat area=cognitive-accessibility rec=14,11
boundary: start against a holder, exit 1
item: stale-heartbeat
promise: The holder is stale; stale heartbeat is exit 1's cause alone.
@row_add n=46 row=contract/start-holder-refusal/already-live area=cognitive-accessibility rec=14
boundary: start against a holder, exit 1
item: already-live
promise: The holder is live, or a fresh replayed beat was read.
@row_add n=47 row=decision/unstattable-home-is-strict-modes-refusal area=cognitive-accessibility rec=16
name: Unstattable home is a strict-modes refusal
rule: A home path under a regular file is a strict-modes refusal with its hint: verify gives the home-state pair with exit 1, the frame verbs the instance-state pair with exit 21.
@row_add n=48 row=decision/printed-names-are-parsed-arguments area=cognitive-accessibility rec=19,37
name: Printed names are the parsed arguments
rule: Names printed in hints and in link result lines are the verb's own parsed ViolaName arguments, never names read back from a reply or the log.
@row_add n=49 row=decision/link-pair-validated-by-wrapper area=cognitive-accessibility rec=20
name: Link pairs validated by the wrapper
rule: viola link and viola unlink never refuse a pair client-side; a pair the wrapper refuses as invalid params reaches the verb as a fault, exit 20, no hint.
@row_add n=50 row=contract/cli-exit-codes/20 area=cognitive-accessibility rec=20
boundary: CLI exit codes
item: 20
promise: A fault, with no hint.
@row_add n=51 row=contract/code-word/format area=cognitive-accessibility rec=21
boundary: code word
item: format
promise: 1 to 64 bytes of [a-z0-9-], a lower-case letter first.
@row_add n=52 row=decision/peer-replies-screened-to-code-words area=cognitive-accessibility rec=21
name: Peer replies screened to code words
rule: A peer's refusal that is no code word reads unknown, and a string detail that is no code word reads nothing (null under --json and on MCP).
@row_add n=53 row=decision/unlisted-code-words-stay-visible area=cognitive-accessibility rec=22
name: Unlisted code words stay visible
rule: A code word outside the closed refusal list is shown in the document and in refused: <word>, and takes unknown's exit 14.
why: visible by design
@row_add n=54 row=contract/fault-body/message area=cognitive-accessibility rec=23
boundary: fault body under --json and on MCP
item: message
promise: A code outside the five known reads the fixed message wrapper fault with null data.
@row_add n=55 row=contract/human-fault-line/wrapper-fault area=cognitive-accessibility rec=23
boundary: human fault line
item: wrapper fault
promise: error: wrapper fault  <code>.
@row_add n=56 row=contract/json-fault/internal-error area=cognitive-accessibility rec=24
boundary: CLI fault under --json
item: internal-error
promise: Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's start, mcp and hook stay silent there.
@row_add n=57 row=decision/client-euid-refusal-test-strategy area=cognitive-accessibility rec=26
name: Client euid refusal test strategy
rule: No foreign peer is driven: the client euid refusal's decision is tested over injected euids and its real read on Linux, and each client seam's wiring is read only by forced refusals in remove-the-guard runs.
@row_none n=3 rec=27 type=5
@row_none n=4 rec=28 type=5
@row_none n=5 rec=29 type=5
@row_add n=58 row=decision/json-failure-documents-are-clean area=cognitive-accessibility rec=30
name: --json failure documents are clean
rule: Each --json failure is one document with a literal exit, empty stderr, no ESC byte and a body holding no path or upstream text.
guard: cli_error_documents_hold_no_path_or_upstream_text
@row_add n=59 row=decision/error-bodies-have-writer-and-witness area=cognitive-accessibility rec=31
name: Error bodies have a writer and a witness
rule: Each error body names a writer and a witness that stand, and each product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_add n=60 row=decision/thiserror-messages-interpolate-nothing area=cognitive-accessibility rec=31
name: thiserror messages interpolate nothing
rule: No thiserror message interpolates a value.
guard: thiserror_messages_interpolate_nothing
@row_add n=61 row=decision/one-hint-per-code area=cognitive-accessibility rec=32
name: One hint per code
rule: No two codes share a hint.
guard: no_two_codes_share_a_hint
@row_add n=62 row=decision/no-hint-names-release area=cognitive-accessibility rec=32,55
name: No hint names release
rule: No hint names the release verb.
guard: no_hint_names_release
@row_none n=6 rec=33 type=5
@row_none n=7 rec=34 type=5
@row_add n=63 row=decision/fixed-cli-strings-pinned-by-literal area=cognitive-accessibility rec=35
name: Fixed CLI strings pinned by literal
rule: Every fixed CLI string is pinned by a literal in a test, so any copy change fails CI until the pinned string is updated on purpose.
@row_add n=64 row=decision/link-result-lines-one-write-no-sgr area=cognitive-accessibility rec=36
name: Link result lines in one write, no SGR
rule: The link and unlink result lines are each one write with no SGR in any mode.
guard: link_verb_lines_are_the_design_lines_in_one_write
@row_add n=65 row=contract/viola-link-line/time area=cognitive-accessibility rec=37
boundary: viola link line
item: time
promise: The reply's since as UTC clock time in whole seconds with Z.
@row_add n=66 row=decision/state-carried-by-printed-word area=visual-design-verification rec=39
name: State carried by its printed word
rule: Each board state is carried by its printed word (stale, DIALOG <kind>, expired, budget-paused, unverified-cli, idle, busy, n/a), never by colour alone.
@row_add n=67 row=decision/colour-decision-inputs area=visual-design-verification rec=39
name: Colour decision inputs
rule: The colour decision reads NO_COLOR (present, empty included), TERM (exactly dumb; 256color), COLORTERM and whether stdout is a terminal, and picks a truecolor, 256 or 16 depth.
@row_add n=68 row=decision/unwrapped-name-truncation area=visual-design-verification rec=39
name: Unwrapped NAME truncation
rule: On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus ..., never inside an escape; a wrapped NAME is never cut, and piped output and --json keep every name whole.
guard: cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it
@row_add n=69 row=contract/viola-list-stale-row/escape-sequences area=visual-design-verification rec=40
boundary: viola list stale row on a terminal
item: escape sequences
promise: Exactly DIM, BOLD, RESET, DIM, RESET, with the word stale printed beside the dim.
@row_add n=70 row=decision/cli-a11y-by-literal-tests area=a11y-testing-tool-pick rec=41
name: CLI a11y checked by literal tests
rule: With no a11y tool for terminal output, output discipline is checked by tests that drive the viola binary as a process and pin each fixed string by a literal.
@row_add n=71 row=decision/tui-boundary-driver-pin area=a11y-testing-tool-pick rec=41
name: TUI boundary driver
rule: The tui boundary is driven by a portable-pty outer PTY on the workspace's exact =0.9.0 pin, which a11y inherits as-is and never re-pins.
@row_none n=8 rec=42 type=S
@row_add n=72 row=absence/rendered-page area=bootstrap-phases-derive-for-route-setup-project rec=43,45,46,53
capability: A rendered page, and with it any browser a11y tooling
@row_none n=9 rec=44 type=7
@row_add n=73 row=decision/manual-screen-reader-pass area=bootstrap-phases-derive-for-route-setup-project rec=47
name: Manual screen reader pass
rule: The screen reader check is a manual Orca pass owned by the founder, with no automated test spec.
@row_add n=74 row=decision/a11y-runs-in-existing-test-suites area=bootstrap-phases-derive-for-route-setup-project rec=48,52
name: a11y runs in the existing test suites
rule: The CLI and TUI a11y cases are ordinary nextest cases in the test job's coverage suite, with no a11y gate, lint, aggregation, upload step, separate driver or suite value of their own.
@row_add n=75 row=decision/a11y-failures-are-nextest-failures area=bootstrap-phases-derive-for-route-setup-project rec=49,54
name: a11y failures are nextest failures
rule: An a11y failure surfaces only as a nextest failure in the harness run JSON, with no a11y violation rows and no a11y artifact of its own.
@row_none n=10 rec=50 type=0
what: a note on how the route and setup-project skills use the bootstrap phase list
@row_none n=11 rec=51 type=S
@row_add n=76 row=decision/gate-requires-coverage-doctest area=ci-integration rec=53
name: Gate requires coverage and doctest
rule: The test job runs the harness and then gate --require coverage,doctest, with skips never allowed.
@row_add n=77 row=decision/stdin-classifier-non-editing-list area=keyboard-test-harness rec=55
name: Closed non-editing stdin list
rule: Only focus reports, mouse reports and terminal replies matched by exact grammar leave the wheel alone; every other byte, including a sequence one field or prefix away from a listed shape, takes the wheel.
why: ratified, the exact grammars chosen after a narrower option was shown
@row_add n=78 row=decision/human-key-takes-the-wheel area=keyboard-test-harness rec=55
name: Human key takes the wheel
rule: A human key during a driver send is delivered unblocked; a key written while a verified send waits in the readiness gate takes the wheel and the send is refused human-typing with nothing typed.
guard: send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
@row_add n=79 row=decision/zero-viola-bytes-on-the-terminal area=keyboard-test-harness rec=55
name: Zero viola bytes on the terminal
rule: The outer-PTY stream holds no viola-originated bytes (no message literals, hint:, SGR or cursor control the child did not emit), compared byte for byte with an unwrapped run.
@row_add n=80 row=decision/statusline-output-passed-byte-for-byte area=keyboard-test-harness rec=55
name: Statusline output passed byte for byte
rule: viola hook statusline hands the user's own status line output to the child byte for byte and writes nothing to the terminal itself.
guard: passthrough_with_a_statusline_source_adds_no_viola_bytes
@row_add n=81 row=decision/live-readings-never-gate area=keyboard-test-harness rec=55
name: Live readings never gate
rule: Live readings of the takeover clause on a real terminal are supplemental and never gating.
@row_add n=82 row=decision/boundary-tests-sequenced-on-records area=keyboard-test-harness rec=55
name: Boundary tests sequenced on records
rule: The keyboard boundary cases are sequenced on log records, never on a timer, and each boundary clause is its own nextest case.
@row_add n=83 row=decision/boundary-check-never-inspects-claude-content area=keyboard-test-harness rec=55
name: Boundary check never inspects claude content
rule: The keyboard boundary check never inspects claude content.
