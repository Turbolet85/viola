@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=50 class=stale
leaf-text: the test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on
source: dec-j2mfetpl «The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.»
replace-with: the test-only live rig reads its child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--dialogs` replays a pasted dialog prompt's recorded variants in claim order
source: ctr-xkyep5qa «Beside --turn-stop, --turn-fail <variant> ends the turn with the recorded set's StopFailure.<variant> payload in place of Stop.default.»
replace-with: `--turn-fail <variant>`, beside `--turn-stop`, ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`, `--dialogs` replays a pasted dialog prompt's recorded variants in claim order
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: runs absolute exec-form hooks from `<plugin-dir>/hooks/hooks.json`
source: ctr-dfv5qikx «--settings-hooks <file> names a Claude settings file whose hooks run beside the plugin's; a command string with no args is split on spaces and never run through a shell.»
replace-with: runs absolute exec-form hooks from `<plugin-dir>/hooks/hooks.json` (and, under `--settings-hooks <file>`, a Claude settings file's hooks beside the plugin's, a command string with no args split on spaces and never run through a shell)
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: its live arm refused under CI and its rehearsals over the fake agent run in CI
source: dec-k4rtddnt «The SF arm's rehearsals run unwrapped in CI; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it.»
replace-with: its live arm refused under CI and its rehearsals over the fake agent run in CI (the SF arm's rehearsals unwrapped; its contained rehearsal a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it)
