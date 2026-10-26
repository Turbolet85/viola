# Task — obs · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
Moves the workspace's exact `tokio` pin from `=1.53.1` to a patched version (`>=1.53.3`, `=` pin form kept) and the locked tokio with it, so `cargo deny check`'s advisories family no longer fails on RUSTSEC-2026-0386 (`unsound`, the safe `AsyncFd` constructors) and CI's `supply-chain` job reads green.
RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) stays the only advisory ignore: no ignore widening, no `skip` / `allow`, no licence exception, no `deny.toml` policy change.
Companion crates (`tokio-macros`, `tokio-stream`, `tokio-util`) move only where the new tokio requires it; no other dependency moves; no source file changes unless the bump forces one (an `AsyncFd` deprecation read by clippy `-D warnings`).
Re-runs and reads back the gates around it: the tokio bans (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`), `scripts/deny-probes.sh`, fmt, clippy, the workspace tests, `scripts/rmcp-features.sh`, `scripts/release-check.sh`, and closes the red `supply-chain` read of ci#40257114709 against its log.

## Reached files
- Cargo.toml · edits `[workspace.dependencies]` tokio pin row
- Cargo.lock · edits the tokio package entry (and tokio-macros / tokio-stream / tokio-util entries only where the new tokio requires it)
- deny.toml · re-reads `cargo deny check` (advisories, licences, sources, bans; ignore list unchanged)
- deny-sync.toml · re-reads the per-sync-crate tokio ban
- scripts/sync-crates.txt · re-reads the sync crate list the tokio ban runs over
- scripts/channel-tokio-free.sh · re-reads `viola-channel`'s tokio-free normal graph check
- scripts/deny-probes.sh · re-reads the proof that every ban fires
- scripts/rmcp-features.sh · re-reads rmcp's release-graph feature set
- scripts/release-check.sh · re-reads the release-build artifact check
- crates/viola-mcp/Cargo.toml · re-reads Tokio containment
- crates/viola-channel/Cargo.toml · re-reads Tokio containment (the `tokio` feature)
- crates/viola-e2e/Cargo.toml · re-reads Tokio containment (test-only)
- .github/workflows/ci.yml · re-reads the `supply-chain` job

## Needs
- The supply-chain telemetry trigger (security Vector 9): obs dependencies stay Tokio-free in the sync crates, add no C-building crate and pass `cargo deny`; the bump must leave all three true.
- RUSTSEC-2026-0386 cleared from `cargo deny check` at CI's pinned cargo-deny version, with RUSTSEC-2017-0008 still the only ignore and no other advisory, licence, ban or source finding.
- The build failure condition on a `cargo deny` failure (Tokio in a sync crate, a C-building crate, veil `toggle`, banned obs crates): the chunk's lock and pin change must keep every one of those bans passing.
- The veil `toggle` feature ban (`[[bans.features]]` in cargo-deny) stays exactly as is: no `deny.toml` policy line changes.
- Tokio containment as the obs scope records it: Tokio only in `viola-mcp` (the stdio MCP surface, Tokio allowed), `viola-channel` behind its `tokio` feature and the test-only `viola-e2e`; `viola-channel` and `viola-state` instrumentation stays sync, with no Tokio in their graphs.
- The `viola mcp` surface's rmcp 3.4.1 server with features exactly `server` + `transport-io`: a tokio move must not change rmcp's feature set or version range (`>=3.4.1, <3.5`).
- The `AsyncFd` constructor deprecation in the patched tokio: a call in viola's own code would fail clippy `-D warnings` (a build failure condition); a dependency's own call (interprocess's tokio backend) is not linted by viola's gate. P3 measures both.
- The clippy build failure conditions (`disallowed-macros`, `print_stdout`, `print_stderr`, `dbg_macro`) stay green across any forced source edit.
- No wire contract, exit code, refusal word or log line changes: the closed `event` enum, the `detail` code catalog and `schemas/diag-line.v1.json` / `schemas/diag-detail.v1.json` are untouched.
- The release build still carries `viola` only, with no test-only binary or test-only feature (`scripts/release-check.sh`).
- The red CI read ci#40257114709 (`b1249fb1`, job `supply-chain`) folded into this chunk and closed against its job log: RUSTSEC-2026-0386 alone stands behind it.
- CI's `supply-chain` job green on the chunk's push, the full row read at the wrap.
- `fuzz/Cargo.lock` holds no tokio, so the fuzz lockfile's own advisories / sources audit is not moved.

## Standing conventions
- S1 · a source line edited only because the bump forces it keeps the span and log discipline: `#[instrument(skip_all, fields(..))]`, events only through `obs_event!`, no field outside the Section 6 catalog · §4 Span / Trace Coverage and §11 Obs Anti-Patterns (Spans / Traces, Logs) of the body

## Not needed
- The six must-trace path scenarios (run start, confirmed send, wait / last, dialog answer, wheel, budget governor) and their required fields: no path's spans or events change.
- The log-field and `detail` code catalogs and the per-module log level table: no field, code, target or level is added or changed.
- Metric coverage (the derived `viola.<area>.<measure>` metrics and their grouping keys): no metric is added.
- PII scrubbing data classes and the veil `Redact` integration points: no payload type or field is touched.
- Error capture and panic hooks, including how `viola mcp`'s Tokio runtime contains a spawned task's panic: the scope names no change to that behaviour.
- The zero-unlogged-panics invariant, the hyperfine performance budgets and the heartbeat liveness flip: no hook, `run` or heartbeat code changes.
- The logging-sensitive triggers of security Vectors 1 to 8 (IPC, paste text, hook stdin, MCP rejects, CLI / env, filesystem, child spawn): no boundary they govern is touched.
- Telemetry strategy naming (span, event, metric naming, service identity): nothing is named anew.
- The obs scope's tier justification and its pending reconciliation items: the chunk changes no tier or carve-out.
- The print-lint exemption for the output modules (obs anti-patterns, Logs): no output module is edited.
