
## 2026-09-24-diagnostics-plane — config.json diagnostics_level validation row, MAX_FRAME consumer
**Section:** §Input Validation (Configuration values row; Constants)
**Change:**
- The `config.json` row names the closed `diagnostics_level` (info | debug; any other value falls back to info and is reported as `parse-rejected`), the `MAX_FRAME` cap, and the read's home in the root-bin `viola::obs`.
- The `MAX_FRAME` consumer list adds the `config.json` read.
**Why:** chunk 2026-09-24-diagnostics-plane shipped the read (report Schema/config). The security detector returned no violation and noted both omissions. The orchestrator raised them as routine accurate-additions.

Sweep: `budget thresholds, GUI port\)` and the config-row wording over all seven masters. The only amend-site is security :230 (arch :365/:483 were amended in their own entry). 0 remaining.
