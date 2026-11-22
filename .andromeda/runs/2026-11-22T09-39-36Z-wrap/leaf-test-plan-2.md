@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-20: A plan's non-leg entry that reads a file a rou
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--report-version`, `--trim-prompt-end`
source: ctr-jyoov477 «The argv-only --wake-after-ms <ms>, capped at 5 000, fires once, that long after the first Stop carrying a non-empty session_crons, a keyless recorded UserPromptSubmit, and under --turn-stop its turn end.»
replace-with: `--report-version`, the argv-only `--wake-after-ms <ms>` (capped at 5 000: fires once, that long after the first Stop carrying a non-empty `session_crons`, a keyless recorded UserPromptSubmit, and under `--turn-stop` its turn end), `--trim-prompt-end`
