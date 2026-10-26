# obs · SLO Invariants & Telemetry Budgets — category read

## Carried
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8) Every `run` process the chunk's Windows listener and squatted-name tests start must, after init step 4, write exactly one single-line panic record on a panic; the chunk adds no retry-once policy, so the CARRY's red `send_budget_paused...case_4` cannot be closed by a retry.
  > every panic in `run`, `hook`, `mcp` and `cli` **after init step 4** produces **exactly one** single-line `level:"ERROR", event:"panic"` record in its home-level role file, with no multi-line backtrace in home-level files, and `hook` still exits 0. Retry-once policies for panic handling are forbidden, in parallel with the tests' `retries = 0`.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:10) The exemption list is closed: the squatted-name start refusal and any new listener-property check sit outside it, so the chunk must not move an endpoint check ahead of the role-file open without amending this list.
  > **Bounded exemptions** (no file exists, so no line can be written):
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:19) A `run` refused at start for a squatted name has its `process-start` in a home-level role file, so a panic there that shows only as an exit code fails the build.
  > The failure condition "a panic evidenced by an exit code without its line" applies only to processes whose `process-start` is in a home-level role file.
- (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28) A change to the Windows pipe listener's security attributes or open mode must keep the spine and `pre-tool-use` hooks' hyperfine `max` under the spine deadline in the per-OS `perf` job, the Windows leg included.
  > hyperfine `max <` spine deadline in `target/agent-run/artifacts/perf-<hook>.json`, judged by `gate --require perf` (each spine row and the `pre-tool-use` row required by name; the perf session boots unstamped, so `pre-tool-use` runs on an unverified CLI). The deadline value is tests-owned (provisionally 1.0 s; the hook's own provisional 750 ms connect deadline sits below it); obs requires only that the max sample is gated
- (.andromeda/obs-plan.md:643) A listener failure the chunk raises in `viola-channel` (a held name, a refused remote client) and passes into `run` must be logged once at `WARN`/`ERROR` with a closed code, its chain only in the instance detail file.
  > - **Module-boundary error logging:** every typed error that crosses a crate seam is logged once at `WARN`/`ERROR` with a closed `detail` / `error_code`. The full chain goes only to the instance detail file.
- (.andromeda/obs-plan.md:645) The fix for the CARRY's missing `send-refused` line under libtest threads must not wait for a log line by sleeping.
  > - **Deterministic emission:** events key on hook events and `events.ndjson` offsets. Nothing is derived from screen content, and no log-based wait relies on sleeps (creator explicit).
- (.andromeda/obs-plan.md:654) The chunk's CI runs on all three OSes must read zero panic lines, zero secret-scan hits and zero schema-conformance failures.
  > - **Definition:** zero tolerance on three counts: 0 `event:"panic"` lines in any CI artifact (as G2 counts them: the `fake-agent` seam's deliberate panic at exactly `src/cmd/hook/seam.rs:<digits>` excepted, §9), 0 secret-scan hits, 0 schema-conformance failures.
- (.andromeda/obs-plan.md:658) A nonzero count on any of the three keeps this chunk red until its root cause is fixed inside the chunk.
  > - **Action threshold:** any nonzero count makes the chunk red until the root cause is fixed within that chunk (tests quarantine rule). There is no on-call or paging.
- (.andromeda/obs-plan.md:662) The CARRY's acceptance (three plain `cargo test` runs, or a fixed log capture) must establish the cause; a retry that masks the red is not acceptable.
  > **Cross-input parallel:** the tests' zero-flakiness budget (`retries = 0`) ↔ obs' zero-unlogged-panics invariant. Retry policies mask real signals in both cases.
- (.andromeda/obs-plan.md:667) Any `diagnostics/` file a Windows test home writes must stay DACL-protected; the chunk's pipe DACL work must not loosen the home's own DACL.
  > - A secret-scan hit, or a `diagnostics/` file that is not 0600 (or not DACL-protected on Windows).
- (.andromeda/obs-plan.md:668) A log line the chunk adds for a listener outcome (a new `detail` code or field) must be in the catalog and `schemas/diag-line.v1.json` before it is emitted.
  > - A schema-conformance failure: unknown `event` value, missing required field, or a field outside the catalog.
- (.andromeda/obs-plan.md:748) The CARRY's flaky log-capture assertion is a flaky telemetry assertion, so it may not be closed by a retry-once policy.
  > - NEVER add retry-once policies for panics or flaky telemetry assertions.

## Bindings
- **Zero unlogged panics, no retry ↔ test-plan §10 (`retries = 0`, quarantine rule)**: the CARRY's red case_4 is fixed at its cause or measured green three times under plain `cargo test`, never retried (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:8), (.andromeda/obs-plan.md:662), (.andromeda/obs-plan.md:748)
- **Error budget ↔ obs §9 CI Integration (G2, G4, the `secret-scan` step)**: the three zero counts are read from the in-job gate exit codes of the chunk's Windows, macOS and Linux runs (.andromeda/obs-plan.md:654), (.andromeda/obs-plan.md:658)
- **Spine-hook perf row ↔ test-plan §10 (spine deadline, tests-owned) and the hook's 750 ms connect deadline**: a Windows listener change is judged by `gate --require perf` on the Windows `perf` job (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:28)
- **Module-boundary error logging and schema conformance ↔ obs §6 Log Coverage (level mapping, `detail` catalog) and security-plan §Authentication & Authorization (`squatted-name`, exit 1 unchanged)**: a listener failure is logged once with a closed code that is already in the catalog, and no exit code or refusal word changes (.andromeda/obs-plan.md:643), (.andromeda/obs-plan.md:668)
- **Windows DACL on `diagnostics/` ↔ security-plan Vector 7 (home DACL) and the pipe DACL this chunk asserts**: the pipe's protected DACL work leaves the home and `diagnostics/` DACL protection unchanged (.andromeda/obs-plan.md:667)
- **Bounded exemptions ↔ obs §7 (pre-init window before step 4)**: the squatted-name refusal stays after the role-file open, so its panics are counted (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:10), (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:19)
