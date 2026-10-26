# test-plan · Test Strategy
Read whole. The master's broad view is .andromeda/test-plan.md; this file holds its Test Strategy detail.

## 2. Test Strategy

<!-- detail: values · strategy -->

- **Level:** Unit
  - **Coverage focus:** Pure functions, isolated module logic
  - **Tier-specific depth (Comprehensive, viola):** Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
  - **Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
- **Level:** Integration
  - **Coverage focus:** Cross-module + IPC + disk state + process boundaries (no DB exists)
  - **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- **Level:** E2E
  - **Coverage focus:** Critical user-facing paths via surfaces
  - **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
- **Level:** Property-based
  - **Coverage focus:** Invariant assertions over random inputs
  - **Tier-specific depth (Comprehensive, viola):** Required. proptest 1.11.0 over all 6 parser surfaces named by the property-test trigger, with `proptest-regressions/` committed. cargo-fuzz 0.13.2 targets run as a time-boxed ubuntu nightly job, plus a corpus replay gate: today `viola_name` (the pre-parser seed target), `channel_frame` (the wrapper channel's ndjson framing), `hook_stdin` (the `viola hook` stdin parser), `vt100_feed` (the vt100 feed under `catch_unwind`), `paste_text` (`validate_paste_text`) and `claude_agents_json` (the `claude agents --json` parser, chunk 2026-10-16-unwrapped-sessions-on-the-board), then the remaining parsers as they land.
- **Level:** Performance / Load
  - **Coverage focus:** Budget assertions on perf-critical paths
  - **Tier-specific depth (Comprehensive, viola):** Required (performance-budget trigger present). hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10). The heartbeat staleness flip at >5 s is checked by `viola_state::liveness::classify`, a pure function fed the beat's age (no clock).
- **Level:** Chaos / Fault
  - **Coverage focus:** Failure injection (process / disk / clock)
  - **Tier-specific depth (Comprehensive, viola):** Required (chaos trigger present). Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
- **Level:** Mutation
  - **Coverage focus:** Test-suite strength on changed code
  - **Tier-specific depth (Comprehensive, viola):** Required (Founder Direction 1), at the epoch boundary: `/andromeda-code-audit` runs cargo-mutants 27.1.0 per unit, `run --mutants` [`--file`] runs it `--in-diff`, and `run --mutants --package <member>` runs it over one whole member (the boundary-tier form), both through `--test-tool=nextest` on demand (named only, never in the default or `--all`). No per-chunk, pre-push or CI mutation gate. Any `missed` outcome the harness does not leave out as host-excluded (§3 `run` step 4, Verdict), any `timeout` outcome, or more `unviable` than `caught` outcomes (`unviable-exceeds-caught`), makes that `run` fail.
