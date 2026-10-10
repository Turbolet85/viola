
## 2026-10-10-windows-mutation-grade — the obs-code mutation reading on Windows is its own job; "missed" reads after the host exclusion
**Section:** §9 CI Integration (Pipeline integration, Mutation row) · §10 SLO Invariants & Telemetry Budgets (Build / deploy failure conditions)
**Change:**
- §9 Mutation row: the Windows reading of `src/panic_frames.rs` is the job `mutants (viola-panic-frames)`: 23 tested, 14 caught, 0 unviable, 0 missed, 9 left out by the harness as `#[cfg(unix)]` twins the Windows build never compiles, from that job's `run` document in run 38036448183, the dispatch of 2026-10-10 (was "9 of 9 caught, run 37174673472").
- §9 Mutation row: red at that run is a timed-out obs-code mutant, a missed one the harness does not leave out as host-excluded, or unviable outnumbering caught (was "a missed or timed-out obs-code mutant").
- §10: "a surviving cargo-mutants mutant in obs code" is the harness document's `survived`; a missed mutant the host never compiles is left out.
**Why:** the chunk split the Windows workflow's `viola` item per file, so the panic-frame code has a job of its own, and the harness now leaves host-excluded missed mutants out of the count (test-plan §3 `run` step 4).
**Kept:** the invocation form in the row (`run --mutants --package <member> --file …` on `windows-2025`) and the words dispatch-only and report-only are unchanged. §3 states nothing about the `run` document's `mutants` object, so the §3 bind with test-plan holds without an edit here.
**Ref:** .andromeda/runs/2026-10-10T08-56-51-wrap/
