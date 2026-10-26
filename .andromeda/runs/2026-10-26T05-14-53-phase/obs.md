# obs extract

## Relevance
partial: the chunk moves only a dependency pin and the lock (tokio, plus companions only where tokio forces them). It adds no span, event, field, metric or log line. Obs still applies in three places: its supply-chain trigger (Tokio containment, the obs crate bans in `cargo deny`), the CI gates that rule on the bump, and the `mcp` role's Tokio-runtime panic containment.

## Constraints
- obs-plan §1 Obs Scope Summary (trigger "logging-sensitive, security Vector 9 (supply chain)") requires that obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny` on all three targets. The bump must keep Tokio built only where it is built today (`viola-mcp`, `viola-channel`'s `tokio` feature, the test-only `viola-e2e`). Whether the new tokio or its companions pull Tokio into a sync crate's graph, or pull in a C-building crate, is research's question.
- obs-plan §9 CI Integration (stage table, Lint / typecheck row) requires a `cargo check` of the sync crates without Tokio, and requires `cargo deny check` to keep the veil `toggle` ban and the bans on `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender` and tracing-subscriber's `env-filter` feature. A companion crate the new tokio requires must not bring any of these in.
- obs-plan §10 SLO Invariants & Telemetry Budgets (Build / deploy failure conditions) makes these red: a `cargo deny` failure (Tokio, C-crate, veil `toggle`, banned obs crates), a clippy `disallowed-macros` / `print_stdout` / `print_stderr` / `dbg_macro` failure, and a `scripts/lint-probes.sh` failure. Under `-D warnings`, a deprecation the patched tokio adds (the `AsyncFd` safe constructors) in viola's own code would be one of these reds. Whether viola's own code calls them is research's question.
- obs-plan §7 Error Capture & Reporting (the `mcp` per-role panic behaviour) requires that a panic inside a spawned Tokio task is logged by the panic hook and contained at the task boundary, with the process continuing. A main-thread panic must reach the main-thread catch site and write `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`. The new tokio version must keep this behaviour. Whether a test already pins it under the moved runtime is research's question.
- obs-plan §3 Trace context propagation (key file) requires that `viola-mcp`'s spawned futures carry their span through `tracing::Instrument::instrument(span)` / `.in_current_span()`. The bump must not change that hand-off.
- obs-plan §11 Obs Anti-Patterns (§Logs) bans renaming a tests field or adding an `event` value without a Decisions Log entry. The scope's "no log line changes" is the obs reading of that ban for this chunk.
- obs-plan §8 PII Scrubbing & Compliance (unscanned uploads, admissible by content) requires that the ci.yml `supply-chain` artifact (`target/supply-chain/deny.json`, `deny-fuzz.json`, `zizmor.json`) holds only cargo-deny and zizmor diagnostics: crate names and versions, advisory ids, registry URLs and workflow-relative locations, with no absolute path, argv, log content or user input. The guard stays binding when that artifact's content changes.

## Patterns to follow
- Read the `supply-chain` job's verdict from its machine-readable artifact (`target/supply-chain/deny.json`) or from the job log through `gh`, never from a human summary (per obs-plan §2 Telemetry Strategy, agent-readable invariants: every verdict is a machine assertion that sets an exit code).
- Keep the logging stack's own dependencies untouched: tracing 0.1.44 (Tokio-free facade) and tracing-subscriber 0.3.23 (per obs-plan §3 Logging stack). A lock move that shifts either one is outside this chunk's "no other dependency is bumped" line, and research has to surface it.
- Prove a ban by a probe that fires, not by a silent pass: the cargo-deny obs bans are checked alongside `scripts/deny-probes.sh` (per obs-plan §9 CI Integration, stage table).

## Anti-patterns to avoid
- Never let an OTel SDK, OTLP exporter or `opentelemetry-stdout` arrive, transitively included, through a companion move. `opentelemetry-otlp`'s default client spins up a Tokio runtime (per obs-plan §11 Obs Anti-Patterns, §Telemetry Strategy).
- Never admit `sentry` or any reporter through the lock move. Its default `tokio` / `native-tls` transport breaks cargo-deny (per obs-plan §11 Obs Anti-Patterns, §Error Reporting).
- Never close a red by a re-run. Under `retries = 0` a re-run is no substitute, and the CI gates stay exit-code verdicts (per obs-plan §11 Obs Anti-Patterns, §CI).

## Contract bindings
- obs ↔ security: the `cargo deny` run that carries obs's bans (veil `toggle`, the opentelemetry / sentry / tracing-appender / `env-filter` bans) is the same run security-plan §Dependency Security governs, with its ignore list (RUSTSEC-2017-0008 only) and its no-widening rule. Obs adds bans to it and owns no advisory policy.
- obs ↔ tests: G2 (zero panics) and G4 (schema conformance) run over the integration and E2E homes under `target/e2e-home/`, the `mcp` homes included (test-plan §10 quality gates). They are the obs evidence that the moved Tokio runtime changed no logged behaviour.

## Acceptance criteria contributions
- `cargo deny check` at CI's pinned version passes with every obs ban still in force and still firing: `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender`, the tracing-subscriber `env-filter` feature and the veil `toggle` feature (per obs-plan §9 CI Integration).
- The sync crates still build without Tokio after the bump (`cargo check` of the sync crates without Tokio, `scripts/channel-tokio-free.sh`), and no C-building crate enters the graph (per obs-plan §1 Obs Scope Summary).
- G2 counts 0 `event:"panic"` lines and G4 counts 0 schema-conformance failures on the chunk's CI run. No `event` value or log field is added or renamed (per obs-plan §10 SLO Invariants & Telemetry Budgets).
- The `supply-chain` artifact uploaded on the chunk's run carries no absolute path, argv or log content (per obs-plan §8 PII Scrubbing & Compliance).