- **Level:** Contract
  - **Coverage focus:** Fake agent vs recorded `viola verify` fixtures: the spine, the dialog tier and the framing tier
  - **Tier-specific depth (Comprehensive, viola):** Required (contract-test-against-sandbox trigger). insta 1.48.0 in check mode pins decision bodies (the dialog tier). The dialog-tier fixtures are recorded by live `viola verify --record` Runs C and D as `fixtures/claude/<ver>/<Event>.<stem>-<n>.json` variants (stems `questions`, `parallel`, `permission`, `plan`; 12 under each of `2.1.288`, `2.1.287` and `2.1.302`), scrubbed and `unclean`-checked like the spine. The framing-tier fixtures are recorded the same way by Run B's five added pastes as `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` (stems `paste`, `compact`, `clear`; six under each of `2.1.287` and `2.1.302` — `paste-1`, `paste-2`, `paste-3`, `compact-1`, `clear-1` ×2 — none under `2.1.288`); `contract_fake_agent_drift` also replays the compact variant byte for byte once resumed into its own session (`fake_agent_compact_replay_matches_every_recorded_compact_set`). They supersede `2.1.287`'s four fixtures relayed from the viola-lab prototype (the founder's live ruling, 2026-10-04), which stay committed with their provenance under `RELAYED.md`'s dated "Superseded" section.
 The fake agent's hook sequences are not snapshots: `contract_fake_agent_drift` compares each hook's stdin bytes (the receipt's `stdin_hex`) byte for byte with the recorded `fixtures/claude/<ver>/<Event>.default.json`, the spine order a test literal, and replays every dialog variant byte for byte under `--dialogs` and every framing variant byte for byte under `--framing`, and compares the `statusline` receipts' `stdin_hex` byte for byte, in order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing` (`fake_agent_statusline_replay_matches_every_recorded_statusline_set`, Unix; the synthetic `statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay`); a set that holds no framing variant is run without `--framing` and must exit 1 (its `local-command-clear` row fails on the echo), its dialog tier still compared byte for byte. jsonschema 0.57.0 checks that every `fixtures/claude/*/StatusLine.*.json` status line payload conforms to `schemas/claude-statusline-fixture.v1.json`, every `fixtures/claude/*/Agents.*.json` agents fixture to `schemas/claude-agents-fixture.v1.json`, every `fixtures/claude/*/Mcp.*.json` MCP exchange to `schemas/claude-mcp-fixture.v1.json`, every other `fixtures/claude/*/*.json` hook payload to `schemas/claude-fixture.v1.json`, and every `Screen.*.json` screen fixture to `schemas/claude-screen.v1.json`; `contract_fake_agent_drift` also compares the key and type shape of the fake agent's `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`, chunk 2026-10-16-unwrapped-sessions-on-the-board).

<!-- /detail -->

<!-- detail: values · naming -->

- **Directory pattern:** the arch uses flat `crates/viola-<area>` plus the root bin, with no declared test crate. This plan adds one test-only crate; see Decisions Log.
  - **Unit tests:** inline `#[cfg(test)] mod tests` at the bottom of each source file.
  - **Crate-level integration tests:** `crates/viola-<area>/tests/<topic>.rs`.
  - **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`. `events.rs` holds the one `events.ndjson` reader and its wait (`events`, `wait_events`) and the `boot` that waits for the session-start record; `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code, and `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions. `fake.rs` holds `empty_bin`, `claude_bin` and `claude_in`, the one place a test plants the fake agent as `claude` (chunk 2026-10-16-unwrapped-sessions-on-the-board). No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.
  - **Tokio-client E2E suites and the harness binary `viola-harness`:** `crates/viola-e2e/` (publish = false). This covers the rmcp client, cross-surface parity and critical-path scenarios, as `tests/{path<N>,mcp,cross}_<topic>.rs` (a numbered Path binary `path<N>_<topic>.rs`, e.g. `path6_budget_harness.rs`). The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).
  - **Why a test-only crate:** `crates/viola-e2e` hosts `viola-harness` and the tokio-based test clients (rmcp client), so the root package and the sync crates carry no tokio dev-dependency. It is never a root of the tokio ban: the ban runs `deny-sync.toml` once per crate in `scripts/sync-crates.txt`, each as the sole root, so `viola-e2e`'s tokio graph lies outside it with no `wrappers` allowlist and no `--exclude` (an `--exclude` false-fails on a feature-unified optional tokio).
  - **Fake agent:** `src/bin/viola-fake-agent.rs`, a root-package `[[bin]]` with `required-features = ["fake-agent"]`.
  - **Fuzz targets:** `fuzz/fuzz_targets/<parser>.rs`.
  - **Recorded fixtures:** `fixtures/claude/<cli-version>/`.
  - **trycmd cases:** `tests/cmd/*.toml`.
  - **insta snapshots:** `tests/snapshots/` and `crates/*/src/snapshots/`.
  - **Harness shims:** `scripts/agent-run.{sh,ps1}`.

<!-- /detail -->

<!-- detail: values · strategy -->

| Test-scope Sec 5 trigger | Primary layer(s) | Plan location |
|---|---|---|
| IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) |
| Paste injection (V2) | unit (proptest) + integration + E2E | §4 viola-core; §6 Scenario 2 |
| Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
| MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
| CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
| Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
| Child spawn (V8) | integration + E2E | §5 process boundary; §6 Scenario E2 |
| Supply chain (V9) | CI gates | §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`) + `nightly.yml` weekly advisories (root and fuzz lockfiles) |
| property-test | property | §4 proptest; §9 fuzz job |
| chaos-test | chaos | §6 Chaos suite |
| performance-budget | perf | §10 Performance budgets |
| cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
| multi-os-compat | CI matrix | §9 Matrix builds |
| multi-version-compat | unit + contract | §4 viola-agent-claude; §7 fixtures |
| contract-test-against-sandbox | contract | §6 Contract suite; §7 |
| error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
| mutation testing | mutation | §3 `run` step 4; §10 |
| discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |

<!-- /detail -->
