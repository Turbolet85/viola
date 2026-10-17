### Heartbeat ticks

_[Standard+ tier. The stall-detection intent is kept, with viola-specific mechanisms instead of 10–30 s tick lines (D-13).]_

- **Tick interval:**
  - `run`: the existing `<instance>/heartbeat` file, touched every **1 s** (arch contract). It is not logged per tick: a 1 s line into a never-rotated file would grow without bound.
  - `mcp`, `hook` and short-lived CLI verbs: no tick (request-driven).
- **Tick event format:**
  - No process-log line carries a liveness transition: no process watches another instance's heartbeat.
  - Short-lived readers (`list`) expose `liveness` in `--json`, and the agent reads it from the `agent-run status` snapshot.
- **Stall detection:** the agent reads `agent-run status` (`state: degraded` when a booted instance's item is `stale` or absent, a gone instance being omitted from `list`; test-plan §3 Status endpoint shape). The flip threshold is a heartbeat older than 5 s (4.9 s live / 5.1 s stale or gone), read by `list`.
