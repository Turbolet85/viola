# obs · picked accounting · group 4 of 5

## Picked
span-trace-coverage#1.1 · carried · the CLIENT span list must record the two new verbs' dispatch spans
span-trace-coverage#1.2 · carried · `channel.dispatch` is the SERVER span the two new methods are served under
span-trace-coverage#2.1 · carried · the cli row's dispatch-span-per-verb list and `cli-<name>.ndjson` pair rule the two new verbs
span-trace-coverage#2.2 · carried · the `viola run` row's `channel.dispatch` span is where the wrapper serves `link` / `unlink`
span-trace-coverage#2.4 · carried · the channel row's `channel-request` / `channel-response` fields and its never-params rule govern the two new methods
span-trace-coverage#3.1 · carried · the six frame verbs' server check inside the dispatch span becomes eight with the two new verbs
span-trace-coverage#3.2 · carried · one captured-span refusal case per CLI frame verb is owed for each new verb
span-trace-coverage#5.1 · carried · the implicit link record shares `LinkSet`, which gains its `remove` caller, and keeps `driver` / `driven` out of process-log lines
span-trace-coverage#5.2 · not in scope · the send scenario's own span attributes (readiness gate, paste, confirm window); the chunk adds no send span and edits none
span-trace-coverage#5.3 · carried · the self-reported `from_trust` shape and the client server-verify refusal shape are what the new verbs' frames and refusals copy
span-trace-coverage#5.4 · carried · the `Drop`-guard `channel-response` and the exit-21 `detail` / `during` catalog rule the new methods and verbs
span-trace-coverage#6.1 · not in scope · the `wait` / `last` span tree; the chunk does not change those verbs
span-trace-coverage#6.3 · carried · its `strict-modes-failed` note is the other side of the P4 fork over a strict-modes check for these two verbs alone
span-trace-coverage#6.4 · not in scope · `wait.block`'s close; the chunk does not change `wait`
span-trace-coverage#7.3 · not in scope · the dialog / `answer` path's lines; no hook or dialog path changes and the implicit link on a taken `answer` stands as built
span-trace-coverage#8.1 · not in scope · the wheel and `pause` / `release` span tree; the chunk adds no wheel path
span-trace-coverage#8.3 · not in scope · `release`'s own lines and its `release-from-driver` refusal; the chunk's `-32602` causes are their own and touch no `release` path
span-trace-coverage#11.1 · carried · E3 states the explicit methods are not built and no product path appends `unlink`; this chunk builds both
span-trace-coverage#12.1 · carried · the replay's link half must derive `links` with the product `unlink` lines the chunk first writes
span-trace-coverage#13.1 · not in scope · the healing appender's own behaviour; the `unlink` line goes through the existing append unchanged
span-trace-coverage#15.1 · not in scope · `verify` / `plugin install` and the hidden capture arms; no verify child or plugin change
metric-coverage#2.2 · carried · `link` / `unlink` become `method` values of the channel latency histogram
metric-coverage#2.9 · carried · the two new `-32602` refusals join `viola.compat.v_rejected`'s recorded over-count
metric-coverage#3.1 · not in scope · `viola.wait.outcome`'s counting rule; the chunk adds no `wait` call
