
## 2026-09-28-hook-perf-gate — the second fake-agent test seam `FAKE_AGENT_HOOK_PANIC`
**Section:** §Established Decisions [Naming]; §Conventions Environment variables; §Occupied Resources → Environment variables (Test seams); §Cross-cutting Patterns Config management
**Change:**
- Was one ratified exception (`FAKE_AGENT_PUMP_DELAY_MS`); now two test seams, each read only under `cfg(feature = "fake-agent")` and absent from release builds.
- `FAKE_AGENT_HOOK_PANIC`: `fn panic_if_asked` in `src/cmd/hook/seam.rs` (module declared without a cfg), called in `hook()` right after `viola_obs_init`, before the stdin lock; panics only on exactly `1` with the fixed 4 608 B payload `"forced-hook-panic ".repeat(256)` → one codes-only `panic` role line + one `detail-hook.ndjson` line over 4 KiB, exit 0, empty streams. Named in product source only in that file; configures nothing, disables no control, widens no redaction.
**Why:** the fail-open contract proven on a real panic in the real binary. A boundary widening, ratified by the founder live on 2026-09-28 (the seam at 06:21; the seam with G2's exact-path exemption at 09:52:07; relay the Viola overseer).
**Ref:** .andromeda/runs/2026-09-28T07-37-52-wrap/
