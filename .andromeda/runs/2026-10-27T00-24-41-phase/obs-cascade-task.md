# Task — obs · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the race where two first `viola run`s (or a `run` and a `viola verify`) of one version on a fresh home both find `bin/<key>/viola(.exe)` absent and both replace it, so that no agent's hook spawn of the pinned path fails while another start's write is in flight.
The fix changes only how concurrent absent-copy writes behave in `viola_state::pin::pin_bytes` / `pin_exe` (via `replace_private_shared` → `replace_private`). The hash-before-reuse rule, the per-start `plugin/<key>/` rewrite and the home-tree strict-modes check keep their rules.
A two-sided witness: the race reproduced red before the fix (the unstamped two-instance harness boot, or a narrower concurrent-pin control) and clean after it on the same rig, beside the single-instance control.
A measurement with no product change unless the founder rules at P4: two clean release builds of one committed tree (each from an empty target dir, through `scripts/release-check.sh`'s build), compared by file hash and by ELF section and note, recording only measured differences.
Re-reads CI `ci#40425866096` on `99977116` at Setup, still in progress there, never read as green.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm)
- crates/viola-state/src/fs.rs
- src/cmd/run.rs · edits the start's pin_bytes call, pin_companions (the Windows x64 conpty/ arm, if P3 finds it shares the race)
- src/cmd/verify.rs · edits the pin_exe call
- scripts/release-check.sh · re-reads its release build (the release profile and CARGO_TARGET_DIR it uses)
- viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/unstamped-boot-race.md · re-reads the measured 6-of-24 / 0-of-24 race
- viola-0.1.0/chunks/2026-10-09-inner-cr-and-crlf-in-a-sent-text/evidence/live-preconditions.md · re-reads the prior build-hash measurement

## Needs
- The `viola run` start sequence to child spawn scenario: the pin step sits inside it. The `pinned_bin` path, endpoint and companion paths or hashes appear in no span or line, and a sideload outcome is never a refusal or an exit change.
- The start refusal for a tampered pinned copy stays the fixed two-line stderr pair through `src/human.rs` `refuse`, with no path or pid. The `human::refuse(` site counts the plan records (`run` 7, `verify` 6) change if the fix adds or removes a refusal site.
- Any new or changed exit-1 cause on the absent-copy write (for example a lost race that cannot be settled) gets a distinct closed kebab-case `detail` code on `process-exit`, with no path or pid in it, recorded in the `detail` code catalog.
- Vector 7 refusal telemetry for a pinned-exe hash mismatch: the existing line shape and level stay unchanged when a concurrent start finds a present copy and re-hashes it.
- A typed error that crosses the `viola-state` seam from the pin write (an io error on rename or open) is logged once at WARN or ERROR with a closed code. `StateError` keeps its fixed `Display`, and the full chain goes only to the instance `detail-<process>.ndjson`.
- Instrumentation added in `viola-state` stays sync and Tokio-free: the sync-crate ban over its graph.
- `pin_companions`' existing `ERROR_SHARING_VIOLATION` wait and the `sideload_fallback` values on `process-start{subject:"claude-child"}`, including `strict-modes-failed`, stay as they are if the exe's Windows arm takes the same wait.
- The hook surface the race breaks: a `viola hook` spawn of the pinned path that fails gives receipt `ran:false` and no `session-start`. Hook fail-open stays as it is: exit 0, stderr empty, `hook-<name>.ndjson` only, a silent no-op on a refused or absent instance.
- `viola verify`'s logging when it pins: `process-start` / `process-exit` go to `cli-<name>.ndjson` only with `VIOLA_NAME` set. Its home-tree check runs before the log, and a refused tree writes no line.
- `plugin/<key>/` rewritten whole at every start through `replace_private_shared`: if P3 finds a concurrent rewrite can fail a reader of `hooks.json` / `.mcp.json`, that failure is observed only as the hook's or mcp's own fail-open or exit outcome, never as a new line holding a path.
- The two-sided witness runs through the test harness (`scripts/agent-run.sh` boot / status / logs). `status`'s `state` / `last_error` closed enum and the merged `logs` output are where a failed boot and a missing `session-start` are read.
- Deterministic emission: the race witness waits on hook events and `events.ndjson` offsets, never on sleeps or screen content.
- Zero-flakiness: a race reproduction has no retry-once policy, and a red it shows stays open until its cause is known.
- `service.version` is compile-time `env!("CARGO_PKG_VERSION")`. Any build change the measurement motivates (a linker flag, a path remap, a profile key) must still give a `panic_location` of the form G2's single exemption compares as a whole string (`src/cmd/hook/seam.rs:<digits>`). It must also inject no CI or env metadata into a product line.
- Every Cargo profile keeps `panic = "unwind"` (G3) under any profile key the reproducible-build measurement leads the founder to change.
- The release build measurement uses the release artifact only. A build-id note or symbol difference it records goes in the chunk's evidence, never into a log line.

