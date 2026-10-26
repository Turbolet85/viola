# architecture · picked accounting

## Picked
stack-and-technologies#1.6 · proposed · the Wrapper IPC row now also has its clients read the serving process's pid through interprocess `peer_creds().pid()` (SO_PEERCRED, GetNamedPipeServerProcessId on the SQOS-adopted end), pin unchanged
stack-and-technologies#1.17 · proposed · sysinfo's start time is now also the observed side of every frame-writing client's server check, not only the liveness signal
established-decisions#5.1 · not touched · its rule is the events.ndjson appenders and reader, and the report reaches no events code: liveness.rs is the only viola-state file changed, and every log claim here still holds
established-decisions#7.1 · proposed · the IPC decision's SQOS-only client now also reads the server's peer pid and verifies pid + start time before the first frame (F3, F4); fork 1 on unreadable snapshots is held PROVISIONAL (F9)
established-decisions#9.1 · proposed · the spine tier's lost-endpoint rule now also covers a server that is not the recorded wrapper (F7), and SessionEnd takes the direct append when its server check fails
established-decisions#10.1 · proposed · a dialog hook whose server check fails sends no hook.dialog and exits 0 with no body, a new fail-open path this build writes as server-verify-failed
established-decisions#13.1 · not touched · its SessionEnd residual «(the channel did not take it)» already covers a SessionEnd whose server check fails, and the report changes only pause and release's client connect, not the wheel
established-decisions#14.1 · proposed · the held explicit verbs' «liveness-only pre-check» crossing is now followed by the server check on Linux and Windows for every CLI frame verb
established-decisions#17.1 · proposed · the snapshot's pid + start time are now also the server identity, read with the endpoint in one read through live_endpoint's LiveEndpoint
conventions#2.1 · proposed · exit 21 now also covers a server-verification failure on the six CLI frame verbs, its document unchanged and the new hint last (F5)
conventions#3.1 · proposed · the MCP frame tools now return the instance-unreachable isError result on a server mismatch, with no frame and no line (F6)
standard-contracts#2.0 · proposed · its «five product callers» of read_snapshot measures three now (hook.rs:204, hook/statusline.rs:142, run.rs:379); src/cmd/client.rs holds none
standard-contracts#5.1 · proposed · beside the send method, the pre-frame server check every frame-writing client now makes (F1)
standard-contracts#6.1 · not touched · its claims are the wrapper's own handling of a wait frame (params, start offset, wake, deadline in src/run/wait.rs), which the report leaves unchanged; the client's check is F1
standard-contracts#7.1 · proposed · hook.event is now sent only once the hook's server check has passed, the first dated exception retired on Linux and Windows
occupied-resources#13.1 · proposed · cli-<name>.ndjson's exit-21 process-exit now also writes server-verify-failed with during connect, and hook-decision writes it (F8); mcp.ndjson writes nothing for it
occupied-resources#19.1 · not touched · its WITHIN count is dated to chunk 2026-10-10-budget-governor's wrap, a reading the session cannot move, and the report's Counts move no WITHIN pattern
