proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 23
    section: ## Established Decisions
    change: In [Screen Model], `one `parse-rejected{parser:"vt100-feed", detail:"oversize"}` per poisoning episode` reads `one `parse-rejected{parser:"vt100-feed"}` per poisoning episode, its `detail` `panicked` for a caught panic and `oversize` for a dropped message`; the rest of the line is unchanged.
    sidecar: established-decisions [Screen Model] — a caught vt100 panic logs `parse-rejected` detail `panicked`, a message the full queue drops `oversize` (was: `oversize` for both), as measured at chunk 2026-10-27-bounded-inputs-at-every-boundary.
    rationale: The report's Spec claims disproved by measurement names established-decisions.md:23 — the code writes `panicked` for a caught panic (src/run/gate.rs:49, asserted at :443) and `oversize` for a dropped copy (src/run/gate.rs:72, asserted at :487); plan-carried under Expected amendments, architecture [Screen Model].
    basis: src/run/gate.rs:49
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 106
    section: ## Established Decisions
    change: Add after line 106 a paragraph line of [MCP] (two-space indent) — As landed (chunk 2026-10-27-bounded-inputs-at-every-boundary): `viola mcp` reads stdin through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` per line, its `\n` counted, handed to rmcp as the reader of its `(reader, writer)` transport (which `transport-io` already enables; no rmcp feature change). A line of exactly the cap passes whole; a longer line releases no byte to rmcp, sets a cut flag and then reads as end of stream, so the line is refused whole and the session ends, never truncated or skipped (the wrapper channel's close-on-oversize rule its precedent, §Standard Contracts).
    sidecar: established-decisions [MCP] — the `viola mcp` stdin line cap at `MAX_FRAME` through `CappedLines`, a longer line refused whole and ending the session (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Symbols / APIs (`viola_mcp::stdin::CappedLines`, `viola_mcp::serve`) and Decisions & corrections (refused whole, never truncated or skipped; end of stream, never an I/O error); Expected amendments name [MCP]'s stdin bound as new text in its own body.
    basis: crates/viola-mcp/src/stdin.rs:17
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 106
    section: ## Established Decisions
    change: Add after the F1 line a paragraph line of [MCP] (two-space indent) — After a session a cut ended, `viola_mcp::serve` writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN through `obs_event!`, holding no byte of the line, then returns what the session gave: `Ok(())` (exit 0, as at a closed stdin) once initialized, or `ServeError::Initialize` (exit 11) when the cut lands before `initialize`; that second path is driven by no test, its owner given at the chunk's P5.
    sidecar: established-decisions [MCP] — the one `parse-rejected{mcp-stdin, oversize, count:1}` line after a cut and `serve`'s exits (0 once initialized, 11 before `initialize`) (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Symbols / APIs (`serve`'s private `reject_oversize_line`, lib.rs:93-102, and its returns) and Deviations from intent 1 (the pre-`initialize` cut, driven by no test, owner given at P5).
    basis: crates/viola-mcp/src/lib.rs:93
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 106
    section: ## Established Decisions
    change: Add after the F2 line a paragraph line of [MCP] (two-space indent) — The bound on rmcp's input belongs to the reader viola hands rmcp, never to the codec: rmcp 3.4.1's stdio `AsyncRwTransport::receive` reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` serves only the writer side (as read at that chunk's P3 in the cargo registry).
    sidecar: established-decisions [MCP] — why the stdin bound sits in viola's reader: rmcp 3.4.1 reads stdin lines unbounded and its codec's max length serves only the writer (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Cross-project / external claims (rmcp 3.4.1, `src/transport/async_rw.rs` lines 125–140) and Decisions & corrections (a bound on rmcp's input belongs to the reader viola hands it).
    basis: viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md:28
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 20
    section: ## Stack and Technologies
    change: In the Serialization row's purpose cell append — ; serde_json's `unbounded_depth` feature is banned by a `deny.toml` `[[bans.features]]` table, so every parse of external bytes keeps serde_json's default recursion limit, and `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires.
    sidecar: stack-and-technologies Serialization — serde_json `unbounded_depth` banned in `deny.toml` with a firing deny probe (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Schema / config (`deny.toml` :58-62, its stated reason) and Harness / gate surface (`scripts/deny-probes.sh` :32 row `serde_json-unbounded-depth`); no dependency added or bumped, so the row's versions stand.
    basis: deny.toml:58
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 17
    section: ## Established Decisions
    change: Add after line 17 a continuation line of [PTY] (one-space indent) — OPEN, PROVISIONAL (chunk 2026-10-27-bounded-inputs-at-every-boundary): on a Windows host, how the child's panicking wide character (`e4 b8 ad`) reaches the outer terminal after a contained vt100 feed panic under a 1-column ConPTY is not read; `tests/chaos_feed_panic.rs` asserts those bytes on Unix and on Windows only that no viola literal appears. Owner: the working-route entry "Windows-only live measurements" (the P5 card's CARRY).
    sidecar: established-decisions [PTY] — OPEN: the Windows ConPTY reading of the child's wide character after a contained feed panic, owed to "Windows-only live measurements" (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Coverage of new surfaces (the tui passthrough after a feed panic, Windows byte reading `unrunnable-here`) and Decisions & corrections (ConPTY re-renders the child's output; a wide character on a 1-column screen has no guaranteed rendering), carried by inputs#I3's CARRY on «Windows-only live measurements».
    basis: tests/chaos_feed_panic.rs:109
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 29
    section: ## Occupied Resources
    change: In the `mcp` sentence, `it serves the five tools over stdio until stdin closes` reads `it serves the five tools over stdio until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`)`; the line then passes 2 000 chars, so its text from `` `verify` takes`` onward moves to a continuation line after it.
    sidecar: occupied-resources res-binaries — the hidden `mcp` also ends at a stdin line past `MAX_FRAME`, as at a closed stdin (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Symbols / APIs (`serve`'s doc comment now reads until stdin closes or a stdin line passes `MAX_FRAME`; exit 0 once initialized, `ServeError::Initialize` exit 11 before) and Untouched (the `mcp` exits at a cut are its existing closed-stdin outcomes, no exit code added).
    basis: crates/viola-mcp/src/lib.rs:93
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 123
    section: ## Occupied Resources
    change: In the `mcp.ndjson` list, after `(the `config-json` rejections included)` insert `, one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN after a session a stdin line past `MAX_FRAME` ended (no byte of the line)`, and `process-exit` on stdin EOF reads `process-exit` on stdin EOF or that cut.
    sidecar: occupied-resources res-filesystem diagnostics/ — `mcp.ndjson` gains the one `parse-rejected{mcp-stdin, oversize, count:1}` WARN line (chunk 2026-10-27-bounded-inputs-at-every-boundary).
    rationale: Report Schema / config (`schemas/diag-line.v1.json` :87, `parse-rejected.parser` gains `mcp-stdin`) and Symbols / APIs (`reject_oversize_line` writes it through `obs_event!` after the session; `process-exit` 0 as at a closed stdin); the role file's registered line list does not name it.
    basis: schemas/diag-line.v1.json:87
note: Not proposed (no picked entity carries it): F5 (cross-cutting-patterns, the bounded-reader census) is the new-detail pass's; the [Snapshot writer] compare bound and the chaos-home entry's passthrough half (architecture.md:345) are Touches on unpicked entities; the deny probe count 13/13 → 14/14 sits in the registry key build-system.md:8; the body's [PTY] OPEN line (architecture.md:27) may name F6 beside its two opens.
