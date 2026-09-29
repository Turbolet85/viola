### PID file

- **Location:** N/A as a product pid file, per test-scope Sec 3. Product pids are in `<home>/instances/<name>/snapshot.json` (`pid`, `started_at`, `child_pid`) with liveness in `<home>/instances/<name>/heartbeat`. The UI pid comes from `GET /api/info` `pid` and `<home>/ui/<port>.url`. The harness's own process record is `target/agent-run/<session>/session.json` (`supervisor_pid`, `wrapper_pid`, `ui.pid`).
- **Lifecycle:**
  - `session.json` is written by `boot` once readiness passes.
  - It is read by `status`, `logs` and `cleanup`.
  - Kill targets are always verified by pid + start time via sysinfo 0.39.6 before signalling, so a reused pid is never killed.
  - `session.json` is removed by `cleanup`.
