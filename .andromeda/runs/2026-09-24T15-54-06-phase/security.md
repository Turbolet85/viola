# security extract

## Relevance
Partial. The fuzz-lockfile audit CARRY is squarely security (supply chain), and so are the CI workflow edits: the release job and the cargo-tree/cargo-modules steps. The architecture-tree listing and the code-graph planes are outside security, except where they would add a new toolchain or dependency.

## Constraints
- The `supply-chain` job must run `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`. The operator-ratified test-only exemption assigns this audit to this chunk (per security-plan §Dependency Security, the `fuzz/` bullet under `deny.toml` additions; §Security Decisions Log, 2026-09-24 entry, Conditions). Whether the weekly `nightly.yml` `cargo deny check advisories` run should also cover `fuzz/Cargo.lock` is an open decision. §Dependency Security CI integration currently scopes that run to the root lockfile only.
- The fuzz exemption holds only while `fuzz` never joins the root `[workspace]` and `libfuzzer-sys`/`viola-fuzz` are never linked into a shipped binary (per §Security Decisions Log 2026-09-24, Conditions; §Threat Model Summary, Supply chain trust boundary). The new release-build check is where this can be witnessed. Whether a root `cargo build --release` stays clear of it today is research's question.
- Any `ci.yml` edit (release job, cargo-tree/cargo-modules steps, fuzz audit step) must keep these rules (per §Dependency Security, CI integration):
  - every action is pinned by full commit SHA with a version comment;
  - workflow-level `permissions: {}` and job-level `contents: read`;
  - event-payload values reach steps only through `env:`;
  - toolchains come only from `rustup` (no toolchain action);
  - the zizmor step still passes.
- New CLI tools such as `cargo-modules` get minimum version floors, not exact pins. Install them through the already-pinned `taiki-e/install-action` or with `cargo install --locked` (the `cargo-fuzz` precedent) (per §Dependency Security, Pinning and the Tool-version syntax note, CI integration).
- The tokio ban stays in `deny-sync.toml`, run with each sync crate as the sole root, and `scripts/deny-probes.sh` must keep proving every ban live (per §Dependency Security, `deny.toml` additions, the arch-bans bullet). This chunk only re-confirms it and must not weaken or restructure it.
- v1 has no release artefacts, signing or deploy stage (per §Threat Model Summary, Infrastructure CI/CD). The `cargo build --release` job is a build check only and must not publish or upload binaries. The v1.x release-workflow items (cargo-auditable, signing, no `rust-cache`) are out of scope here (per §Security Decisions Log, v1.x release prerequisites).
- Any new CI upload, such as cargo-tree/modules output or code-graph output, must either wait on the `viola-harness secret-scan` or be admissible by content: repo-relative paths only, never absolute paths or argv (per §Bootstrap phases, `secret-scanning-ci-gate`; §Secret Management, "Secret scanning in CI").

## Patterns to follow
- Scope the fuzz audit with `--manifest-path fuzz/Cargo.toml` and the `advisories sources` families only. The C-build ban intentionally does not apply to the exempt fuzz graph (per §Dependency Security, the `fuzz/` bullet).
- Install tools the way `nightly.yml` installs `cargo-fuzz@0.13.2` (`cargo install --locked`), or through the SHA-pinned `taiki-e/install-action` (per §Dependency Security, CI integration).
- Treat the `cargo tree -e features` check that rmcp resolves to `server` + `transport-io` as a complement to the `deny.toml` `[[bans.features]]` gate, which denies `transport-streamable-http-server` and `auth`, not a replacement for it (per §Dependency Security, `deny.toml` additions; §Security Anti-Patterns → API).
- Every ban-level gate has a live probe (`scripts/deny-probes.sh`). A new supply-chain gate should come with an equivalent proof that it fires (per §Dependency Security, `deny.toml` additions).

## Anti-patterns to avoid
- NEVER reference a GitHub Action by a mutable ref (`@stable`, `@v2`). Use a full SHA plus a version comment (per §Security Anti-Patterns → Code Patterns).
- Do not add a TypeScript/npm toolchain (`scip-typescript`, Playwright deps) to CI or the repo as part of "deciding" the TS plane without a security-plan amendment. §Dependency Security names only cargo-deny and zizmor as audit tools, so an npm graph would be an unaudited supply-chain entry point (per §Dependency Security, Audit tool; §Threat Model Summary, Supply chain).
- NEVER commit local `--home` test directories to git (per §Security Anti-Patterns → Secrets). This matters if the code-graph or harness output places state under tracked paths.

## Contract bindings
- The CI security gate is bound to the tests plan §CI Integration: one `ci.yml`, with the supply-chain job as a gate job and the `viola-harness gate` verdict if a new gate joins it.
- Upload admissibility is bound to obs-plan §9's artifact canary scan (`viola-harness secret-scan`) and the NEVER-log floor (§Bootstrap phases, `logging-redaction-wire`).
- The dependency policy is bound to architecture §Infrastructure Patterns → Build system (`deny-sync.toml`, `exclude = ["fuzz"]`, workspace members), which this chunk's architecture-tree and dependency-policy wording must stay consistent with.

## Acceptance criteria contributions
- (security) The `supply-chain` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` and it passes on the chunk's pushed sha (per security-plan §Dependency Security, `fuzz/` bullet; §Security Decisions Log 2026-09-24, Conditions).
- (security) zizmor passes on `.github/workflows/`, and every `uses:` in `ci.yml`/`nightly.yml` is a full SHA from the pinned set or a newly pinned SHA with a version comment (grep verifies no mutable refs) (per §Dependency Security, CI integration; §Security Anti-Patterns → Code Patterns).
- (security) The root workspace members exclude `fuzz`, and the release build and `cargo tree` for `viola` show no `libfuzzer-sys`/`viola-fuzz` (per §Security Decisions Log 2026-09-24, Conditions).
- (security) `cargo deny check` over the root graph, the per-sync-crate sole-root `deny-sync.toml` tokio ban and `scripts/deny-probes.sh` all stay green (per §Dependency Security, `deny.toml` additions).

## Relevant amendment history
- **2026-09-24-quality-gates:** ratified `fuzz/` as a test-only workspace outside `cargo deny`, committed both lockfiles, added `download-artifact` v8.0.1 as the fifth pinned action, set the three rustup toolchains, and assigned the `fuzz/Cargo.lock` advisory/source audit to this chunk.
  - Why: operator ruling "Ratify + CARRY audit".
  - Implication: once the audit lands, the "owed" wording goes stale in three places, and each needs a wrap amendment and sweep: §Dependency Security (`fuzz/` bullet and CI integration Job 4, "root `Cargo.lock` only"), §Threat Model Summary (Supply chain trust boundary) and the Decisions Log Conditions.
- **2026-09-24-supply-chain-and-workflow-gates:** placed the tokio ban in `deny-sync.toml`, run per sync crate as sole root and proven by `scripts/deny-probes.sh`; added the ubuntu supply-chain job; made `nightly.yml` a separate advisory workflow.
  - Why: this is the existing gate that this chunk re-confirms rather than rebuilds.
- **2026-09-24-three-os-ci-headless-harness-skeleton:** set the SHA-pinned action set, installed toolchains through rustup reading `rust-toolchain.toml`, and passed event values only via `env:`.
  - Why: the workflow-hardening baseline that any new release or cargo-modules step must keep.
- **2026-09-24-observability-gates:** CI runs the artifact canary scan before test-home uploads.
  - Why: it sets the admissibility rule for any new upload this chunk adds.
