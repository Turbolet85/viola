
## 2026-10-19-overseer-switch-to-viola — verify's print-mode probe leaves an empty CLI project dir
**Section:** §Established Decisions ([Plugin Scope] rationale, `viola verify`'s transient unwrapped children).
**Change:** The print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript), the same accepted residual as Runs B–D's transcripts, as measured on 2.1.296 and 2.1.302 on the Linux dev host. Was unstated (the rationale named Runs B–E's residual only).
**Why:** The omission was measured at the switch's verify residual count (4 new `~/.claude/projects/` entries per verify, one of them this dir). The founder accepted it into the same class by his own word, live, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
