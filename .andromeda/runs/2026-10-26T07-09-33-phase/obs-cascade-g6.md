# obs · cascade read · group 6 of 7

## Carried
- (.andromeda/obs-plan-detail/log-coverage.md:28) A CLI verb whose server verification fails or cannot be made must end in `process-exit` exit 21 with the closed `detail` `server-verify-failed`, at WARN, and any new exit-21 code must be a member of the schema's closed enum with its stated count re-derived from the enum's members.
  > - `process-exit` exit 21: `instance-dead`, `strict-modes-failed`, `server-verify-failed`.
  (.andromeda/obs-plan-detail/log-coverage.md:27)
  > `schemas/diag-line.v1.json` closes the `process-exit` `detail` at 13 values (the enum's members; 12 before chunk 2026-10-16-unwrapped-sessions-on-the-board, 11 before chunk 2026-10-13-interrupted-verify-cleanup, 9 before chunk 2026-10-10-viola-revive).
  (.andromeda/obs-plan.md:460)
  > | `warn` | `state-recovered`; `parse-rejected`; `hook-decision` with a fail-open `detail`; `process-exit` exit 21; `channel-response` with `-32600`/`-32602` |
  (.andromeda/obs-plan.md:537)
  > - MCP rejects are `isError` codes only, mirrored by codes-only log lines.
- (.andromeda/obs-plan-detail/log-coverage.md:11) The exit-21 verification line must carry only the catalogued `process-exit` fields — `detail` and `during` (`connect|call`) naming where the check failed — with the server pid, the snapshot's `pid` and `started_at` and the endpoint in no field or `detail` value, and a typed verification error crossing the crate seam must end in that codes-only line.
  > | `process-exit` | `subject`, `exit_code` (self), `child_exit_status` (child/probes), `shell_exit_status` (statusline), `exit_source` (`handle-wait|kill-fallback`), `detail`, `during` (`connect|call`), `duration_ms` |
  (.andromeda/obs-plan.md:533)
  > - Module-boundary errors (Standard+): every typed error that crosses a crate seam ends in a logged `process-exit{detail}`, `channel-response{result_class:"error", error_code}`, `hook-decision{detail}` or `send-refused`. Each carries codes only, at `WARN`/`ERROR` per Section 6.
  (.andromeda/obs-plan.md:420)
  > **Metric label cardinality discipline:** grouping keys are closed enums only: `event`, `detail`, `refusal`, `method`, `hook_event`, `subject`, `parser`, `process`. `corr`, `conn`, `pid`, `child_pid` and `instance` are join keys, never grouping keys in aggregate reports.
  (.andromeda/obs-plan.md:470)
  > | `rmcp`, `portable_pty`, `sysinfo`, `interprocess` | `OFF` (D-11; failures are captured at the viola seam as viola events) |
- (.andromeda/obs-plan-detail/log-coverage.md:30) The hook's mismatch path must fail open with one `hook-decision` whose `detail` is `server-verify-failed` (or `strict-modes-failed` if P4 takes the strict-modes half), at WARN, with stdout and stderr empty and no `hook.event` or `hook.dialog` frame written; the statusline arm, only re-read, keeps its three codes and its silent refused check.
  > - `hook-decision` fail-open codes (security Vector 4): `oversize-stdin`, `malformed-json`, `channel-unreachable`, `server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`. `viola hook statusline` uses three of the seven (the rule: the `detail` codes the arm's fail-open paths write): `oversize-stdin`, `malformed-json`, and `deadline`, which on that arm is the 5 s PROVISIONAL bound on the user's command, not a dialog deadline. Its refused instance check writes no `hook-decision` and no other line (chunk 2026-10-10-budget-governor; before it the arm wrote `strict-modes-failed`, four of the seven).
  (.andromeda/obs-plan-detail/log-coverage.md:16)
  > | `hook-invoked` / `hook-decision` | `hook_event` (`session-start|user-prompt-submit|pre-tool-use|permission-request|stop|session-end|notification|post-tool-use|post-tool-use-failure|statusline`), `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `budget_written`, `detail`, `duration_ms` |
  (.andromeda/obs-plan.md:310)
  > - `hook` writes only its decision body to stdout and nothing to stderr.
- (.andromeda/obs-plan-detail/log-coverage.md:33) The strict-modes telemetry rule is P4's fork: if the clients take the strict-modes half, the rule's "no arm writes the word" readings on `process-exit` exit 21 and on `hook-decision` must change with the clients that then write it; if not, they must keep holding.
  > - Filesystem refusals (Vector 7): `strict-modes-failed` (on `process-exit` exit 21, and on `parse-rejected{parser:"ledger-stamps"}` from `run`'s version gate, at `warn`, with no path; `viola revive`'s start arm and `viola revive --list` both refuse on `viola_state::strict::check_instance` with the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1 and no log line; on `hook-decision` the schema's `detail` enum keeps the word and no arm writes it, since `viola hook statusline`'s refused check leaves no line and the nine hook events run no instance check; chunk 2026-10-10-budget-governor; `viola list`'s refused scan of the home, `instances/` or an instance exits 21 on both forms and writes no line either — no `process-start`, no `process-exit`, no `strict-modes-failed` — so on `process-exit` exit 21 no arm of this build writes the word;
- (.andromeda/obs-plan-detail/log-coverage.md:12) A client whose verification refuses must write no `channel-request` for the frame it never sends, and every call made after a passed verification must carry its `conn` on both client lines, with `params` and result bodies never recorded.
  > | `channel-request` | `method`, `conn` / `srv_conn`, `from`, `from_trust`, `sender`, `v`, `after`, `timeout_ms` |
  (.andromeda/obs-plan-detail/log-coverage.md:13)
  > | `channel-response` | `method`, `conn` / `srv_conn`, `result_class` (`ok|refusal|error`), `refusal`, `detail`, `error_code` (`-32700|-32600|-32601|-32602|-32603`), `outcome`, `duration_ms` |
  (.andromeda/obs-plan.md:479)
  > - **IPC invocation** (viola-channel): `channel-request` / `channel-response` with method, `corr`, `conn`, result class, error code and `sender`. **No argument digest**: `params` and result bodies are never recorded, not even hashed.
- (.andromeda/obs-plan-detail/log-coverage.md:14) A `send` refused before any frame by a failed verification must be logged codes-only — any send line the plan writes for it carries `text_bytes`, never the text, and `side` `client`, with `conn` / `srv_conn` and `rpc_id` left to the wrapper side.
  > | `send-issued` / `send-confirmed` / `send-refused` | `from`, `from_trust`, `text_bytes` (the typed text's length: the sent text with every CR LF pair and every other CR as one LF, and without its trailing CR and LF characters), `confirmed`, `refusal`, `detail`, `side` (`client|wrapper`), `wheel`, `window`, `used_percentage`, `resets_at`, `paused`, `duration_ms`, `conn` / `srv_conn` and `rpc_id` (wrapper side only: the originating `send` request's connection and JSON-RPC `id`, D-30) |
- (.andromeda/obs-plan-detail/metric-coverage.md:23) The send terminal state must stay derivable: a client-side line the plan writes for a send refused by a failed verification is counted by `side` and closed `detail`, and one it does not write must leave the counter's reading unchanged.
  > | Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
  (.andromeda/obs-plan-detail/metric-coverage.md:10)
  > | Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
- (.andromeda/obs-plan-detail/metric-coverage.md:20) The hook latency the chunk re-measures, the added peer-credential and sysinfo start-time read included, must stay inside the hyperfine `max` gate per `hook_event`, read from `hook-decision.duration_ms` raw samples, with `session-end`'s quantile reading report-only.
  > | Hook latency per `hook_event` (Scenario 4/6; perf trigger) | `viola.hook.duration_ms` ← `hook-decision.duration_ms` | histogram | ms | raw samples; gate = hyperfine `max` |
  (.andromeda/obs-plan-detail/metric-coverage.md:11)
  > | Histogram | Latency distribution | Raw `duration_ms` samples per event, with no bucketing; quantiles by sort/index in `jq` |
  (.andromeda/obs-plan-detail/metric-coverage.md:36)
  > Canonical quantile snippet (report-only; the gates use `max`):
  > ```sh
  > # -R + fromjson? skip a torn line as G2 does (jq -s aborts on one); awk 1 ends each file's last line with a newline
  > awk 1 diagnostics/hook-*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="hook-decision" and .hook_event=="session-end")|.duration_ms]|sort
  >   |{n:length,p50:.[length/2|floor],p95:.[length*0.95|floor],max:.[-1]}'
