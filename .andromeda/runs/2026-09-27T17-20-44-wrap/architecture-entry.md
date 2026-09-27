
## 2026-09-27-epoch-2-cleanup — host mutation scratch, run archive, test-support feature, wsl-exec.sh, release-check feature refusal
**Section:** §Occupied Resources (Workspace crates, Environment variables, Filesystem, Repository) · §Infrastructure Patterns (Project directory structure: `scripts/`; CI/CD approach: target job 6, the local pre-push gate bullet)
**Change:**
- Workspace crates: viola-channel's test-only `test-support` feature (`pub mod test_support`), enabled only by the root `[dev-dependencies]`.
- Env vars: `AGENT_RUN_KEEP_FAILED` was "read only by the root test chain"; now also read by viola-e2e's `harness_lifecycle` `Booted` guard.
- Filesystem: test-only `<temp dir>/viola-pty-watch/<test name>.report`.
- Repository: `target/run-archive/<n>/` (newest 10, never uploaded) and the host mutation scratch `<repo parent>/viola-mutants-scratch/` (guard, wipe, `TMP`/`TEMP` + `--output`, path never printed).
- Tree: `scripts/wsl-exec.sh` (operator aid; no gate/harness/plan runs a command through it); `release-check.sh` also refuses a test-only feature.
- CI/CD: job 6 fails first on a `test-support` / `fake-agent` artifact (probe `5/5`); the pre-push bullet names the Windows leg's host scratch; the operator pass no longer starts with stopping rust-analyzer (as measured: three pre-pushes and two scoped runs with it running).
**Why:** the Epoch 2 cleanup chunk as built. `wsl-exec.sh` is a boundary widening ratified live by the overseer at this wrap under the founder's 2026-09-27 ruling (security-plan Decisions Log). Rejected: registering the split submodules in the tree (registry over-reach) and naming cargo-machete in §Stack (the code audit's instrument, not a project gate).
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/
