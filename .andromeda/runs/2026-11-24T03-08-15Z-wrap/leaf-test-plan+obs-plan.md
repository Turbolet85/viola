@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: and `send`, `wait`, `last`, `answer`, `pause` and `release` (on their named instance;
source: dec-4vztpgai «Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release, focus) run the instance strict-modes check before opening any log;»
replace-with: and `send`, `wait`, `last`, `answer`, `pause`, `release` and `viola focus` (on their named instance;
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: a strict-modes refusal is exit 1 (under `--json` the `strict-modes-failed` document).
source: ctr-jtexcmkf «Logs as cli into the named instance's cli-<name>.ndjson, a process-start and a closed-detail process-exit inside the cli.focus dispatch span, which carries no fields.»
replace-with: a strict-modes refusal is exit 1 (under `--json` the `strict-modes-failed` document). `viola focus` logs as `cli` into the named instance's `cli-<name>.ndjson`: a `process-start` and a closed-detail `process-exit` inside the `cli.focus` dispatch span, which carries no fields.
@leaf_edit n=3 leaf=.claude/rules/observability.md line=28 class=omission
leaf-text: `schemas/diag-line.v1.json` closes `process-exit`'s;
source: ctr-amu66bwf «viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (not on Hyprland, nothing spawned), unreachable (no compositor»
replace-with: `schemas/diag-line.v1.json` closes `process-exit`'s; `viola focus` ends exit 1 with one `process-exit` detail: `window-not-found` (nothing dispatched), `unsupported` (not on Hyprland, nothing spawned), `unreachable` (no compositor answering, provisional), or the hyprctl seam's `oversize`, `malformed` or `refused`;
@leaf_edit n=4 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: on `send`, `wait`, `last`, `answer`, `pause` or `release` (exit 21,
source: ctr-4yotlmdu «The six frame verbs, link / unlink and focus exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.»
replace-with: on `send`, `wait`, `last`, `answer`, `pause`, `release` or `viola focus` (exit 21,
