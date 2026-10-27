## 2026-10-27-bounded-inputs-at-every-boundary — serde_json's `unbounded_depth` banned in `deny.toml`
**Section:** §Dependency Security (dependency-security.md: a new `values · deps` group; body: `deny.toml` additions, `[[bans.features]]`) · §Security Anti-Patterns (Input: the `unbounded_depth` ban)
**Change:**
- `[[bans.features]]` gains a row beside rmcp's: `serde_json` denies `unbounded_depth`, its reason that every parse of external bytes keeps serde_json's default recursion limit. The `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.
- The Input ban: was "NEVER enable serde_json's `unbounded_depth` feature."; now it adds "`deny.toml` bans it (§Dependency Security)".
**Why:** the chunk added the `deny.toml` table (`crate = "serde_json"`, `deny = ["unbounded_depth"]`) and its probe row, so a ban the plan only wrote is now enforced by the gate. No dependency was added or bumped, and `cargo deny check` stays clean.
**Kept:** the Channel frames line's "serde_json's default depth limit, never `unbounded_depth`" at §Input Validation, a true claim, stands as written.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the deny-probes verdict at 14/14, its counting rule named
**Section:** §Dependency Security (dependency-security.md: the arch-bans item, now in the `values · deps` group after the serde_json one)
**Change:**
- The proof clause: was "`13/13 banned, control clean`", per 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate and per 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-channel's Tokio client pin at tokio 1.53.3 (their other claims stand); now "`14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe", as measured at the chunk's report. The rest of the item stands as written.
**Why:** the new `serde_json-unbounded-depth` probe row brings the `probes=(` rows to 13, and the planted channel probe `channel-tokio-edge` makes 14; the chunk's gate and pre-push runs read `deny-probes: 14/14 banned, control clean`. A count in a master carries the rule that produced it, so the rule now stands beside the number.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — `viola mcp`'s stdin capped at `MAX_FRAME` per line, an over-long line ending the session
**Section:** §Input Validation (input-validation.md: a new Boundary entity, MCP stdin; the `values · input` Constants list; body: a new MCP stdin line after the MCP tool inputs line, the Constants line) · §Security Anti-Patterns (Input: the bounded-reader ban; API: the request size limit)
**Change:**
- New boundary, MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines): each line passes `viola_mcp::stdin::CappedLines` at `MAX_FRAME`, its `\n` counted, before rmcp reads it; a line of exactly the cap passes whole; a longer line releases no byte, is refused whole, never truncated or skipped, and the reader then reads as end of stream, so the session ends as a stdin close does; at the inner end a held unterminated line within the cap is released.
- On the cut: after the session ends `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` through `obs_event!`, holding no byte of the line, and returns exit 0 for an initialized session, `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.
- How: `CappedLines` (crate-private), handed to rmcp 3.4.1 as `(CappedLines::new(stdin, MAX_FRAME), stdout)` through the `(reader, writer)` transport `transport-io` already enables, no rmcp feature added.
- Both Constants lists name `viola mcp`'s stdin lines among `MAX_FRAME`'s capped reads.
- Input ban adds: NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session. API: was "`MAX_FRAME` on the channel"; now "on the channel and on each `viola mcp` stdin line".
**Why:** rmcp 3.4.1's stdio transport reads each line with an unbounded `read_until`, and `JsonRpcMessageCodec::new_with_max_length` bounds only the writer, so the bound sits on the reader viola hands rmcp, never on the codec. A truncated line could dispatch a padded valid request, and a skipped one would leave the peer's request unanswered with no signal; the channel's close-on-oversize rule is the precedent. The unbounded read was named this chunk's subject by overseer1's ruling under the founder's night word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the vt100 passthrough after a caught feed panic, witnessed on the outer terminal
**Section:** §Input Validation (input-validation.md: the PTY output bytes (vt100) entity, a new Witness clause)
**Change:**
- New clause: after a caught feed panic `chaos_feed_panic_refuses_send_and_keeps_passthrough` reads the outer terminal: on Unix the child's bytes (`e4 b8 ad`) reach it, and on every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears on it; the Windows byte reading is owed to "Windows-only live measurements".
**Why:** the chunk's test now asserts the passthrough on the outer terminal, green on ubuntu, macOS and windows-2025. ConPTY re-renders the child's output, and a wide character on a 1-column screen has no guaranteed rendering, so Windows reads the literal absence only. The byte reading is the P5 card's CARRY on "Windows-only live measurements", approved by delegate overseer1 (founder-delegated), PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded-reader census, the compare read bounded by its own length, the own-executable reads named
**Section:** §Input Validation (input-validation.md: the `values · input` Constants entity, three new clauses; body: the Constants line)
**Change:**
- Bounded by its own length, not `MAX_FRAME`: `replace_private_shared`'s fallback compare reads its target through `take(bytes.len() + 1)` (`holds_exactly`), never the whole file, its outcome a whole-file compare's for an equal, a longer, a shorter, a different and an absent target. The body's Constants line says so.
- Enumerated and censused: `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary's bounded read and its witness in `BOUNDARIES`; its census `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit.
- Not external input: viola's reads of its own executable for the pinned copy. The census's `EXCEPTIONS` rows name them, and the body's Constants list names them no external input.
**Why:** the compare read now stops one byte past the bytes it compares, with the same outcome as a whole-file compare. The census turns "every external reader capped" into a test over the product sources rather than a list kept by hand: with the compare-read change undone it reads red at that read. Hashing viola's own binary reads no external bytes, so those reads are named exceptions, not caps.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the MCP-over-stdio threat vector bounded at its stdin lines
**Section:** §Threat Model Summary (threat-model-summary.md: a new `values · boundary-mcp` group; body: the MCP over stdio vector line)
**Change:**
- New group, vector MCP over stdio, the driver's stdin lines: each stdin line reaches rmcp only through viola's `CappedLines` reader at `MAX_FRAME`, its `\n` counted: an over-long line from the driver is refused whole and ends the MCP session after the earlier lines are answered, never truncated, skipped or dispatched in part, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN holding no byte of it.
- The body vector line adds, after the input check: each stdin line capped at `MAX_FRAME` before rmcp reads it, an over-long line refused whole and ending the session, never dispatched in part.
**Why:** the driver, an LLM agent, writes `viola mcp`'s stdin, and rmcp reads those lines unbounded. The chunk measured a line past `MAX_FRAME` ending the session after the earlier lines are answered: exit 0, stderr empty, stdout only the earlier frames, one `parse-rejected` line, and no byte of the line in any home-level file, green on all three CI OSes.
**Kept:** the rest of the vector stands: the driver answers the driven session's PermissionRequest outside viola, and leaving `release`, `pause`, `link` and `unlink` off the tool list is an affordance, not an enforcement.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
