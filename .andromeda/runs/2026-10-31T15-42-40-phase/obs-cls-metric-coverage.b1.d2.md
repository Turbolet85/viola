# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the Counter row's `detail` grouping is what counts process-exit `detail` values, which the chunk adds to, so the new codes must group under it
metric-coverage#1.2 · no · latency histogram over raw `duration_ms` per event, keyed on no word the chunk moves
metric-coverage#1.3 · no · gauge reads `budget.read_at` and `liveness`, neither a detail nor a code the chunk moves
metric-coverage#2.1 · no · hook latency from `hook-decision.duration_ms`; the hook edge stays as it is and the catalogue adds no hook cause
metric-coverage#2.2 · no · channel latency from `channel-response.duration_ms` per method; no code the chunk moves
metric-coverage#2.3 · no · send readback latency from `send-confirmed.duration_ms`; no send-confirmed value moves
metric-coverage#2.4 · no · `send-refused` reasons and details belong to exits 10–14, outside the catalogue, and the chunk moves none of them
metric-coverage#2.5 · no · counts `channel-response{method:"wait"}.outcome`; no outcome value or wait-answer line changes
metric-coverage#2.6 · no · `state-recovered.detail` is torn-line healing, which the chunk does not change
metric-coverage#2.7 · no · `list --json` `skipped` is a success-path field the chunk does not move
metric-coverage#2.8 · no · `parse-rejected` detail belongs to the parser, outside any exit-21 or exit-1 cause
metric-coverage#2.9 · yes · reads channel `error.data` detail values (`-32602` `unsupported-version`, `invalid-params` on send, link, unlink) through the error layer and frame-verb paths the chunk reaches
metric-coverage#2.10 · yes · keyed on `process-exit`, to which the chunk adds CLI lines and detail values; a new self-exit line must not be counted as a child or probe exit
metric-coverage#2.11 · no · standing S6 · zero tolerance on event:"panic" lines across test homes
metric-coverage#3.1 · yes · states the `server-verify-failed` `process-exit` shape at WARN with `during:"connect"`, on the server check the chunk edits and the table covers
metric-coverage#3.2 · yes · states the no-line absence of the `refuse_untrusted` refusal and its `instance-unreachable` document, which the chunk's contract and table must meet
metric-coverage#4.0 · no · quantile snippet over `hook-decision` `session-end` `duration_ms`; no moved word
