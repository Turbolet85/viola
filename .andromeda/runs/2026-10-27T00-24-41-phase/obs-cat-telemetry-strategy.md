# obs · Telemetry Strategy — category read

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:10) Any span the race fix adds or changes on the `viola-state` pin seam (`pin_bytes` / `pin_exe` / `replace_private_shared`), reached from `run`'s start and `verify`, must be declared with `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`, never a bare or serialized span.
  > every crate seam: `viola-pty` spawn/write/wait/kill, `viola-channel` client call and server dispatch, `viola-state` snapshot/heartbeat/tail, `viola-agent-claude` hook parse, `viola-mcp` tool calls | Declared in code with tracing 0.1.44 `#[instrument(skip_all, name = "<area>.<operation>", fields(...))]`.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:11) A fault the fix newly observes on the absent-copy write (a lost race, a wait on an in-flight write, a same-bytes acceptance) is logged, if at all, as a closed `event` value, never as free text.
  > Every boundary call, state transition, refusal, fault/heal and process lifecycle, as the closed `event` enum (Section 3) | **The primary signal.**
- (.andromeda/obs-plan-detail/telemetry-strategy.md:12) A new pin failure crossing out of `viola-state` must be a typed error with a fixed `Display`, ending in an exit code plus a distinct `detail` code on `process-exit`.
  > typed thiserror 2.0.20 errors with fixed `Display`, exit codes plus distinct `detail` codes on `process-exit`
- (.andromeda/obs-plan-detail/telemetry-strategy.md:13) The two-sided race witness and the release-build comparison take their counts from harness or ndjson output (or the build's own files), never from an in-process instrument added to the product.
  > No in-process instruments and no exporter. Metrics are derived by `jq` over the ndjson
- (.andromeda/obs-plan-detail/telemetry-strategy.md:20) A panic on the pin path after init step 4 must still produce its one line; the exemption list is closed, so moving the pin or a new lock or wait ahead of the role file's open adds no unlisted no-line window.
  > Every panic after init step 4 produces exactly one single-line structured record in its home-level role file: location, thread, role and instance.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:33) The hash-before-reuse refusal keeps its `pinned-hash-mismatch` code, and `pin_companions`' refused companion keeps `sideload_fallback` `strict-modes-failed` with the exit unchanged; the race fix changes neither.
  > §6 detail catalog Filesystem refusals (`strict-modes-failed`, `pinned-hash-mismatch`, `symlink-ignored` via `parse-rejected{parser:"state-entry"}`), §9 secret-scan-before-upload, §4 Scenario 1 / §6 `process-start.sideload_fallback` `strict-modes-failed` (a `conpty/` companion its owner/DACL check refuses: the child on the inbox ConPTY, exit unchanged)
- (.andromeda/obs-plan-detail/telemetry-strategy.md:39) A Windows-specific arm of the exe pin (for example a sharing-violation wait), read on the CI runner only, must keep the same file sink and line schema on every OS.
  > | multi-platform-exporter-compat | §3 Log file location (0600/0700 vs home DACL), §3 Logging stack (concurrent-append check on windows-2025 / macos-latest / ubuntu-latest), §4 Scenario 1 (`endpoint_kind`, `pty_backend`), §9 per-OS artifacts |
- (.andromeda/obs-plan-detail/telemetry-strategy.md:43) Any new exit-1 cause on `run`'s start or `verify` from the pin path needs its own distinct `detail` code with no path or pid, while a strict-modes refusal made before the log opens still writes no line.
  > §6 detail code catalog, §12 D-01; a strict-modes refusal made before any process log opens (the six frame verbs and `link` / `unlink`, exit 21; `run`, `revive`'s start and `verify`, exit 1) writes no line and carries no `detail` code: its machine-readable form is the exit code
- (.andromeda/obs-plan.md:286) The race witness (red before the fix, clean after it, the single-instance control beside it) and the build comparison must each give their verdict as a `jq` or harness assertion that sets an exit code.
  > - No human-review-gated log analysis. Every verdict is a `jq` / harness assertion that sets an exit code.
- (.andromeda/obs-plan.md:293) A new pin span must take an `<area>` from the closed list (`state` for the crate seam, `run` / `cli.verify` for its callers), with a static name and no path in it.
  > - **Span naming:** `<area>.<operation>` in snake_case. `<area>` is one of `run`, `pty`, `channel`, `state`, `hook`, `statusline`, `send`, `wait`, `last`, `answer`, `pause`, `release`, `cli`, `mcp`. `cli.<verb>` names the dispatch span of a short-lived verb that has no dedicated client span (for example `cli.list`, `cli.verify`);
- (.andromeda/obs-plan.md:294) A new `event` value for the pin race is an enum extension through the Section 12 log, never an ad hoc string.
  > - **Event naming:** the `event` field is a closed kebab-case enum (the tests list plus the Section 12 extensions).
- (.andromeda/obs-plan.md:683) The pin runs inside the must-trace path "`viola run` start sequence to child spawn", so the fix must not remove or skip any span or event already on it.
  > - NEVER skip spans or events on the 6 must-trace paths (Section 4). The agent loses its ability to debug.

## Bindings
- **Pin seam spans ↔ §4 Scenario `viola run` start sequence to child spawn**: the span form and area (.andromeda/obs-plan-detail/telemetry-strategy.md:10) (.andromeda/obs-plan.md:293) apply on the start path, and the must-trace rule (.andromeda/obs-plan.md:683) keeps its existing spans and events; the span-trace-coverage scenario holds the `pinned_bin` never-in-a-span rule.
- **`pinned-hash-mismatch` / `sideload_fallback` ↔ security.md §Local trust boundary (Home; Windows DLL loads) and obs §6 detail catalog**: the hash-before-reuse refusal and the companion fallback keep their codes (.andromeda/obs-plan-detail/telemetry-strategy.md:33).
- **Distinct exit-1 `detail` codes ↔ obs §6 detail catalog and architecture §Conventions (exit codes)**: a new pin failure cause gets its own code, codes-only (.andromeda/obs-plan-detail/telemetry-strategy.md:12) (.andromeda/obs-plan-detail/telemetry-strategy.md:43).
- **Panic exemptions ↔ obs §10 Zero unlogged panics**: no new pre-log window from a reordered pin (.andromeda/obs-plan-detail/telemetry-strategy.md:20).
- **Witness verdicts ↔ test-plan §3 harness (`agent-run` / `gate`)**: exit-code assertions over harness output, no product instrument (.andromeda/obs-plan-detail/telemetry-strategy.md:13) (.andromeda/obs-plan.md:286).
- **Windows exe-pin arm ↔ obs §3 Logging stack (per-OS) and the CI matrix**: one sink and schema on all three OSes (.andromeda/obs-plan-detail/telemetry-strategy.md:39).
