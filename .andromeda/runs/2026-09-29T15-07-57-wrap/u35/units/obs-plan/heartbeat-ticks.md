### Heartbeat ticks

_[Standard+ tier. The stall-detection intent is kept, with viola-specific mechanisms instead of 10–30 s tick lines (D-13).]_

- **Tick interval:**
  - `run`: the existing `<instance>/heartbeat` file, touched every **1 s** (arch contract). It is not logged per tick: a 1 s line into a never-rotated file would grow without bound.
  - `ui`: `Sse::keep_alive` every **15 s**. This is a tests perf gate, observed by the tests at `advance(15 s)` and not logged per keep-alive.
  - `mcp`, `hook` and short-lived CLI verbs: no tick (request-driven).
- **Tick event format:**
  - Transition-only `liveness-changed` (D-02), emitted by the long-lived reader `ui` when a watched instance's state changes. Fields: `subject_instance`, `liveness_from`, `liveness_to` (`live|stale|gone`), `heartbeat_age_ms`, `pid_alive` (sysinfo 0.39.6 pid + start-time check).
  - Short-lived readers (`list`) expose `liveness` in `--json` and `/api/sessions` instead, and the agent reads it from the `agent-run status` snapshot.
- **Stall detection:** the agent reads `agent-run status` (`state: degraded` when any item is `stale`), or `agent-run logs --process ui | jq 'select(.record.event=="liveness-changed")'`. The flip threshold is `heartbeat_age_ms > 5000` (4.9 s live / 5.1 s stale or gone).
