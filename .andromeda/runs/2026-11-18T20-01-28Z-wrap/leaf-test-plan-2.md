@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_read leaf=.claude/rules/testing.md lines=121
last: - 2026-11-18: When a change makes the product write into a u
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `-
@leaf_edit n=1 leaf=.claude/rules/testing.md line=34 class=omission
leaf-text: A home a test gives to `viola verify --record` comes from the shared record-home helper and fixture, and no component of its path is named `home`.
source: dec-ioj2wnmy «A home a test gives to viola verify --record comes from the shared record-home helper and fixture with no path component named home, and the live rig's record arm refuses record-home, exit 3, uncounted»
replace-with: A home a test gives to `viola verify --record` comes from the shared record-home helper and fixture with no path component named `home`, and the live rig's record arm refuses `record-home` (exit 3, uncounted) on a scratch base or its canonical form naming `home`, before any credentials read or ledger line.
