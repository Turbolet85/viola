@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/testing.md lines=126
last: - 2026-11-20: A new booted case belongs to a gate entry's fil
@leaf_edit n=1 leaf=.claude/rules/testing.md line=39 class=stale
leaf-text: the 11 identity-floor names
source: dec-j64chwgj «Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test.»
replace-with: the identity-floor names
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: runs absolute exec-form hooks from `<plugin-dir>/hooks/hooks.json`
source: ctr-2cypc2sa «The fake agent gives every SessionStart hook child each name of its set's recorded env names, always with the fixed value fake-agent.»
replace-with: runs absolute exec-form hooks from `<plugin-dir>/hooks/hooks.json` (every SessionStart hook child given each name of its set's recorded env names, always with the fixed value `fake-agent`)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: the `Agents.*.json` agents fixtures against `schemas/claude-agents-fixture.v1.json` (`agents_fixtures_pass_hygiene`),
source: ctr-dxsktenx «The recorded env-names fixture Env.recorded-1.json holds names only, never a value, in the shape v 1 plus added; it is recorded by viola verify through the record scrub and checked by its own hygiene class.»
replace-with: the `Agents.*.json` agents fixtures against `schemas/claude-agents-fixture.v1.json` (`agents_fixtures_pass_hygiene`), the env-names fixture `Env.recorded-1.json` (names only, never a value, in the shape `v` 1 plus `added`, recorded by `viola verify` through the record scrub) by its own hygiene class,
