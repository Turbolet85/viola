# P3 — what harness cleanup reads after a force-killed wrapper on Unix

Measured 2026-11-01T16:11:58Z to 16:12:10Z on the Linux dev host at HEAD 6ba49f53, tree unchanged, through
`scripts/agent-run.sh`, `df -h /tmp` read first (tmpfs, 23G free). Homes not kept (`AGENT_RUN_KEEP_HOMES` unset).

Control, session `p3-control`: `boot` exit 0, `cleanup` exit 0:
`{"v":1,"cmd":"cleanup","ok":true,"cleaned":["p3-control"],"processes_gone":true,"endpoint_gone":true,"home_removed":true,"killed":[]}`

Subject, session `p3-kill`: `boot` exit 0; the wrapper this script booted, its exe confirmed under the repository, took one
SIGKILL; `cleanup` exit 1:
`{"v":1,"cmd":"cleanup","ok":false,"cleaned":["p3-kill"],"processes_gone":true,"endpoint_gone":false,"home_removed":true,"killed":[]}`

The socket file: present before the kill, present 1 s after it, present after `cleanup`. The script then removed that one
leftover, by the path its own snapshot recorded.

Reading: the hypothesis holds. `unconnectable` on Unix is `!Path::new(endpoint).exists()`
(`crates/viola-e2e/src/harness/cleanup.rs:145`), and a killed wrapper's `Guard` never runs, so the file stays.
