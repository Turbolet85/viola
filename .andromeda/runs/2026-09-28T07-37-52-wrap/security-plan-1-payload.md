
## 2026-09-28-hook-perf-gate — test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption, ratified by the founder
**Section:** §Input Validation (test-seam rows); §Secret Management → Storage; §Security Anti-Patterns → Universal; §Security Decisions Log (`2026-09-28`, test seam)
**Change:**
- A second test-seam row: `FAKE_AGENT_HOOK_PANIC`, a closed value (panics only on exactly `1`), fixed synthetic payload, fired after `viola_obs_init` so `hook` still fails open; named only in `src/cmd/hook/seam.rs`; compiled out of release builds.
- Storage and Universal: "the one variable / the one carve-out" (`FAKE_AGENT_PUMP_DELAY_MS`) → two test seams; the ban now names another seam or another G2 exemption as needing its own entry.
- New Decisions Log entry: the seam, and G2 (`scripts/g2-zero-panics.sh`) not counting a panic line at exactly `src/cmd/hook/seam.rs:<digits>` (whole-string compare, never prefix, suffix, regex or home path); Conditions: `release-check` `viola only`, G2's `--probe` look-alike and empty-scope controls before every check.
**Why:** a boundary widening (a test build reads a new input; a zero-panics gate admits one location). Ratified by the founder, live: the seam at 06:21 on 2026-09-28, the seam with the G2 exemption (shown to him as new) at 09:52:07; relay the Viola overseer.
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
