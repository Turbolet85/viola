### metric-coverage#1.1 · values · metrics · ## 5. Metric Coverage · lines 8–10
| Type | Use case | Derived from |
|------|----------|---------------------|
| Counter | Occurrences of an `event` / `detail` / `refusal` value | `awk 1 diagnostics/*.ndjson | jq -R -n '[inputs|fromjson?|select(.event=="send-refused")]|group_by([.side,.detail])|map({side:.[0].side,d:.[0].detail,n:length})'`, over home-level role files only (detail files repeat lines; a torn line is skipped as in G2) |
### metric-coverage#2.2 · values · metrics · ## 5. Metric Coverage · lines 21–21
| Channel call latency per `method` | `viola.channel.duration_ms` ← `channel-response.duration_ms` | histogram | ms | raw samples |
### metric-coverage#2.4 · values · metrics · ## 5. Metric Coverage · lines 23–23
| Send terminal state (design readback words `read back` / `unable` / `unconfirmable`) | `viola.send.outcome` ← count of `send-confirmed{confirmed}` / `send-refused{refusal,detail,side}` | counter | 1 | n/a |
### metric-coverage#2.5 · values · metrics · ## 5. Metric Coverage · lines 24–24
| `wait` outcome (design CLI `viola wait`) | `viola.wait.outcome` ← `channel-response{method:"wait"}.outcome` | counter | 1 | n/a |
### metric-coverage#2.8 · values · metrics · ## 5. Metric Coverage · lines 27–27
| Parse rejections per parser (creator-explicit) | `viola.parse.rejected` ← `parse-rejected{parser, detail, count}` | counter | 1 | n/a |
### metric-coverage#2.9 · values · metrics · ## 5. Metric Coverage · lines 28–28
| `unknown` fallback / higher-`v` rejections | `viola.compat.unknown` ← `send-refused{refusal:"unknown"}`; `viola.compat.v_rejected` ← `channel-response{error_code:-32602}` — over-counts: `-32602` also answers `send`'s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`), `release` carrying `from` and a `link` / `unlink` with a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a `driver` equal to `driven`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells the causes apart | counter | 1 | n/a |
### metric-coverage#2.10 · values · metrics · ## 5. Metric Coverage · lines 29–29
| Child / probe exits | `viola.child.exit` ← `process-exit{subject, child_exit_status, exit_source}` | counter | 1 | n/a |
### metric-coverage#3.1 · values · metrics · ## 5. Metric Coverage · lines 36–36
- `viola.wait.outcome` counts only the `wait` calls answered by a `channel-response`: a `wait` refused before any frame writes none. The CLI's server-verification refusal (Linux and Windows) is one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`, counted by the Counter row's `detail` grouping; MCP's `wait` tool writes no line for it.
### metric-coverage#3.2 · values · metrics · ## 5. Metric Coverage · lines 37–37
- A CLI `wait` refused by the instance strict-modes check (`client::refuse_untrusted`: exit 21, the instance pair or the `instance-unreachable` document) writes no line at all, neither a `process-exit` nor a `channel-request`, so neither `viola.wait.outcome` nor the Counter row's `detail` grouping counts it; MCP's `wait` tool refused by its `check_instance` (`instance-unreachable`, `detail: null`) writes none either. The exit code or the `isError` code is its only record.
