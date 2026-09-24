# obs extract

## Relevance
Partial. This chunk changes no instrumentation. Obs applies in three places: the architecture tree must list obs-owned artifacts (`schemas/`, workspace `clippy.toml`, `target/secret-scan/`), the new CI steps in `ci.yml` (release build, fuzz audit, cargo-tree/modules) must obey obs's CI gates, and the new release profile must obey obs's panic-strategy invariant. Obs tier is Standard (1), with Minimal-tier exporter carve-outs (per obs-plan §1).

## Constraints
- Every Cargo profile must keep `panic = "unwind"`, including any `[profile.release]` that target job 6 adds or relies on. G3 fails on `panic`/`_PANIC` = `abort` in `Cargo.toml`, `config.toml` or `*.yml`/`*.yaml`, so a release job's env or rustflags must not set it either (per obs-plan §7, "Unwinding is required"; §9 Gate commands G3; §10 Build / deploy failure conditions).
- The architecture tree and occupied-resources entries must name these obs-owned artifacts:
  - both schemas, `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json` (per obs-plan §3 Bootstrap phases → log-format-schema-emit; §8 Default-deny posture / Detail-file scope);
  - the single workspace `clippy.toml` with its `disallowed-macros` (per obs-plan §3 Bootstrap phases → logger-stack-install);
  - `scripts/lint-probes.sh` (per obs-plan §3 Bootstrap phases → logger-stack-install);
  - the `target/secret-scan/` hit-report directory (per obs-plan §8 Integration points item 6 → Scan failure).
  The plan also names other `target/` dirs that belong in the report-dir inventory: `target/e2e-home/`, `target/agent-run/`, `target/nextest/ci/junit.xml` and `perf/*.json` (per obs-plan §9 Telemetry artifact handling). Whether HEAD's tree already lists each one is for P3 research to answer.
- `e2e-web/schemas/a11y-row.v1.json` is tests-owned and separate from the obs schemas. The tree must not fold it into `schemas/` (per obs-plan §8 Default-deny posture; §3 Log format JSON schema).
- The print-ban exemptions stay intact. Only `viola-e2e` omits `[lints] workspace = true`. The fake agent is a root-package `[[bin]]` behind feature `fake-agent` and uses a crate-level `#![allow]`. The CI member-list assertion names only `viola-e2e`. Any release-output check or workspace-policy rewording must not change this (per obs-plan §3 Bootstrap phases → obs-ci-gate-wire; §11 Logs).
- Any new job or step in `ci.yml` (release build, `cargo deny --manifest-path fuzz/Cargo.toml`, `cargo tree -e features`, `cargo modules`) uses SHA-pinned actions only. If it uploads anything, the upload must follow the scan-gated upload rules (per obs-plan §11 CI; §9 Step order and conditions).
- Any new upload that holds process output must either be covered by the secret scan or be recorded in §8 item 6 as admissible by content. A release-binary artifact would be a new unscanned upload (per obs-plan §8 Integration points item 6, "Unscanned uploads, admissible by content").
- `cargo deny` keeps its obs bans on the workspace: `opentelemetry-otlp`, `opentelemetry-stdout`, `sentry`, `tracing-appender`, the veil `toggle` feature and the tracing-subscriber `env-filter` feature. Extending `supply-chain` with a fuzz-manifest run must not weaken the workspace run (per obs-plan §3 Bootstrap phases → otel-sdk-install and pii-scrubbing-wire; §9 Pipeline integration, Lint / typecheck row).

## Patterns to follow
- Each gate is its own `run:` step with `shell: bash` on all three OSes. It fails closed: a missing tool or a regex/I-O error fails the step, never passes it. rg gates pass only on exit 1 (per obs-plan §9 Gate commands).
- Where a gate could pass vacuously, check first that its scope is non-empty, as G2's `test -n "$(find …)"` does. The release-output "no test binary" check should do the same: assert the release dir has binaries before asserting which ones are absent (per obs-plan §9 Gate commands G2).
- Every CI verdict is an exit-code assertion (`jq -e`, `test`), never output a human reads (per obs-plan §11 CI; §11 Universal).
- G1 scopes the whole workspace from its root, including `fuzz/`. This is a precedent for how obs treats the separate `fuzz/` workspace, and the code-graph plane's fuzz decision can mirror it (per obs-plan §9 Gate commands, G1 comment).

