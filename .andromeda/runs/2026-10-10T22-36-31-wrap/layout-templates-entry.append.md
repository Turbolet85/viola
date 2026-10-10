
## 2026-10-10-budget-governor — the two release --budget output lines
**Section:** §Output structure — wheel, handoff and dialog verbs
**Change:** the verb block shows both outcomes of `viola release <name> --budget`: `<name>  budget gate lifted until <end>  wheel <holder>` when a pause stood and `<name>  budget gate open  wheel <holder>` when none did, each one stdout line, exit 0, the wheel left where it is. `<end>` is the reply's `override_until` as `HH:MM:SSZ` on the current UTC date and `YYYY-MM-DD HH:MM:SSZ` on any other; a value that is no RFC 3339 instant prints escaped. Was the time-only lifted line alone. The date form and the `budget gate open` line are marked planning forms, the founder's wording owed.
**Why:** the block showed one of the two lines the verb now prints, and only one form of its time. The two new forms landed on the operator's word and are not worded by the founder.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/
