# security-plan — amendments

## 2026-09-24-three-os-ci-headless-harness-skeleton — pinned actions and toolchain source
**Section:** §Dependency Security (CI integration; Pinning) · §Threat Model Summary (Supply chain entry point)
**Change:**
- The SHA-pinned action set is now exactly what `ci.yml` uses:
  - `actions/checkout` v7.0.1 (with `persist-credentials: false`)
  - `Swatinem/rust-cache` v2.9.2
  - `taiki-e/install-action` v2.87.19
  - `actions/upload-artifact` v7.0.1
- `dtolnay/rust-toolchain` is replaced by a `rustup toolchain install` step reading `rust-toolchain.toml`.
- The toolchain is the exact 1.98.1 pin, and the workspace `rust-version` floor is 1.96.
- Event-payload values reach steps only through `env:`.

**Why:** the chunk shipped `ci.yml` and `rust-toolchain.toml` (report: Harness / gate surface, Schema / config; expected amendment 2). Sweep `dtolnay|@stable|Swatinem` over security-plan: lines 131, 320 and 330–331 were amended; line 548 (the mutable-ref ban) needs no change.

## 2026-09-24-three-os-ci-headless-harness-skeleton — rejected: interim `--home` and R8-strip Decisions-Log entries
**Section:** none (proposals rejected)
**Change:** none.
**Why:**
- The walking-skeleton `viola run` has two known gaps:
  - `--home` is not canonicalised or strict-modes-checked, and the Windows protected DACL is not set;
  - it spawns the child with the full inherited environment, so the R8 `CLAUDE*` strip is not applied yet.
- Both are sequencing deferrals (playbook rule 1), owned by markerless route entries rather than by body prose:
  - "Home and code-bearing file integrity" and "CLI machine contract — global --home" own the first. Both receive a `CARRY:` pin at this wrap's route-resolve.
  - "PTY wrapper on Windows" owns the second; that entry already names the CLAUDE* strip.
- The §Input Validation CLI row, §Secret Management and the §Anti-Patterns Data Protection ban stay as the target.

## 2026-09-24-supply-chain-and-workflow-gates — trust boundary, CI jobs, deny.toml additions, nightly.yml
**Section:** §Threat Model Summary → Supply chain (Trust boundary) · §Architecture Overview → CI/CD · §Dependency Security (`deny.toml` additions, CI integration)
**Change:**
- Trust boundary: now names `deny.toml` with its four families plus the sole-root `deny-sync.toml` tokio ban.
- CI/CD: adds `nightly.yml` and the ubuntu supply-chain job.
- `deny.toml` additions: the heading no longer says arch lists only licences and bans. The arch-bans bullet places the tokio ban in `deny-sync.toml` per sync crate as sole root, and records that `scripts/deny-probes.sh` proves every ban live.
- CI integration: the weekly advisory run is a separate workflow, `nightly.yml` (weekly `schedule` + `workflow_dispatch`, no cache), not a trigger on `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. The fan-out had 0 proposals from this doc's detectors, which all held. Its return flagged these sites as restatements of the claims the pass retires, so the orchestrator raised them as routine cascade dependents. Sweep: see architecture-amendments.md, same entry heading. For this master, 5 sites were amended (:134, :161, :308, :314, :327). The pinned-action set was left unchanged, because no new action was added.

## 2026-09-24-diagnostics-plane — config.json diagnostics_level validation row, MAX_FRAME consumer
**Section:** §Input Validation (Configuration values row; Constants)
**Change:**
- The `config.json` row names the closed `diagnostics_level` (info | debug; any other value falls back to info and is reported as `parse-rejected`), the `MAX_FRAME` cap, and the read's home in the root-bin `viola::obs`.
- The `MAX_FRAME` consumer list adds the `config.json` read.
**Why:** chunk 2026-09-24-diagnostics-plane shipped the read (report Schema/config). The security detector returned no violation and noted both omissions. The orchestrator raised them as routine accurate-additions.

Sweep: `budget thresholds, GUI port\)` and the config-row wording over all seven masters. The only amend-site is security :230 (arch :365/:483 were amended in their own entry). 0 remaining.

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
