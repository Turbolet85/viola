@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: `release` and `viola focus` (on their named instance
source: dec-4vztpgai «Commands acting on an instance (revive's start arm, adopt's start, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus, switch) run the instance strict-modes check before opening any log»
replace-with: `release`, `viola focus` and `viola switch` (on their named instance
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: a refused scan writes no line.
source: ctr-ku2k4wbr «Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a process-exit inside the cli.switch dispatch span; a refusal ends process-exit with no detail, as send's does.»
replace-with: a refused scan writes no line. `viola switch` logs as `cli` into the named instance's `cli-<name>.ndjson`: a `process-start` and a `process-exit` inside the `cli.switch` dispatch span; a refusal ends `process-exit` with no detail, as `send`'s does.
@leaf_edit n=3 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: `release` or `viola focus` (exit 21, the instance pair
source: ctr-7tjreakr «A refused instance check of send, wait, last, answer, pause, release, link, unlink or switch exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.»
replace-with: `release`, `viola focus` or `viola switch` (exit 21, the instance pair
@leaf_edit n=4 leaf=.claude/rules/observability.md line=40 class=stale
leaf-text: the eight CLI frame verbs one `process-exit{exit_code:21
source: ctr-hjfzl3cb «The socket directory is refused, on the nine frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.»
replace-with: the nine CLI frame verbs one `process-exit{exit_code:21
@leaf_edit n=5 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: The lineage line and the launch line written at every run, revive and adopt start, and the window line a start under `--origin` writes after its launch line, are
source: dec-3umgztdt «The launch line written at every run, revive and adopt start and at every switch is log-only product audit data: it wakes no wait and opens no span, writes no process-log line and no terminal byte of its own.»
replace-with: The lineage line written at every run, revive and adopt start, the launch line written there and at every switch, and the window line a start under `--origin` writes after its launch line (and a switch writes again after its own launch line), are
