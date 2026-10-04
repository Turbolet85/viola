
## 2026-10-03-mutation-scoring-completion — pre-push native on the Linux host
**Section:** §3 → 5-command implementation (`pre-push` block; Closed enums `pre-push`) · §3 → Bootstrap phases (`ci-tool-install`) · §9 CI Integration (tool-pin paragraph) · §10 Performance budgets (Status)
**Change:**
- `pre-push` is Linux-host only, runs natively in the working tree (no clone, no sync), every child `/usr/bin/env -i HOME=<home> PATH=<home>/.cargo/bin:<home>/.local/viola-node/bin:/usr/local/bin:/usr/bin:/bin` from the repository root, `<home>` the passwd field 6 (two PATH-only probes). Stages `host · tools · linux-tests`, `ok:true` when `linux-tests` is green; the document is `{v,cmd,ok,reason?,detail?,stage,linux{run,browser,gate}}`, never a home or repository path.
- Closed enums: `reason` `pre-push-linux-only` (exit 2), `tool-missing`, `tool-pin-mismatch`, `linux-document-unreadable`; `detail` adds `passwd-home`; `stage` `host`, `tools`, `linux-tests`. Retired: `pre-push-windows-only`, `sync-failed`, `sync-mismatch`, the details `wsl-distro-ubuntu`, `source-path`, `patch`, `clone`, `fetch`, `reset`, `clean`, `apply`, `cache`, the stages `sync`, `cache`, `vm-release`, `windows-tests`, and the document's `sync`, `cache`, `vm`, `windows` sections.
- §9 and Bootstrap: `pre-push` checks the host's pins and installs nothing; the WSL provisioning sentences are retired. §10 perf Status: "`test` and the pre-push carry no perf step".
**Why:** the WSL gate retired with the Windows dev host (overseer, founder-delegated, at plan review: in place, no clone). These closed-enum changes are recorded here in place of a Decisions Log entry.
**Ref:** .andromeda/runs/2026-10-04T01-02-04-wrap/