## Standing conventions
- S1 · each new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, carrying only fields named in the Section 6 catalog and passing `schemas/diag-line.v1.json` (default-deny, G4) · §6 Log Coverage / .andromeda/obs-plan-detail/log-coverage.md
- S2 · a new `event` value or `detail` code needs an Obs Decisions Log entry, and a `detail` code holds no path or pid · §11 Logs, §12 / .andromeda/obs-plan-detail/log-coverage.md
- S3 · a new span is named `<area>.<operation>` in snake_case with a static name and an explicit field allow-list, never bare `#[instrument]`, `err` or `ret` (G1) · §4 / .andromeda/obs-plan-detail/span-trace-coverage.md
- S4 · every new integration or E2E test home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 / .andromeda/obs-plan-detail/ci-integration.md
- S5 · new test output counts toward the zero budgets: 0 unlogged panics (G2), 0 schema-conformance failures (G4), 0 secret-scan hits · §10 / .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
- S6 · a new CI step (for example a clean-build comparison) is its own `run:` step with `shell: bash` on all three OSes, uses Actions pinned by SHA, and keeps the G2 / G4 / secret-scan `if: always()` ordering with uploads gated on the scan · §9 / .andromeda/obs-plan-detail/ci-integration.md
- S7 · no `print!` / `eprintln!` / `dbg!` in product code paths, and no raw tracing level macro or `event!` outside `viola_core::obs` · §11 Logs / .andromeda/obs-plan-detail/obs-anti-patterns.md

## Not needed
- The Budget governor scenario, the statusline shell-out and `budget.json`: the chunk touches no budget gate or status line.
- The Confirmed `send` (CL-1) scenario, `COMPACT_CONFIRM_WINDOW` and `text_bytes`: no send path changes.
- The `wait` / `last` readback scenario and its server-verify-failed line: no frame client changes.
- The Dialog → `answer` scenario, `hook-invoked` `corr` timing and `dialog_id`: no dialog path changes.
- The wheel / `release` scenario and `release-from-driver`: no wheel change.
- The channel `corr` + `conn` join, `channel-request` / `channel-response` and `-32603`: no channel method is added or changed.
- The `viola mcp` surface: `mcp.ndjson`, its stderr fallback and `isError` codes.
- E3 `link` / `unlink`, E5 snapshot replay (`state-recovered` `snapshot-replayed`) and torn-line healing: no state reader or appender changes.
- Heartbeat liveness transitions and the stale-heartbeat chaos class.
- The capability ledger read in `run.version_gate` and `cli_verified`: the stamps are untouched.
- Derived metrics (`viola.compat.v_rejected` and the cardinality discipline): no metric is added.
- veil `Redact` on channel, hook and MCP payload types: no payload type is added.
- An external error reporter (sentry) or an OTel SDK / OTLP exporter: none is in scope.
- `viola list`'s colour facts (`NO_COLOR` / `TERM` / `COLORTERM`) and the `claude agents --json` parse.
