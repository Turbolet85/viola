# Task — test-plan · 2026-10-27-bounded-inputs-at-every-boundary

## The session
The session bounded `viola mcp` stdin: `serve` now hands rmcp a crate-private line reader, `CappedLines`, capped at `MAX_FRAME`. A line past the cap passes no byte and ends the session as a stdin close does. viola then writes exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line. It exits 0 on an initialized session and 11 (`ServeError::Initialize`) when the cut comes before `initialize`.
`replace_private_shared`'s fallback compare now reads its target through `take(bytes.len() + 1)` (new private `holds_exactly`) and no longer reads the whole file. The outcome is unchanged for an equal, longer, shorter, different or absent target.
`deny.toml` now bans the serde_json `unbounded_depth` feature, and the deny probes gain a firing row for it: the verdict moves from `13/13` to `14/14 banned, control clean`. `schemas/diag-line.v1.json` adds `mcp-stdin` to the `parse-rejected.parser` enum, which goes from 9 to 10 values.
Added: a new E2E contract binary `tests/contract_bounded_readers.rs` (a 40-row `BOUNDARIES` table, a 2-row `EXCEPTIONS` table, and a census of every product reader); an `mcp_tools` oversize-line E2E case; six `stdin::tests` unit cases and a `serve` child case; a bounded-compare unit case in `fs.rs`. `tests/chaos_feed_panic.rs` now also asserts that passthrough continues to the outer terminal.
Measured: CI `ci#40458923095` green, 15/15 checks on three OSes. Path 1's WATCH tally is now 1/3 green (15.010 s on ubuntu). Nothing was retired, and no dependency, tool version, harness verb, CI step, exit code, error variant, MCP tool or env var was added.

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
- §9 CI Integration build-failure line: the `scripts/deny-probes.sh` verdict changes from `deny-probes: 13/13 banned, control clean` to `14/14`. The same count appears in the ci detail's supply-chain values, where the wording "the twelve bans one probe each" now becomes thirteen.
- §9 Supply-chain job: cargo deny now carries a `[[bans.features]]` ban (crate serde_json, deny `unbounded_depth`) that its probe proves fires. This is a ban the supply-chain stage must list.
- §1 coverage trigger, security-vector-coverage (child spawn), and §6 Chaos suite: the "forced vt100 panic → `input-not-ready` with passthrough continuing" claim is now witnessed on the outer terminal. On Unix the child's bytes `e4 b8 ad` must arrive. On every OS no viola literal (`panic`, `parse-rejected`, `vt100`, `unable`, `hint:`) may appear. Scenario text that says passthrough continues may need this added.
- §1 coverage trigger, security-vector-coverage (MCP), and §6 Security sweep: the MCP negatives now include a stdin line past `MAX_FRAME`. It is refused whole and ends the session, never truncated or skipped. It is driven E2E by `mcp_tools`.
- §1 coverage trigger, property-test / §6 Property suite: framing at the `MAX_FRAME` ± 1 boundary now also covers the MCP stdin reader. It is held by literal unit cases (`16 * 1024 * 1024`), not by a proptest.
- §1 entity `viola-mcp`, and the surface ipc-internal (MCP stdio): the stdio server's input is now line-capped at `MAX_FRAME`. Entity or surface text that describes the MCP stdin path picks up this bound.
- §4 Unit, the viola-mcp bullet: until now it lists only handler re-validation and `tools/list`. The crate now has `stdin::tests` and `serve` child cases.
- §4 Unit, the viola-state bullet: the list of `fs` helpers covered gains `replace_private_shared`'s bounded compare read.
- §6 Contract suite (`contract_*.rs`): a new member, `contract_bounded_readers`, sits beside the fixture contract binaries. It runs in the E2E layer under `run --e2e`.
- §6 Schema conformance (obs owns the schemas, tests own the check): the diag-line schema's `parser` enum now admits `mcp-stdin`. The new `mcp_tools` case validates its line against `schemas/diag-line.v1.json`.
- §6 Error sanitization and secret scan: the new `mcp_tools` oversize case asserts that its canary appears in no home-level file, which shows that no byte of the over-long line reaches disk.
- §2 naming conventions: the new binaries follow the root `contract_` prefix and the `viola-e2e` `mcp_` prefix. A naming claim that lists the root contract binaries may need the new one added.
- §10 / §2 Zero-flakiness and deadline rules: the new spawning tests carry kill deadlines (the `serve` child at 5 s, the `mcp_tools` child at 8 s). Waits are bounded by the child's exit and never use a sleep.
- Indirect, the security trigger "frame over `MAX_FRAME`": that rule had covered only the wrapper channel. Its refuse-whole, close-the-session precedent now also covers MCP stdin, so a claim that `MAX_FRAME` bounds only the channel frame is no longer the whole story.
- Indirect, the test inventory: the gates now read `run --unit` 2575 passed, the integration filter 184 passed, `nextest-e2e` 49 + 11 passed and coverage 3251 passed. Only a master site that states a count is touched, and a master states no test-case count.

