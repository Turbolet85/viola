# obs extract

## Relevance
Relevant. This chunk is obs-plan §3 Bootstrap phases `obs-ci-gate-wire`, the last obs bootstrap phase. Its subject is the §9 gate set and the §10 error-budget measurement surface.

## Constraints
- **Lint bans and exemptions.** Per obs-plan §3 Bootstrap phases → `obs-ci-gate-wire` (bullet 2, as amended 2026-09-24):
  - `[workspace.lints.clippy]` must set `print_stdout` / `print_stderr` / `dbg_macro` = `deny`.
  - Every product member (the root bin `viola` and each product `crates/viola-*`) must carry `[lints] workspace = true`. A member without it silently escapes the ban.
  - Only `viola-e2e` may omit it, and it must carry its own `[lints.clippy]` table without the print lints.
  - The fake agent is a root-package `[[bin]]`. Its exemption is a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`. It cannot have a separate lint table.
- **Raw-tracing ban.** Per obs-plan §3 `obs-ci-gate-wire` bullet 1 and §3 `logger-stack-install` (`disallowed-macros` exemption):
  - One workspace `clippy.toml` must ban raw `tracing::{event,info,warn,error,debug,trace}` through `disallowed-macros` everywhere outside `viola_core::obs`.
  - `obs_event!` passes because its inner `::tracing::event!` carries `#[allow(clippy::disallowed_macros)]`.
  - Whether HEAD's macro already carries that allow is for research to check.
- **Local allows.** Per obs-plan §11 Logs (print-ban bullet), local `#[allow]` is admissible only on the output modules that own `--json`, human CLI stderr, the hook decision body and the one-time `ui` launch line, plus the fake agent's crate-level allow. Which of these modules exist at HEAD is research's question.
- **Gate commands.** Per obs-plan §9 Gate commands:
  - G1, G2 and G3 are copied verbatim into `ci.yml`, each as its own `run:` step with `shell: bash` on all three OSes.
  - A SHA-pinned install of an official PCRE2 ripgrep release, followed by a `rg --pcre2-version` check step, must come before G1 and G3.
  - Only rg exit 1 passes G1 and G3. Exit 0, exit 2 and exit 127 all fail (also in §10 Build/deploy failure conditions).
  - G1's scope is the whole workspace: root bin, `crates/*`, `tests/` and `fuzz/`.
  - Any local-gate equivalent of G3 (item 5 of the scope) must keep the same fail-closed exit semantics and the same file globs (`Cargo.toml`, `config.toml`, `*.yml`, `*.yaml`, hidden files included).
- **G2 and G4.**
  - G2 must first assert that the scope is non-empty, then count `event:"panic"` over home-level role files only, skipping torn lines. It excludes `detail-*.ndjson` (per obs-plan §9 G2 and §10 Counting rule).
  - G4 (`id: schema-conformance`, `if: always()`, after the last test step) validates role lines against `schemas/diag-line.v1.json` and `instances/*/diagnostics/detail-*.ndjson` lines against `schemas/diag-detail.v1.json`. It skips and counts non-JSON lines. It prints only file, line number and the failing keyword, never the line content (per obs-plan §9 G4, §8 Detail-file scope).
- **Step order.** Per obs-plan §9 Step order and conditions:
  1. Tests, then G2 and G4, both `if: always()`.
  2. The `if: failure()` harness capture into `target/agent-run/`, before the scan.
  3. `id: secret-scan`, `if: always()`, over `target/e2e-home/**/diagnostics/*.ndjson`, `target/agent-run/*` and `target/nextest/ci/junit.xml`.
  4. The `diag-<os>`, `harness-<os>` and `junit-<os>` uploads, gated on `steps.secret-scan.outcome == 'success'` combined with `always()` / `failure()`, never bare.
  5. The `secret-scan-<os>` hit-report upload from `target/secret-scan/`, `if: always() && steps.secret-scan.outcome == 'failure'`.

  Uploads use SHA-pinned `actions/upload-artifact` v7.0.1 with `retention-days: 7` (§9 Telemetry artifact handling, Artifact retention). `harness-<os>` must be its own artifact, not `diag-<os>`, because artifacts are immutable.
- **Homes and panic hook.**
  - `ci.yml` must set `AGENT_RUN_KEEP_HOMES=1` so homes survive until G2, G4, the scan and the uploads have read them (per obs-plan §9 Step order item 1, overseer fix pass 2 B1).
  - Integration and E2E homes must live under `target/e2e-home/` (per §9 Pipeline integration, Integration tests row). Whether HEAD's tests and harness already place homes there is research's question (scope item 6).
  - The panic hook must be the first statement of `main`, must never call the default hook, and must never write to stderr (per obs-plan §3 OTel SDK init step 1 and §7 Panic hooks).
  - Every profile must keep `panic = "unwind"` (§7 Panic hooks, "Unwinding is required"). G3 enforces this.

## Patterns to follow
- **Two-way lint proofs** (obs-plan §3 `logger-stack-install` and `obs-ci-gate-wire`): an `obs_event!` in a caller crate lints clean while a raw `tracing::info!` in the same crate fails; a `println!` in a `hook`/`run` path fails while an output module's `#[allow]` passes.
- **Non-empty-scope check before any vacuous-pass gate** (obs-plan §9 G2): `test -n "$(find … -print -quit)"`, and `jq -R -n -e` with `fromjson?` so an empty input or a torn line cannot silently pass or crash.
- **Evidence without content** (obs-plan §8 Integration points item 6, Scan failure; §9 G4):
  - The scan hit report and the G4 output carry file, line, byte offset or pattern class, or the schema keyword.
  - They never carry matched bytes or line content, because detail lines carry the canary.
