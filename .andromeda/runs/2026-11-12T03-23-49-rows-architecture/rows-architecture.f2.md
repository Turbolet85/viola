@row_add n=1 row=decision/separate-lock-files area=established-decisions rec=1,2,3
name: Locks on separate .lock files
rule: State locks are std `File::lock` on separate `.lock` files, opened for write, so no file is both appended to and exclusively locked.
why: A layout chosen while Windows, where append plus exclusive lock fails, was a target, and kept.
@row_add n=2 row=decision/tempfile-snapshot-writer area=established-decisions rec=4,5
name: tempfile as the snapshot writer
rule: Snapshots and every other replaced file are written through the shared helpers built on tempfile `persist` and `persist_noclobber`; no call site carries its own temp + rename code, a failure leaves the target as it was, and the pinned copy is never replaced.
why: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
@row_add n=3 row=decision/ipc-interprocess-local-socket area=established-decisions rec=6
name: interprocess local_socket for IPC
rule: Inter-process messaging uses interprocess `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam; `hook` and `wait` block on the channel, so nothing polls.
why: One API over Unix sockets, sync and Tokio flavours, and no daemon.
@row_add n=4 row=decision/identity-by-instance area=established-decisions rec=7,8
name: Identity by instance, never by location
rule: Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` or an explicit `ViolaName` argument, never through the working directory; instances are identified by `ViolaName`, endpoints by its short hash, and there are no database keys.
why: Two sessions in one folder never share records; identity is by instance, never by location (R5).
@row_add n=5 row=contract/mcp-tools/send area=established-decisions rec=9,10,11
boundary: MCP tools
item: send
promise: One of the five MCP tools; takes `{target, text}`, `target` a `ViolaName`.
@row_add n=6 row=contract/mcp-tools/wait area=established-decisions rec=9,10,11
boundary: MCP tools
item: wait
promise: One of the five MCP tools; takes `{target, after?, timeout_ms?}`.
@row_add n=7 row=contract/mcp-tools/last area=established-decisions rec=9,10,11
boundary: MCP tools
item: last
promise: One of the five MCP tools; takes `{target}`.
@row_add n=8 row=contract/mcp-tools/answer area=established-decisions rec=9,10,11
boundary: MCP tools
item: answer
promise: One of the five MCP tools; takes `{target, dialog_id, response}`, with `response` in the channel `answer` shape.
@row_add n=9 row=contract/mcp-tools/list area=established-decisions rec=9,10,11
boundary: MCP tools
item: list
promise: One of the five MCP tools; takes `{}` and shows the driver the sessions.
@row_add n=10 row=contract/hook-tiers/sync-with-decision area=established-decisions rec=12,13
boundary: Hook tiers
item: sync with a decision
promise: PreToolUse with matcher `AskUserQuestion|ExitPlanMode`, and PermissionRequest, run sync and may reply with a decision.
@row_add n=11 row=contract/hook-tiers/async area=established-decisions rec=12,13
boundary: Hook tiers
item: async
promise: Notification, PostToolUse and PostToolUseFailure run with `async: true` and print nothing.
@row_add n=12 row=decision/hooks-fail-open area=established-decisions rec=14,15,16
name: Hooks fail open
rule: `viola hook` always exits 0, including on parse errors, channel failures and caught panics, and at once when `VIOLA_NAME` is absent; a JSON body is a decision, no body no decision; exit 2 is never produced and `hook` never writes to stderr.
why: Exit 2 blocks Claude Code, and a viola bug must never block the session.
except: The hidden `--capture` arms, called only by `viola verify`'s probes, read no `VIOLA_*` and open no channel, still exiting 0 with empty stderr.
