# obs extract

## Relevance
Relevant. The chunk builds the `viola-pty` seam and the `run` spawn and exit path, which obs-plan §1 lists as an instrumentable entity. It also carries the R8 never-log floor and touches the CI install step that gate G1 depends on. Channel, snapshot, heartbeat and UI parts of the plan are out of scope.

## Constraints
- **`run` must not print to the terminal.** obs-plan §3 (harness contract intro, Logging stack → Sink) says `run` writes nothing to the terminal except the child's own bytes. No stdout sink on any role, and `with_writer` must always be explicit. This matches the chunk's "zero own bytes" rule. §7 Panic hooks requires `viola_panic_hook` to be the first statement in `main`, never calling the default hook, and live before `pty.spawn` (§3 OTel SDK init, per-role anchors).
- **What spawn and exit must log.** obs-plan §4 Scenario 1 and §6 Boundary-call wrappers → PTY seam:
  - on a successful spawn, `process-start{subject:"claude-child", child_pid, pty_backend:"conpty", env_stripped_count, env_stripped_known}`;
  - child exit only from the handle-wait thread, as `process-exit{subject:"claude-child", child_exit_status, exit_source:"handle-wait"|"kill-fallback"}`, never on reader EOF;
  - then `process-exit{subject:"self", exit_code, duration_ms}`.

  Whether the existing `log_child_exit` / `exit_source` code already emits these exact fields is research's question.
- **Refusal and spawn-failure codes.** A `.cmd` / `.bat` child exits 1 with `detail:"batch-script-child"`. obs-plan §6 detail code catalog, §4 Scenario 1 (`run.pin_copy` outcome) and D-01 (Founder Direction 4) require codes only, with no path or pid. A ConPTY or spawn `Err` returned from the seam becomes `process-exit{exit_code:1, detail:"internal-error"}`, with the chain only in `detail-run.ndjson` (obs-plan D-23, §7 scrubbing layer 3).
- **R8 logging and path hygiene.** Per obs-plan §8 Data classification, §11 PII Scrubbing and D-14:
  - The strip may log only `env_stripped_count`, plus `env_stripped_known` with names taken from the compile-time known list. Unknown `CLAUDE*` names are counted, not named.
  - Values never appear in any file, detail files included.
  - Env maps are never passed to a span or event.
  - The resolved shim / `.exe` path follows the Medium rule for absolute paths: never in home-level lines (§8; `pinned_bin` is never logged).
- **How spans are declared.** obs-plan §4 intro and §11 Spans require `#[instrument(skip_all, name="pty.<op>", fields(...))]` with no `err` / `ret`. obs-plan §2 names seam spans `pty.spawn` / `pty.write` / `pty.wait` / `pty.kill`, and `pty.spawn` is a CLIENT span.
  - obs-plan §11 Telemetry Strategy and Logs ban spans or events per byte or per chunk in the PTY pump read loop. Log boundary outcomes only.
  - The `portable_pty` target stays `OFF`, and failures are captured from the seam's returned `Result`s (§6 per-module levels, D-11).
- **Worker threads.** obs-plan §7 Panic hooks (`run` worker threads) requires the PTY pump and handle-wait thread bodies to each run inside `catch_unwind`, with an `Err` routed to `main`'s `run` exit path. A worker panic that leaves `main` running without a `process-exit` is forbidden. Per obs-plan §3 Trace context propagation → Internal async boundaries, the threads take a cloned `Span` via `in_scope`, and `instance` is passed as an explicit value.
- **New crate lint rules.** Per obs-plan §3 Bootstrap phases → logger-stack-install and obs-ci-gate-wire, and §11 Logs:
  - `crates/viola-pty` depends on `tracing = "0.1.44"` only (no tracing-subscriber), carries `[lints] workspace = true`, and emits only through `obs_event!`.
  - No raw `tracing::*!` macros, and no `print*` / `eprint*` / `dbg!`.
  - The CI member-list assertion must still hold with the new member.

## Patterns to follow
- **Every line through one macro.** `obs_event!(ObsEvent::ProcessStart, subject=…, …)` attaches `event`, `process` and `instance` from `ProcessCtx`. `corr` is omitted, since it is null (key absence) for `process-*` events (obs-plan §3 Logging stack; §3 Log format → Null encoding).
- **Span names and attributes.** Names follow `<area>.<operation>` snake_case under the `pty` area. Attributes are closed enums only (`pty_backend`, `exit_source`) and never high-cardinality (obs-plan §2 Naming conventions; §5 cardinality discipline).
- **Codes-only lines, content in the detail file.** Home-level `run-<name>.ndjson` lines carry codes only. Content-bearing chains go through `viola::obs::report_internal_error` into the lazily opened `detail-run.ndjson` (obs-plan §3 Sink; §7).
- **Error types.** A thiserror `PtyError` with fixed-message `Display` and no path fields (obs-plan §7 Platform pick, scrubbing layer 1).

