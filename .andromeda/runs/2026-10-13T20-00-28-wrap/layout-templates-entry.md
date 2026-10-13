
## 2026-10-13-interrupted-verify-cleanup — verify's interrupted form
**Section:** §Surface: cli → Output structure — `viola verify` · Component — Primary content block 2 · Component — Footer / terminator
**Change:**
- Output structure gains a third example: a run stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` on stderr, with stdout empty, no summary and no stamp, and exits 1. The exit list reads: 1 on a failing row, a refusal or a stop.
- The pair joins verify's fixed-message exit-1 pairs in Primary content block 2.
- Was: the summary closes the run. Now it closes a completed run; a stopped run ends on its `hint:` line.
**Why:** The operator's P4 answer gave the interrupted run exit 1 and a fixed pair; the wording is a planning form with the founder's wording owed.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