## New facts
- F1 · e2e-test-strategy · `tests/contract_bounded_readers.rs` (E2E layer) has a `BOUNDARIES` table of 40 rows. Each row names one external reader, its bound and the existing test that witnesses it. Its `EXCEPTIONS` table has 2 rows, the own-executable reads named as no external input. Its tests are `boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one` and `serde_json_unbounded_depth_is_banned_in_deny_toml`.
- F2 · e2e-test-strategy · The bounded-reader census walks the product roots (it carries the `andromeda:walks-tree` token). A reader call passes if it is bounded by a `take(` within its own statement, or if an `EXCEPTIONS` needle matches anywhere on the call's whole source line. A self-check plants an unbounded read and asserts it reads as a hit. With the `fs.rs` bound undone, the census went red at `crates/viola-state/src/fs.rs:354`.
- F3 · e2e-test-strategy · `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`) starts from a fresh home of its own. A `viola mcp` line past `MAX_FRAME`, sent after earlier requests, must give: exit 0, empty stderr, stdout holding only the earlier frames, and exactly one schema-valid `parse-rejected{mcp-stdin, oversize, count:1}` line holding no byte of the line. The case was green on all three CI OSes.
- F4 · unit-test-strategy · viola-mcp has six unit cases, `stdin::tests` `capped_lines_*`. A line of exactly the cap passes whole and one byte more is cut, including at the literal `MAX_FRAME`; the count restarts after each newline; an over-long line split across reads is cut, and a fitting one split across reads is joined. It also has the child cases `serve_ends_the_session_at_a_line_past_max_frame` and `serve_on_a_closed_stdin_is_the_initialize_error`.
- F5 · unit-test-strategy · `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` (viola-state) asserts that the compare reads at most `bytes.len() + 1`. Its outcome equals a whole-file compare on every OS for an equal, longer, shorter, different and absent target. The read-only-dir refusal of a longer target runs on Unix only.
- F6 · ci-integration · The deny probes have a row `serde_json-unbounded-depth`, which fires `error[feature-banned]` against the `deny.toml` serde_json `unbounded_depth` feature ban.
- F7 · e2e-test-strategy · A known test-depth gap: five `BOUNDARIES` rows name a witness that reads the bounded read but drives nothing past its cap. They are the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the three `config.json` readers (thresholds, hold, `claude_env_keep`). For these, only the needle row and the census hold the cap.
- F8 · e2e-test-strategy · A known coverage gap: an MCP stdin cut that lands before `initialize` completes ends with `ServeError::Initialize` (exit 11), the same as a closed stdin. No test drives it.
- F9 · e2e-test-strategy · `chaos_feed_panic_refuses_send_and_keeps_passthrough` asserts only the literal absence on Windows. Its Windows byte oracle for the panicking wide character is owed to the route entry «Windows-only live measurements», a PROVISIONAL CARRY, because ConPTY re-renders a wide character on a 1-column screen.

## Untouched
- §3 Test Harness Contract: no `agent-run` verb, flag, usage `detail`, session-record field or status shape changed. `contract_bounded_readers` runs under the existing `run --e2e`.
- §5's MCP adapter-level handler tests (`crates/viola-mcp/tests/handlers.rs`, `viola_mcp::Tools`) and the five-tool `tools/list`: no tool, handler or re-validation changed.
- Critical-path scenarios Paths 2 to 7, E1 to E5: no step or verification signal changed. The Path 1 WATCH tally lives on the working route, not in Path 1's text.
- §7 fixtures and the fake agent: no `fixtures/claude/` set, `Mcp.` exchange, fake script or fake-agent mode was added or recorded.
- The statusline pass-through rows (Path 6, the hook statusline entity, the budget governor) share only the word passthrough with the chaos change.
- §10 coverage thresholds, perf budgets, the mutation gate and the zero-flakiness rule are unchanged. Coverage passed at or above 85 / 95 / 80 per OS.
- §9 matrix, workflows, caches, the rmcp release-graph assertion (still `rmcp-features: server transport-io`), release-check, the ConPTY vendor checks and tool pins are unchanged. No dependency was bumped and `Cargo.lock` is untouched.
- §4's viola-core, viola-pty, viola-channel and viola-agent-claude unit lists and the root-bin unit list: no code under test in these crates changed.
- The exit-cause matrix and the security control negatives: no exit code, refusal or error variant was added.