## Anti-patterns to avoid
- Any byte on `run`'s stdout or stderr from telemetry, including a default panic hook, `log_internal_errors(true)` or an implicit writer (obs-plan §11 Logs, §11 Error Reporting).
- Logging any `CLAUDE*` value or env map, the resolved absolute exe path, or PTY stream content in any home-level line or span field (obs-plan §11 PII Scrubbing, §11 Spans).
- Inferring child exit from PTY EOF, or logging inside the pump loop (obs-plan §4 Scenario 1 Cleanup; §11 Telemetry Strategy).

## Contract bindings
- **obs ↔ tests, log schema.** New `process-start` / `process-exit` fields must already be in the §6 catalog, which `schemas/diag-line.v1.json` enforces with `unevaluatedProperties: false`. Real output from the switched `supervise` / `run_cli` PTY tests is checked by G4 (obs-plan §8 Default-deny posture; §9 G4).
- **obs ↔ tests, harness homes.** Test homes driven through the PTY `supervise` and the `Wrapper` must live under `target/e2e-home/`, with `AGENT_RUN_KEEP_HOMES=1`. Otherwise G2, G4, the secret scan and the `diag-<os>` upload miss them (obs-plan §9 Integration tests row, Step order 1).
- **obs ↔ tests, E2 secret scan.** The tests-owned secret-scan test covers `CLAUDE*` values across all `diagnostics/` (obs-plan §3 Log format → Constraints; §8 item 6). The canary and fixture values are tests-owned, and the E2 fixtures must be synthetic.
- **obs ↔ security.** The R8 NEVER-log floor (Vector 6) and the `.cmd` / `.bat` refusal (Vector 8) are security-owned lists. Obs owns only how they are logged (obs-plan §8; §2 Trigger coverage map, Vectors 6 and 8).
- **obs ↔ CI, ripgrep install.** G1 and G3 depend on the SHA-pinned PCRE2 ripgrep install step and its `rg --pcre2-version` check, and must fail closed: rg exit 127 or 2 is a failure (obs-plan §9 G1; §10 Build / deploy failure conditions). Hardening `install-ripgrep.sh` against curl exit 35 must keep the checksum verification and the fail-closed behaviour.

## Acceptance criteria contributions
- A PTY-spawned `run` produces:
  - `process-start{subject:"claude-child", pty_backend:"conpty", env_stripped_count, env_stripped_known}`;
  - `process-exit{subject:"claude-child", exit_source:"handle-wait"}`;
  - `process-exit{subject:"self", exit_code, duration_ms}`.

  All lines are in `run-<name>.ndjson` and pass G4. No line comes from reader EOF, and viola writes 0 bytes of its own to the terminal (per obs-plan §4 Scenario 1; §6 PTY seam).
- A `.cmd` / `.bat`-resolved child is refused with `process-exit{exit_code:1, detail:"batch-script-child"}`. There is no child `process-start`, and no path or pid appears in the line (per obs-plan §6 detail code catalog).
- With synthetic R8 variables set, a scan of every `diagnostics/*.ndjson`, detail files included, finds 0 hits for their values. `env_stripped_known` holds only compile-time-list names (per obs-plan §8 table, D-14).
- A panic injected into the PTY pump or the handle-wait thread yields exactly one `event:"panic"` line, then `process-exit{detail:"internal-error"}` and exit 1. G2 over `target/e2e-home` stays green on clean runs (per obs-plan §7 `run` worker threads; §10 Zero unlogged panics).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton.** The fake agent is exempt from the print lints only through the crate-level `#![allow]` in `src/bin/viola-fake-agent.rs`, and only `viola-e2e` lacks `[lints] workspace = true`. Why: lints are per package. The new `viola-pty` member must not widen that exemption.
- **2026-09-24-observability-gates.** The raw-tracing ban is enforced by clippy `disallowed-macros` on the level macros plus a fail-closed raw-`event!` grep in `scripts/lint-probes.sh`. A `lint-probes.sh` failure is a build failure. Why: clippy cannot exempt the `event!` inside `obs_event!`. The new `viola-pty` call sites fall under both.
- **2026-09-24-log-redaction-and-never-log-floor.** anyhow is scoped to the root-bin dispatch edge plus `viola::obs::report_internal_error`. Why: the reporter renders the chain into detail files. A PTY spawn failure chain goes through this reporter.
- **2026-09-24-diagnostics-plane.** `corr` is a caller-supplied typed field, and default-deny uses a top-level `unevaluatedProperties: false`. Why: this is how it shipped. Any field the chunk adds beyond the §6 catalog fails G4.
- **2026-09-24-workspace-tree-and-code-graph-planes.** The `supply-chain` artifact (`deny.json` and related files) is admissible unscanned only because of its content. Why: operator ratification. The chunk's `deny.toml` RUSTSEC-2017-0008 ignore must keep that artifact free of absolute paths or log content.
