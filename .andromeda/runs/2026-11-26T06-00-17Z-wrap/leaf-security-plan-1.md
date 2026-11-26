@leaf_group group=security-plan-1
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=61 class=omission
leaf-text: and the send is refused `not-delivered` / `paste-cut` (exit 13).
source: ctr-fo4qskxo «A send whose paste was cut at its compiled bound, or refused while the cut mark stands, is answered with result.refusal not-delivered/paste-cut; the CLI exits 13 with it.»
replace-with: and the send is refused `not-delivered` / `paste-cut` (exit 13), as is any send refused while the cut mark stands.
