# obs · CI Integration — category read

## Carried
- (.andromeda/obs-plan-detail/ci-integration.md:24) The pin-race witness (a concurrent two-instance boot or a narrower concurrent-pin control) must put every viola home it makes under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it; the chaos home is the only carve-out and this chunk adds none.
  > Their viola homes must live under `target/e2e-home/` (the directory layout is tests-owned), so G2, G4, the secret scan and the `diag-<os>` upload cover them. A `hook` panic exits 0, so a home outside that root would hide it. The one carve-out: `tests/chaos_feed_panic.rs`'s `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload
- (.andromeda/obs-plan-detail/ci-integration.md:59) The witness's failing runs (a hook spawn failing with `ran:false`) must still reach G2 and G4: both run `if: always()` after the tests, so a red witness step never skips them.
  > 1. Integration and E2E tests (every viola home under `target/e2e-home/` but the one `viola-chaos-*` home of `tests/chaos_feed_panic.rs`, outside G2, G4 and the secret scan by the founder's ruling of 2026-10-04), then G2 and G4, **both** `if: always()`.
- (.andromeda/obs-plan-detail/ci-integration.md:59) A witness that boots two instances on one fresh home (or a Path 7 returned to two instances, if P4 decides so) must leave its homes in place until the gate steps have read them.
  > **Homes are kept until the gate steps have run** (overseer fix pass 2, B1): `ci.yml` sets `AGENT_RUN_KEEP_HOMES=1`, so neither harness `cleanup` nor a test home's drop deletes a home before G2, G4, the secret scan and the scan-gated uploads have read it (test-plan §3 `cleanup` step 6).
- (.andromeda/obs-plan-detail/ci-integration.md:36) Any panic the changed absent-copy pin arm or its concurrent start can raise must be counted by G2 in the `test` and `perf` jobs, which stays at zero; no new exemption is added.
  > **G2, zero panics.** It runs as `scripts/g2-zero-panics.sh` (fail-closed; `tool-missing: jq` without jq), `--probe` first, in the `test` and `perf` jobs; the snippet below is its logic.
- (.andromeda/obs-plan-detail/ci-integration.md:41) Any line the pin fix adds or changes (a wait, a same-bytes acceptance, a refusal) must conform to the two schemas G4 checks, so a new field or `detail` code goes into the schema with the code.
  > Scope: every line in the G2 file scope is checked against `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json` (§8).
- (.andromeda/obs-plan-detail/ci-integration.md:51) Any build change the reproducible-build measurement leads to (a profile key, a rustflag, a cargo config entry, a workflow env) must not set an abort panic strategy anywhere G3 reads, and G3 must still pass only on rg exit 1.
  > # G3: no abort panic strategy (profile `panic` key, `-C panic` rustflag, CARGO_PROFILE_*_PANIC env) in Cargo.toml, cargo config or workflows (§7); default regex engine; only rg exit 1 passes
- (.andromeda/obs-plan.md:594) If P4 adds a CI leg for the measurement on the three OSes, it must go in the one push/PR workflow `ci.yml` on its native matrix, with no new collector job.
  > **Platform:** GitHub Actions; the single push/PR workflow `.github/workflows/ci.yml` (beside the scheduled `nightly.yml`, which runs the weekly `cargo deny check advisories` and a `fuzz` job: 120 s per target on the `fuzz/rust-toolchain.toml` nightly, uploading `fuzz/artifacts/` on `failure()` only, see §8 item 5, and the dispatch-only `windows-mutants.yml`, the boundary audit's Windows mutation leg, which uploads nothing), native matrix `windows-2025` / `macos-latest` / `ubuntu-latest` (arch CI/CD Platform). No OTLP collector job.
- (.andromeda/obs-plan.md:612) A reproducibility fix must not get there by injecting CI or build identity from env into product log lines; the product reads no CI env.
  > **CI-specific resource attributes:** none are read by the product. Env vars are not a configuration channel, so `deployment.environment`, `ci.run.id` and `git.commit.sha` are **never** injected into viola log lines.
- (.andromeda/obs-plan.md:614) Any artifact a new CI step uploads (a measurement record, a witness's diagnostics) must keep the 7-day retention.
  > **Artifact retention:** `retention-days: 7`, below the 90-day default (obs-research CI pattern).
- (.andromeda/obs-plan.md:739) Any upload a new step adds must be gated on the secret scan and go through the SHA-pinned upload action.
  > - NEVER upload `diagnostics/` artifacts before the secret-scan step passes.
  > - NEVER use unpinned Actions. `actions/upload-artifact` v7.0.1 is pinned by commit SHA and checked by zizmor.
- (.andromeda/obs-plan.md:741) The race witness is probabilistic (6 of 24 boots failed), so it has to prove itself red and clean on one run, because a CI re-run cannot stand in for a reading.
  > A re-run is not a substitute under `retries = 0`.
- (.andromeda/obs-plan.md:742) The witness's verdict, and any automated CI form of the two-build comparison, must be a machine assertion that sets an exit code, never a reading left for review.
  > NEVER use human-review-gated log analysis. Every CI telemetry verdict is a `jq` / hyperfine assertion that sets an exit code.

## Bindings
- **Witness homes under `target/e2e-home/` ↔ test-plan §3**: the directory layout and `cleanup` step 6 are tests-owned; obs requires the root so G2, G4, the scan and the upload cover it, and requires `AGENT_RUN_KEEP_HOMES=1` to keep the homes (.andromeda/obs-plan-detail/ci-integration.md:24), (.andromeda/obs-plan-detail/ci-integration.md:59)
- **G3 ↔ the universal `panic = "unwind"` invariant and obs §7 panic hooks**: every Cargo profile keeps unwind; a reproducible-build profile or rustflag change is checked by G3 (.andromeda/obs-plan-detail/ci-integration.md:51)
- **G4 ↔ obs §6 Log Coverage / §8 default-deny schemas**: a new pin-arm field or `detail` code needs its catalog and schema entry before G4 passes (.andromeda/obs-plan-detail/ci-integration.md:41)
- **Scan-gated uploads ↔ obs §8 PII Scrubbing (secret scan, admissible uploads)**: a new upload is scanned first, and on a failed scan only the hit report goes up (.andromeda/obs-plan.md:739)
- **CI platform ↔ security-plan §Dependencies and CI**: actions pinned by full SHA; the measurement's build path is `scripts/release-check.sh`'s, which the security plan owns (.andromeda/obs-plan.md:594)
- **No CI identity in lines ↔ obs §2 resource attributes**: `service.version` stays `env!("CARGO_PKG_VERSION")`, and no build stamp is read from env (.andromeda/obs-plan.md:612)
