@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: Run B's agents read as `Agents.recorded-1.json` (Run B's own row only)
source: ctr-dxsktenx «The recorded env-names fixture Env.recorded-1.json holds names only, never a value, in the shape v 1 plus added; it is recorded by viola verify through the record scrub and checked by its own hygiene class.»
replace-with: Run B's agents read as `Agents.recorded-1.json` (Run B's own row only), the env-names fixture `Env.recorded-1.json` (names only, never a value: `v` 1 plus `added`)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=25 class=omission
leaf-text: (arch §Standard Contracts); a version
source: ctr-asj7rjxh «Holds read, added, unknown, other_count and unshaped from the identity-floor probe, names only, with every kept name removed.»
replace-with: (arch §Standard Contracts; `measured.env_probe` holds the identity-floor probe's `read`, `added`, `unknown`, `other_count` and `unshaped`, names only, every kept name removed); a version
