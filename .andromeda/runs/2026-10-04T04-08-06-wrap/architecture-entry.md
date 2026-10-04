
## 2026-10-04-windows-boundary-mutation-workflow — the dispatch-only Windows mutation workflow
**Section:** §Infrastructure Patterns → CI/CD approach · §Infrastructure Patterns → Project directory structure · §Occupied Resources (`AGENT_RUN_CHUNK_BASE`, `<repo parent>/viola-mutants-scratch/`)
**Change:**
- CI/CD approach: "two workflows" → three. `windows-mutants.yml`:
  - `workflow_dispatch` only, no `inputs:`, dispatched only at the epoch-boundary audit; never a gate, a required check or a `ci.yml` dependency, and it adds no check to a push run;
  - one job `mutants (<package>)` on `windows-2025`: `contents: read`, `timeout-minutes: 120`, `fail-fast: false`, a six-package `matrix.include` of each package's Windows-gated `files`;
  - checkout, `rustup toolchain install`, install-action with `cargo-nextest@0.9.146,cargo-mutants@27.1.0`, then one pwsh step running `scripts/agent-run.ps1 run --mutants --package <member> --file …` from step `env:`;
  - no cache, upload, secret, `concurrency:` or `needs:`; its jobs read red while scoped files carry compiled-out `#[cfg(unix)]` twins.
- `tests/contract_windows_mutation_scope.rs` keeps the file lists and pins equal to the sources and to ci.yml.
- "Neither workflow has a `concurrency:` block" → "No workflow …".
- The concurrency parenthetical was "CI runs no mutation job"; it now reads: no push or pull-request run carries one, and the workflow uploads nothing.
- The jobs paragraph's "CI runs no mutation job" → "`ci.yml` runs no mutation job", with the audit's Windows leg named.
- Directory tree: `windows-mutants.yml` added under `.github/workflows/`.
- `AGENT_RUN_CHUNK_BASE` row: was "CI runs no mutation job and never sets it"; now no CI workflow sets it, and the workflow's `--package` arm reads no base.
- `viola-mutants-scratch` row: the workflow runs the Windows `HOST_SCRATCH` arm on the runner, so the scratch sits beside the runner's checkout; documents carry only `scratch_bytes`.
**Why:** founder ruling C2 (2026-10-04, relayed by the overseer) ships a dispatch-only, non-blocking Windows mutation leg while keeping the 2026-09-28 ruling: no chunk, pre-push or blocking CI mutation gate. Not a boundary widening: it adds no permission, secret, input class, upload or action pin.
**Kept:** `ci.yml` and `nightly.yml` byte-unchanged; the push check count stays 15.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/