## Anti-patterns to avoid
- NEVER set an abort panic strategy, including for a smaller or faster release build (per obs-plan §7; §10 Build / deploy failure conditions).
- NEVER use unpinned Actions, and NEVER upload diagnostics-bearing artifacts before the secret scan passes (per obs-plan §11 CI).
- NEVER use human-review-gated verdicts for new gates. Every check must set an exit code (per obs-plan §11 CI).

## Contract bindings
- obs ↔ arch: the arch §Project directory structure, workspace and dependency policy, and §Occupied Resources must list the obs artifacts named in §3 and §8, and target job 6 must obey §7's unwind invariant and G3 (per obs-plan §3 Bootstrap phases; §7; §9).
- obs ↔ tests: `viola-harness` (`crates/viola-e2e`) and `viola-fake-agent` are the two print-ban exemptions. Checking that the release output carries neither binary is the same test-only boundary seen from the build side. If a new gate joins the `viola-harness gate` verdict, the harness's one-JSON-document stdout contract still applies (per obs-plan §3 Bootstrap phases → obs-ci-gate-wire).
- obs ↔ security: the secret-scan hit-report directory (`target/secret-scan/`) and the upload-admissibility inventory in §8 item 6. Any new CI upload from this chunk binds here (per obs-plan §8 Integration points item 6).
- obs ↔ a11y: the a11y row schema lives at `e2e-web/schemas/a11y-row.v1.json`, under the `e2e-web/` directory that this chunk lists in the tree. That schema is tests-owned, not obs-owned (per obs-plan §8 Default-deny posture).

## Acceptance criteria contributions
- (obs) After the release job lands, G3 still exits 1 (no match) over `Cargo.toml`, `config.toml` and `*.yml`/`*.yaml`. No profile, rustflag or `CARGO_PROFILE_*_PANIC` env sets `abort` (per obs-plan §7, "Unwinding is required"; §9 Gate commands G3).
- (obs) The arch tree names `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`, the workspace `clippy.toml`, `scripts/lint-probes.sh` and `target/secret-scan/`, verified by grep against HEAD's `architecture.md` (per obs-plan §3 Bootstrap phases; §8 Integration points item 6).
- (obs) Every action in the new or changed `ci.yml` steps is SHA-pinned (zizmor stays green). Any new upload is either scan-gated (`steps.secret-scan.outcome == 'success'`) or listed as admissible by content (per obs-plan §11 CI; §8 Integration points item 6).
- (obs) The CI member-list assertion still passes: every product member has `[lints] workspace = true`, and only `viola-e2e` lacks it (per obs-plan §3 Bootstrap phases → obs-ci-gate-wire).

## Relevant amendment history
- 2026-09-24-three-os-ci-headless-harness-skeleton. The fake agent is a root-package `[[bin]]` exempted by a crate-level `#![allow]`, and only `viola-e2e` carries its own lint table. This is relevant because the chunk's check that the release output has no `viola-fake-agent`/`viola-harness` relies on the same packaging: the fake agent is behind a feature in the root package, and the harness is a `viola-e2e` bin.
- 2026-09-24-supply-chain-and-workflow-gates. `ci.yml` stays the single push/PR workflow, and `nightly.yml` sits beside it. This is relevant because the fuzz advisories audit is to go in the push/PR `supply-chain` job, not in nightly.
- 2026-09-24-observability-gates. The raw-tracing ban was split into two parts: the `clippy.toml` path ban on the level macros, and the fail-closed raw-`event!` grep in `scripts/lint-probes.sh`, run on the Linux lint leg. This is why `clippy.toml` and `lint-probes.sh` are obs artifacts the tree must list, and why they are lint-job steps that job-6 wiring must not disturb.
- 2026-09-24-quality-gates. §8 item 6 now records unscanned uploads that are admissible by content (`mutants-verdict-<os>.json`, nightly `fuzz/artifacts/`), and §9 Platform/Mutation were updated. This chunk's prereq CI witness checks that chunk's `mutants-verdict` union job. Any new upload this chunk adds must be entered in the same inventory.
