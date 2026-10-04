
## 2026-10-03-mutation-scoring-completion — pre-push native on the Linux host
**Section:** §Stack and Technologies (CI/CD row, Browser e2e row) · §Infrastructure Patterns → CI/CD approach · §Infrastructure Patterns → Project directory structure · §Occupied Resources (CI workflow data lines, test-side install sites, `target/pre-push/`)
**Change:**
- Was: `pre-push` runs on the Windows dev host, drives a WSL2 `Ubuntu` clone (sync, tree-id check, 40 GiB cache), terminates the VM (`vm-release`), then runs `windows-tests` on the host with `CARGO_BUILD_JOBS=16`, refusing `pre-push-windows-only` elsewhere; provisioned by `scripts/wsl-provision.sh` (with an operator-only root `--install-deps`).
- Now: `pre-push` runs on a Linux host only (`pre-push-linux-only`, exit 2, elsewhere), natively in the working tree (no clone, no sync). Every child is `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` from the repository root, `<home>` the passwd entry's field 6 read by two PATH-only probes (`tool-missing` `passwd-home` on failure). Stages `tools → linux-tests`; it installs nothing.
- `scripts/wsl-exec.sh` and `scripts/wsl-provision.sh` leave the directory tree; `target/pre-push/` and `~/.cache/viola-provision/` leave Occupied Resources; the install sites live in the Linux host user's passwd home; `wsl-provision.sh` leaves the `NODE_PIN_*` parsers.
- The CI/CD hyperfine clause and the jobs clause no longer cite the WSL provisioning; `run --mutants` lists the whole-member `--package <member>` form.
**Why:** the dev host is Linux since 2026-10-03; the WSL clone was that host's filesystem bridge (overseer, founder-delegated, at plan review). No widening: only HOME and PATH cross, HOME from the passwd entry, never the harness's `$HOME` (shown to the founder at plan review).
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/
