@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=5
@row_none n=3 rec=3 type=7
@row_add n=1 row=decision/diagnostics-strict-modes-fail-open area=error-handling rec=4
name: Diagnostics files failing strict modes fail open
rule: A diagnostics folder, role file or detail file that is a symbolic link or has a mode another user may use is a failed open, checked before anything is narrowed: the role writer writes nowhere and the detail line is dropped.
@row_add n=2 row=decision/paste-text-control-character-set area=input-validation rec=5
name: Paste text control-character set
rule: Sent text allows LF, CR and TAB and refuses every other C0 control (ESC above all), DEL and every C1 control, checked over decoded characters, never raw bytes.
why: Multi-line text is normal for relays, plans and answers; refusing the rest stops a bracketed-paste breakout through an embedded end-of-paste sequence.
@row_add n=3 row=decision/reject-dont-strip-paste-text area=input-validation rec=5
name: Reject, don't strip
rule: A sent text that holds a refused character is refused whole; no refused character is ever removed and the rest typed.
@row_add n=4 row=decision/typed-text-line-ending-normalisation area=input-validation rec=5
name: Typed text line-ending normalisation
rule: After validation a sent text is typed with every CR LF pair and every other CR as one LF and every trailing CR and LF removed; that typed text alone is used for local-command classification, delivery confirmation, byte count and the paste.
why: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged and the delivery-confirmation match stays an exact equality.
@row_add n=5 row=decision/empty-typed-text-refused area=input-validation rec=5
name: Empty typed text refused
rule: A text whose typed text is empty is refused not-delivered empty-text, by the client before any frame and by the wrapper before anything is reserved, issued or typed.
why: Ratified.
@row_add n=6 row=decision/paste-validation-on-both-sides area=input-validation rec=5
name: Paste validation on both sides
rule: Paste-text validation runs client-side (CLI send and the MCP send tool) before any frame and again in the wrapper before anything is typed, the wrapper's check authoritative and first in its refusal order.
@row_add n=7 row=contract/send-refusal/control-character area=input-validation rec=5
boundary: send refusal
item: control-character
promise: A sent text holding a refused character is refused not-delivered with detail control-character (CLI exit 13), checked first, before the Conventions refusal order, with nothing typed.
@row_add n=8 row=contract/send-refusal/empty-text area=input-validation rec=5
boundary: send refusal
item: empty-text
promise: A sent text whose typed text is empty is refused not-delivered with detail empty-text (CLI exit 13), checked after control-character and before the Conventions refusal order.
@row_add n=9 row=decision/dialog-free-text-control-character-set area=input-validation rec=6
name: Dialog free text control-character set
rule: Dialog free text (an answer's message and question answer values) follows the paste-text set, refused control-character first, in viola answer before any frame (exit 13) and again, authoritatively, in the wrapper's answer handler.
@row_add n=10 row=contract/channel-frames/oversize-frame area=input-validation rec=7
boundary: channel frames
item: over-limit frame
promise: A frame is read under a MAX_FRAME cap; an over-limit frame gets -32600 and the connection is closed.
@row_add n=11 row=decision/channel-json-default-depth-limit area=input-validation rec=7
name: Channel JSON default depth limit
rule: Channel frames are parsed with serde_json's default recursion limit of 128, never unbounded depth.
@row_add n=12 row=contract/channel-frames/version area=input-validation rec=7
boundary: channel frames
item: v
promise: A frame whose v fails the version check gets -32602.
@row_add n=13 row=decision/channel-closed-types area=input-validation rec=7
name: Channel closed types
rule: Channel fields are closed types: behavior is a closed enum (allow/deny, approve/revise), dialog_id an unsigned 64-bit integer, and link and unlink names pass the viola name check.
@row_add n=14 row=contract/channel-frames/send-params area=input-validation rec=7
boundary: channel frames
item: send params
promise: text is a string and from is absent, null or a valid viola name; anything else is -32602 invalid params with data null, before paste validation runs; the envelope types no method param, so a mistyped from never gets -32600.
@row_add n=15 row=contract/channel-frames/wait-last-params area=input-validation rec=7
boundary: channel frames
item: wait and last params
promise: after and timeout_ms are each absent or a non-negative integer that fits u64, else -32602 invalid params with data null; from follows the send rule; unknown fields are ignored.
@row_add n=16 row=contract/channel-frames/hook-dialog-params area=input-validation rec=7
boundary: channel frames
item: hook.dialog params
promise: kind is a closed dialog kind, data an object, hook_event closed, and tool, input and continuation closed or absent; an invalid frame is not registered.
@row_add n=17 row=contract/channel-frames/answer-params area=input-validation rec=7
boundary: channel frames
item: answer params
promise: dialog_id is u64, from follows the send rule, response maps into the closed per-kind enums, then paste validation runs on its free text.
@row_add n=18 row=contract/channel-frames/pause-params area=input-validation rec=7
boundary: channel frames
item: pause params
promise: pause's from follows the send rule.
@row_add n=19 row=contract/channel-frames/release-params area=input-validation rec=7
boundary: channel frames
item: release params
promise: budget is absent or a bool, else -32602 with data null; from is absent or null, a string from is -32602 with data reason release-from-driver, any other type -32602 with data null.
@row_add n=20 row=contract/channel-frames/link-unlink-params area=input-validation rec=7
boundary: channel frames
item: link and unlink params
promise: driver and driven are valid viola name strings; a missing, non-string or invalid name, a driven other than the serving wrapper's name, or a driver equal to driven is -32602 with data null and no line; from is never read.
@row_add n=21 row=contract/channel-frames/hook-event-revalidation area=input-validation rec=7
boundary: channel frames
item: hook.event
promise: kind is one of the five hook kinds, data an object, and for prompt-submitted text is a string and origin is harness or human; an invalid event is not appended.
@row_add n=22 row=decision/client-reply-code-words-only area=input-validation rec=7
name: Client reads reply refusals as code words only
rule: A client reading a reply (CLI or MCP) takes a refusal or string detail only as a code word, and a fault only through the channel's sanitised-fault reader.
@row_add n=23 row=contract/channel-response-log/detail area=input-validation rec=7
boundary: channel-response log line
item: detail
promise: The three -32602 faults log detail unsupported-version, invalid-params or release-from-driver at WARN on server and client lines, with no param name or value; every other channel-response line has no detail.
@row_add n=24 row=contract/hook-stdin/oversize area=input-validation rec=8
boundary: hook stdin
item: oversize payload
promise: A payload over MAX_FRAME is detected, never silently truncated, and fails open: exit 0, no body, hook-decision detail oversize-stdin and parse-rejected parser hook-stdin detail oversize in diagnostics.
@row_add n=25 row=decision/hook-unknown-event-silent-exit area=input-validation rec=8
name: Hook unknown event exits silently
rule: The hook event argument maps into a closed set of nine events (statusline taken first, apart); an unknown event, a usage error or an absent or invalid VIOLA_NAME exits 0 with nothing written anywhere.
@row_add n=26 row=decision/hook-instance-check-first area=input-validation rec=8,11
name: Hook instance check before anything
rule: Hook event, dialog and statusline paths canonicalise the instance directory and run the instance strict-modes check before diagnostics level and obs init; a missing, malformed or refused instance exits 0 with no config, stdin, line, frame or append.
@row_add n=27 row=contract/hook-stdin/not-one-object area=input-validation rec=8
boundary: hook stdin
item: malformed-json
promise: A payload that is not one JSON object fails open with malformed-json.
@row_add n=28 row=decision/hook-security-fields-closed-enums area=input-validation rec=8
name: Hook security fields in closed enums
rule: Hook parsing is tolerant, but the fields that decide security (dialog kind and decision body) map into closed enums; a malformed dialog payload fails open with exit 0 and no body, and stdout takes one write.
@row_add n=29 row=decision/hook-dialog-reply-read-bound area=input-validation rec=8
name: Hook dialog reply read bound
rule: The hook's read of a dialog reply is bounded at the maximum dialog hold plus 5 s (3605 s), the wrapper replying null at its own configured hold's end.
@row_add n=30 row=decision/question-option-text-crosses-boundary area=input-validation rec=8
name: Question option text crosses into events
rule: A question option's description and preview are taken only as strings, else null, never refusing the dialog, and cross into the events log, hook.dialog, CLI wait --json and the MCP wait.
why: A ratified boundary widening.
@row_add n=31 row=decision/stop-background-tasks-count-only area=input-validation rec=8
name: Stop background tasks carried as a count only
rule: Stop's background_tasks is carried only as its length, no element read; null when absent, null or not an array, and a non-array also writes a drift entry.
why: Ratified with the option-text widening.
@row_add n=32 row=decision/statusline-payload-fails-silent area=input-validation rec=8
name: Statusline payload fails silent
rule: The statusline hook payload takes the hook stdin cap and one-object rule, each failure ending in exit 0, empty stderr, nothing printed and no budget file.
@row_add n=33 row=decision/budget-json-single-writer area=input-validation rec=8
name: budget.json has a single writer
rule: budget.json is written only by the statusline hook arm, and only when the payload holds a rate_limits object.
@row_add n=34 row=contract/budget-json/window area=input-validation rec=8
boundary: budget.json
item: window
promise: A window that is absent or not an object is the word "unknown".
@row_add n=35 row=contract/budget-json/used-percentage area=input-validation rec=8
boundary: budget.json
item: used_percentage
promise: Kept only from 0 to 100, else "unknown".
@row_add n=36 row=contract/budget-json/resets-at area=input-validation rec=8
boundary: budget.json
item: resets_at
promise: Read as epoch seconds or RFC 3339 and written as RFC 3339 UTC with milliseconds; a malformed value becomes "unknown".
@row_add n=37 row=decision/hook-capture-arm area=input-validation rec=8
name: Hook capture arm
rule: The hidden capture arm, called only by verify's probe plugins, writes stdin raw to an exclusively claimed new 0600 file in an absolute existing directory, reads no VIOLA_* variable, opens no channel, writes nothing on failure, keeps stderr empty and exits 0.
why: A ratified boundary widening.
@row_add n=38 row=decision/capture-answers-closed-probe-ids area=input-validation rec=8
name: Capture answers from closed probe ids
rule: --answers is read only with --capture; each answer file is read through a 64-byte cap and matched exactly against the closed probe answer ids, the body built only through the dialog decision-body builder.
why: Ratified: the hook answers, never a key.
@row_add n=39 row=decision/statusline-capture-arm area=input-validation rec=8
name: Statusline capture arm
rule: The hidden statusline capture arm writes the payload raw to a claimed 0600 file and a shell verdict of exactly bin-sh when the --shell word equals /bin/sh byte for byte, else other, the word itself written nowhere; it prints nothing, exits 0 and opens no channel.
why: Ratified.
@row_add n=40 row=decision/mcp-handlers-validate-raw-arguments area=input-validation rec=9
name: MCP handlers validate raw arguments
rule: MCP tool schemas are advisory only; each handler validates the raw argument object itself into closed types and paste validation, never through the SDK's typed deserialisation.
why: rmcp does not enforce the schemas server-side, and a serde failure would carry serde text.
@row_add n=41 row=contract/mcp-tool-inputs/invalid-params area=input-validation rec=9
boundary: MCP tool inputs
item: invalid-params
promise: A failed argument is isError true with structuredContent error invalid-params and detail one of target, text, after, timeout_ms, dialog_id, response, text refused: invalid-params, and no frame.
@row_add n=42 row=contract/mcp-tool-inputs/unknown-tool area=input-validation rec=9
boundary: MCP tool inputs
item: unknown tool name
promise: An unknown tool name is JSON-RPC -32602.
@row_add n=43 row=contract/mcp-tool-inputs/timeout-ms area=input-validation rec=9
boundary: MCP tool inputs
item: timeout_ms
promise: wait's timeout_ms is absent, null or a u64, clamped to 50 000 ms, which is also its default.
@row_add n=44 row=decision/mcp-stdin-line-cap-on-reader area=input-validation rec=10
name: MCP stdin line cap on the reader
rule: Each MCP stdin line is capped at MAX_FRAME, newline counted, by the reader handed to the SDK; a longer line is refused whole, never truncated or skipped, and ends the session as a stdin close does.
why: rmcp's stdio transport reads each line unbounded and its codec length bound covers only the writer.
@row_add n=45 row=contract/mcp-stdin/oversize-line area=input-validation rec=10
boundary: viola mcp stdin
item: oversize line
promise: After the session ends one WARN parse-rejected parser mcp-stdin detail oversize count 1, no byte of the line; exit 0 for an initialized session, exit 1 with both streams empty when cut before initialize completes.
@row_add n=46 row=decision/cli-names-through-name-check area=input-validation rec=11
name: CLI names through the viola name check
rule: Every viola name argument on the CLI goes through the viola name check, one parser shared by the verbs.
@row_add n=47 row=decision/home-strict-modes-before-read area=input-validation rec=11
name: Home strict-modes check before any read
rule: --home and VIOLA_DIR are canonicalised and pass the home strict-modes check before any snapshot, statusline command or ledger read.
@row_add n=48 row=decision/verify-home-tree-check area=input-validation rec=11
name: verify checks the home tree first
rule: viola verify checks the home, ledger, bin and plugin directories before its cli log (a refusal is exit 1, no step line), and its stamp update refuses before it creates ledger or reads the stamps.
@row_add n=49 row=contract/cli-send-text/input area=input-validation rec=11
boundary: viola send text
item: text input
promise: The text comes from stdin or --file only, capped at MAX_FRAME; over the cap or not UTF-8 is exit 2 with one fixed line; a second positional argument is a usage error.
@row_add n=50 row=decision/frame-verbs-instance-check-first area=input-validation rec=11
name: Frame verbs check the instance first
rule: CLI frame verbs and link/unlink run the instance strict-modes check first; a refusal is exit 21 with the instance pair, nothing opened under the home and no line.
@row_add n=51 row=contract/cli-json-error/strict-modes-failed area=input-validation rec=11
boundary: CLI frame verb --json error
item: strict-modes-failed
promise: A refused instance check under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21.
@row_add n=52 row=decision/frame-connect-check-order area=input-validation rec=11
name: Checks before a frame is sent
rule: After the instance check, a CLI frame verb or MCP frame tool runs the liveness pre-check, the endpoint directory check (socket-dir-refused), the connect, the peer euid decision (server-verify-failed) and the server check, all before any frame.
@row_add n=53 row=decision/mcp-frame-tools-refuse-quietly area=input-validation rec=11
name: MCP frame tool refusals
rule: An MCP frame tool refused by the instance, endpoint directory or peer check returns instance-unreachable, with no frame and no line.
@row_add n=54 row=decision/pause-release-from-viola-name area=input-validation rec=11
name: pause and release take from from VIOLA_NAME
rule: The CLI pause and release verbs take their from only from VIOLA_NAME.
@row_add n=55 row=contract/revive-arguments/id area=input-validation rec=11,21
boundary: viola revive arguments
item: --id
promise: --id accepts only the session-id shape, anything else a usage error with exit 2; --list conflicts with --id, --fork and the trailing arguments.
@row_add n=56 row=decision/revive-trailing-words-as-argv area=input-validation rec=11
name: revive trailing words as argv
rule: The words after -- in viola revive go to the child as argv elements of a direct spawn.
@row_add n=57 row=contract/revive-json/error-document area=input-validation rec=11
boundary: viola revive --json
item: error document
promise: A refusal prints exactly {"v":1,"error":"<code>","detail":null} on stdout, stderr empty, exit 1, the code one of the closed revive codes.
@row_add n=58 row=contract/revive-json/strict-modes-failed area=input-validation rec=11
boundary: viola revive --json codes
item: strict-modes-failed
promise: A closed exit-1 code of viola revive --json.
@row_add n=59 row=contract/revive-json/already-live area=input-validation rec=11
boundary: viola revive --json codes
item: already-live
promise: A closed exit-1 code of viola revive --json.
@row_add n=60 row=contract/revive-json/no-session area=input-validation rec=11,21
boundary: viola revive --json codes
item: no-session
promise: A closed exit-1 code: an --id the log does not hold, or a log with no session, exit 1 before any spawn.
@row_add n=61 row=contract/revive-json/session-live area=input-validation rec=11
boundary: viola revive --json codes
item: session-live
promise: A closed exit-1 code of viola revive --json.
@row_add n=62 row=contract/revive-json/cwd-missing area=input-validation rec=11,20
boundary: viola revive --json codes
item: cwd-missing
promise: A closed exit-1 code: the recorded cwd is absent or not an existing directory, no child started.
@row_add n=63 row=contract/revive-json/stale-heartbeat area=input-validation rec=11
boundary: viola revive --json codes
item: stale-heartbeat
promise: A closed exit-1 code of viola revive --json.
@row_add n=64 row=contract/revive-json/pinned-hash-mismatch area=input-validation rec=11
boundary: viola revive --json codes
item: pinned-hash-mismatch
promise: A closed exit-1 code of viola revive --json.
@row_add n=65 row=contract/revive-json/squatted-name area=input-validation rec=11
boundary: viola revive --json codes
item: squatted-name
promise: A closed exit-1 code of viola revive --json.
@row_add n=66 row=contract/revive-json/socket-dir-refused area=input-validation rec=11
boundary: viola revive --json codes
item: socket-dir-refused
promise: A closed exit-1 code of viola revive --json.
@row_add n=67 row=contract/revive-json/internal-error area=input-validation rec=11
boundary: viola revive --json codes
item: internal-error
promise: A closed exit-1 code, printed for an internal error of the start path, which is still logged with its chain; a child-pump error prints no document.
@row_add n=68 row=contract/revive-list-json/document area=input-validation rec=11
boundary: viola revive --list --json
item: sessions document
promise: Prints {"v":1,"ok":{"sessions":[{"ts","cause","id"}]}} in log order: codes, closed words and session-shaped ids only, never a path or pid.
@row_add n=69 row=decision/revive-strict-modes-before-use area=input-validation rec=11
name: revive checks strict modes before use
rule: Both viola revive arms check the instance's files before any snapshot or log value is used, and the start arm also checks the home tree before its wrapper log; a refusal is exit 1 with no log and no child.
@row_add n=70 row=decision/link-unlink-driven-instance area=input-validation rec=11
name: link and unlink act on the driven instance
rule: link and unlink act on the driven instance, joined only from its parsed name and checked before any snapshot read; a driver equal to driven is refused by the wrapper, not the client.
@row_add n=71 row=decision/cli-home-resolution-order area=input-validation rec=11
name: CLI home resolution order
rule: Every CLI verb and viola mcp take the home from --home, else the VIOLA_NAME/VIOLA_DIR instance's home when the pair checks, else the user's .viola, canonicalised when it exists; nothing is created.
why: A ratified widening of what chooses a CLI verb's home.
except: hook keeps its own shape check.
@row_add n=72 row=decision/list-scan-checks-before-open area=input-validation rec=11
name: viola list checks before opening
rule: viola list checks the home, instances and every instance, never following a link, before any file under the home is opened.
@row_add n=73 row=decision/config-tolerant-parse area=input-validation rec=12
name: Tolerant config parse
rule: config.json is parsed tolerantly, unknown keys counted, its v checked and its read capped at MAX_FRAME; a bad value falls back to its default and is reported as parse-rejected.
@row_add n=74 row=contract/config-json/five-hour area=input-validation rec=12
boundary: config.json
item: five_hour
promise: A JSON whole number 0 to 100, default 90; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_add n=75 row=contract/config-json/seven-day area=input-validation rec=12
boundary: config.json
item: seven_day
promise: A JSON whole number 0 to 100, default 85; any other value takes the default with one parse-rejected config-json malformed line; no value switches the budget gate off.
@row_add n=76 row=contract/config-json/dialog-hold-s area=input-validation rec=12
boundary: config.json
item: dialog_hold_s
promise: A JSON whole number of seconds from 60 to 3600, default 60; any other value takes 60 with one parse-rejected line; no value makes the hold zero or unbounded.
@row_add n=77 row=decision/dialog-hold-config-only area=input-validation rec=12
name: Dialog hold set only in config
rule: The dialog hold is read once at the wrapper's start from config.json only; no env var or flag sets it.
why: A ratified boundary widening.
@row_add n=78 row=contract/config-json/port area=input-validation rec=12
boundary: config.json
item: port
promise: Parsed into a u16.
@row_add n=79 row=contract/config-json/diagnostics-level area=input-validation rec=12
boundary: config.json
item: diagnostics_level
promise: The closed set info or debug; any other value falls back to info and is reported as parse-rejected.
@row_add n=80 row=contract/config-json/claude-env-keep area=input-validation rec=12
boundary: config.json
item: claude_env_keep
promise: An array of at most 32 names each matching ^CLAUDE[A-Z0-9_]*$; a bad value is rejected whole and reported as parse-rejected; an identity-floor name in it has no effect.
@row_add n=81 row=decision/fake-agent-pump-delay-seam area=input-validation rec=13
name: Pump delay test seam
rule: FAKE_AGENT_PUMP_DELAY_MS exists only in test-feature builds, a u64 of milliseconds capped at 5 000 that holds the run pump back after the child starts; it configures nothing, disables no control and widens no redaction.
why: Lets a test land a host resize in the spawn-to-pump window; ratified.
@row_add n=82 row=decision/fake-agent-hook-panic-seam area=input-validation rec=14
name: Hook panic test seam
rule: FAKE_AGENT_HOOK_PANIC exists only in test-feature builds and forces a fixed-payload hook panic only when exactly 1; the hook still fails open; it configures nothing, disables no control and widens no redaction.
why: Ratified.
@row_add n=83 row=decision/terminal-colour-facts area=input-validation rec=15
name: Terminal colour facts read for list only
rule: NO_COLOR, TERM and COLORTERM are read only by viola list's human mode, only compared, never echoed, logged or set, and decide only whether and how deep SGR is written around already escaped rows.
why: Ratified.
@row_add n=84 row=decision/no-width-or-colour-override-env area=input-validation rec=15
name: No width or colour override env
rule: COLUMNS, CLICOLOR, CLICOLOR_FORCE and CI are not read; the board's width comes from the host terminal size, no env read.
@row_add n=85 row=decision/events-reader-bounded-lines area=input-validation rec=16
name: Event log reader bounded lines
rule: The one event log reader takes each line through a MAX_FRAME cap, newline counted, and counts torn lines, unknown kinds and unknown fields per read without logging them; an unknown kind is not returned, and a reader never writes.
@row_add n=86 row=decision/link-restore-trusts-written-names area=input-validation rec=16
name: Link restore trusts names it wrote
rule: Link-set restore uses a link or unlink line's driver and driven only when both are strings and does not parse them as names again; it appends and logs nothing.
why: The wrapper wrote them, each having passed the viola name check.
@row_add n=87 row=decision/torn-last-line-healed-on-append area=input-validation rec=16
name: Torn last line healed on append
rule: The next append heals a torn last line by writing one LF ahead of its line in the same single write under the append lock, and logs one state-recovered line with a detail code, basename and offset, no content.
@row_add n=88 row=decision/state-reads-after-strict-modes area=input-validation rec=16
name: State files read after strict modes
rule: Own state files are read only after the home strict-modes check at every entry point, and the snapshot read and log replay write no file.
@row_add n=89 row=decision/replayed-snapshot-fallbacks area=input-validation rec=16
name: Replayed snapshot fallbacks
rule: Over a replayed snapshot no endpoint is given, hook connect fails open as channel-unreachable, statusline prints nothing, and the collision check judges liveness by the heartbeat alone.
@row_add n=90 row=decision/classified-snapshot-read-only area=input-validation rec=16
name: Classified snapshot read only
rule: No product code calls the unclassified snapshot read; product reads go through the classified read or the log replay.
@row_add n=91 row=decision/version-gate-strict-stamps-read area=input-validation rec=16
name: Version gate strict stamps read
rule: viola run's version gate reads the ledger stamps after the strict-modes check, unlocked and capped; a refusal, unreadable or malformed file is one parse-rejected ledger-stamps WARN and cli_verified false.
@row_add n=92 row=decision/statusline-command-trust area=input-validation rec=16
name: Statusline command trusted by strict modes
rule: The snapshot's statusline command is trusted only because of the strict-modes check; its sources are not on the trusted-file list.
why: Write access to it equals code execution.
@row_add n=93 row=decision/budget-json-checked-read area=input-validation rec=16
name: budget.json checked read
rule: budget.json is read only after path checks on the home and the file, under a shared lock, capped and tolerant, v 1 only; every failure is no reading and no line, and the read never writes or creates anything.
why: The gate refusing a driver on an untrusted file is ratified.
@row_add n=94 row=decision/tailed-paths-valid-instance-logs area=input-validation rec=17
name: Tailed paths only valid instance logs
rule: Only instances/<name>/events.ndjson under the canonical home is tailed, where the name passes the viola name check; symlinks and names that fail parsing are ignored.
@row_add n=95 row=decision/claude-agents-read-fail-open area=input-validation rec=18
name: claude agents read bounded, fail open
rule: The claude agents --json child is a direct spawn under the R8 strip, stdout capped and killed at 5 s, parsed tolerantly into rows or closed oversize or malformed; a failed read leaves list's board unchanged and passes revive.
@row_add n=96 row=decision/unwrapped-session-name-not-target area=input-validation rec=18,20
name: Unwrapped session name never a target
rule: An unwrapped row's name is never a viola name or a valid target in CLI list or the MCP list tool, and human-mode list prints it escaped.
@row_add n=97 row=decision/verify-pty-output-caught area=input-validation rec=18
name: verify PTY output bounded and caught
rule: verify's PTY output is capped at MAX_FRAME, fed into a screen under a panic catch, and read only against compiled signature literals.
@row_add n=98 row=decision/vt100-feed-under-panic-catch area=input-validation rec=19
name: Screen feed under a panic catch
rule: The screen model is fed off the passthrough under a panic catch; a caught panic poisons the model until the host size changes, confirmed send reads input-not-ready, and byte passthrough continues.
why: vt100 panics are reachable at real small sizes.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=99 row=decision/vt100-feed-bounded-queue area=input-validation rec=19
name: Screen feed bounded queue
rule: Each feed copy is offered without blocking to a 256-message queue after the human's write; a full queue drops the copy and poisons the model, never blocking the passthrough.
@row_add n=100 row=decision/claude-agents-resolved-from-path area=input-validation rec=20
name: claude agents resolved from the caller's PATH
rule: list, revive's preflight and the MCP list tool resolve claude for claude agents --json from the calling process's PATH and current directory; its output is untrusted and bounded by MAX_FRAME.
why: Same-user trust, an accepted risk; the MCP widening ratified.
@row_add n=101 row=decision/run-resolves-absolute-program area=input-validation rec=20
name: run resolves its child program itself
rule: viola run resolves its child program itself and hands the PTY an absolute path: a given path as given, a bare name looked up on PATH; a program not found is ProgramNotFound.
@row_add n=102 row=decision/revive-recorded-cwd-spawn-dir area=input-validation rec=20,16
name: revive spawns in the recorded cwd
rule: viola revive spawns claude in the snapshot's recorded cwd, read only after the strict-modes check, used as the spawn directory and for nothing else, and required to exist; program resolution and the version gate never read it.
why: A ratified boundary widening.
@row_add n=103 row=decision/session-id-argv-closed-shape area=input-validation rec=21
name: Session id on child argv
rule: A session id placed on a child's command line has the closed shape (36 characters, hyphens at 8, 13, 18, 23, hex elsewhere) and is one argv element of a direct spawn, never a shell, built only by one resume-args builder.
why: A ratified crossing of a logged value onto a child's command line.
@row_add n=104 row=decision/verify-run-e-resume-target area=input-validation rec=21
name: verify Run E resume target
rule: verify's Run E resumes only the session id of Run B's first SessionStart capture when it has the session-id shape, without fork; with none Run E spawns nothing and resume-session fails.
why: Ratified.
@row_add n=105 row=decision/refusals-hold-no-paths-pids-ids area=input-validation rec=21
name: Refusals hold no paths, pids or ids
rule: No refusal line or hint holds the recorded directory, a pid or a logged id; revive --list prints only session-shaped ids.
@row_add n=106 row=decision/statusline-source-selection area=input-validation rec=22
name: Statusline source selection
rule: viola run reads the statusline command from the home's statusline-source.json when it exists, else only for the default home the user's Claude settings; a command only from a non-empty command-type statusLine with no NUL; viola writes neither source.
why: Ratified, the file in the viola home chosen.
@row_add n=107 row=decision/statusline-shell-out area=input-validation rec=23
name: Statusline shell-out
rule: The recorded statusline command, the only shell-out, runs only in hook statusline as /bin/sh -c with the command as one argument; stderr is discarded, stdout capped and printed byte for byte, never parsed, and no log line carries the command.
why: A ratified boundary widening.
@row_add n=108 row=decision/statusline-deadline area=input-validation rec=23
name: Statusline command deadline
rule: The wait for the statusline command is bounded at 5 s (detail deadline); a failing command prints nothing.
state: provisional
@row_add n=109 row=absence/file-upload area=input-validation rec=24
capability: A file upload surface
@row_add n=110 row=absence/webhook area=input-validation rec=24
capability: A webhook surface
@row_add n=111 row=absence/database area=input-validation rec=24
capability: A database or SQL surface
@row_add n=112 row=decision/max-frame-shared-cap area=input-validation rec=25
name: One shared MAX_FRAME cap
rule: One 16 MiB MAX_FRAME constant bounds every external input read: channel frames, hook stdin, statusline source and output, answer input, MCP stdin lines, send text, child output, ndjson lines, config and stamps.
why: Sized above the largest expected permission input (Write-tool file bodies).
@row_add n=113 row=decision/max-frame-checked-against-ledger area=input-validation rec=25
name: MAX_FRAME checked against data
rule: verify records the largest hook payload per event kind, stamped per CLI version; a stamped maximum within a factor of 4 of MAX_FRAME is a signal to revisit the cap.
@row_add n=114 row=decision/bounded-readers-census area=input-validation rec=25
name: Every product reader bounded or named
rule: Every product reader call is bounded in its statement or named as an exception, and each external boundary's bounded read has a named witness.
guard: product_readers_are_each_bounded_or_named
except: Reads of viola's own executable for the pinned copy are not external input.
@row_add n=115 row=decision/no-stored-secrets area=secret-management rec=26
name: No stored secrets
rule: There are no stored secrets in v1, and environment variables are not a configuration channel.
except: Test-only seams compiled out of release builds, and the terminal colour facts read by viola list's human mode.
@row_add n=116 row=decision/inherited-credentials-never-serialised area=secret-management rec=26
name: Inherited credentials never serialised
rule: Inherited CLAUDE* credentials stay only in the wrapper's environment and are never serialised into events, snapshots, diagnostics or the ledger.
@row_add n=117 row=decision/r8-claude-env-strip area=secret-management rec=26
name: R8 CLAUDE environment strip
rule: Every inherited CLAUDE* name is removed from the child except a name in claude_env_keep, and the identity floor is removed whatever that set says; the strip and logs handle names only, never values.
why: Ratified.
@row_add n=118 row=decision/identity-floor-eleven-names area=secret-management rec=26
name: Identity floor stays eleven names
rule: The identity floor stays the eleven names; the plugin, project and env-file names are not identity names.
why: Ratified; the prefix rule already removes them when inherited outside the persistent set.
@row_add n=119 row=contract/claude-child-process-start/env-stripped-known area=secret-management rec=26
boundary: process-start claude-child line
item: env_stripped_known
promise: Names the stripped environment names, never a value.
@row_add n=120 row=contract/claude-child-process-start/env-kept area=secret-management rec=26
boundary: process-start claude-child line
item: env_kept
promise: Names the kept environment names, never a value.
@row_add n=121 row=decision/tests-use-own-home area=secret-management rec=27
name: Tests use their own home
rule: Development handles secrets as production does, and tests use their own --home directory.
@row_add n=122 row=decision/pre-push-linux-only area=secret-management rec=27
name: Pre-push gate Linux only
rule: The local pre-push gate runs only on a Linux host; elsewhere it exits 2 with pre-push-linux-only.
@row_add n=123 row=decision/pre-push-empty-environment-launch area=secret-management rec=27
name: Pre-push children launched with an empty environment
rule: The pre-push gate launches every child natively with an emptied environment carrying exactly HOME (from the passwd entry, never its own $HOME) and PATH of constant dirs, so no host value crosses; it has no other launcher and no uid-0 launch.
why: Keeps inherited CLAUDE* credentials out of every child.
except: A host-valued assignment is a boundary widening needing its own Decisions Log entry.
