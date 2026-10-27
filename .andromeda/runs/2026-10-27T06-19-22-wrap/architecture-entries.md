## 2026-10-27-bounded-inputs-at-every-boundary — bounded readers, a cross-cutting pattern held by a census
**Section:** §Cross-cutting Patterns (architecture.md: a broad bullet «Bounded readers» after «Untrusted upstream text»; the subjects line); cross-cutting-patterns.md (a new `procedure · crosscut-bounded-readers` group)
**Change:**
- The body gains the broad bullet: every product reader of external bytes is bounded in its own statement (`take`) or named as an exception (the own-executable reads); the contract binary `tests/contract_bounded_readers.rs` enumerates the boundaries and censuses the product sources.
- The detail gains the group «Bounded readers at every boundary»: no product reader reads an external byte source to its end unbounded. The E2E contract binary (a `contract_` binary, run under `run --e2e`) holds it: its `BOUNDARIES` table, one `Boundary {` block per bounded read, names each read and its witness test, checked by `boundaries_each_name_a_bounded_read_that_stands`; its `EXCEPTIONS` table names the own-executable reads, which are no external input. The census `product_readers_are_each_bounded_or_named` walks the product sources and judges each reader call bounded over its statement up to the call, matching an `EXCEPTIONS` needle against the call's whole source line; its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one` reads a planted unbounded read as a hit and passes a bounded one; `serde_json_unbounded_depth_is_banned_in_deny_toml` reads the `deny.toml` feature ban. A new reader lands bounded in its statement or as a named exception, or the census goes red.
- The subjects line gains `crosscut-bounded-readers`.
**Why:** the chunk's acceptance, met: the census reads every product reader bounded in-statement or an `EXCEPTIONS` row, and its self-check reads a planted unbounded read as a hit. The needle is matched against the whole source line because the statement ends at the call's `(` and the executable argument the needle names lies past it (an implement deviation, its census hazard). The body bullet was raised so the new detail group stands under a broad line, the body's Cross-cutting Patterns holding none for the rule.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [PTY] OPEN: the Windows reading of the wide character after a feed panic
**Section:** §Established Decisions [PTY] (established-decisions.md: a new `pending · dec-pty` group)
**Change:**
- A new OPEN, PROVISIONAL: on a Windows host, how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal after a contained vt100 feed panic under a 1-column ConPTY is not read; `tests/chaos_feed_panic.rs` asserts those bytes on Unix and, on Windows, only that no viola literal appears on the outer terminal. The reading and a Windows byte oracle from it are owed to "Windows-only live measurements".
**Why:** ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so the Windows byte reading was unrunnable here; the chaos case is green on every CI OS, Windows reading the literal absence only. The owner is the P5 card's Beyond-the-plan CARRY on «Windows-only live measurements», approved by delegate overseer1 (founder-delegated, relayed by the operator), PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [Screen Model] the poisoning episode's parse-rejected detail
**Section:** §Established Decisions [Screen Model] (established-decisions.md: `dec-screen-model`)
**Change:**
- Was «one `parse-rejected{parser:"vt100-feed", detail:"oversize"}` per poisoning episode» (for a caught panic and a dropped message alike); now one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, its `detail` `panicked` for a caught panic and `oversize` for a message the full queue drops. The rest of the entry is unchanged.
**Why:** a spec claim disproved by measurement: the code writes `panicked` for a caught panic and `oversize` for a dropped copy, each asserted by a unit case of `src/run/gate.rs`; the security-plan and obs-plan details already said so.
**Kept:** the body's [Screen Model] entry, which names no `detail`, stands unchanged.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — [MCP] the viola mcp stdin line cap
**Section:** §Established Decisions [MCP] (architecture.md: the body entry; established-decisions.md: three new `dec-mcp` groups)
**Change:**
- The body [MCP] entry gains: `viola mcp`'s stdin is read under a `MAX_FRAME` line cap, each line's newline counted; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, with one `parse-rejected{parser:"mcp-stdin"}` line holding no byte of it; the bound sits in the reader viola hands rmcp, because rmcp 3.4.1's stdio transport reads each line unbounded.
- Detail: the reader is the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` bytes per line, handed to rmcp as the reader of its `(reader, writer)` stdio transport (the path `transport-io` already enables; rmcp's features stay `server` + `transport-io`). A line of exactly the cap passes whole; at the end of stdin a held unterminated line within the cap is released; a longer line releases no byte, sets the reader's cut flag and then reads as end of stream. The wrapper channel's close-on-oversize rule is its precedent.
- Detail: after a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns `Ok(())` once the session initialized (exit 0, as at a closed stdin) or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.
- Detail: rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` bounds only the writer side (as read in rmcp 3.4.1's crates.io source); `CappedLines` signals a cut as end of stream, never as an I/O error, so rmcp writes nothing of its own.
**Why:** rmcp's unbounded `read_until` on `viola mcp`'s stdin was named this chunk's own subject (overseer1's ruling under the founder's night word). A truncated line would dispatch a padded valid request and a skipped one would leave the peer's request unanswered with no signal, so the line is refused whole. A cut before `initialize` keeps what a closed stdin before `initialize` gives (an implement deviation). No exit code, tool, method or error variant was added.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the mcp verb's session end and its mcp.ndjson line
**Section:** §Occupied Resources (occupied-resources.md: `res-binaries`, the subcommands line; `res-filesystem`, the `diagnostics/` `mcp.ndjson` list)
**Change:**
- The hidden `mcp` serves the five tools until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`) (was: until stdin closes). The subcommands line's text from «`verify` takes» onward moved, unchanged, to a continuation line.
- `mcp.ndjson` gains one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line); `process-exit` is written on stdin EOF or at that cut (was: on stdin EOF).
**Why:** the cut's exits are `mcp`'s existing closed-stdin outcomes, no exit code added; `schemas/diag-line.v1.json`'s `parse-rejected.parser` enum gains `mcp-stdin`, and `serve` writes the line after the session ends.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the serde_json unbounded_depth ban and the deny probe count
**Section:** §Stack and Technologies (stack-and-technologies.md: the Serialization row); §Infrastructure Patterns → Build system (registry key `architecture/build-system.md`: the Dependency policy bullet)
**Change:**
- The Serialization row's purpose adds: serde_json's `unbounded_depth` feature is banned in `deny.toml` (Build system), so every parse of external bytes keeps serde_json's default recursion limit.
- The Dependency policy's feature bans gain `serde_json/unbounded_depth`, its probe row `serde_json-unbounded-depth`.
- The probe verdict: was `deny-probes: 13/13 banned, control clean`, per «2026-10-17-mcp-server-for-drivers — the Tokio client and viola-channel's tree-form ban»; now `deny-probes: 14/14 banned, control clean`, the count one per `probes=(` row of `scripts/deny-probes.sh` plus the planted `channel-tokio-edge`.
**Why:** `deny.toml` gains a `[[bans.features]]` table denying serde_json's `unbounded_depth`, and `scripts/deny-probes.sh` gains its probe row, measured `14/14` with a clean control; the count now carries the rule that produced it, and every master site of it moved in this pass.
**Kept:** the Serialization row's versions: no dependency was added or bumped.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
