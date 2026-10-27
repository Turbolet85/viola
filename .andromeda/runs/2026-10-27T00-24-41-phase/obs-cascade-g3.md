# obs · cascade read · group 3 of 5

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) Any new exit-1 cause the fix gives the absent-copy write (a lost race that cannot be settled) must surface as a distinct `detail` code on `process-exit`, and a pin-write error crossing the `viola-state` seam must keep its fixed `Display`.
  > typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit`
  (.andromeda/obs-plan-detail/telemetry-strategy.md:38)
  > | chaos-instrumentation | §6 `state-recovered` / `process-exit` `exit_source` + `during`, §10 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:33) When a concurrent first start finds the pinned copy present and re-hashes it, the refusal on a difference must stay the existing `pinned-hash-mismatch` filesystem refusal, and if `pin_companions` changes on Windows x64 its `sideload_fallback` `strict-modes-failed` outcome must stay a degrade with the exit unchanged.
  > §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged)
- (.andromeda/obs-plan-detail/telemetry-strategy.md:43) A `detail` code the fix adds must be recorded in the detail code catalog under D-01, and `run`'s and `verify`'s home-tree refusal, made before their logs open and ahead of the edited pin calls, must keep writing no line and carrying no `detail` code.
  > §6 detail code catalog, §12 D-01; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) The two-sided race witness must read the failed boot and the missing `session-start` from the process role files merged with `events.ndjson` by `agent-run logs`, and its red and clean verdicts must each be an assertion that sets an exit code.
  > **The primary signal.** tracing-subscriber 0.3.23 JSON-per-line, one file per process role under `<home>/diagnostics/`, merged with `events.ndjson` by `agent-run logs`
  (.andromeda/obs-plan.md:286)
  > No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:45) The race witness, before and after the fix, must decide on hook events and `events.ndjson` offsets only, never on the child's screen content.
  > | creator-explicit (no screen scraping) | §4 Scenario 2 (vt100 gate outcome only), §11 |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:35) Any lock, dependency or instrumentation the fix adds to `viola-state` must stay inside the logging stack's feature set and pass the cargo-deny gates and the sync-crate Tokio ban, and a build change the reproducible-build measurement leads the founder to make must leave `service.version` the compile-time package version.
  > | logging-sensitive Vector 9 (supply chain) | §3 Logging stack (feature set), §9 cargo-deny gates, §11 |
  (.andromeda/obs-plan.md:297)
  > `service.version` = `viola_core::VERSION` = `env!("CARGO_PKG_VERSION")`.

## Bindings
- **Errors (`detail` codes on `process-exit`) ↔ log-coverage's detail code catalog and the Obs Decisions Log**: a lost-race exit cause on the pin write is one closed code, recorded in the catalog under D-01, holding no path or pid, (.andromeda/obs-plan-detail/telemetry-strategy.md:12) with (.andromeda/obs-plan-detail/telemetry-strategy.md:43)
- **Vector 7 filesystem refusals ↔ security-plan's hash-before-reuse rule**: a present copy is re-hashed and refused on difference, never overwritten, and its telemetry keeps the `pinned-hash-mismatch` shape and level, (.andromeda/obs-plan-detail/telemetry-strategy.md:33)
- **Structured logs ↔ test-plan's harness (`agent-run.sh` boot / status / logs)**: the race witness's red and clean readings come from the merged logs and `status`, by exit-code assertion and never by screen, (.andromeda/obs-plan-detail/telemetry-strategy.md:11) with (.andromeda/obs-plan-detail/telemetry-strategy.md:45)
- **Vector 9 supply chain ↔ CI integration's cargo-deny and sync-crate gates, and the reproducible-build measurement**: a `viola-state` dependency or a build-profile change stays gated and keeps the compile-time version, (.andromeda/obs-plan-detail/telemetry-strategy.md:35) with (.andromeda/obs-plan.md:297)
