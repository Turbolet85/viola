
## 2026-09-24-observability-gates — CI runs the obs artifact canary scan before uploads
**Section:** §Secret Management ("Secret scanning in CI") · §Bootstrap phases (secret-scanning-ci-gate)
**Change:**
- "Not in v1" / "Not wired in v1" now reads "no repo secret scanner in v1". That stays true: no scanner was researched.
- Both sites record that CI runs obs-plan §9's artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`) before every test-home upload, against this plan's NEVER-log floor, and names its classes. It never prints or writes matched bytes.
- The `mutants.out/` upload is noted as unscanned: a CARRY on "Quality gates".
**Why:** chunk 2026-09-24-observability-gates (report Changes: Symbols / APIs `secret-scan`, Harness / gate surface). The previous text read as "CI does no secret scanning", which the new CI contradicts. The Decisions Log entry and the Threat Model's verbatim CI list are history and are unchanged.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `Not wired in v1`: 0 hits after the apply;
  - `Secret scanning in CI:** Not`: 0 hits.
- `security-plan.md:601` (the Decisions Log line "Secret scanning in CI (no scanner researched)") needs no change; it is still true of a repo scanner.
- Leaves: `security-summary.md` and `.claude/rules/security.md` state no scanner claim; 0 hits.
