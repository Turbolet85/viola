
## 2026-09-28-capability-ledger-and-viola-verify — verify's outcome on stdout, the uninstrumented `hook --capture` arm, the `boot` readiness owner
**Section:** §2 Telemetry Strategy (exemption list); §4 instrumentation table (`cli (viola hook)`), Edge flows (`verify`, the capture arm, verify in CI) and the `viola run` start scenario (readiness pointer); §10 Zero unlogged panics (bounded exemptions)
**Change:**
- `verify`'s agent-readable outcome: exit 0 all pass / 1 a failing row or refusal, plus one stdout step line per row and the last stdout line `stamped <version>  N pass  N fail` (was "the … stderr summary"); refusals are the stderr `unable:` + `hint:` pair, a fault exactly `error: internal error`.
- The hidden `hook --capture` arm is uninstrumented by design: no `VIOLA_*`, no obs init, no role or detail line, no `hook-invoked`/`hook-decision`; the §4 hook row excepts it, and it is §10's bounded exemption 5 (restated in §2).
- `verify` in CI: today `tests/cli_verify.rs`; `boot` step 4 and `stamped_home` join with "Verify-stamped test homes and harness", which also owns the `boot` line-3 readiness check (was "Capability ledger and viola verify").
**Why:** the report's Symbols (verify's streams, the capture arm's no-obs-init) and its third disproved claim; the capture arm is the founder's ratification of 2026-09-28 20:24:32.
**Kept:** §6's `process-start`/`process-exit` requirement for child spawns stands: `verify`'s `--version` read and print-mode probe log none yet, and the "Verify-stamped test homes and harness" entry carries the fix (overseer ruling at this wrap: the spec stays right).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
