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
