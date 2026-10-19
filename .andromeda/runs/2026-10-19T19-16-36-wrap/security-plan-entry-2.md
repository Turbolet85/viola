
## 2026-10-19-overseer-switch-to-viola — verify's print-mode probe residual accepted
**Section:** §Data Protection (Accepted risk: verify's trusted runs).
**Change:** After "Run A leaves nothing": the print-mode probe leaves the CLI's empty project dir for its cwd `ledger/probes/<pid>/` under `~/.claude/projects/` (a `memory/` dir, no transcript), accepted into the same class, as measured on 2.1.296 and 2.1.302 on the Linux dev host. Was unstated.
**Why:** Measured at the switch's verify residual count. The founder's own word, live, relayed by the overseer (`inputs#I21`), accepts it into the class of the Run B–D transcripts. viola itself still writes nothing under `~/.claude`: this dir is the CLI's write.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
