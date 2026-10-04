
## 2026-10-04-readiness-gate-and-timing-constants — the named spine bound, the vt100 property and fuzz target, the fixture-repo leak's cause
**Section:** §2 (Property-based row) · §3 → 5-command-implementation, bootstrap-phases-derive-for-route-setup-project · §6 (Property suite, cargo-fuzz paragraph) · §7 (Fake agent modes) · §10 (Spine deadline, perf table)
**Change:**
- §10 Spine deadline: the gate reads `viola_core::SPINE_DEADLINE` (1.0 s), imported by `viola-harness` `gate.rs` `perf()` (was a provisional `1.0 s` "until arch names it"); the hook's connect deadline is the provisional `CONNECT_DEADLINE` = 750 ms, asserted below it. The spine-hooks and `pre-tool-use` table rows read `max <` `viola_core::SPINE_DEADLINE` (1.0 s).
- §3 → 5-command-implementation: perf JSONs are judged against `viola_core::SPINE_DEADLINE`, breach text `{file} max {max} >= {bound}` (was `< 1.0` (`SPINE_DEADLINE_S`)); the fuzz-replay seed list adds `vt100_feed`: 6.
- §6 Property suite: the vt100 feed property landed at `cases: 512` (`screen::tests::feed_prop_a_caught_panic_poisons_until_resize`). cargo-fuzz: `vt100_feed` joined (6 synthetic seeds) with a silent panic hook over libfuzzer-sys's aborting one; a panic escaping the catch still aborts. §2 lists it among the replayed targets.
- §7 Fake agent: `--vt100-panic-bytes` lands with confirmed `send`'s chaos case; its bytes are measured (a 24×1 PTY + a wide character), so no new env seam.
- §3 → bootstrap-phases: the 21 half-removed fixture repos' cause is the detached `git maintenance run --auto` each fixture commit spawned; fixture repos run git with `-c maintenance.auto=false`. Measured on the `Pass` tests (4 and 5 per 200 rounds without, 0 and 0 with); the full mutation run under the fix is not measured.
**Why:** the readiness-gate chunk named the spine constant, landed the vt100 property and target, and carried CARRY 4's two-sided witness.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/
