
## 2026-10-20-epoch-5-cleanup — the headroom floor, the hook hangup named, the scratch control made, E4 retitled
**Section:** §3 → `test-data-bootstrap` (Mechanism: `prepare_home_base`) · §3 → `5-command-implementation` (`boot` step 2; `run --mutants`' scratch witness) · §7 Test Data & Fixtures (the fake agent's exit bullet) · §10 Quality Gates & Coverage Targets (Stack adjustments) · §6 E2E Test Strategy (Scenario: E4)
**Change:**
- Test data bootstrap and `boot` step 2: a linked base whose filesystem reads less than `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs` is refused with `e2e-home backing: {free} MiB free, below the {floor} MiB floor`. The decision is the plain `headroom_refusal(free, floor_mib)`, unit-tested on every OS. There is one compiled constant per keeper, set by no variable, flag or config key, its basis the EDQUOT reading of about 8 GiB plus 1 GiB. A plain base reads none; `boot` reads it as `build-failed`.
- The scratch witness: the plain sibling is granted the Users group's write explicitly (`icacls … /grant *S-1-5-32-545:(OI)(CI)(W)`, exit asserted), so it reads `Writable` whatever its parent grants. Was a plain-created sibling that inherited the grant.
- The fake agent's exit bullet: on Unix every command it spawns (each hook, the status line command) runs in a process group of its own. A hangup of its PTY, at its exit or when a killed wrapper closes the PTY, no longer reaches a hook in its exit-time coverage write. Was: «a hook is in its PTY's foreground process group».
- Stack adjustments: a second measured mechanism, the hangup cutting a `viola hook`'s profile, with 0 refused of 800 in 200 runs of `chaos_wait_parked_wrapper_killed_exits_21` after the change. The census witness gains `--case <binary stem> <test name> …`.
- E4 reads «none in 0.1.0» over an `N/A` gap marker (was «SSE `Last-Event-ID` resume»).
**Why:** the floor stops a run before the quota'd tmpfs's EDQUOT. The census named the truncated profile's killer two-sided. A control made explicitly holds inside cargo-mutants' owner-only copy too. E4 named a retired web scenario.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/