- **Exit-code-only verdicts** (obs-plan §10 Error budget, Measurement): panics are measured by G2's exit code, schema failures by G4's, and scan hits by the `secret-scan` step's. The uploaded `diag-<os>` is diagnosis input, not the measurement surface.
- **Obs code under cargo-mutants** (obs-plan §9 Pipeline integration, Mutation row; §10 Build/deploy failure conditions):
  - The panic hook and `main` / dispatch obs call sites are mutation-tested like product code.
  - A surviving (not caught) mutant in obs code is a build failure. This bears on scope item 7: timeouts must become caught mutants.
  - Whether HEAD's in-test deadlines already sit below the per-test mutants timeout is research's question.

## Anti-patterns to avoid
- **Uploads** (per obs-plan §11 CI):
  - NEVER upload `diagnostics/` before the secret scan passes.
  - NEVER use a bare `always()` / `failure()` on uploads.
  - NEVER use unpinned Actions.
  - NEVER lose a failing job's diagnostics. G2, G4 and the scan are `if: always()`, and a re-run is not a substitute under `retries = 0`.
- **Retries and timeouts** (per obs-plan §11 SLO): NEVER add retry-once policies for panics or flaky telemetry assertions. For the mutants fold, raising the timeout or retrying is out; fixing the cause is the path. NEVER define a gate without an exit-code-setting assertion (also §11 CI, human-review-gated analysis).
- **Env metadata** (per obs-plan §11 CI and §9 CI-specific resource attributes): NEVER inject CI env metadata into product log lines. `AGENT_RUN_KEEP_HOMES` is harness/test configuration, not a product log input.

## Contract bindings
- **obs ↔ tests §3 (`cleanup` step 6, `AGENT_RUN_KEEP_HOMES`):** the home-retention mechanism that G2's non-empty check depends on. The test-home layout under `target/e2e-home/` is tests-owned (obs-plan §9 Pipeline integration; §9 Step order item 1).
- **obs ↔ tests (secret-scan):** the scan's test body and canary value are tests-owned. obs owns step placement, `if:` conditions, upload gating and the hit-report shape (obs-plan §3 Bootstrap phases Ownership; §8 Integration points item 6).
- **obs ↔ tests (G4):** obs owns `schemas/diag-line.v1.json` and `schemas/diag-detail.v1.json`. The G4 check body and the JSON Schema validator crate are tests-owned (obs-plan §9 G4).
- **obs ↔ security (NEVER-log floor):** security owns the floor list, and the obs gates (secret scan, G1) enforce it mechanically (obs-plan §3 Bootstrap phases Ownership; §11 PII Scrubbing).

## Acceptance criteria contributions
- Removing `[lints] workspace = true` from any product member makes the CI member-list assertion fail. Only `viola-e2e` lacks it. A `println!` in a `hook`/`run` path fails clippy while the output-module `#[allow]` sites and the fake agent's crate-level `#![allow]` pass. A raw `tracing::info!` outside `viola_core::obs` fails `disallowed-macros` while `obs_event!` lints clean. (per obs-plan §3 Bootstrap phases → `obs-ci-gate-wire`; §11 Logs)
- `ci.yml` contains the SHA-pinned PCRE2 rg install plus the `rg --pcre2-version` step, and G1, G2 and G3 verbatim as separate `shell: bash` steps. G1 and G3 fail on rg exit 0, 2 or 127. G2 fails on an empty `target/e2e-home` role-file scope. (per obs-plan §9 Gate commands; §10 Build/deploy failure conditions)
- In `ci.yml`, the step order is: G2 and G4, then the failure-only harness capture, then the `if: always()` secret scan, then the `diag-`, `harness-` and `junit-<os>` uploads gated on `steps.secret-scan.outcome == 'success'`, then the `secret-scan-<os>` upload on scan failure. Uploads are pinned to v7.0.1 with `retention-days: 7`, and `AGENT_RUN_KEEP_HOMES=1` is set. (per obs-plan §9 Step order and conditions; §9 Telemetry artifact handling)
- A green CI run leaves at least one home-level role file under `target/e2e-home/`. G2 counts 0 `event:"panic"` lines, G4 counts 0 conformance failures, and the scan has 0 hits. (per obs-plan §10 Error budget; §9 Pipeline integration)

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton** (§3 `obs-ci-gate-wire`, §11 Logs):
  - The fake agent is a root-package `[[bin]]`, so it cannot have its own lint table. It is exempted by a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs`.
  - Only `viola-e2e` omits `workspace = true`, and the CI member-list assertion names only `viola-e2e`.
  - Why: the prior chunk's Deviation 7 showed a separate lint table is impossible for a root-package bin. This is the basis of scope item 3.
- **2026-09-24-supply-chain-and-workflow-gates** (§9 Platform): `ci.yml` stays the single push/PR workflow, and `nightly.yml` beside it runs only the weekly `cargo deny check advisories`. This chunk's gates therefore go in `ci.yml`, not a new workflow.
- **2026-09-24-diagnostics-plane** (§3 Logging stack, §8 Default-deny, §11 Logs):
  - The schemas enforce default-deny with top-level `unevaluatedProperties: false`, and `diag-detail.v1.json` is self-contained with the event enum inlined. G4 depends on both.
  - `obs_event!` attaches `event`, `process` and `instance` but not `corr`; the caller supplies `corr`. The §11 raw-tracing-ban justification was narrowed to match.
  - A CARRY (`corr` required in `diag-line`) sits on the wrapper-channel chunk, not here.
- **2026-09-24-log-redaction-and-never-log-floor** (§7 Platform pick): anyhow is scoped to the root-bin dispatch edge and its catch-site reporter `viola::obs::report_internal_error`. This is context for the `main` / `dispatch` panic and exit surface touched by the mutants fold (scope item 7). §7 panic and catch-site mandates are unchanged.
