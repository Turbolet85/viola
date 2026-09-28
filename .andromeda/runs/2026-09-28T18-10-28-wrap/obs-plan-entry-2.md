
## 2026-09-28-capability-ledger-and-viola-verify — the panic backtrace as raw, never-symbolised frames; the `cli` role's internal-error line
**Section:** §7 Panic hooks (the detail line's backtrace; Per-role behaviour, short-lived `cli`)
**Change:**
- The detail-line `backtrace` is raw frames from `src/panic_frames.rs` (`RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix): up to 62 strings `0x<ip> <module path> base=0x<base> +0x<offset>` (or `0x<ip> ?`), resolved offline against the pinned copy, still not gated by `RUST_BACKTRACE` (was "comes from `std::backtrace::Backtrace::force_capture()`").
- Short-lived `cli`: when an instance resolves, `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` to `cli-<name>.ndjson` (as `run` does), the chain only in `detail-cli.ndjson`, then exactly `error: internal error\n` in one `write_all`, no hint, exit 1.
**Why:** symbolising 72 frames cost 351.4 ms of a 402.7 ms hook run on the Windows runner and pushed the forced-panic hook to 1.50 s against the 1.0 s spine bound; raw capture measured 23.7 ms (as measured at CI runs 36436266196, 36435153705, 36448654074). The overseer directed this amendment.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
