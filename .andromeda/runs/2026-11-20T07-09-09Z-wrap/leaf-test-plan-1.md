@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/testing.md lines=123
last: - 2026-11-19: `tests/contract_verify_home.rs` reads any `"veri
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--turn-fail <variant>`, beside `--turn-stop`, ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`,
source: ctr-z4ppjw6l «contract/fake-agent-cli/stop-crons · viola-fake-agent command line · --stop-crons — The argv-only --stop-crons <n>, 0 to 4, puts n synthetic elements in every Stop's session_crons.»
replace-with: `--turn-fail <variant>`, beside `--turn-stop`, ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`, the argv-only `--stop-crons <n>`, 0 to 4, puts n synthetic elements in every Stop's `session_crons`,
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--dialogs` replays a pasted dialog prompt's recorded variants in claim order,
source: ctr-jyoov477 «contract/fake-agent-cli/wake-after-ms · viola-fake-agent command line · --wake-after-ms — The argv-only --wake-after-ms <ms>, capped at 5 000, fires once, that long after the first Stop carrying a non-empty session_crons,»
replace-with: `--dialogs` replays a pasted dialog prompt's recorded variants in claim order, the argv-only `--wake-after-ms <ms>`, capped at 5 000, fires once, that long after the first Stop carrying a non-empty `session_crons`, a keyless recorded UserPromptSubmit, and under `--turn-stop` its turn end,
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=29 class=omission
leaf-text: harness turns never flip it but
source: dec-eh7ud4xy «decision/fired-wake-keeps-wheel · A fired scheduled wake keeps the driver's wheel — A keyless human-filed prompt-submitted is relabelled origin harness with no wheel move when no stdin key set the typed mark,»
replace-with: harness turns never flip it, nor does a fired scheduled wake (a keyless human-filed `prompt-submitted` is relabelled origin harness with no wheel move when no stdin key set the typed mark, the last turn end counted at least one scheduled wake and the CLI is verified, else it takes the human wheel; `path5_a_fired_wake_keeps_the_driver_wheel_its_next_send_and_its_dialog`), but
