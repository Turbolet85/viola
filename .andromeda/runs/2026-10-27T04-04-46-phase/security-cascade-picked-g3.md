# security · picked accounting · group 3 of 3

## Picked
error-handling#1.1 · carried · the hook stdin cap, the one-object parse and the capture arms' `take(MAX_FRAME + 1)` / `take(64)` bounds are the chunk's own boundary, each failure failing open
error-handling#1.2 · carried · `src/cmd/hook.rs` is reached whole and a bound the chunk adds there keeps the server-check failure frameless and fail-open
error-handling#1.3 · carried · the chunk re-reads that a replayed snapshot carries no `endpoint` / `pid` / `started_at`, and the hook's `connect` over it fails open with no frame
error-handling#1.4 · carried · `src/cmd/hook/statusline.rs` is reached whole and the chunk re-reads that a replayed snapshot carries no `statusline_command`
error-handling#1.5 · carried · `crates/viola-mcp/src/tools.rs` is reached whole; the handlers' new raw-argument checks sit beside the server-check refusal shape they must keep
error-handling#1.6 · carried · the MCP handlers' argument checks and reads must keep `check_instance` before the liveness pre-check and its unreachable result
error-handling#1.7 · not in scope · the `link` / `unlink` record/remove fault and the CLI's reply mapping live in `src/run/links.rs` and `src/cmd/link.rs`, which the chunk does not reach
error-handling#1.8 · carried · the chunk bounds `send` / `answer`'s stdin / `--file` read, which must come after `client::refuse_untrusted` and before any frame
error-handling#1.9 · carried · `src/cmd/run.rs` and `src/cmd/revive.rs` are reached whole; every bounded read the chunk adds at a start comes after `check_home_tree`
security-anti-patterns#1.1 · carried · the chunk reaches every frame client and caps the snapshot read whose values the server check takes, which must stay strict-modes checked in the same process
security-anti-patterns#4.1 · carried · `crates/viola-state/src/stamps.rs` and `src/run/version_gate.rs` are reached whole; a bounded, closed-type stamp read keeps the relied-set rule and writes nothing
security-anti-patterns#4.2 · carried · `update_stamps`'s capped read is the chunk's own boundary, and its strict-modes check stays before the create and again under the lock
security-anti-patterns#5.1 · carried · the chunk reads `config.json` values into closed types and re-verifies `validate_paste_text`, neither of which any value, variable or flag may switch off
