@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps 
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/rules/security.md line=19 class=stale
leaf-text: (exception: StopFailure is registered with no capability ledger row and no verify probe, owned by the entry A live StopFailure payload)
source: dec-t4wfjgzz «except: StopFailure is registered with no capability ledger row and no verify probe.»
replace-with: (exception: StopFailure is registered with no capability ledger row and no verify probe)
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: The live rig's own UserPromptSubmit capture (a ratified boundary widening) reads the hook payload under `MAX_FRAME` with the default JSON depth and writes one owner-only line of key names matching `[a-z_]{1,40}`, others counted, and one closed source word, never a value; a non-object or over-long payload writes unread.
source: dec-raynvazb «The rig's hook capture reads a payload under MAX_FRAME, default JSON depth, and writes one owner-only line: key names matching [a-z_]{1,40}, others counted; a UserPromptSubmit source word or a StopFailure»
replace-with: The live rig's own hook capture (a ratified boundary widening) reads a payload under `MAX_FRAME` with the default JSON depth and writes one owner-only line: key names matching `[a-z_]{1,40}`, others counted; a UserPromptSubmit source word or a StopFailure error word of the CLI's 13, else `other`; two text lengths, never a text; a bad payload writes unread.
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: the VH arm, run under the real home, makes no copy and reads no credentials file.
source: dec-fytc7kyo «The live rig's SF arm runs the real claude inside a network-closed bwrap sandbox with its own network namespace; an absent or failing sandbox, or one sharing the rig's namespace, refuses the arm with»
replace-with: the VH arm, run under the real home, makes no copy and reads no credentials file. The live rig's SF arm runs the real claude inside a network-closed bwrap sandbox with its own network namespace; an absent or failing sandbox, or one sharing the rig's namespace, refuses the arm with `refused: sandbox`, exit 3, uncounted, before any credentials read or ledger line.
