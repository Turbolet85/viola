@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=21 class=stale
leaf-text: Every `viola hook` arm but the two `--capture` arms (the event and dialog paths and `viola hook statusline`) runs its instance check
source: dec-4vztpgai «decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the»
replace-with: Every `viola hook` arm but the `hook <event> --capture` arm (the event and dialog paths) runs its instance check
@leaf_edit n=2 leaf=.claude/rules/observability.md line=21 class=stale
leaf-text: its stdout carries the user's status-line output, never telemetry.
source: ctr-aniczyao «contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body.»
replace-with: its stdout carries the decision body, never telemetry.
@leaf_edit n=3 leaf=.claude/rules/observability.md line=38 class=stale
leaf-text: The hidden capture arms, `hook <event> --capture` and `hook statusline --capture` (the latter runs no instance check and no shell-out, opens no role file and prints nothing), do no obs init and write no line
source: dec-5zl47735 «decision/hook-capture-arms-unlogged · Hidden capture arms write no lines — The hidden capture arm of viola hook (hook <event> --capture) initialises no observability and writes no lines.»
replace-with: The hidden capture arm, `hook <event> --capture`, does no obs init and writes no line
@leaf_edit n=4 leaf=.claude/rules/observability.md line=40 class=retired
leaf-text: (start or `--list`) or on `viola hook statusline`, or `viola mcp`'s refused
source: dec-5orxlfrh «decision/statusline-silent-without-instance · Statusline arm silent without a valid instance — The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses»
replace-with: (start or `--list`), or `viola mcp`'s refused
@leaf_edit n=5 leaf=.claude/rules/observability.md line=40 class=retired
leaf-text: (the seventh exemption; on `viola hook statusline` the founder's own answer, 2026-10-13)
source: dec-5orxlfrh «decision/statusline-silent-without-instance · Statusline arm silent without a valid instance — The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses»
replace-with: (the seventh exemption)
