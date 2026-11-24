@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=81
last: - 2026-11-24: A live-reading script prints verdict words fro
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=13 class=omission
leaf-text: and every board read it makes runs with no `claude` on its PATH).
source: dec-athrwpuo «decision/live-rig-root-outside-repositories · Live rig root outside every repository — A live rig's live run refuses a root inside a repository, and runs its stand-in agent from a target dir of its own. · provisional»
replace-with: and every board read it makes runs with no `claude` on its PATH; its live run refuses a root inside a repository and runs its stand-in agent from a target dir of its own).
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: it reads its mode and reply set from files beside its exe, never from the env;
source: ctr-wd7fcf5g «contract/fake-foot/spawn-record · the fake foot stand-in · spawn record — Standing as foot, the fake agent appends one line holding each spawn's pid and argv to a spawn record beside itself, and still writes its argv file.»
replace-with: it reads its mode and reply set from files beside its exe, never from the env, and standing as `foot` it still writes its argv file and appends one line holding each spawn's pid and argv to a spawn record beside itself;
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: a Drop guard that kills, then waits, still covers the normal end;
source: dec-zivbe3i3 «decision/stand-in-terminal-tree-teardown · One shared teardown ends a stand-in terminal's tree — A stand-in terminal's process tree is ended by one shared test teardown: it kills first only a leaf whose»
replace-with: a Drop guard that kills, then waits, still covers the normal end, and a stand-in terminal's process tree is ended by one shared test teardown that kills first only a leaf whose recorded parent still runs, waits for every other member under one deadline, and at that deadline kills only a pid whose start time still matches;
