@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=51 class=omission
leaf-text: on Hyprland, which only a live reading proves (the fakes prove the placement words and the window records only)
source: dec-zysuqw2t «decision/window-placement-proven-live · Real window placement proven only live — A real terminal window placed on its parent's workspace, or on the workspace its snapshot recorded when viola up re-opens it,»
replace-with: on Hyprland, which only a live reading proves, a placing open's kept view only by the gated live reading R7 (the fakes prove the placement words and the window records only)
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--turn-fail <variant>`, beside `--turn-stop`, ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`,
source: ctr-jyoov477 «contract/fake-agent-cli/wake-after-ms · viola-fake-agent command line · --wake-after-ms — The argv-only --wake-after-ms <ms>, capped at 5 000, fires once, that long after the first Stop carrying a non-empty session_crons»
replace-with: `--turn-fail <variant>`, beside `--turn-stop`, ends the turn with the recorded set's `StopFailure.<variant>` payload in place of `Stop.default`, the argv-only `--stop-crons <n>` (0 to 4) puts n synthetic elements in every Stop's `session_crons`, the argv-only `--wake-after-ms <ms>` (capped at 5 000) fires once, that long after the first Stop carrying a non-empty `session_crons`, a keyless recorded UserPromptSubmit and, under `--turn-stop`, its turn end,
