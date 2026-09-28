
## 2026-09-28-capability-ledger-and-viola-verify — `MAX_FRAME` consumers extended, verify's CI split
**Section:** Input Validation Constants; Bootstrap phases (CI/CD, "run only locally")
**Change:**
- `MAX_FRAME` also bounds the `--version` reads of `run`'s gate and `viola verify` (stdout and stderr drained through the cap, killed at 5 s), the `hook --capture` stdin, and the `ledger/stamps.json` read (`take(MAX_FRAME + 1)`: over the cap reads as absent in `update_stamps`, an error in `read_stamps`).
- "The real `claude` CLI and `viola verify` run only locally" is now: the real CLI runs only locally; `verify` runs in CI only against the fake agent, its real-CLI probe and `--record` only locally.
**Why:** the chunk's new readers are capped as §Input Validation requires; only the list was stale. The CI line cited architecture's CI/CD note, amended in the same pass.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