- (.andromeda/obs-plan-detail/metric-coverage.md:25) Under the founder's hold the five re-read snapshot readers stay on `read_snapshot`, so no client may write a `state-recovered` line and the recovery counter must read only `read_snapshot_or_replay`'s lines unless his word moves them.
  > | Heal / recovery (chaos) | `viola.state.recovered` ← `state-recovered.detail` | counter | 1 | n/a |
  (.andromeda/obs-plan-detail/log-coverage.md:19)
  > | `state-recovered` (D-03) | `detail` (`torn-line-healed|snapshot-replayed|snapshot-unsupported-v`), `file` (basename only: `events.ndjson` on `torn-line-healed`, `snapshot.json` on the two snapshot details), `offset` (`torn-line-healed` only: the log's length before the healing append), `v_seen` (`snapshot-unsupported-v` only), `scanned` (the two snapshot details only, written by `read_snapshot_or_replay`: the instance dir's own last component when it parses as a `ViolaName`, schema pattern `^[a-z][a-z0-9-]{0,31}$`, absent otherwise — never a path or a literal `null`; `torn-line-healed` carries none; chunk 2026-10-17-cli-machine-contract) |

## Bindings
- **log-coverage exit-21 catalog ↔ the body's level map and error capture**: `server-verify-failed` on `process-exit` exit 21 (.andromeda/obs-plan-detail/log-coverage.md:28) is written at WARN (.andromeda/obs-plan.md:460) and is where a typed verification error crossing the seam ends, codes only (.andromeda/obs-plan.md:533).
- **log-coverage `hook-decision` fail-open ↔ metric-coverage hook latency**: the fail-open line's `duration_ms` (.andromeda/obs-plan-detail/log-coverage.md:16) is the sample the hook-latency gate reads (.andromeda/obs-plan-detail/metric-coverage.md:20), so the mismatch path is measured by the same gate.
- **log-coverage send lines ↔ metric-coverage send outcome**: whatever client-side `send-refused` the plan writes (.andromeda/obs-plan-detail/log-coverage.md:14) is counted by `side` and `detail` (.andromeda/obs-plan-detail/metric-coverage.md:23).
- **log-coverage `state-recovered` ↔ metric-coverage recovery counter**: the hold that keeps the clients on `read_snapshot` (.andromeda/obs-plan-detail/log-coverage.md:19) keeps the counter's source unchanged (.andromeda/obs-plan-detail/metric-coverage.md:25).
- **log-coverage strict-modes rule ↔ the exit-21 and fail-open catalogs**: P4's fork over the strict-modes half (.andromeda/obs-plan-detail/log-coverage.md:33) decides whether `strict-modes-failed` is written on exit 21 (.andromeda/obs-plan-detail/log-coverage.md:28) and on `hook-decision` (.andromeda/obs-plan-detail/log-coverage.md:30).
- **metric-coverage cardinality ↔ log-coverage `process-exit`**: `pid` stays a join key, never a grouping key or `detail` value (.andromeda/obs-plan.md:420), on the exit-21 line's fields (.andromeda/obs-plan-detail/log-coverage.md:11).
