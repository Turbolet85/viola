@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: it reads its mode and reply set from files beside its exe, never from the env.
source: ctr-iwj5axrp «The fake agent's modes and options are taken from argv and from a fake-args file beside its exe, one argument per line within MAX_FRAME, put ahead of argv so argv's own value wins, never from the environment»
replace-with: it reads its mode and reply set from files beside its exe, never from the env; otherwise its modes and options come from argv and from a `fake-args` file beside its exe (one argument per line within `MAX_FRAME`, put ahead of argv so argv's own value wins), never from the env.
