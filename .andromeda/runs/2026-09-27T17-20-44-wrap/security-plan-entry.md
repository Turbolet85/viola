
## 2026-09-27-epoch-2-cleanup — wsl-exec.sh ratified (overseer, live), test-only feature proven absent, host scratch guard
**Section:** §Secret Management (the WSL crossing bullet) · Decisions Log (new `2026-09-27` entry)
**Change:**
- §Secret Management: the pre-push gate stays the only harness launcher into WSL2; `scripts/wsl-exec.sh` added as the one other launcher (operator aid; the same `env -i` form with the distro's HOME and PATH; only operator-typed argv and `--cd` cross). A checkable invariant: no gate, harness command or plan entry runs a distro command through it, its own `--probe` exempt.
- Decisions Log: viola-channel's `test-support` feature and release-check's refusal of `test-support` / `fake-agent` artifacts (probe `5/5`), the host mutation scratch guard (exact name, never the repository or an ancestor, no fallback), and the `wsl-exec.sh` widening with its ratification and Conditions.
**Why:** the `wsl-exec.sh` crossing is a boundary widening (playbook, never routine), ratified LIVE by the overseer at this wrap under the founder's 2026-09-27 ruling — recorded as the overseer's word, not the founder's own. The overseer then narrowed the invariant to running a command through it, so its `--probe` self-test stays a gate entry.
**Kept:** the 2026-09-26 entry's Conditions line stands as history; the new entry records it enforced.
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/
