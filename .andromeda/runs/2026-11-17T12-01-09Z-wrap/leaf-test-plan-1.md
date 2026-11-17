@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is not
@leaf_edit n=1 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: the started-child guard and runner in `cli.rs`
source: dec-gzyga4om «Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper or fake agent»
replace-with: the started-child guard and runner in `cli.rs` (every long-lived viola child a root test spawns goes through its tied builder: tied to the spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, spawned only on a thread that outlives it, with the Drop guard's kill then wait still covering the normal end; the census `cross_killed_test_census_finds_no_untied_viola_spawn` fails any other spawn of the viola binary constant)
