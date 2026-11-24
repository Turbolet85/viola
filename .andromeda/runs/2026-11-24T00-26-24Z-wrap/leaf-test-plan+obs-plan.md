@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: `state.heartbeat_start` › `pty.spawn` (CLIENT), with no budget span.
source: ctr-6f6hckro «pty.spawn (CLIENT); under --origin, run.window follows the trust step, before state.snapshot_write.»
replace-with: `state.heartbeat_start` › `pty.spawn` (CLIENT), with no budget span; under `--origin`, `run.window` (its one field `outcome`, never an address, pid or workspace) follows the trust step, before `state.snapshot_write`.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: The lineage line and the launch line written at every run and revive start are events-log product audit data: each wakes no wait and opens no span
source: dec-zbecdqti «The window line a start under --origin writes after its launch line is log-only product audit data: it wakes no wait, opens no span and is never copied into a process-log line.»
replace-with: The lineage line and the launch line written at every run and revive start, and the window line a start under `--origin` writes after its launch line, are events-log product audit data: each wakes no wait and opens no span
@leaf_edit n=3 leaf=.claude/rules/observability.md line=42 class=omission
leaf-text: its prompt and the composed socket address appear in no line.
source: ctr-yt4sqped «Subject of the process-start and process-exit pair of the foot terminal window that viola open spawns, written into the cli role file of the opened name.»
replace-with: its prompt and the composed socket address appear in no line. `viola open` writes one `process-start` / `process-exit{subject:"terminal"}` pair around the foot terminal window it spawns, into the opened name's `cli-<name>.ndjson`.
@leaf_edit n=4 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: their request inside spans `cli.link` / `cli.unlink`, once their instance check of it has passed.
source: ctr-n7m2srv7 «Logs as cli into the opened instance's cli-<name>.ndjson, its work inside the cli.open dispatch span.»
replace-with: their request inside spans `cli.link` / `cli.unlink`, once their instance check of it has passed. `viola open` logs as `cli` into the opened instance's `cli-<name>.ndjson`, its work inside the `cli.open` dispatch span; its refusals `terminal-not-found` and `not-live` are each one `process-exit{exit_code:1, detail}`, and a strict-modes refusal is exit 1 (under `--json` the `strict-modes-failed` document).
