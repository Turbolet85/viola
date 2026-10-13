
## 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair
**Section:** §1 Surface: cli → the CLI verbs entity · §8 Error recovery → CLI
**Change:** A `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary. It prints only one `unable: verify was stopped before it finished` line and one `hint: run viola verify again to stamp this CLI` line last on stderr (plain ASCII, design-system pattern 2, pinned by the stop table in `tests/cli_verify.rs`), and exits 1. The stop path adds no time bound: its waits end at the stop, and its kill and pump join end at once.
**Why:** The operator's P4 answer gave the interrupted run exit 1 and a fixed pair; the wording is a planning form with the founder's wording owed.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
