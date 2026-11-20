@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/testing.md lines=124
last: - 2026-11-20: A move of the fake agent's default CLI version
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=59 class=omission
leaf-text: Stamps conflict resolved: `viola verify` against the fake agent is the only writer of `ledger/stamps.json` in tests and CI.
source: dec-drosu4yn «Only the wrapper writes snapshot.json and only verify writes stamps.json, the verify a keyed start runs included; every other writer-capable entry point leaves both byte-identical.»
replace-with: Stamps conflict resolved: `viola verify` against the fake agent (the verify a keyed start runs included) is the only writer of `ledger/stamps.json` in tests and CI; every other writer-capable entry point leaves it byte-identical.
