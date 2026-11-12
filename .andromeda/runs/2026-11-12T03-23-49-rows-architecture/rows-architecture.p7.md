@row_add n=1 row=decision/mcp-server-on-rmcp area=established-decisions rec=1
name: MCP server on rmcp over stdio
rule: The MCP server viola serves to drivers is built on the rmcp SDK over stdio, pinned to one minor version.
why: rmcp is the official SDK and tracks the MCP spec; the pinned minor limits churn from its near-weekly releases.
@row_add n=2 row=contract/mcp-tools/send area=established-decisions rec=1
boundary: MCP tools
item: send
promise: Takes `{target, text}`; `target` is a `ViolaName`.
@row_add n=3 row=contract/mcp-tools/wait area=established-decisions rec=1
boundary: MCP tools
item: wait
promise: Takes `{target, after?, timeout_ms?}`.
@row_add n=4 row=contract/mcp-tools/last area=established-decisions rec=1
boundary: MCP tools
item: last
promise: Takes `{target}`.
@row_add n=5 row=contract/mcp-tools/answer area=established-decisions rec=1
boundary: MCP tools
item: answer
promise: Takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
@row_add n=6 row=contract/mcp-tools/list area=established-decisions rec=1
boundary: MCP tools
item: list
promise: Takes `{}` and shows the driver the sessions.
@row_add n=7 row=contract/mcp-tools/refusal area=established-decisions rec=1
boundary: MCP tools
item: refusal
promise: A refusal is an `isError: true` tool result mapped from the channel's `result.refusal`.
@row_add n=8 row=contract/mcp-tools/from area=established-decisions rec=1
boundary: MCP tools
item: from
promise: The MCP server adds `from` from its own `VIOLA_NAME` only when it has a name of its own.
@row_add n=9 row=decision/human-controls-cli-only area=established-decisions rec=1
name: Human wheel and link controls are CLI-only
rule: `release`, `pause`, `link` and `unlink` are offered on the CLI only, never as MCP tools on the driver's surface.
why: They are human controls over the wheel and link topology, so a driver is never offered a way to hand the wheel back; driver links are already recorded from `from`.
@row_add n=10 row=decision/driver-release-deterred-not-enforced area=established-decisions rec=1
name: Release from a driver deterred, not enforced
rule: A `release` whose params carry a string `from` is refused, which deters a driver; a driver can still run CLI `viola release` outside a wrapped session, because v1 trusts every process of the same OS user.
why: `from` is self-reported, so the refusal deters rather than enforces.
@row_add n=11 row=contract/channel-release/release-from-driver area=established-decisions rec=1
boundary: Wrapper channel release method
item: release-from-driver
promise: A `release` whose `params` carry a string `from` is refused with `-32602` and `data.reason` `release-from-driver`.
@row_add n=12 row=decision/mcp-wait-cap area=established-decisions rec=1
name: MCP wait capped at 50 s
rule: MCP `wait` sends `timeout_ms` as the smaller of the given value and 50 000 ms, and 50 000 ms when none is given.
why: The cap sits below the Claude CLI's stdio tool-call default, so the tool call never times out on the client first.
@row_add n=13 row=contract/mcp-tool-errors/invalid-params area=established-decisions rec=1
boundary: MCP tool errors
item: invalid-params
promise: An argument object that breaks the tool's schema gives the `invalid-params` result with a closed field `detail` and no frame.
@row_add n=14 row=contract/mcp-tool-errors/-32603 area=established-decisions rec=1
boundary: MCP tool errors
item: -32603
promise: An internal handler fault is the JSON-RPC error `-32603` `internal error` with `data: null`.
@row_add n=15 row=decision/mcp-stdin-oversize-refused-whole area=established-decisions rec=2
name: Oversize MCP stdin line refused whole
rule: An MCP stdin line longer than the frame cap reaches rmcp as no byte and ends the session as at a closed stdin; the line is never truncated and never skipped.
why: A truncated line could dispatch a padded valid request, and a skipped one leaves the peer unanswered with no signal; follows the wrapper channel's close-on-oversize rule.
@row_none n=1 rec=3 type=4
relied: Every MCP frame tool refuses a home or target instance directory failing the strict-mode check with the `instance-unreachable` result and detail `strict-modes-failed`, sending no frame.
@row_none n=2 rec=4 type=4
relied: Every unreachable cause a frame tool meets surfaces as the `instance-unreachable` result with no frame and no line.
@row_add n=16 row=contract/mcp-stdin-oversize/parse-rejected area=established-decisions rec=5,6
boundary: MCP stdin oversize process log
item: parse-rejected
promise: A session ended by an oversize line writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN, holding no byte of the line.
@row_add n=17 row=contract/viola-mcp-exit-codes/0-oversize-after-initialize area=established-decisions rec=5
boundary: viola mcp exit codes
item: 0 after an oversize line once initialized
promise: An oversize line after `initialize` completed ends `viola mcp` with exit 0, as at a closed stdin.
@row_add n=18 row=contract/viola-mcp-exit-codes/1-oversize-before-initialize area=established-decisions rec=5,6
boundary: viola mcp exit codes
item: 1 on an oversize line before initialize
promise: An oversize line before `initialize` completes ends `viola mcp` with exit 1 and both streams empty.
@row_add n=19 row=decision/mcp-stdin-bound-in-own-reader area=established-decisions rec=7
name: MCP stdin bound in viola's own reader
rule: The MCP stdin line bound sits in the reader viola hands rmcp, never in rmcp's codec, and a cut reads as end of stream, never as an I/O error.
why: rmcp's stdio transport reads each line unbounded and its codec bounds only the writer side; end of stream keeps rmcp from writing anything of its own.
@row_add n=20 row=decision/exec-form-command-hooks area=established-decisions rec=8
name: Exec-form command hooks
rule: Hooks are exec-form `command` hooks running the pinned viola copy with `hook <event>` as arguments.
why: Exec form skips shell startup; `http` hooks need a fixed URL and so a daemon; `mcp_tool` is unavailable at SessionStart.
@row_add n=21 row=contract/hook-tiers/sync-with-decision area=established-decisions rec=8
boundary: Hook tiers
item: sync with a decision
promise: PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest.
@row_add n=22 row=contract/hook-tiers/ordering-spine area=established-decisions rec=8
boundary: Hook tiers
item: sync with no reply body (the ordering spine)
promise: SessionStart, UserPromptSubmit and Stop each write their `hook.event` and exit 0 within viola's spine deadline, below the 5 s `timeout` they get in `hooks.json`.
@row_add n=23 row=contract/hook-tiers/session-end area=established-decisions rec=8
boundary: Hook tiers
item: SessionEnd
promise: Sync within about 1 s: the channel, else a direct try-lock append of the event, else nothing; `hooks.json` gives it no `timeout` key, and `run` writes `session-end` from the process exit if no hook arrived.
@row_add n=24 row=contract/hook-tiers/async area=established-decisions rec=8
boundary: Hook tiers
item: async
promise: Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
@row_add n=25 row=decision/spine-deadline area=established-decisions rec=8
name: Spine deadline of 1 s
rule: Every ordering-spine hook finishes within one built-in spine deadline of 1 s, the same for every event.
guard: the perf gate (test-plan §10)
@row_add n=26 row=decision/hook-connect-deadline area=established-decisions rec=8
name: Hook connect deadline
rule: The connect deadline a hook passes to the channel client is 750 ms, kept below the spine deadline.
guard: a unit test asserting it below the spine deadline
state: provisional
@row_add n=27 row=decision/spine-hook-loses-event-not-turn area=established-decisions rec=8
name: Spine hook loses the event, never delays the turn
rule: A spine hook that cannot reach or write the endpoint within the deadline records the failure in `diagnostics/` and exits 0, so the event is lost.
why: The event is lost rather than the turn delayed; a driver recovers through `wait`'s `timeout_ms`.
@row_add n=28 row=decision/session-end-budget-never-raised area=established-decisions rec=8
name: SessionEnd budget never raised
rule: The 1.5 s SessionEnd budget is never raised.
@row_add n=29 row=decision/hook-tier-map-verified area=established-decisions rec=8
name: Hook tier map is a verified ledger row
rule: The hook tier map is a ledger row that `viola verify` re-stamps.
guard: viola verify
@row_add n=30 row=contract/hook-diagnostics/socket-dir-refused area=established-decisions rec=9,14
boundary: Hook diagnostics
item: socket-dir-refused
promise: A hook meeting a refused socket directory sends no frame, exits 0 with empty streams and records one `hook-decision` at WARN with `detail` `socket-dir-refused`.
@row_add n=31 row=contract/hook-diagnostics/server-verify-failed area=established-decisions rec=9,14
boundary: Hook diagnostics
item: server-verify-failed
promise: A hook whose serving process has another euid, is not the snapshot's pid and start time, or cannot be checked sends no frame, exits 0 with no body and records `server-verify-failed`.
@row_add n=32 row=decision/hooks-fail-open area=established-decisions rec=10
name: Hooks fail open
rule: A hook never blocks the session: it exits 0 always, never exits 2, and exits 0 at once when `VIOLA_NAME` is absent.
why: Exit 2 is a blocking error in Claude Code's contract, and a viola bug must never block the user's session.
except: The hidden `--capture` arms called only by `viola verify`'s probes read no `VIOLA_*`, open no channel and still always exit 0 with empty stderr.
@row_add n=33 row=contract/hook-exit/0-with-body area=established-decisions rec=10
boundary: Hook exit and body
item: exit 0 with a JSON body
promise: A decision.
@row_add n=34 row=contract/hook-exit/0-no-body area=established-decisions rec=10
boundary: Hook exit and body
item: exit 0 with no body
promise: No decision: the dialog renders for the human.
@row_add n=35 row=contract/hook-exit/2 area=established-decisions rec=10
boundary: Hook exit and body
item: exit 2
promise: Never produced.
@row_add n=36 row=contract/probe-capture/payload-file area=established-decisions rec=10
boundary: Hook probe capture directory
item: `<PascalEvent>.<k>.json`
promise: Each captured payload is claimed exclusively at 0600 under the first free `k`, which counts across events, so names sort in arrival order and two captures never share a name.
@row_add n=37 row=contract/probe-capture/statusline-shell-file area=established-decisions rec=10
boundary: Hook probe capture directory
item: `StatusLine.<k>.shell`
promise: Holds exactly `bin-sh` when the `--shell` word equals `/bin/sh` byte for byte, else `other`; the word itself is written nowhere.
@row_add n=38 row=decision/dialog-hold area=established-decisions rec=10
name: Dialog hold per instance
rule: A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start.
state: provisional
@row_add n=39 row=decision/viola-ends-the-dialog-wait area=established-decisions rec=10
name: Viola, never Claude Code, ends the dialog wait
rule: The dialog hook bounds its reply read at the hold ceiling plus 5 s, and the dialog hooks' `timeout` in `hooks.json` sits above that by a pinned margin.
why: Viola ends the wait rather than Claude Code, and the rendered plugin stays byte-identical across starts.
@row_add n=40 row=contract/dialog-hold-end/null-reply area=established-decisions rec=10
boundary: Dialog hold end
item: null reply
promise: At the hold's end the wrapper replies `null`, the hook exits 0 with no body, the dialog renders for the human, and a later `answer` for that dialog is refused `not-delivered` with detail `unknown-dialog`.
@row_add n=41 row=decision/hook-never-writes-stderr area=established-decisions rec=10
name: Hook logs only to diagnostics
rule: A hook never writes to stderr; its codes-only log goes to the home-level diagnostics and content-bearing detail only to the instance's diagnostics, and a failure to write either is ignored.
@row_add n=42 row=contract/event-line/dialog-expired area=established-decisions rec=11
boundary: The event line
item: dialog-expired
promise: Appended once by the wrapper at a dialog hold's end with `{dialog_id, dialog_kind}` and no question, plan or tool text, waking a parked `wait`; a hand-back or an answer appends none.
@row_none n=3 rec=12 type=8
@row_none n=4 rec=13 type=4
relied: A dialog hook whose server check fails sends no `hook.dialog` and exits 0 at once with no body, so the dialog renders for the human.
@row_none n=5 rec=15 type=4
relied: A hook whose instance check refuses or finds no instance exits 0 before reading config, stdin or writing any line.
@row_none n=6 rec=16 type=9
@row_add n=43 row=decision/byte-source-wheel-claim area=established-decisions rec=17
name: Byte-source wheel claim
rule: Any human editing key since the last turn boundary moves the wheel to the human, and automated `send` is then refused `human-typing`.
why: `viola run` owns the real stdin, so it knows byte provenance exactly.
@row_add n=44 row=decision/closed-non-editing-byte-list area=established-decisions rec=17
name: Closed list of non-editing bytes
rule: Only a closed list of focus reports, mouse reports and terminal replies, each by exact grammar, never takes the wheel; every other byte does, and a reply no terminal was measured to write is not added on a guess.
why: Terminal replies read as typing took the wheel from a live driver with no key typed.
@row_add n=45 row=decision/wheel-classifier-only-observes area=established-decisions rec=17
name: Wheel classifier only observes
rule: The stdin classifier returns every byte whole and unchanged, and a read holding an editing key moves the wheel before it returns.
@row_add n=46 row=decision/atomic-bracketed-send area=established-decisions rec=17
name: Atomic bracketed-paste send
rule: A bridge send is written as one bracketed-paste write plus CR, and human bytes arriving during it are held and passed on straight after.
@row_add n=47 row=decision/human-prompt-confirms-wheel-take area=established-decisions rec=17
name: Human prompt confirms the wheel take
rule: An unsent prompt the hook filed as `human` moves the wheel before its line is appended; a `harness` prompt and the in-flight driver prompt never move it.
@row_add n=48 row=decision/harness-prompts-not-human area=established-decisions rec=17
name: Harness prompts are not a human take
rule: A prompt whose raw start is one of the compiled harness prefixes is filed as a harness turn and never moves the wheel.
except: A human who types such a tag at a prompt's start is filed `harness`, an accepted side effect.
@row_add n=49 row=decision/wheel-starts-with-driver area=established-decisions rec=17
name: Wheel starts with the driver
rule: The wheel starts with `driver` when `viola run` starts, so automation is accepted until the first human editing key.
@row_add n=50 row=decision/send-never-queued area=established-decisions rec=17
name: Send never queued
rule: A `send` while a turn runs or while another `send` is in flight is refused `not-delivered` with detail `turn-running`, never queued, and nothing is typed for it.
@row_add n=51 row=decision/human-interrupt-moves-wheel area=established-decisions rec=17
name: Human interrupt moves the wheel
rule: A human interrupt is human input and moves the wheel to the human; only a `release` that returns a human-held wheel clears the running-turn state.
why: A turn can end with no `turn-ended`, since Claude Code's Stop hook does not fire on a user interrupt.
@row_add n=52 row=decision/wheel-returns-only-through-release area=established-decisions rec=17
name: Wheel returns only through CLI release
rule: CLI `viola pause` moves the wheel to the human without a keystroke, and the wheel returns to the driver only through CLI `viola release`.
@row_add n=53 row=contract/send-refusal/manual-pause area=established-decisions rec=17
boundary: Send refusal details
item: manual-pause
promise: After `viola pause`, automated `send` is refused `human-typing` with detail `manual-pause`.
@row_add n=54 row=decision/human-wheel-renders-dialogs area=established-decisions rec=17
name: Human-held wheel renders dialogs at once
rule: While the wheel is `human`, `hook.dialog` is answered `null` at once and `answer` is refused `human-typing`; a dialog pending when the wheel moves to the human is answered `null` at once.
why: A dialog held by the hook is never hidden from the human for the rest of the hold.
@row_add n=55 row=decision/budget-pause-gates-only-send area=established-decisions rec=17
name: Budget pause gates only send
rule: `budget-paused` gates only `send`, never `hook.dialog` or `answer`.
why: A running turn finishes.
@row_add n=56 row=decision/links-recorded-as-log-events area=established-decisions rec=18
name: Links recorded as log events
rule: A link `driver → driven` is recorded when a `send` or `answer` comes from inside a wrapped session to another instance, or by `viola link`, and removed by `viola unlink`; the snapshot's link set is derived by replaying those events.
why: The log stays the single source of truth for the link set as for everything else.
@row_add n=57 row=contract/event-line/link area=established-decisions rec=18
boundary: The event line
item: link
promise: The driven wrapper appends a `link` line, `source: wrapper`, only when the pair is not already in its link set; a refused `send` or `answer` and a `from` equal to the wrapper's own name link nothing.
@row_add n=58 row=contract/event-line/unlink area=established-decisions rec=18
boundary: The event line
item: unlink
promise: `viola unlink` appends an `unlink` line with `source: wrapper` and `{driver, driven}`.
@row_add n=59 row=contract/channel-link/already-linked area=established-decisions rec=18
boundary: Wrapper channel link method
item: already_linked
promise: A `link` of an existing pair adds no line and answers its held `since` with `already_linked: true`.
@row_add n=60 row=contract/channel-link/-32603 area=established-decisions rec=18
boundary: Wrapper channel link recording
item: -32603
promise: A link that cannot be recorded is `-32603`, with nothing issued or the answer not taken.
@row_add n=61 row=decision/instance-check-before-any-read area=established-decisions rec=19
name: Instance check before any read
rule: The client verbs check the home and target instance directories before any role file, log or snapshot read.
why: Opening a log narrows an existing home to 0700, so a check made after it would pass a home at 0770.
@row_add n=62 row=contract/cli-exit-codes/21 area=established-decisions rec=19
boundary: CLI exit codes
item: 21
promise: The instance check, the socket-directory check, the peer euid check or the server identity check refused.
@row_add n=63 row=contract/process-log/link-channel-lines area=established-decisions rec=20
boundary: Wrapper process log
item: link and unlink channel lines
promise: The `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no driver or driven name.
@row_none n=7 rec=21 type=4
@row_add n=64 row=decision/budget-governor-soft-stop area=established-decisions rec=22
name: Budget governor soft stop
rule: Once a configured threshold is crossed, automated `send` is refused `budget-paused`; human keys are never blocked, a running turn finishes, and the pause lifts when `resets_at` passes or on `viola release --budget`.
why: The per-model weekly window is hidden, so thresholds are set low and the governor stays visible and conservative.
@row_add n=65 row=decision/budget-default-thresholds area=established-decisions rec=22
name: Budget default thresholds
rule: The defaults are `five_hour` at or above 90 % and `seven_day` at or above 85 %, read once at the wrapper's start, and no value switches the gate off.
@row_add n=66 row=decision/release-budget-per-instance area=established-decisions rec=22
name: Budget release per instance
rule: `release --budget` lifts the pause for one instance only, until the crossed window's `resets_at`, and leaves the wheel; a plain `release` never lifts a budget pause, and an override never outlives its wrapper.
@row_add n=67 row=decision/budget-gate-never-polls area=established-decisions rec=22
name: Budget gate evaluated at three points
rule: The wrapper evaluates the budget gate only at start, when a `send` arrives and at each `turn-ended`, never by polling, so a snapshot can read paused after the reset until the next point.
@row_add n=68 row=decision/budget-gate-fail-open area=established-decisions rec=22
name: Budget gate fails open
rule: No reading, an unknown figure, a passed or unparseable `resets_at`, an unreadable `budget.json` or a bad threshold leaves the gate open or at its default.
@row_add n=69 row=contract/send-refusal/order area=established-decisions rec=22
boundary: Send refusal details
item: order
promise: `control-character` → `empty-text` → `human-typing` → `budget-paused` → `turn-running` → the readiness gate's wait.
@row_add n=70 row=decision/only-send-reads-budget-gate area=established-decisions rec=22
name: Only send reads the budget gate
rule: `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the budget gate.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=71 row=decision/statusline-wrapped-per-session area=established-decisions rec=22
name: Statusline wrapped only per viola session
rule: The statusline is wrapped only for sessions started by `viola run`, through a per-session settings override that runs the user's own statusline command unchanged.
why: The user's global statusline config is never touched.
@row_none n=8 rec=23 type=9
