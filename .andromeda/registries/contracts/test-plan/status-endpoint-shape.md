### Status endpoint shape

This is the `agent-run status` document. Its nested `list` member is the verbatim `viola list --json` document (see Section 1, Status endpoint shape).

```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": null,
  "uptime_ms": null,
  "last_error": "null | <harness code: list-exit-<code> (e.g. list-exit-21) | list-exit-none>",
  "list": "<viola list --json document>",
  "instances": [{ "name": "<ViolaName>", "wrapper_pid": "<pid>", "alive": "<pid + start-time liveness>" }]
}
```

`status` runs `viola list` since chunk 2026-10-17-end-to-end-path-binary-and-harness-legs (5-command `status`). `state` is `ready` when `list` exits 0 and every booted instance is an item `wrapped:true, liveness:"live"` (an unwrapped item is ignored; a session that booted none reads `ready`). It is `degraded` when a booted instance is `stale` or absent (a gone instance is omitted from `list`), or the document holds no `items`. It is `down` when `list` exits non-zero (`last_error` `list-exit-<code>`), gives no exit code, cannot be spawned or has no `supervise.json` to run from (`list-exit-none`), or exits 0 with no JSON object on stdout (`list-exit-0`, `list:null`). `list` is the parsed document, the exit-21 error document included; `pid` and `uptime_ms` stay `null`; `instances` keeps the pid + start-time `alive`. `ok` is true only for `ready`; the exit is 0 for `ready`, 1 otherwise, 2 for an unknown session. The agent polls this document through the `status` command to verify state transitions during E2E scenarios.
