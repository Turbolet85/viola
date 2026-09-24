
## 2026-09-24-quality-gates — fuzz lockfile exemption, fifth pinned action, fuzz and MSRV toolchains, unscanned uploads, declined concurrency
**Section:** Threat Model Summary (Supply chain entry point and trust boundary; Infrastructure CI/CD) · Dependency Security (`deny.toml` additions; Pinning ×2; CI integration: cargo-deny scope, nightly fuzz job, declined `concurrency:`, pinned actions, toolchains) · Bootstrap phases `secret-scanning-ci-gate` · Security Decisions Log (new 2026-09-24 entry)
**Change:**
- `fuzz/` is a separate workspace whose `fuzz/Cargo.lock` (`libfuzzer-sys =0.4.13`, which builds C++, and `arbitrary =1.4.2`) sits outside `cargo deny`. This is a test-only exemption, conditional on never linking and never joining the root `[workspace]`, with its advisory/source audit owed to "Workspace tree and code-graph planes".
- Both lockfiles are committed.
- `actions/download-artifact@3e5f45b… # v8.0.1` joins the pinned set (5).
- Toolchains: `rust-toolchain.toml` 1.98.1, `fuzz/rust-toolchain.toml` `nightly-2026-09-20`, and MSRV `1.96` via `RUSTUP_TOOLCHAIN`, all through rustup.
- `nightly.yml` gains the fuzz job.
- zizmor pedantic `concurrency-limits` is declined (why).
- The `mutants.out/` CARRY is closed by removal. Two unscanned uploads are admissible by content: the verdict JSON (repo-relative source locations and outcomes only, never absolute paths), and the nightly `fuzz/artifacts/` from the synthetic corpus.
**Why:** chunk 2026-09-24-quality-gates. Operator rulings at wrap P2 (the overseer, founder-delegated): "Ratify + CARRY audit"; "Ratify both by content", "repo-relative source locations only, never absolute paths" (verified: 118/118 names in the local leg verdict start `crates/`).
**Sweep:** `mutants\.out` 3 hits, the amended `:384`, the new Decisions entry, and none stale. `no toolchain action` 2 hits, both amended to name the three rustup toolchains. `upload-artifact` in pinned-set phrasing: every site lists download-artifact. Threat Model amended in place per the prior-wrap precedent (sidecar entries at lines 4 and 30). Leaves re-derived: `.claude/rules/security.md` (fuzz exemption bullet), `.claude/docs/commands.md` (deny scope). `.claude/docs/security-summary.md` recomputed with no change (it states no action set, CI jobs or upload list).
