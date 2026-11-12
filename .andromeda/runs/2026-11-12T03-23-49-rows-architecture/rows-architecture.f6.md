@row_add n=1 row=contract/cli-exit-codes/21 area=conventions rec=1,2,3,4,5
boundary: CLI exit codes
item: 21
promise: Instance unreachable: no live endpoint, a server that is not the recorded wrapper, a refused strict-modes check of the board, the plugin install tree or the target or driven instance, or a refused socket directory; no frame is written.
@row_add n=2 row=contract/mcp-tool-results/success area=conventions rec=6,7
boundary: MCP tool results
item: success
promise: A success is a tool result with `isError: false` and the channel `ok` payload as `structuredContent`; for `list`, the caller-side `{items, budget, skipped}` payload.
@row_add n=3 row=contract/mcp-json-rpc-errors/32602 area=conventions rec=6,7
boundary: MCP JSON-RPC errors
item: -32602
promise: An unknown tool name is the JSON-RPC error `-32602`; JSON-RPC errors are only the MCP connection's own faults.
@row_add n=4 row=contract/mcp-json-rpc-errors/32603 area=conventions rec=6,7
boundary: MCP JSON-RPC errors
item: -32603
promise: An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is `-32603` `internal error` with `data: null`.
@row_add n=5 row=absence/database area=conventions rec=8,9
capability: A database
@row_add n=6 row=contract/list-payload/budget area=standard-contracts rec=10,11,12
boundary: list payload
item: budget
promise: A top-level `budget`, `{five_hour, seven_day, read_at, paused}` or `"unknown"`; each window is `{used_percentage, resets_at}`, a field `"unknown"` when unparseable and a window `"unknown"` when omitted; `paused` only says the newest reading crosses a threshold.
@row_add n=7 row=contract/snapshot-envelope/shape area=standard-contracts rec=13,14
boundary: snapshot envelope
item: shape
promise: A snapshot is atomically replaced JSON of the shape `{v, written_at, writer, data}`.
@row_add n=8 row=contract/ledger-stamps/writer area=standard-contracts rec=15,16
boundary: ledger stamps envelope
item: writer
promise: `ledger/stamps.json` is atomically replaced JSON written only by `viola verify`; its `writer` is the literal `"verify"`, not a binary version.
@row_add n=9 row=contract/instance-snapshot/links area=standard-contracts rec=17,18,19
boundary: instance snapshot
item: links
promise: `links` is the set derived by replaying `link`/`unlink` events, as `{driver, driven, since}` with `since` the `link` line's `ts`; the wrapper restores it at every start, a revived one included, and a failed append leaves it as it was.
@row_add n=10 row=contract/list-payload/ok area=standard-contracts rec=20,21
boundary: list payload
item: ok
promise: The `list` `ok` payload, CLI `--json` and MCP `list` alike, is `{items, budget, skipped}`; `list` is not a channel method and needs no live wrapper.
@row_add n=11 row=contract/mcp-refusal-result/shape area=standard-contracts rec=22,23
boundary: MCP refusal result
item: result shape
promise: `{"content":[{"type":"text","text":"refused: budget-paused"}],"isError":true,"structuredContent":{"refusal":"budget-paused","detail":"five-hour"}}`
@row_add n=12 row=decision/no-shell-out area=standard-contracts rec=25,26
name: No shelling out
rule: Nothing shells out through `sh` or `bash`; child processes are spawned directly.
except: `hook statusline` runs the user's own statusline command, read at start from one named source, through `/bin/sh -c` with the same stdin.
@row_add n=13 row=decision/connect-to-recorded-endpoint area=occupied-resources rec=27,28
name: Clients connect to the recorded endpoint
rule: The wrapper resolves the endpoint path once and records it in the instance snapshot; every other process connects to that recorded `endpoint` and none rebuilds the path.
why: Processes spawned inside the session may run with a different environment from the wrapper.
@row_add n=14 row=decision/no-abstract-namespace-sockets area=occupied-resources rec=27,28
name: No abstract-namespace sockets
rule: Linux abstract-namespace socket names are never used.
why: They carry no filesystem permissions and any local user could connect.
