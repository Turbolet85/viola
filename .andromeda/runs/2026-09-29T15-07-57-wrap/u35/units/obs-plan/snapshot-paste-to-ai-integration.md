### Snapshot / paste-to-AI integration

N/A: there is no pulse-class receiver in the stack. The paste-to-AI workflow runs through the tests harness instead of a tail of a single log file:
- `scripts/agent-run.sh logs` / `scripts/agent-run.ps1 logs` merges `events.ndjson` (`{"src":"events","instance","offset","record"}`) with every `diagnostics/*.ndjson` (`{"src":"diag","file","record"}`) and every instance detail line from `instances/*/diagnostics/detail-*.ndjson` (`{"src":"diag","file","instance","record"}`; the added `instance` tells apart the same `detail-<process>.ndjson` basename across instances, per tests `logs`). Torn lines appear as `{"torn":true,"offset":n}`. The merged output is filterable by `--instance`, `--kind`, `--process`, `--after`.
- `agent-run status` is the aggregate health snapshot: `state`, `last_error` closed enum, `api_sessions_equal_list`.
- Canonical agent queries:
  - To follow one send across every side: `agent-run logs | jq -c 'select(.record.instance=="B" and (.record.event|IN("send-issued","send-confirmed","send-refused")) and .record.corr==412)'`, where `B` is the send's target.
    - A send `cursor` is a byte offset into the target's own `events.ndjson`, so it is unique per target instance only. Two wrappers with identical start sequences (common in E2E) can both hold a send at 412, so `corr` alone mixes sends.
    - `send-*` lines with a cursor `corr` are emitted only by the target wrapper (`side:"wrapper"`, D-27), so the jq `instance` predicate is the target's. The caller-side `send-refused{side:"client"}` has `corr` null and is read from the caller's `channel-response{conn}` and `process-exit` instead.
    - The harness `--instance` filter is still not used. It would also drop the caller's `channel-*` lines, which carry the caller's instance (see Trace context propagation).
    - The `event` filter keeps out an unrelated `channel-request` whose JSON-RPC `id` happens to be 412.
    - The `channel-*` lines of the same send are reached from a matched send line: its `(conn, rpc_id)` equals their `(conn, corr)` (D-30). When refusals at the same offset must be told apart, add `and .record.rpc_id==<id>` to the query.
    - The hook side (`hook-invoked{hook_event:"user-prompt-submit"}`, `corr` null) joins by the wrapper's `instance` + `hook_event` + `timestamp` inside `run.confirm_window`;
  - `jq -c 'select(.record.level=="ERROR" and (.file|startswith("detail-")|not))'` lists every error and panic once. The detail-file duplicate of a panic line is excluded (§10).
- The harness `logs` glob must also cover `instances/*/diagnostics/detail-*.ndjson` (D-08). This additive harness change is part of tests amendment D-21.
