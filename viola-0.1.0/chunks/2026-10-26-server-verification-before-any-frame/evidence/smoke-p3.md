# P3 smoke — the verified clients against a real wrapper

2026-10-26, from 08:48Z, Linux dev host, this chunk's working tree.

- `bash scripts/agent-run.sh boot --session p-svf-p3 --instance builder` (stdout to a scratch file, never piped): exit 0,
  `"ok":true`, one instance `builder` with its wrapper and child pids. Boot's readiness stage `start_records` requires
  `events.ndjson` record three `session-start{source:"hook"}`, which `viola hook session-start` sends through
  `hook.event` — now after the server check — so a passing boot is the hook path's positive reading against a real
  wrapper.
- `bash scripts/agent-run.sh status --session p-svf-p3`: `ok` true, `state` `ready`.
- The session's own build (`target/harness/debug/viola`), `VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN` removed, `--home` the
  session's home:
  - `viola last builder --json` → `{"v":1,"ok":{"last_assistant_message":null,"ts":null}}`, exit 0
  - `viola wait builder --timeout-ms 500 --json` → `{"v":1,"ok":{"timed_out":true}}`, exit 0
  - its `cli-builder.ndjson`: `channel-request` / `channel-response` method `last`, `process-exit` exit 0; then the
    same for `wait` — each verb's frame reached the wrapper, no `server-verify-failed`.
- `bash scripts/agent-run.sh cleanup --session p-svf-p3`: exit 0, `ok` true, `processes_gone` true, `endpoint_gone` true,
  `home_removed` true.

The P2 block's own smoke entries (14 cleanup, 15 boot, 16 status, 19 cleanup, session `p-svf-smoke`) read green in all
three block runs; entry 19's cleanup step reads `viola last --json` exit 21 on the gone name through the edited client.
