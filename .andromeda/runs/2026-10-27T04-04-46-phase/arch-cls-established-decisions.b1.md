# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · yes · Tokio containment: only viola-mcp builds a runtime, while run, hook, send and the channel-server paths stay std threads, the chunk's stated containment need
established-decisions#2.1 · yes · revive's recorded-cwd check (an existing directory, else cwd-missing, never read by program resolution or the version gate) that the chunk's revive and bounded version-gate spawn reach; the ConPTY, host_stdin and pump-writer parts are not reached
established-decisions#3.1 · yes · a caught vt100 feed panic degrades readiness to input-not-ready while the child's passthrough never waits on the model, the re-verified degrade in gate.rs, screen.rs and pump.rs
established-decisions#4.1 · yes · validate_paste_text then typed_text, and the empty-text refusal before any frame and before the first wheel read, the paste rule the chunk re-verifies in cmd/send.rs and run/send.rs
established-decisions#5.1 · yes · the events.ndjson reader's three skip counts, torn-line tolerance and never-rewritten rule, the chunk's events reader bound
established-decisions#6.1 · no · the replace helper's temp-file, mode and retry rules bind a write site, and this chunk adds none (its reach is the budget.json and stamps.json reads; verify's stamp writing is Not needed)
established-decisions#7.1 · no · the pinned copy's no-clobber placement (place_absent) is Not needed beyond the pinned key's shape
established-decisions#7.2 · no · the placed test hook of place_private_new_with is a test parameter of the pinned copy's placement, which is Not needed
established-decisions#7.3 · no · the write handle closed before placement against ETXTBSY is the pinned copy's placement, which is Not needed
established-decisions#8.1 · yes · wait's after and timeout_ms arguments re-validated on MCP and the CLI, the one-pending hook.dialog null reply and the closed unknown-dialog detail, and answer by dialog_id; the SQOS open and endpoint naming are not reached
established-decisions#9.1 · no · the peer-pid accessor feeds only the server identity check, owned by Unix endpoint and home hardening (Not needed)
established-decisions#10.1 · no · the macOS no-pid gap feeds only the server identity check and the macOS socket checks, owned by Unix endpoint and home hardening (Not needed)
established-decisions#11.1 · yes · a higher-v or unreadable snapshot read as absent through replay, and no src/ reader calling read_snapshot, the chunk's snapshot.json bound
established-decisions#12.1 · yes · the statusline arm's snapshot read through the replay, so a replayed snapshot runs no statusline command and prints nothing
established-decisions#13.1 · yes · the MCP tool inputs send, wait, last, answer and list with their invalid-params and -32602 refusals, release's from and budget checks, and wait's timeout cap
established-decisions#14.1 · no · check_instance before live_endpoint on each frame tool is the home strict-modes check, owned by Unix endpoint and home hardening (Not needed)
established-decisions#15.1 · yes · its nine events are the closed hook event argument list the chunk bounds, and the endpoint-unreachable exit 0 is the hook's fail-open; the tiers and deadlines are Not needed
established-decisions#16.1 · yes · the shared connect's replay read (a replayed or absent snapshot gives channel-unreachable with fail-open exit 0) in cmd/hook.rs; verify_server is owned by Unix endpoint and home hardening
