
## 2026-10-04-windows-boundary-mutation-workflow — the Windows leg of the boundary audit
**Section:** §9 CI Integration (Platform · Mutation row · the workflow / tool-pin paragraph · the concurrency note · Test report format `mutants.out`) · §10 Quality Gates (Mutation gate)
**Change:**
- Platform: `windows-mutants.yml` (dispatch-only) joins `ci.yml` and `nightly.yml`.
- Mutation row: was "none in CI"; now no push/PR job or gate. The audit's Windows leg is `windows-mutants.yml` (`workflow_dispatch` only, no inputs, `mutants (<package>)` per package on `windows-2025`, `fail-fast: false`, 120 min) running `scripts/agent-run.ps1 run --mutants --package <member> --file …`, report-only, no cache or upload.
- Workflow paragraph: three workflows. The new one's `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` is asserted equal to the `test` job's line by `tests/contract_windows_mutation_scope.rs`.
- Concurrency note: three workflows, zizmor pedantic `concurrency-limits` 2 low → 3 low.
- `mutants.out` bullet: "no CI job runs mutation" → no push/PR job; the dispatch workflow uploads nothing.
- §10: the audit measures `cfg(windows)` code by dispatching the workflow, verdict `package`, never a gate. Its jobs read red until the audit classifies compiled-out `#[cfg(unix)]` twins (19 of 25 misses in run 37174673472; 0 timeouts over 508).
**Why:** founder ruling C2 (2026-10-04); the 2026-09-28 no-gate ruling stands.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/
