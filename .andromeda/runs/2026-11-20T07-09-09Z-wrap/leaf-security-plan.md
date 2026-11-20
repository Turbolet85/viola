@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=36 class=stale
leaf-text: into its own CC scratch home only
source: dec-nyf6efjo «except: The test-only live rig seeds a stamp kept from its own verify scratch home into its own CC or R3W scratch home only.»
replace-with: into its own CC or R3W scratch home only
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: The live rig keeps a stamp from its CS scratch home owner-only and no-clobber and seeds it only into its own CC scratch home; CC with no kept stamp refuses before its ledger line and before any scratch.
source: dec-cgqgi7dg «The live rig keeps a stamp from its CS or WS scratch home owner-only and no-clobber, and seeds it only into its own CC or R3W scratch home;»
replace-with: The live rig keeps a stamp from its CS or WS scratch home owner-only and no-clobber and seeds it only into its own CC or R3W scratch home; CC with no kept stamp refuses before its ledger line and before any scratch, R3W before its ledger line.
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: The live rig's config-top reading keeps only
source: dec-rbtmsa6k «The live rig's R3W arm reads from the events log in its own scratch home only viola's own wheel lines and the prompt's origin word, and records the captured prompt's key names, never their values.»
replace-with: The live rig's R3W arm reads from the events log in its own scratch home only viola's own wheel lines and the prompt's origin word, and records the captured prompt's key names, never their values. The live rig's config-top reading keeps only
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: The live rig's own hook capture
source: dec-zhaje3vf «The live rig's HB arm types one x and one DEL into a fresh box in its own unwrapped pty before any send, and records only cell attributes and counts, never screen text.»
replace-with: The live rig's HB arm types one x and one DEL into a fresh box in its own unwrapped pty before any send, and records only cell attributes and counts, never screen text. The live rig's own hook capture
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=71 class=omission
leaf-text: matched on the prompt's raw start;
source: dec-qtnov5wk «except: The wrapper relabels a keyless human-filed prompt told as a CLI-scheduled wake origin harness.»
replace-with: matched on the prompt's raw start, and the wrapper relabels a keyless human-filed prompt told as a CLI-scheduled wake origin harness;
@leaf_edit n=6 leaf=.claude/rules/security.md line=25 class=omission
leaf-text: `scheduled_wakes` on turn-ended, no element read or kept
source: dec-cbbijivg «Stop's session_crons is carried only as its length, scheduled_wakes on turn-ended, which the wheel rule also reads; no element is read or kept;»
replace-with: `scheduled_wakes` on turn-ended, which the wheel rule also reads; no element read or kept
