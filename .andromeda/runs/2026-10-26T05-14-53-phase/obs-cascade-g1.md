# obs · cascade read · group 1 of 2

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:412) The bumped tokio pin and lock must leave the obs dependencies Tokio-free in the sync crates, add no C-building crate and pass `cargo deny`, the security Vector 9 signal the chunk's acceptance re-runs.
  > - **Required telemetry signal:** The logger dependency is tracing-subscriber `>=0.3.20` (0.3.23 pinned by tests). Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
  (.andromeda/obs-plan-detail/telemetry-strategy.md:35)
  > | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
  (.andromeda/obs-plan.md:560)
  > Its non-default `toggle` feature (`VEIL_DISABLE_REDACTION` / `veil::disable()`) is **banned** by cargo-deny `[[bans.features]] crate = "veil" deny = ["toggle"]`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:243) The `cargo deny` advisories family the chunk re-runs must read no finding against tracing-subscriber, which stays at 0.3.23 above the RUSTSEC-2025-0055 floor while only tokio and its required companions move in the lock.
  > Version 0.3.23 satisfies the security Vector 9 floor of `>=0.3.20` (RUSTSEC-2025-0055).
  (.andromeda/obs-plan.md:135)
  > `tracing` with tracing-subscriber 0.3.23, `fmt().json().flatten_event(true).with_current_span(false)`
- (.andromeda/obs-plan-detail/obs-scope-summary.md:170) Sync instrumentation with no Tokio rests on the `cargo deny` ban, so the bump must keep the Tokio ban per sync crate (`deny-sync.toml` as sole root over `scripts/sync-crates.txt`) and the C-crate ban passing, with no ban changed to reach green.
  > Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:225)
  > `cargo deny` bans Tokio in the sync crates' graph and bans C-building crates (arch Observability Hints, Build policy).
  (.andromeda/obs-plan.md:131)
  > and `cargo deny` bans Tokio in the sync crates and C-building crates
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) `viola-channel`'s normal graph must stay Tokio-free without its `tokio` feature under the new tokio, as `scripts/channel-tokio-free.sh`, which the chunk re-reads and re-runs, witnesses.
  > - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan.md:57)
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on interprocess local sockets (sync client and server; Tokio client behind the `tokio` feature)
- (.andromeda/obs-plan-detail/obs-scope-summary.md:60) `viola-state` must stay Tokio-free under the bump, as read by the `deny-sync.toml` sole-root ban the chunk re-runs over `scripts/sync-crates.txt`.
  > - **Reason if not fully instrumentable:** N/A. Stack bans Tokio here, so instrumentation must be sync.
  (.andromeda/obs-plan.md:63)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Stack bans Tokio here.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:78) `viola mcp` is the Tokio-allowed surface where a source edit the bump forces (an `AsyncFd` deprecation read by clippy `-D warnings`) would land, and any such edit must write no telemetry to stdout.
  > - **Reason if not fully instrumentable:** N/A. stdout is reserved for MCP frames (arch Stack and Obs-Relevant Conventions), so no telemetry may go to stdout.
  (.andromeda/obs-plan.md:125)
  > - **Surface:** ipc-internal (`viola mcp`: stdio MCP server, Tokio allowed)
  (.andromeda/obs-plan.md:127)
  > stdout is forbidden because it carries MCP frames, so the writer is set explicitly
- (.andromeda/obs-plan-detail/obs-scope-summary.md:50) interprocess must stay at 2.4.4 under the lock change (no dependency but tokio and its required companions moves), the version whose tokio backend's `AsyncFd` use P3 measures as outside viola's clippy gate.
  > - **Source:** arch excerpt, Stack (interprocess 2.4.4), Workspace, Standard Contracts (IPC methods `send · wait · last · answer · pause · release · link · unlink · hook.dialog · hook.event`)
  (.andromeda/obs-plan-detail/span-trace-coverage.md:21)
  > none (interprocess 2.4.4 emits no `log`/`tracing`)
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40) The `supply-chain` job the chunk re-runs uploads `deny.json`, `deny-fuzz.json` and `zizmor.json` under this admissibility rule, which must keep holding on the bumped run: cargo-deny and zizmor diagnostics only, with no absolute path, argv, log content or user input.
  > - the ci.yml `supply-chain` artifact (`target/supply-chain/deny.json`, `deny-fuzz.json` for the `fuzz/Cargo.lock` audit, and `zizmor.json`; `if: always()`, 7 days) holds cargo-deny and zizmor diagnostics only: crate names and versions, advisory ids, registry URLs and workflow-relative locations. It carries no absolute path, argv, log content or user input (as measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes: 0 absolute paths in all three files; the one raw pattern hit was the `s:/` inside `https://`). The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
  (.andromeda/obs-plan.md:574)
  > - Unscanned uploads are admissible by content only: a member that could carry a host path or log content is scanned before upload, or dropped;

## Bindings
- **obs-scope-summary (Vector 9 required signal) ↔ telemetry-strategy (trigger coverage map)**: the Vector 9 signal the bump must keep true is mapped to the `cargo deny` gates the chunk re-runs, (.andromeda/obs-plan-detail/obs-scope-summary.md:412) ↔ (.andromeda/obs-plan-detail/telemetry-strategy.md:35)
- **obs-scope-summary (sync crates Tokio-free, `cargo deny` ban) ↔ ci-integration (Lint / typecheck row, group 2)**: the Tokio and C-crate bans the scope states are run by the lint row's `cargo deny check` and the Tokio-free `cargo check` of the sync crates, one re-run judging both, (.andromeda/obs-plan-detail/obs-scope-summary.md:170) ↔ (.andromeda/obs-plan-detail/ci-integration.md:22)
- **obs-scope-summary (`viola mcp` stdout reserved) ↔ obs-anti-patterns (print bans, group 2)**: a forced edit in the Tokio crate keeps stdout free of telemetry, enforced by the clippy `print_stdout` / `print_stderr` / `dbg_macro` deny lints the chunk re-runs, (.andromeda/obs-plan-detail/obs-scope-summary.md:78) ↔ (.andromeda/obs-plan-detail/obs-anti-patterns.md:8)
- **obs-scope-summary (interprocess 2.4.4) ↔ span-trace-coverage (channel spans)**: the interprocess version the lock leaves unmoved is the one whose tokio backend P3 measures and whose silence under the channel spans keeps holding, (.andromeda/obs-plan-detail/obs-scope-summary.md:50) ↔ (.andromeda/obs-plan-detail/span-trace-coverage.md:21)
- **pii-scrubbing-compliance (supply-chain artifact) ↔ obs-scope-summary (Vector 9)**: the `deny.json` the artifact admits is the `cargo deny` output whose passing is the Vector 9 signal, (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:40) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:412)
