@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=117
last: - 2026-11-15: A removal sweeps every test that reads the rem
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=25 class=stale
leaf-text: (the second and last start record,
source: ctr-hp7s2s2r «Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cau»
replace-with: (the last start record, with a `wheel{holder:"human", cause:"human-input"}` record before it where folder-trust keys pass through the wrapper, a later human key adding none,
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=30 class=omission
leaf-text: the user's settings left byte- and mtime-equal;
source: ctr-wdo4kiwe «A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.»
replace-with: the user's settings left byte- and mtime-equal, and no default home made under `--home`;
@leaf_edit n=3 leaf=.claude/rules/testing.md line=24 class=omission
leaf-text: fake scripts `fixtures/fake-scripts/<scenario>.json` (schema `schemas/fake-script.v1.json`).
source: dec-n5cm3kti «The 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases that must spell an old word, and walked»
replace-with: fake scripts `fixtures/fake-scripts/<scenario>.json` (schema `schemas/fake-script.v1.json`); the 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases that must spell an old word and walked by the fixture hygiene check.
