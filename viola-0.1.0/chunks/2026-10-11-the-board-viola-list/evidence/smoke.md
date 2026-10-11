# Smoke — 2026-10-11-the-board-viola-list

The plan's `smoke` entries (7 cleanup · 8 boot · 9 status · 12 cleanup, session `p-board-smoke`) ran as P2 gates,
green in firings 1 and 3 (`evidence/gates.md`). The harness `status` document runs no `viola list` (its `list`
member stays `null`), so the verb was also read by hand once against a booted harness session.

- `bash scripts/agent-run.sh boot --session p-board-hand --instance builder` (bounded 300 s, output to a file):
  exit 0, `"ok":true`.
- `target/harness/debug/viola --home <the boot document's home> list` with `VIOLA_NAME`, `VIOLA_DIR` and
  `VIOLA_BIN` removed: exit 0, stdout

  ```
  BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0
  NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI
  builder  live   unknown  driver                   none               2.1.287 verified
  ```
- The same with `--json`: exit 0, one document:
  `{"v":1,"ok":{"items":[{"name":"builder","wrapped":true,"liveness":"live","status":"unknown","wheel":"driver","budget_paused":false,"dialog_pending":false,"cli_version":"2.1.287","cli_verified":true}],"budget":"unknown","skipped":{"unknown_kinds":0,"unknown_fields":0,"torn_lines":0}}}`
- `bash scripts/agent-run.sh cleanup --session p-board-hand` (bounded 120 s): exit 0, `ok` true,
  `processes_gone` true, `endpoint_gone` true.

No live `claude` start: the booted child is the fake agent.
