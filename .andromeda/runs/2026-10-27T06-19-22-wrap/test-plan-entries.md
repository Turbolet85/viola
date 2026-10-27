## 2026-10-27-bounded-inputs-at-every-boundary — the serde_json `unbounded_depth` ban and deny-probes 14/14
**Section:** §9 CI Integration (ci-integration.md: the Supply-chain stage's Runs; test-plan.md: Build failure conditions, Supply-chain errors)
**Change:**
- Supply-chain Runs: `deny.toml`'s bans now hold a `[[bans.features]]` table that denies serde_json's `unbounded_depth`. `scripts/deny-probes.sh` ends `deny-probes: 14/14 banned, control clean`: the thirteen bans one probe each, the new ban's row `serde_json-unbounded-depth` firing `error[feature-banned]`, plus probe `channel-tokio-edge` with its channel control. Was `13/13` and the twelve bans, per «2026-10-17-mcp-server-for-drivers — the CI gates: the channel tree form, rmcp features, deny-probes 13/13».
- Supply-chain errors: the deny-probes failure reads a line not ending `deny-probes: 14/14 banned, control clean` (was `13/13`).
**Why:** the chunk made serde_json's depth limit a `deny.toml` feature ban with a firing probe; `cargo deny check` is clean and the probes read 14/14. The count's rule: the `probes=(` rows of `scripts/deny-probes.sh` (now 13) plus the planted channel probe. The Runs clause naming the ban and the new count were written as one edit of the line on the operator's answer at the P2 halt (the overseer as operator, founder-delegated): the contradiction read against the ban clause was the stale 13/13 count the same apply retires.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the feed-panic passthrough witnessed on the outer terminal
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the drivers table's tui row, the Chaos suite's forced vt100 panic mode; test-plan.md: the tui driver, the Chaos suite) · §1 Test Scope Summary (test-scope-summary.md: the tui surface's Signal; test-plan.md: the tui surface, the security-vector-coverage (child spawn) trigger)
**Change:**
- The forced vt100 panic case is `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs`): `send` exits 13 `input-not-ready`, a typed key reaches the child with nothing typed, and the outer terminal shows the child's own panicking bytes (`e4 b8 ad`) on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` on every OS. Was: human passthrough still reaches the receipt, with no outer-terminal reading.
- On Windows the case asserts the literals' absence only; the byte oracle for the panicking wide character is owed to the route entry "Windows-only live measurements".
- The tui surface's signal and the tui driver's verdict sources gain, for this case alone, a byte search of the outer terminal's drained master-reader stream, never a parse of the rendered child screen (was: `events.ndjson`, the receipt and exit detection only). The child-spawn trigger's forced-panic item and the Chaos suite line name the passthrough as witnessed on the outer terminal.
**Why:** the chunk's chaos test now reads the outer terminal, green on ubuntu, macOS and windows-2025. ConPTY re-renders the child's output and a wide character on a 1-column screen has no guaranteed rendering, so the Windows byte reading rides the P5 card's CARRY on «Windows-only live measurements», approved by delegate overseer1 and PROVISIONAL until the founder's own word.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the `viola mcp` stdin line cap's tests
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the ipc-internal (MCP stdio) row, the Security sweep) · §1 Test Scope Summary (test-scope-summary.md: the MCP surface's as-landed entries; test-plan.md: the security-vector-coverage (MCP) trigger) · §4 Unit Test Strategy (unit-test-strategy.md: a new viola-mcp group; test-plan.md: the viola-mcp bullet)
**Change:**
- §1: `viola mcp` reads stdin through `CappedLines` capped at `MAX_FRAME`; a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, exit 0, stderr empty, stdout only the earlier frames. The MCP trigger's required tests add this negative.
- §6: `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`) spawns and owns a `viola mcp` child from a fresh home, writes raw lines from its own writer thread (`initialize`, `notifications/initialized`, one line past `MAX_FRAME`, stdin left open), drains stdout and stderr on their own threads, awaits exit under an 8 s kill deadline: exit 0, stderr empty, stdout only the `initialize` response, exactly one such line in `mcp.ndjson`, valid against `schemas/diag-line.v1.json`, the line's canary in no file under the home.
- §4, a new viola-mcp group: `stdin::tests` hold a line of exactly the cap whole, its `\n` counted, and one byte more releasing no byte, setting the cut flag and reading as end of stream, at the literal `16 * 1024 * 1024` too; the count restarts after each newline; a split over-long line is cut, a split fitting one joined (the six `capped_lines_*` cases); the `serve` child cases, each killed at 5 s: `serve_ends_the_session_at_a_line_past_max_frame`, `serve_on_a_closed_stdin_is_the_initialize_error`.
**Why:** rmcp 3.4.1's stdio transport reads each line unbounded, so the bound belongs to the reader viola hands it. A line is refused whole: a truncated one could dispatch a padded request, a skipped one leaves a request unanswered. Green on the three CI OSes (ci#40458923095).
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded-reader enumeration in the Contract suite
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Contract suite; test-plan.md: the Contract suite)
**Change:**
- `tests/contract_bounded_readers.rs`, a `contract_` binary in the E2E layer (`run --e2e`), holds two literal tables, never the product's own list: `BOUNDARIES`, one row per external reader naming its bounded read, its cap, depth and type rule and the existing test that witnesses it; `EXCEPTIONS`, the reads of viola's own executable (`checked_exe`, `pin_exe`), no external input. Its tests: `boundaries_each_name_a_bounded_read_that_stands` (each row's read in its file, its witness a defined test, each exception a read that stands), `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one`, `serde_json_unbounded_depth_is_banned_in_deny_toml`.
- The census (its file carrying the `andromeda:walks-tree` token) walks the product source roots, comments, strings and test items left out: a reader call passes when `take(` stands in its own statement up to the call, or an `EXCEPTIONS` needle stands on the call's whole source line. Its self-check reads a planted unbounded read as a hit; with the `replace_private_shared` compare bound undone it reads red at exactly `crates/viola-state/src/fs.rs:354`.
**Why:** the chunk's criterion that every external reader is size-capped needs an enumeration no new reader slips past. The exception needle is matched against the whole line because the executable argument it names lies past the call's `(`, outside the statement.
**Kept:** neither master states the tables' row counts: a master names the functions, and the code's rows are the count.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — the bounded compare read of `replace_private_shared`
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-state; test-plan.md: the viola-state bullet)
**Change:**
- viola-state adds the bounded compare: `fs::replace_private_shared`'s fallback, through the private `holds_exactly`, reads the target through `take(bytes.len() + 1)`, never the whole file. `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` holds its outcome to a whole-file compare's on every OS for an equal, a longer, a shorter, a different and an absent target; the refusal of a longer target under a read-only directory runs on Unix only, Windows using a read-only file.
- test-plan.md's viola-state bullet names the bounded compare read (at most one byte past the bytes, its outcome a whole-file compare's).
**Why:** the compare read was the one product read of a home file with no cap; one byte past the compared bytes decides equality as a whole read does. The Windows shape matches the existing case's stand-in.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/

## 2026-10-27-bounded-inputs-at-every-boundary — two owed test gaps, owned by «Sanitised error surfaces»
**Section:** §4 Unit Test Strategy (test-plan.md: the viola-mcp bullet) · §6 E2E Test Strategy (test-plan.md: the Contract suite)
**Change:**
- §4 viola-mcp, not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ.
- §6 Contract suite, Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the same route entry, as a PREREQ (a cap-driving case for each row).
**Why:** both are the chunk's own deviations, which the wrap directive (the operator's request; the overseer, founder-delegated) asked be given an owner or a test-plan note. The next route entry takes both as a PREREQ by the standing rule that a missing test for a clause a chunk landed is folded into the next chunk.
**Kept:** each gap is stated once, in test-plan.md with its owner; the e2e detail's notes restating them were rejected as twins.
**Ref:** .andromeda/runs/2026-10-27T06-19-22-wrap/
