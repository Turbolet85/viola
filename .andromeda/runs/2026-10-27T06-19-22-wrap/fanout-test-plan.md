proposals:
  - detector: D-tests-count-literals
    severity: warning
    section: "§9. CI Integration (Build failure conditions, Supply-chain errors)"
    change: "`scripts/deny-probes.sh` not ending `deny-probes: 14/14 banned, control clean` (one probe per ban, plus the tree form's) — the verdict literal moves from 13/13 to 14/14 with the new `serde_json-unbounded-depth` probe row"
    sidecar: "§9 Supply-chain errors: the deny-probes verdict literal 13/13 → 14/14 (the serde_json `unbounded_depth` feature ban's probe row; chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Counts / qualifiers moved, pattern probe-count: `deny-probes: 13/13 banned, control clean` → `14/14` (rule: the `probes=(` rows of `scripts/deny-probes.sh`, now 13, plus the planted channel probe), `test-plan.md:794` named a changed site; Harness / gate surface names the new row"
    basis: ".andromeda/test-plan.md:794"
  - detector: D-tests-coverage
    severity: warning
    section: "§1. Test Scope Summary (Coverage triggers, security-vector-coverage (child spawn))"
    change: "forced vt100 panic → `not-delivered`/`input-not-ready` with passthrough continuing, witnessed on the outer terminal: the child's own bytes reach it (Unix) and no viola literal (`panic`, `parse-rejected`, `vt100`, `unable`, `hint:`) appears on it (every OS); the Windows byte reading is owed to «Windows-only live measurements»"
    sidecar: "§1 child-spawn trigger: the feed-panic passthrough now witnessed on the outer terminal (Unix bytes, every-OS literal absence; Windows byte reading owed; chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces: `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs:109-130`) asserts the child's `e4 b8 ad` on the outer terminal on Unix and the literal absence on every OS, the Windows byte reading `unrunnable-here` and owed by the P5 card's CARRY; Expected amendments name `test-plan.md:227` a change candidate"
    basis: ".andromeda/test-plan.md:227"
  - detector: D-tests-coverage
    severity: warning
    section: "§6. E2E Test Strategy (Non-path suites, Chaos suite)"
    change: "a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues, the passthrough read on the outer terminal (the child's own bytes on Unix, no viola literal on every OS; the Windows byte reading owed to «Windows-only live measurements»)"
    sidecar: "§6 Chaos suite: the feed-panic case's passthrough named as asserted on the outer terminal (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Same claim as the §1 child-spawn trigger, restated in the Chaos suite; report Expected amendments name `test-plan.md:616` a change candidate; Coverage of new surfaces, `tests/chaos_feed_panic.rs:109-130`"
    basis: ".andromeda/test-plan.md:616"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6. E2E Test Strategy (Drivers per surface, tui)"
    change: "tui (`viola run` passthrough): verdicts from `events.ndjson`, the receipt and the exit status, and for the feed-panic chaos case the drained master reader's bytes — a byte search for the child's own bytes and for viola's literals' absence, never a parse of the rendered screen"
    sidecar: "§6 tui driver: the outer terminal's bytes join the verdict sources for the feed-panic case (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces and Outcome (a11y): the chaos case now reads the outer terminal for the child's `e4 b8 ad` (Unix) and for `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` (every OS), a verdict source the tui driver line does not list"
    basis: "tests/chaos_feed_panic.rs:109-130"
  - detector: D-tests-coverage
    severity: warning
    section: "§1. Test Scope Summary (Surface: tui)"
    change: "its signal is `events.ndjson`, the fake agent's receipt, exit detection on the process handle and, for the feed-panic case, a byte search of the outer terminal (the child's own bytes, viola's literals absent); assertions never parse the rendered child screen for content"
    sidecar: "§1 tui surface: the outer terminal byte search named as a signal of the feed-panic case (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Same claim as the §6 tui driver line (the tui verdict sources), restated in the §1 surface entry; report Coverage of new surfaces, `tests/chaos_feed_panic.rs:123-130` (the literal loop)"
    basis: "tests/chaos_feed_panic.rs:123-130"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1. Test Scope Summary (Coverage triggers, security-vector-coverage (MCP))"
    change: "add: a `viola mcp` stdin line past `MAX_FRAME` is refused whole and ends the session — the earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, one `parse-rejected` `mcp-stdin` / `oversize` WARN line holding no byte of it (`mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`)"
    sidecar: "§1 MCP trigger: the stdin line cap's negative test added (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Symbols / APIs (`viola_mcp::serve` over `CappedLines::new(stdin, MAX_FRAME)`), Coverage of new surfaces (e2e `mcp_tools.rs:441-559`) and Outcome (security, the MAX_FRAME criterion): a new MCP boundary negative the Vector 5 trigger does not list"
    basis: "crates/viola-e2e/tests/mcp_tools.rs:441-559"
  - detector: D-tests-coverage
    severity: warning
    section: "§4. Unit Test Strategy (What unit tests cover, viola-mcp)"
    change: "add: the stdin line cap `CappedLines` — a line within the cap passes whole, a line of exactly the cap (its newline counted) passes, one byte more releases nothing, sets the cut flag and reads as end of stream, across any split of the reads, the cap pinned at the literal `16 * 1024 * 1024` (the `capped_lines_*` cases) — and `serve` ending the session at a line past `MAX_FRAME` (`serve_ends_the_session_at_a_line_past_max_frame`)"
    sidecar: "§4 viola-mcp: the stdin line cap's unit cases and the serve child case named (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Coverage of new surfaces: unit `stdin::tests` `capped_lines_*` (`stdin.rs:188-240`) and the child case `serve_ends_the_session_at_a_line_past_max_frame`; the §4 viola-mcp list names only handler re-validation and `tools/list`"
    basis: "crates/viola-mcp/src/stdin.rs:188-240"
  - detector: D-tests-coverage
    severity: warning
    section: "§4. Unit Test Strategy (What unit tests cover, viola-mcp)"
    change: "add a note: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does — this path is driven by no test (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 1), its owner the one this wrap gives"
    sidecar: "§4 viola-mcp: the uncovered pre-initialize stdin cut recorded as a test gap (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 1; inputs#I4 item 4)"
    rationale: "Report Deviations 1: the cut before `initialize` keeps exit 11 and is driven by no test; D-tests-coverage holds a new path with no test as drift; inputs#I4 item 4 asks an owner or a test-plan note"
  - detector: D-tests-coverage
    severity: warning
    section: "§4. Unit Test Strategy (What unit tests cover, viola-state)"
    change: "add: `replace_private_shared`'s fallback compare reading the target through `take(bytes.len() + 1)`, never the whole file, its outcome a whole-file compare's for an equal, a longer, a shorter, a different and an absent target (`replace_private_shared_reads_no_more_than_one_byte_past_the_bytes`)"
    sidecar: "§4 viola-state: the bounded compare read of `replace_private_shared` named (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Symbols / APIs (`holds_exactly`, `fs.rs:359-368`) and Coverage of new surfaces (unit `fs.rs:385-419`); the §4 viola-state list does not name the compare read's bound"
    basis: "crates/viola-state/src/fs.rs:385-419"
  - detector: D-tests-coverage
    severity: warning
    section: "§6. E2E Test Strategy (Non-path suites, Contract suite)"
    change: "add: the bounded-reader enumeration `tests/contract_bounded_readers.rs` — its `BOUNDARIES` table names each external reader's bounded read and a witness test, its `EXCEPTIONS` the reads that take no external input, a census over the product sources reads every reader bounded in-statement or named (`boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, its self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`), and `serde_json_unbounded_depth_is_banned_in_deny_toml`"
    sidecar: "§6 Contract suite: the bounded-reader enumeration and census named (chunk 2026-10-27-bounded-inputs-at-every-boundary)"
    rationale: "Report Files (new `tests/contract_bounded_readers.rs`), Harness / gate surface (a `contract_` binary under `run --e2e`) and Coverage of new surfaces; Expected amendments name test-plan §5 / §6 for the enumeration (search `enumeration` test-plan 0, new text)"
    basis: "tests/contract_bounded_readers.rs:659-774"
  - detector: D-tests-coverage
    severity: warning
    section: "§6. E2E Test Strategy (Non-path suites, Contract suite)"
    change: "add a note: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap — their caps are held by the needle row and the census alone, a test-depth gap (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 2)"
    sidecar: "§6 Contract suite: the boundary rows whose witness never drives past the cap recorded as a test-depth gap (chunk 2026-10-27-bounded-inputs-at-every-boundary, deviation 2; inputs#I4 item 4)"
    rationale: "Report Deviations 2 and Coverage of new surfaces list these rows (`statusline_shell_out_passes_the_output_through_byte_for_byte`, `run_bounded_returns_the_exit_and_stdout`, the three config readers' type-rule tests) as reading the bound without driving past it; deviation 2 routes it to an owner or a test-plan note through the P2 detectors"
note: "Not proposed (cascade's standing groups): `test-plan-detail/ci-integration.md:15` (13/13 → 14/14 and «the twelve bans one probe each» → thirteen), `e2e-test-strategy.md:336` (passthrough), the suite-contract and MCP detail. D-tests-framework and D-tests-obs-harness: no drift (nextest, `contract_` binary under `run --e2e`; no agent-run verb, status shape or log format changed). Deviation 1's owner, if the wrap gives a route owner at P5 instead of the §4 note, retires that note proposal."
