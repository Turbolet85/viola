proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 54
    section: "## 1. Test Scope Summary"
    change: "Signal gains, for the forced vt100 panic chaos case, a byte search of the outer terminal's drained stream: the child's own bytes (`e4 b8 ad`) arrive on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on every OS; a byte search, never a parse of the rendered child screen for content"
    sidecar: "§1 tui surface Signal: the outer terminal byte search named for the feed-panic case (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces: `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs:109-130`) asserts the child's `e4 b8 ad` on the outer terminal on Unix and the five literals' absence on every OS (the literal loop `:123-130`); the Signal list names only events, the receipt and exit detection"
    basis: "tests/chaos_feed_panic.rs:123-130"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 70
    section: "## 1. Test Scope Summary"
    change: "add after the As landed (the instance check) item: **As landed (the stdin line cap):** `viola mcp` reads its stdin through a line reader capped at `MAX_FRAME`; a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{mcp-stdin, oversize, count:1}` WARN line holding no byte of it (exit 0 on an initialized session); driven E2E by `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`"
    sidecar: "§1 MCP stdio surface: the stdin line cap's as-landed entry (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Symbols / APIs (`viola_mcp::serve` over `CappedLines::new(stdin, MAX_FRAME)`, `reject_oversize_line` `lib.rs:93-102`), Decisions & corrections (refused whole, never truncated or skipped) and Coverage of new surfaces (e2e `mcp_tools.rs:441-559`)"
    basis: "crates/viola-mcp/src/lib.rs:93-102"
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 30
    section: "## 4. Unit Test Strategy"
    change: "add after the pinned copy's placement item: The bounded compare (`fs::replace_private_shared`'s fallback through the private `holds_exactly`): `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` asserts the compare reads the target through `take(bytes.len() + 1)`, never the whole file, its outcome a whole-file compare's on every OS for an equal, a longer, a shorter, a different and an absent target; the read-only-dir refusal of a longer target runs on Unix only (Windows a read-only file)"
    sidecar: "§4 viola-state: the bounded compare read of `replace_private_shared` and its unit case (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Symbols / APIs (`holds_exactly`, `fs.rs:359-368`), Coverage of new surfaces (unit `fs.rs:385-419`), Deviations 5 (the case's per-OS shape)"
    basis: "crates/viola-state/src/fs.rs:385-419"
    fact: F5
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 12
    section: "## 6. E2E Test Strategy"
    change: "the tui row's verdict sources read: Verdicts come from `events.ndjson`, the fake-agent receipt, the exit status and, for the forced vt100 panic chaos case, a byte search of the drained master reader (the child's own bytes on Unix, viola's literals absent on every OS), never a parse of the rendered screen"
    sidecar: "§6 tui driver row: the drained master reader's bytes join the verdict sources for the feed-panic case (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces and Outcome (a11y): `tests/chaos_feed_panic.rs:109-130` reads the outer terminal for `e4 b8 ad` (Unix) and for `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` (every OS), a verdict source the row does not list"
    basis: "tests/chaos_feed_panic.rs:109-130"
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 14
    section: "## 6. E2E Test Strategy"
    change: "the MCP row's notes gain: Beside the rmcp client cases, `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` writes raw stdin lines (earlier requests, then one line past `MAX_FRAME`) from its own writer thread to a `viola mcp` child it spawns and owns, its stdout and stderr drained on their own threads, its exit awaited under an 8 s kill deadline, never a sleep"
    sidecar: "§6 MCP driver row: the raw stdin writer of the oversize case named beside the rmcp client (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report New text, by line (`mcp_tools.rs` 455-466 `Command::new`, 470-479 the out and err threads, 481-498 the writer thread, 500-510 the status loop) and Decisions & corrections (the `mcp_tools` child kills at 8 s); the row names only the rmcp client and the replay writer"
    basis: "crates/viola-e2e/tests/mcp_tools.rs:441-559"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 246
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add after the MCP schema-violating calls: The MCP stdin line past `MAX_FRAME`: `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`) starts from a fresh home of its own and sends a line past `MAX_FRAME` after earlier requests: exit 0, stderr empty, stdout only the earlier frames, exactly one `parse-rejected{mcp-stdin, oversize, count:1}` line valid against `schemas/diag-line.v1.json` and holding no byte of the line, the canary in no home-level file; green on the three CI OSes"
    sidecar: "§6 Security sweep: the MCP stdin line-cap negative (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Outcome (security, the `viola mcp` line past `MAX_FRAME` criterion, green on all three CI OSes), Coverage of new surfaces (e2e `mcp_tools.rs:441-559`, PII: the canary read in every home-level file), New text (`mcp_tools.rs` 540-548 the schema validation, 551-558 the file scan)"
    basis: "crates/viola-e2e/tests/mcp_tools.rs:441-559"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 246
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add after the MCP stdin line-cap negative: A known coverage gap: an MCP stdin cut that lands before `initialize` completes ends with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; no test drives it"
    sidecar: "§6 Security sweep: the uncovered pre-initialize MCP stdin cut recorded as a coverage gap (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 1)"
    rationale: "Report Deviations 1 (driven by no test; inputs#I4 item 4) and Outcome (security: a cut before `initialize` exits 11, outside the criterion); D-tests-coverage holds a new path with no test as drift, recorded here as the named gap"
    fact: F8
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 336
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "forced vt100 panic mode: `send` exits 13 with `input-not-ready`, human passthrough still reaches the receipt (a typed key reaching the child), and the outer terminal receives the child's own bytes (`e4 b8 ad`, Unix) with none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` on it (every OS) — `chaos_feed_panic_refuses_send_and_keeps_passthrough`"
    sidecar: "§6 Chaos suite: the feed-panic passthrough asserted on the outer terminal (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces (`tests/chaos_feed_panic.rs:109-130`) and Outcome (a11y: no viola literal on every OS, the child's `e4 b8 ad` on Unix, `send` refused `input-not-ready` exit 13, a typed key reaching the child); Expected amendments name `e2e-test-strategy.md:336` a change candidate"
    basis: "tests/chaos_feed_panic.rs:109-130"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 336
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add under the forced vt100 panic item: On Windows the case asserts the literal absence only; its byte oracle for the panicking wide character is owed to the route entry «Windows-only live measurements», a PROVISIONAL CARRY, because ConPTY re-renders a wide character on a 1-column screen"
    sidecar: "§6 Chaos suite: the feed-panic case's Windows byte oracle recorded as owed (chunk 2026-10-27-bounded-inputs-at-every-boundary; P5 card CARRY, inputs#I3)"
    rationale: "Report Coverage of new surfaces (byte reading `unrunnable-here` on Windows), Decisions & corrections (ConPTY re-renders a wide character on a 1-column screen) and inputs#I3's Beyond-the-plan CARRY on «Windows-only live measurements», PROVISIONAL"
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 368
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add: `tests/contract_bounded_readers.rs` (E2E layer, `run --e2e`) holds a `BOUNDARIES` table, one row per external reader naming its bound and the existing test that witnesses it, and an `EXCEPTIONS` table naming the own-executable reads as no external input; its tests are `boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, `census_finds_an_unbounded_read_and_passes_a_bounded_one` and `serde_json_unbounded_depth_is_banned_in_deny_toml`"
    sidecar: "§6 Contract suite: the bounded-reader enumeration `contract_bounded_readers` (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Files (new `tests/contract_bounded_readers.rs`), Harness / gate surface (a `contract_` binary under `run --e2e`), Coverage of new surfaces; the table counts (40 / 2) are the code's rows, a count no master states (CLAUDE.md learning: a master names the functions, the code's rows are the count)"
    basis: "tests/contract_bounded_readers.rs:659-774"
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 368
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add: The bounded-reader census walks the product roots (carrying the `andromeda:walks-tree` token): a reader call passes when a `take(` bounds it within its own statement, or when an `EXCEPTIONS` needle matches anywhere on the call's whole source line; its self-check plants an unbounded read and asserts it reads as a hit; with the `fs.rs` compare bound undone the census read red at `crates/viola-state/src/fs.rs:354`"
    sidecar: "§6 Contract suite: the bounded-reader census rule and its control (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Deviations 4 (the exception needle matched against the whole source line), Decisions & corrections (the `andromeda:walks-tree` token; the census hazard) and Outcome (the census red at exactly `fs.rs:354` with step 5 undone, `evidence/controls.md`)"
    basis: "tests/contract_bounded_readers.rs:620-648"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 368
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "add: A known test-depth gap: five `BOUNDARIES` rows name a witness that reads the bounded read but drives nothing past its cap — the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the three `config.json` readers (the thresholds, the dialog hold, `claude_env_keep`); for these only the needle row and the census hold the cap"
    sidecar: "§6 Contract suite: the five boundary rows whose witness never drives past the cap recorded as a test-depth gap (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 2)"
    rationale: "Report Deviations 2 and Coverage of new surfaces (`statusline_shell_out_passes_the_output_through_byte_for_byte`, `run_bounded_returns_the_exit_and_stdout`, the config readers' type-rule tests), routed by inputs#I4 item 4 to an owner or a test-plan note through the P2 detectors"
    fact: F7
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "the deny-probes clause reads: `bash scripts/deny-probes.sh` (a permanent negative probe per ban + a clean control, last line `deny-probes: 14/14 banned, control clean`: the thirteen bans one probe each, plus probe `channel-tokio-edge` …), the rest of the clause unchanged"
    sidecar: "§9 Supply-chain: the deny-probes verdict 13/13 → 14/14 and the twelve bans → thirteen (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Counts / qualifiers moved, pattern `probe-count`: `deny-probes: 13/13 banned, control clean` → `14/14` (the rule: the `probes=(` rows of `scripts/deny-probes.sh`, now 13 at lines 20–32, plus the planted channel probe `channel-tokio-edge`), `test-plan-detail/ci-integration.md:15` named a changed site, also «the twelve bans one probe each» → thirteen"
    basis: "scripts/deny-probes.sh:32"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: "the Runs clause names the feature ban: `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans, the bans holding a `[[bans.features]]` table denying serde_json's `unbounded_depth`), and the deny probes carry the row `serde_json-unbounded-depth`, firing `error[feature-banned]` against that ban"
    sidecar: "§9 Supply-chain: the serde_json `unbounded_depth` feature ban and its deny probe row (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Schema / config (`deny.toml:58-62`, `[[bans.features]]` `crate = serde_json`, `deny = [unbounded_depth]`) and Harness / gate surface (`scripts/deny-probes.sh:32`, its diagnostic `error[feature-banned]`); Outcome (`cargo deny check` clean, `deny-probes: 14/14 banned, control clean`)"
    basis: "deny.toml:58-62"
    fact: F6
note: "F4 (viola-mcp's six `stdin::tests` `capped_lines_*` cases and the two `serve` child cases) has no picked carrier (no viola-mcp unit group picked) — the new-detail pass places it; the body lines (test-plan.md:227, :616, :794, §4 viola-mcp / viola-state, §6 Contract suite) are fanout-test-plan.md's; D-tests-framework and D-tests-obs-harness: no drift (nextest, a `contract_` binary under the existing `run --e2e`, no harness verb, status shape or log format changed)."
