# obs extract

## Relevance
Partial. The chunk adds no product telemetry: no spans, events, fields or sinks. It does touch three obs-owned CI surfaces: the secret-scan scope over `target/agent-run/`, the Mutation row of the pipeline table, and the content rule for the per-leg `mutants-verdict-<os>.json` upload.

## Constraints
- The secret scan's input set is fixed: `target/e2e-home/**/diagnostics/*.ndjson`, `target/agent-run/*` and `target/nextest/ci/junit.xml`, with `id: secret-scan` and `if: always()` (per obs-plan §9 Step order and conditions, step 3). Narrowing the scope to drop stale mutation-leg residue such as `target/agent-run/chunk.diff` must keep the harness capture in scope. That capture is `target/agent-run/{logs.ndjson,status.json}`, written by the `if: failure()` step before the scan (per obs-plan §9 Telemetry artifact handling, `harness-<os>` row; §8 PII Scrubbing, integration point 6). Research should answer whether `secret_scan.rs:78` already separates harness capture from mutation residue.
- Critical-class patterns (`?t=`, token, `Cookie`, `CLAUDE*` values) apply to `target/agent-run/*`. Only the canary rule exempts those files, the same way it exempts `detail-*.ndjson` (per obs-plan §8 PII Scrubbing, integration point 6, the `harness-<os>` bullet). Any exclusion of residue must not create a Critical-pattern blind spot over files that get uploaded.
- A secret-scan hit is a build failure, and the §10 budget is zero hits (per obs-plan §10 Build / deploy failure conditions; §10 zero-tolerance definition). Excluding scan-matching residue is admissible only if the excluded files are never uploaded. Whether `chunk.diff` could reach any artifact path is for research.
- `mutants-verdict-<os>.json` is uploaded unscanned. It is admissible only because its content is repo-relative source locations and mutant outcomes, with no absolute path, argv, log path or test output. `mutants.out/` is never uploaded (per obs-plan §8 PII Scrubbing, integration point 6, "Unscanned uploads, admissible by content"). If the union verdict needs a per-leg "compiling leg" or unviable marker in the verdict file, the added data must stay in that admissible class. Otherwise the guard applies: the file is scanned before upload, or the member is dropped.
- The Mutation row states the union: a mutant is red only when no leg caught it, and before the union a leg whose unviable mutants outnumber its caught ones is red at its own run (per obs-plan §9 Pipeline integration, Mutation row, citing test-plan §10). The new rule, where a mutant unviable on the leg that compiles its code is not a survivor, changes what this row asserts.
- A surviving cargo-mutants mutant in obs code is a build failure (per obs-plan §10 Build / deploy failure conditions). The union change must not let a MISSED obs-code mutant on the compiling leg turn green.
- CI identity (run id, commit sha) never enters product log lines, and env vars are not a configuration channel for the product (per obs-plan §9 CI-specific resource attributes). An `AGENT_RUN_CHUNK_BASE` change is harness and CI configuration only, and must not flow into viola's diagnostics.

## Patterns to follow
- Scan-gated uploads: `diag-<os>`, `harness-<os>` and `junit-<os>` run on `if: always()` or `failure()` combined with `steps.secret-scan.outcome == 'success'`. The hit report goes to `target/secret-scan/` (file, line, offset, pattern class, never the matched bytes) and is uploaded as `secret-scan-${{ matrix.os }}` on scan failure (per obs-plan §9 Step order steps 3–5; §8 integration point 6, "Scan failure").
- Any scan-gated or verdict-bearing CI step keeps an exit-code verdict read with `jq`. No human-review gate (per obs-plan §11 CI; §11 Universal).
- The unscanned-upload inventory records an admissibility-by-content measurement for each member it admits (for example "0 absolute paths as measured"). A new or reshaped verdict member follows the same measure-then-admit pattern (per obs-plan §8 integration point 6).
- Gates are proven both ways, with a planted violation plus a clean control. The chunk's planted-secret red test and its negative MISSED-mutant test fit this pattern (per obs-plan §3 Bootstrap phases, obs-ci-gate-wire; logger-stack-install "proven both ways").

