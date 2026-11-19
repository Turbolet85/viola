@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=121
last: - 2026-11-18: When a change makes the product write into a u
@leaf_read leaf=.claude/rules/verification-harness.md lines=77
last: - 2026-11-19: A CONTENDED host read before a live round whos
@leaf_edit n=1 leaf=.claude/rules/testing.md line=38 class=stale
leaf-text: The one exception: the test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on.
source: dec-j2mfetpl «The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.»
replace-with: The one exception: the test-only live rig reads its child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=13 class=stale
leaf-text: it reads the child's screen by compiled literals only into closed words its rehearsal asserts on
source: dec-g6y7k5xo «The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.»
replace-with: it reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=13 class=omission
leaf-text: its rehearsals over the fake agent run in CI;
source: dec-k4rtddnt «The SF arm's rehearsals run unwrapped in CI; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it.»
replace-with: its rehearsals over the fake agent run in CI (the SF arm's unwrapped; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it);
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-xkyep5qa «Beside --turn-stop, --turn-fail <variant> ends the turn with the recorded set's StopFailure.<variant> payload in place of Stop.default.»
replace-with: `--settings <file>` is accepted, its value consumed and ignored. Beside `--turn-stop`, `--turn-fail <variant>` ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`.
@leaf_edit n=5 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: It reads hook commands from `<plugin-dir>/hooks/hooks.json` (the `--plugin-dir` `run` passes).
source: ctr-dfv5qikx «--settings-hooks <file> names a Claude settings file whose hooks run beside the plugin's; a command string with no args is split on spaces and never run through a shell.»
replace-with: It reads hook commands from `<plugin-dir>/hooks/hooks.json` (the `--plugin-dir` `run` passes), and under `--settings-hooks <file>` from that Claude settings file too, its hooks run beside the plugin's (a command string with no args is split on spaces and never run through a shell).
