@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=80
last: - 2026-11-19: Sweep hazard: `snapshot.json` keeps `pid` and `c
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: and under `--transcript` the launch SessionStart's `transcript_path`.
source: dec-d5qj3lkd «The fake agent gives each hook the recorded fixture bytes, no payload built, save what it sets: a default UserPromptSubmit's typed prompt, the launch SessionStart's source and session_id (--resume) and»
replace-with: under `--transcript` the launch SessionStart's `transcript_path`, and under `--stop-crons` each Stop's `session_crons`.
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--settings <file>` is accepted, its value consumed and ignored.
source: ctr-jyoov477 «The argv-only --wake-after-ms <ms>, capped at 5 000, fires once, that long after the first Stop carrying a non-empty session_crons, a keyless recorded UserPromptSubmit, and under --turn-stop its turn end.»
replace-with: `--settings <file>` is accepted, its value consumed and ignored. The argv-only `--stop-crons <n>` (0 to 4) puts n synthetic elements in every Stop's `session_crons`; the argv-only `--wake-after-ms <ms>` (5 000 at most) fires once, that long after the first Stop carrying a non-empty `session_crons`, a keyless recorded UserPromptSubmit, and under `--turn-stop` its turn end.