## Anti-patterns to avoid
- NEVER upload `diagnostics/` artifacts before the secret-scan step passes, and NEVER lose the failing job's diagnostics through bare `always()` or `failure()` conditions (per obs-plan §11 CI).
- NEVER add retry-once policies for flaky telemetry assertions, and NEVER define a verdict without an exit-code-setting assertion (per obs-plan §11 SLO). This applies to the union gate and the base-resolution probe.
- NEVER use unpinned Actions; `upload-artifact` stays SHA-pinned and zizmor-checked (per obs-plan §11 CI).

## Contract bindings
- **obs ↔ tests (secret scan).** obs owns the scan's scope and its upload gating (§9 step 3, §8 integration point 6). The scan body and canary value are tests-owned (per obs-plan §3 Bootstrap phases, Ownership). The residue-scope change lands in tests-owned `secret_scan.rs` but must keep obs's declared input set.
- **obs ↔ tests (mutation verdict).** The obs §9 Mutation row cites test-plan §10 Mutation gate. The union-rule change is a test-plan amendment, and the obs §9 row is a cross-master citation that must be re-swept to match.
- **obs ↔ security (unscanned uploads).** The admissible-by-content rule for `mutants-verdict-<os>.json` is an operator-ratified boundary (§8 item 6). Widening its content is the never-routine "Boundary widening" class.

## Acceptance criteria contributions
- After the scope change, the secret scan still reads `target/agent-run/{logs.ndjson,status.json}`, `target/e2e-home/**/diagnostics/*.ndjson` and `target/nextest/ci/junit.xml`. A planted Critical-class pattern in the harness capture still fails the scan, and stale mutation-leg residue is no longer read (per obs-plan §9 Step order, step 3; §8 PII Scrubbing, integration point 6).
- Every `mutants-verdict-<os>.json` produced under the new union holds only repo-relative source locations, mutant outcomes and any added leg or viability data. It has 0 absolute paths, argv, log paths or test output, as measured on the chunk's CI artifacts (per obs-plan §8 PII Scrubbing, integration point 6, "Unscanned uploads, admissible by content").
- A MISSED mutant on the compiling leg, including one in obs code, keeps `mutants-verdict` red (per obs-plan §10 Build / deploy failure conditions; §9 Pipeline integration, Mutation row).
- The obs-plan §9 Mutation row agrees with the shipped union rule after the wrap: it is amended, or recorded as unchanged with a reason (per obs-plan §9 Pipeline integration, Mutation row).

## Relevant amendment history
- **2026-09-24-quality-gates:** added `mutants-verdict-<os>.json` to the unscanned-uploads inventory, admissible by content with repo-relative paths only (`mutants.out/` never uploaded). It also named the `mutants-verdict` union gate as the Mutation-row consumer. Why: the overseer ruling "Ratify both by content". Any reshaping of the verdict file in this chunk re-enters that ratified boundary.
- **2026-09-24-epoch-1-cleanup:** the §9 Mutation row gained the per-leg rule that a leg is red when its unviable mutants outnumber its caught ones. It came from a cross-master citation of the test-plan §10 amendment and was folded by the sweep. It is the direct precedent: a test-plan union or unviable change here will again require re-sweeping the obs §9 Mutation row. §1 (the verbatim scope copy) is excluded from sweeps.
- **2026-09-24-workspace-tree-and-code-graph-planes:** added the `supply-chain` artifact to the unscanned-uploads inventory. The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped. It is the ratified pattern to follow if verdict-file content grows.
- **2026-09-24-supply-chain-and-workflow-gates:** `ci.yml` stays the single push/PR workflow, beside the scheduled `nightly.yml`. The chunk-base and union changes belong in `ci.yml`, not in a new workflow.
