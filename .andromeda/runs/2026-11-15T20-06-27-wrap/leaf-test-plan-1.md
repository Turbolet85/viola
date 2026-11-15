@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_read leaf=.claude/rules/testing.md lines=117
last: - 2026-11-15: A removal sweeps every test that reads the remove
@leaf_edit n=1 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: - Never hand-write `ledger/stamps.json` (only `viola verify` against the fake agent), `snapshot.json`
source: dec-ioj2wnmy «decision/record-home-names-no-home · Record homes never named home — A home a test gives to viola verify --record comes from the shared record-home helper and fixture, and no component of its path is named home.»
replace-with: - A home a test gives to `viola verify --record` comes from the shared record-home helper and fixture, and no component of its path is named `home`. Never hand-write `ledger/stamps.json` (only `viola verify` against the fake agent), `snapshot.json`
