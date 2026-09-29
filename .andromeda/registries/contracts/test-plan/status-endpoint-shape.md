### Status endpoint shape

This is the `agent-run status` document. Its nested `list` and `ui` members are verbatim product shapes from the arch GUI HTTP contract (see Section 1, Status endpoint shape).

```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": "<viola ui pid from /api/info, or null without --ui>",
  "uptime_ms": "<now - /api/info.started_at, or null>",
  "last_error": "null | <harness code, e.g. ready-503, list-exit-21, sessions-mismatch>",
  "list": "<viola list --json document>",
  "ui": { "ready": "<GET /ready body>", "sessions": "<GET /api/sessions body>" },
  "api_sessions_equal_list": true,
  "instances": [{ "name": "<ViolaName>", "wrapper_pid": "<pid>", "alive": "<pid + start-time liveness>" }]
}
```

Interim, while `viola list` / `viola ui` are unbuilt: `list`, `ui`, `pid`, `uptime_ms` and `api_sessions_equal_list` are `null`, and `state` is `ready` when every recorded wrapper is alive, else `degraded`. `state` is `ready` when every booted instance has `liveness:"live"` and `/ready` is 200. It is `degraded` when any item is `stale`, `/ready` is 503, or `api_sessions_equal_list` is false. It is `down` when `list` exits non-zero. `ok` is true only for `ready`. The agent polls this document through the `status` command to verify state transitions during E2E scenarios.
