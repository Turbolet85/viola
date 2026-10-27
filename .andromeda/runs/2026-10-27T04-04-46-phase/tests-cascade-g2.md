# tests · cascade read · group 2 of 8

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The chunk's unit-level witness per property and per boundary, and the root bin's `send` / `answer` refusal-ordering cases, must be inline unit cases on all three OSes, the refusal-order matrices held as rstest `#[case]` tables.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay.
  (.andromeda/test-plan.md:289)
  > - **Unit:** inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices, on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
- (.andromeda/test-plan-detail/test-strategy.md:11) The vt100 panic degrade the chunk re-verifies has its gate.rs unit case among the known-cause reds of a plain `cargo test`, so its re-verification must run under nextest, never a plain `cargo test`.
  > **Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
- (.andromeda/test-plan-detail/test-strategy.md:14) The frame cap's `-32600`-and-close, the hook stdin fail-open and the oversize state-file cases must be crate-level integration cases over a real `interprocess` endpoint, a real tempfile home and the built `viola`, never a mocked disk.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:291)
  > - **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
  (.andromeda/test-plan.md:889)
  > - NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
- (.andromeda/test-plan-detail/test-strategy.md:17) Any case the chunk holds at the E2E layer (MCP `send`'s paste refusal through the rmcp client, the Path 1 WATCH read) must run against a harness-booted wrapper over the fake agent, and every parser, refusal-order or threshold case it adds must sit at a lower layer instead.
  > Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:293)
  > - **E2E:** required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
  (.andromeda/test-plan.md:871)
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan-detail/test-strategy.md:20) The chunk's property cases (`validate_paste_text` over decoded chars, the vt100 feed under `catch_unwind`, the hook stdin, statusline and `claude agents --json` parsers) must stand on the named parser surfaces with `proptest-regressions/` committed, and a parser the chunk bounds joins the fuzz target list only as it lands, under the corpus replay gate.
  > Required. proptest 1.11.0 over all 6 parser surfaces named by the property-test trigger, with `proptest-regressions/` committed. cargo-fuzz 0.13.2 targets run as a time-boxed ubuntu nightly job, plus a corpus replay gate: today `viola_name` (the pre-parser seed target), `channel_frame` (the wrapper channel's ndjson framing), `hook_stdin` (the `viola hook` stdin parser), `vt100_feed` (the vt100 feed under `catch_unwind`), `paste_text` (`validate_paste_text`) and `claude_agents_json` (the `claude agents --json` parser, chunk 2026-10-16-unwrapped-sessions-on-the-board), then the remaining parsers as they land.
  (.andromeda/test-plan-detail/test-strategy.md:66)
  > | property-test | property | §4 proptest; §9 fuzz job |
  (.andromeda/test-plan.md:295)
  > - **Property-based:** required — proptest over the parser surfaces with `proptest-regressions/` committed, plus cargo-fuzz targets in a time-boxed nightly job and a corpus replay gate.
  (.andromeda/test-plan.md:952)
  > - NEVER run property or chaos tests without deterministic replay: proptest seeds persist in committed `proptest-regressions/`, and chaos faults come only from explicit process / file operations or existing mockall seams (§2 Property-based and Chaos rows).
- (.andromeda/test-plan-detail/test-strategy.md:26) The forced vt100 panic of `tests/chaos_feed_panic.rs` and the torn or over-long ndjson line must come only from explicit process and file operations or the existing `viola-pty` mockall seam, never a failpoint crate.
  > Required (chaos trigger present). Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
  (.andromeda/test-plan-detail/test-strategy.md:67)
  > | chaos-test | chaos | §6 Chaos suite |
  (.andromeda/test-plan.md:299)
  > - **Chaos / Fault:** required — faults come from process and file operations and from mockall faults on the `viola-pty` seam.
  (.andromeda/test-plan.md:917)
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan-detail/test-strategy.md:29) Each bound the chunk adds to a reader found without one is a guard whose neutralisation a witness must catch, and a witness that starts a product process must spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`; the chunk adds no mutation gate of its own.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
  (.andromeda/test-plan.md:301)
  > - **Mutation:** required at the epoch boundary through `/andromeda-code-audit` and on demand through `run --mutants`, never in the default run or `--all`; there is no per-chunk, pre-push or CI mutation gate, and a missed mutant the harness does not leave out as host-excluded, a timed-out mutant, or more unviable than caught, fails the run.
- (.andromeda/test-plan-detail/test-strategy.md:58) The frame over `MAX_FRAME` refused `-32600` with a close, and the raw `link` / `unlink` frames with a bad name refused `-32602` with no line written, must be held at the IPC vector's integration and E2E layers in the homes it names.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC + Wrapper channel (the MCP impostor case, `crates/viola-mcp/tests/handlers.rs`); §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written) + Security sweep (the hook fail-open impostor rows, off macOS) |
- (.andromeda/test-plan-detail/test-strategy.md:59) The paste rule's re-verification (ESC, other C0, DEL and C1 refused `control-character` ahead of every other refusal, an empty typed text refused `empty-text`) must be held at a unit proptest, an integration case and an E2E case, at every client and authoritatively at the wrapper.
  > | Paste injection (V2) | unit (proptest) + integration + E2E | §4 viola-core; §6 Scenario 2 |
- (.andromeda/test-plan-detail/test-strategy.md:60) The oversize and malformed hook stdin cases, `hook statusline` included, must be assert_cmd integration cases against the built `viola hook`, exit 0 with empty stderr and no body, and rows of the security sweep's fail-open matrix.
  > | Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:61) The MCP refusals the chunk witnesses (a non-`ViolaName` `target`, a non-`u64` `dialog_id`, an open-string `behavior`, control characters in `text`, each `invalid-params` with no frame written) must be traced to this trigger's E2E layer through the rmcp client and its security sweep, beside the crate-level `handlers.rs` cases the scope names.
  > | MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:62) Path-traversal names (`../x`, uppercase, `/`, empty) refused before a path join for CLI positionals and `VIOLA_NAME`, and the oversize `answer` JSON, must be unit cases in `viola-core` and the root bin plus integration cases.
  > | CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
- (.andromeda/test-plan-detail/test-strategy.md:63) The over-long ndjson line counted in `skipped.torn_lines`, the three skip counts that never panic and each oversize state file read by its failure rule must be integration and chaos cases, and a reader's own unit case may plant an oversize or malformed `budget.json`, `snapshot.json` or `ledger/stamps.json` only by altering the writer's own bytes.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 (root `chaos_snapshot_replay.rs`: steps 1–3)
  (.andromeda/test-plan.md:315)
  > - `budget.json` comes only from `viola hook statusline` stdin in every booted-wrapper test; a reader's own unit cases may plant a malformed one only by altering the writer's own bytes (§11 Test Data)
  (.andromeda/test-plan.md:908)
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap, where `budget_read_*` landed in that form).
- (.andromeda/test-plan-detail/test-strategy.md:64) The oversize and malformed `claude agents --json` reads (read as `unknown`) and the forced vt100 panic refusing `send` `input-not-ready` while passthrough continues must be integration and E2E cases driven through the fake agent, never the real `claude`.
  > | Child spawn (V8) | integration + E2E | §5 process boundary; §6 Scenario E2 |
  (.andromeda/test-plan.md:311)
  > - CI never touches the real `claude`
  (.andromeda/test-plan.md:874)
  > - NEVER run the real `claude` CLI or `viola verify` against the real CLI in CI.
- (.andromeda/test-plan-detail/test-strategy.md:69) The paste rule re-verified at both clients must refuse alike at CLI `send` and MCP `send`, traced to this trigger's E2E layer and its cross-surface parity.
  > | cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
- (.andromeda/test-plan-detail/test-strategy.md:75) The Path 1 WATCH must be tallied with no retry and no moved kill line, a recurrence keeping the owner question open, and the vt100 passthrough witness must take its verdict from a signal other than the parsed child screen.
  > | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
  (.andromeda/test-plan.md:309)
  > - no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
  (.andromeda/test-plan.md:899)
  > - NEVER parse the rendered child screen (the vt100 grid, PTY master output) for content. Verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (R7).
  (.andromeda/test-plan.md:951)
  > - NEVER add retry-once policies or any retry budget for flaky tests (§10 Zero-flakiness budget).

## Bindings
- **test-strategy (property layer) ↔ unit-test-strategy / e2e-test-strategy**: the property-test trigger's layer (.andromeda/test-plan-detail/test-strategy.md:20) is filled by the `validate_paste_text` decoded-char property (.andromeda/test-plan.md:377) and the Property suite's list at `cases: 512` (.andromeda/test-plan.md:618); a parser the chunk bounds that has no fuzz target joins only as it lands.
- **test-strategy (property layer) ↔ ci-integration**: a fuzz target the chunk adds joins the corpus replay on push and PR and the nightly time-boxed run (.andromeda/test-plan.md:758), from `fuzz/`, never the root workspace (.andromeda/test-plan-detail/test-strategy.md:20).
- **test-strategy (IPC and integration layers) ↔ unit-test-strategy**: the frame cap is held at the codec's `MAX_FRAME` boundary unit case (.andromeda/test-plan.md:380) and at the IPC vector's integration layer (.andromeda/test-plan-detail/test-strategy.md:58), ± 1 by the property suite (.andromeda/test-plan.md:618).
- **test-strategy (hook stdin and MCP triggers) ↔ e2e-test-strategy**: the hook stdin (.andromeda/test-plan-detail/test-strategy.md:60) and MCP (.andromeda/test-plan-detail/test-strategy.md:61) triggers both route to the Security sweep, which holds the hook fail-open matrix's oversize stdin and malformed JSON rows and the MCP `invalid-params` cases with no frame written (.andromeda/test-plan.md:601).
- **test-strategy (chaos and child-spawn layers) ↔ e2e-test-strategy**: the forced vt100 panic is a chaos fault from an explicit seam (.andromeda/test-plan-detail/test-strategy.md:26) whose passthrough verdict reads no child screen (.andromeda/test-plan.md:899).
- **test-strategy (discipline) ↔ quality-gates-coverage-targets**: the Path 1 WATCH (.andromeda/test-plan-detail/test-strategy.md:75) is ruled by the zero-flakiness budget, `retries = 0` and no `#[ignore]` parking, the chunk red until a root cause is fixed (.andromeda/test-plan.md:854).
- **test-strategy (mutation) ↔ the remove-the-guard rule**: each added bound is a guard owed a bounded witness through `Running` within `WITHIN` (.andromeda/test-plan-detail/test-strategy.md:29), while the mutation run itself stays the epoch boundary's (.andromeda/test-plan.md:301).
