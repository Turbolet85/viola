@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps 
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: or a username split across a row seam is refused;
source: dec-pvviak6k «Screen fixtures keep only signature rows — A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the pro»
replace-with: or a username split across a row seam is refused (save that on a failed recording verify a hidden rig-only option writes one first-screen file of whole scrubbed rows per run short of the input box, kept local and git-ignored, an unclean set refused whole);
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: The live rig runs its child under a private `CLAUDE_CONFIG_DIR` in its own scratch,
source: dec-4yctyxtx «Live rig's child config seeded from bundle logic only — The live rig seeds its child's private config only with keys whose values come from the CLI bundle's own logic, none read from the user's config»
replace-with: The live rig runs its child under a private `CLAUDE_CONFIG_DIR` in its own scratch, seeded only with keys whose values come from the CLI bundle's own logic, none read from the user's config (used-config markers and the browser-tools prompt turned off),
