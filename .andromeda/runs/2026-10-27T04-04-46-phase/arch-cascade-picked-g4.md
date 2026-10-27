# arch · picked accounting · group 4 of 5

## Picked
conventions#1.1 · carried · the closed `RefusalReason` set and the paste rule's refusal order (`control-character`, then `empty-text`, client and wrapper) the chunk re-verifies at every client and on the wrapper
conventions#2.1 · carried · exit 2 for `send` text over `MAX_FRAME`, exit 13 for the paste refusals and exit 20 for a `link` / `unlink` name refused `-32602` are the outcomes of the caps and validators the chunk bounds
conventions#5.1 · carried · the client's own refusals (exit 13) come before any connect, the client-side paste rule before any frame the chunk re-verifies
conventions#6.1 · carried · the frame verbs' first step comes after `send`'s capped text read, in `src/cmd/send.rs` and `src/cmd/client.rs`, reached whole
conventions#7.1 · carried · the MCP handlers' re-validation of every argument (`ViolaName::try_new`, `u64`, `Response::parse`) into closed `invalid-params` field words, no frame written
conventions#9.1 · carried · the MCP frame tools' check order before any frame in `crates/viola-mcp/src/tools.rs`, reached whole, beside MCP `send`'s client-side paste rule
standard-contracts#1.1 · carried · the `list` payload's closed `status` values, the unwrapped row's name never a `target`, and the `"unknown"` budget reading of an unparseable value
standard-contracts#2.0 · carried · a snapshot read through the `MAX_FRAME` bound, an unsupported or unparseable envelope ignored and replayed
standard-contracts#3.0 · carried · a malformed `ledger/stamps.json` is `StampsMalformed` and `cli_verified:false`, the stamps reader the chunk bounds
standard-contracts#4.1 · carried · `update_stamps`'s capped read in `crates/viola-state/src/stamps.rs`, reached whole, its errors fixed messages
standard-contracts#5.5 · carried · a `link` / `unlink` refused `-32602` writes no line, the `driver` / `driven` validation the chunk proves
standard-contracts#8.1 · not in scope · the `send` method's budget-gate read, link record and cursor, not the paste check `src/run/send.rs` only re-reads nor any input bound
standard-contracts#9.1 · carried · `wait`'s `after` / `timeout_ms` closed to a `u64`, anything else `-32602`, unknown fields ignored
standard-contracts#10.1 · carried · `link` / `unlink` read no `from`, a `from` of any shape changing nothing, a channel params rule the chunk's closed-type proof covers
standard-contracts#11.1 · carried · the wrapper's re-validation of `hook.event` into the closed hook kinds and `origin` values, an invalid event not appended
standard-contracts#12.1 · carried · a failed, oversize or malformed `claude agents --json` read leaves the board as read
standard-contracts#13.1 · carried · the typed text (CR LF and CR as one LF, trailing CR and LF removed) the paste rule's re-verified step produces and matching reads
standard-contracts#15.1 · carried · carried with conventions#6.1: the frame verbs' strict-modes refusal document, made before anything under the home is opened, in the reached `src/cmd/client.rs`
standard-contracts#16.1 · carried · `hook statusline`'s stdin through `take(MAX_FRAME + 1)`, the command's stdout through `take(MAX_FRAME)`, its 5 s bound, the closed `user_command` shape and the capture arm
standard-contracts#18.1 · carried · the `claude agents --json` read through `take(MAX_FRAME + 1)`, killed at 5 s, parsed into rows or the closed `AgentsFailure`
