# Task — architecture · 2026-10-27-bounded-inputs-at-every-boundary

## The session
The session bounded the last unbounded external readers. Each `viola mcp` stdin line is now capped at `MAX_FRAME` by a new crate-private reader, `viola_mcp::stdin::CappedLines`, which viola hands to rmcp. A longer line ends the session and writes one `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.
The fallback compare in `viola_state::fs::replace_private_shared` now reads its target through `take(bytes.len() + 1)` (new private `holds_exactly`) and no longer reads the whole file.
`deny.toml` now bans serde_json's `unbounded_depth` feature, and a firing probe was added: `deny-probes: 13/13` became `14/14`.
A new E2E contract binary, `tests/contract_bounded_readers.rs`, lists 40 `BOUNDARIES` rows and 2 `EXCEPTIONS` rows. It also runs a census: every product reader must be bounded in the same statement or named as an exception.
The chaos feed-panic test now also checks that passthrough reaches the outer terminal. On Unix it reads the child's bytes. On every OS it checks that no viola literal appears. The Windows byte reading is owed.
A measurement disproved one claim in architecture. [Screen Model] says a caught vt100 panic logs `oversize`, but the code logs `panicked`; `oversize` is only for a dropped copy. No crate, dependency, IPC method, MCP tool, env var, CLI flag, exit code or error variant was added, and nothing was retired.

## Reached files
- crates/viola-mcp/src/stdin.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-state/src/fs.rs
- schemas/diag-line.v1.json
- deny.toml
- scripts/deny-probes.sh
- tests/contract_bounded_readers.rs
- tests/chaos_feed_panic.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/verification-matrix.json

## Touches
- established-decisions [Screen Model] (dec-screen-model): the line saying a caught vt100 panic and a message dropped by the full queue each give `parse-rejected{parser:"vt100-feed", detail:"oversize"}` is wrong. A caught panic gives `panicked` and a dropped copy gives `oversize`. Measurement disproved the line at established-decisions.md:23. The code is `src/run/gate.rs:49` and `:72`.
- established-decisions [MCP] (dec-mcp): the body now needs to state the stdin line bound and how an over-long line ends the session.
- established-decisions [Snapshot writer] (dec-snapshot-writer): `replace_private_shared`'s fallback compare now reads at most the bytes plus one through `holds_exactly`. The outcome is the same as a whole-file compare. Its callers, `pin_and_plugin` and `pin_companions`, are unchanged.
- established-decisions [Concurrency / Backend Framework] (dec-concurrency-backend-framework): the new `AsyncRead` reader sits inside `viola-mcp`, the one crate allowed to use Tokio. This is a re-read: Tokio containment holds.
- stack-and-technologies (stack): the serde_json entry gains the `deny.toml` `[[bans.features]]` ban on `unbounded_depth`. The rmcp entry stays at `server` + `transport-io`, read by `scripts/rmcp-features.sh`. The reader goes through rmcp's async-rw `(reader, writer)` transport, which `transport-io` already enables.
- the [Validation] decision in the body and its "serde_json default depth, never `unbounded_depth`" stance: a `deny.toml` feature ban with a firing deny probe now enforces it.
- The registry contract key `build-system.md:8` (architecture infrastructure registry) carries the deny probe verdict count. It moves from `13/13` to `14/14`: 13 `probes=(` rows plus the planted `channel-tokio-edge`.
- occupied-resources res-filesystem, the `diagnostics/` entry: `mcp.ndjson` can now carry one `parse-rejected{mcp-stdin, oversize, count:1}` WARN line. The line holds no byte of the line that was cut.
- occupied-resources res-repository, the `schemas/diag-line.v1.json` entry: the `parse-rejected.parser` enum gains `mcp-stdin`, so the parser catalog grows from 9 to 10 values.
- occupied-resources res-filesystem, the test-only chaos home entry `<temp dir>/viola-chaos-*/home`: `tests/chaos_feed_panic.rs` now also checks the passthrough after the forced feed panic. On Unix the child's `e4 b8 ad` must reach the outer terminal. On every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` may appear on it.
- occupied-resources res-binaries, the hidden `mcp` subcommand entry: it now ends at a stdin line past `MAX_FRAME` the same way it ends at a closed stdin. It exits 0 once initialized, and exits 11 (`ServeError::Initialize`) when the cut lands before `initialize`.
- cross-cutting-patterns, Diagnostic output channels (in the body): the cut reads as end of stream, never as an I/O error. rmcp writes nothing of its own, and the one log line is viola's, through `obs_event!`. The rule that `mcp` writes only MCP frames to stdout still holds.
- cross-cutting-patterns, Crash-safe disk writes (in the body) and `replace_private` atomicity: re-read and unchanged. Only the compare read in front of the replace is bounded.
- standard-contracts contract-channel-methods, the frame-cap paragraph: the wrapper channel's close-on-oversize rule is the stated precedent for the MCP stdin rule. The architecture.md:177 claim of `channel-frame` was re-read and is unchanged.
- The test inventory: `contract_bounded_readers` is a new `contract_` E2E binary that runs under `run --e2e`. `mcp_tools` gains the case `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`. `viola-mcp` gains 6 `stdin::tests` unit cases and the child case `serve_ends_the_session_at_a_line_past_max_frame`. `viola-state` gains `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes`.

## New facts
- F1 · established-decisions · `viola mcp` reads stdin through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` per line, with the `\n` counted. A line of exactly the cap passes. A longer line releases no byte to rmcp, sets a cut flag and then reads as end of stream. The line is refused whole and the session ends; it is never truncated or skipped.
- F2 · established-decisions · After a session ended by a cut, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN, holding no byte of the line. It then returns what the session gave: `Ok(())` (exit 0) once initialized, or `ServeError::Initialize` (exit 11) when the cut lands before `initialize`. That second path is driven by no test, and its owner was given at P5.
- F3 · established-decisions · The bound on rmcp's input belongs to the reader viola hands rmcp, never to the codec. rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` serves only the writer side.
- F4 · stack-and-technologies · `deny.toml` bans serde_json's `unbounded_depth` feature in a `[[bans.features]]` table. Its stated reason is that every parse of external bytes keeps serde_json's default recursion limit. `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires.
- F5 · cross-cutting-patterns · Every product reader of external bytes is bounded in the same statement (`take`) or named in an `EXCEPTIONS` row. `tests/contract_bounded_readers.rs` enforces this: 40 `BOUNDARIES` rows each name a bounded read and its witness, 2 `EXCEPTIONS` rows cover the own-executable reads, and a census walks the product sources. Five rows have witnesses that never drive past the cap, a test-depth gap whose owner was given at P5.
- F6 · established-decisions · OPEN, PROVISIONAL: on a Windows host, how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal after a contained vt100 feed panic under a 1-column ConPTY. Windows asserts only that no viola literal appears. Owner: "Windows-only live measurements", carried by the P5 card.

## Untouched
- IPC endpoints (named pipe and Unix socket naming, the per-user 0700 socket dir, the SQOS open): no endpoint or channel transport changed.
- The channel method set, its `params` and `ok` payloads, the `list` payload, the event line and the event `data` per kind: no method, kind or field was added.
- The MCP tool set (`send · wait · last · answer · list`), the MCP result and refusal shapes (conv-mcp-results), and the Claude Code integration names `mcp__plugin_viola_viola__<tool>`: no tool or result shape changed.
- The CLI exit code table (conv-exit-codes) and `RefusalReason` with its details (conv-refusal-reason): no code, reason or detail was added. The `mcp` exits at a cut are its existing closed-stdin outcomes.
- [Error Handling] and the Rust error types convention: `ServeError` is unchanged and the three open divergences stand as written.
- Environment variables (res-env-vars) and config management (crosscut-config): no env var, `config.json` key or CLI flag was added.
- Workspace crates and module boundaries (dec-module-boundaries), and the sync-crates list: no crate was added or removed. `viola-mcp` gains only a crate-private module.
- Dependency versions in stack and Inherited Defaults (rmcp 3.4.1, serde_json 1.0.151, tokio, tempfile): none added or bumped, and `Cargo.lock` was not touched.
- The capability ledger, its rows and stamps (crosscut-capability-ledger, dec-cli-version-compatibility, contract-ledger-stamps): no ledger row was added or re-stamped.
- Session liveness, the wheel, links, the budget governor, hook transport and the hook contract: none of their code paths changed.
- The instance snapshot and snapshot envelope contracts, and the snapshot replay readers: no field or reader changed.
- `fuzz/` targets and corpus counts (six targets) under res-repository: no fuzz target or seed was added.
- The frontend, GUI and real-time entries: still deferred to 0.2.0.
