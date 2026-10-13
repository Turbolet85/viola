
## 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair
**Section:** §Surface: cli → Component Patterns 5 (`verify`) · Exit-code phraseology (row 1)
**Change:** A `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary. Its stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (pattern 2's fixed form, plain ASCII, no path or pid), stdout is empty, and it exits 1. Exit-code row 1 lists the pair beside verify's refusals.
**Why:** The operator's P4 answer chose exit 1 with a fixed pair over a new code. The wording is a planning form on the operator's word, and the founder's wording is owed in his next batch.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
