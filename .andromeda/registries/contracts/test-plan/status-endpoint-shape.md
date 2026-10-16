### Status endpoint shape

This is the `agent-run status` document. Its nested `list` member is the verbatim `viola list --json` document (see Section 1, Status endpoint shape).

```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": null,
  "uptime_ms": null,
  "last_error": "null | <harness code, e.g. list-exit-21>",
  "list": "<viola list --json document>",
  "instances": [{ "name": "<ViolaName>", "wrapper_pid": "<pid>", "alive": "<pid + start-time liveness>" }]
}
```

Interim, while `status` runs no `viola list` (built at chunk 2026-10-11-the-board-viola-list; the harness reading it is owed to the Epoch 5 entry "End-to-end path binary and harness legs"): `list`, `pid` and `uptime_ms` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`. `state` is `ready` when every booted instance has `liveness:"live"`. It is `degraded` when any item is `stale`. It is `down` when `list` exits non-zero. `ok` is true only for `ready`. The agent polls this document through the `status` command to verify state transitions during E2E scenarios.
