# Classified — obs · span-trace-coverage.b2

## Answers
span-trace-coverage#7.2 · no · the wrapper's wheel-change span sits in no reached unit, and a refused pause or release never reaches the wheel
span-trace-coverage#7.3 · yes · the release client's channel-request and channel-response lines for method release, none for a frame never written (Needs), with release.rs editing the pre-check
span-trace-coverage#7.4 · no · the wheel cleanup and its events.ndjson audit record belong to the wrapper's wheel path, which the chunk does not change
span-trace-coverage#8.1 · no · the budget-governor span tree (Not needed), the statusline arm writing no frame and its snapshot read only re-read
span-trace-coverage#8.2 · no · the budget-governor span attributes and the budget lift's trace (Not needed), the lift's release pair being carried by 7.3
span-trace-coverage#8.3 · no · the statusline arm's lines (budget governor, Not needed), its read_snapshot call only re-read and this entity ruling over no check of it
span-trace-coverage#8.4 · no · the statusline shell-out cleanup and its deadline (budget governor, Not needed)
span-trace-coverage#9.1 · no · the E2 R8 strip fields on claude-child (Not needed), no client spawning a child
span-trace-coverage#10.1 · no · E3 link and unlink (Not needed, held for Epoch 6), the implicit link being written by the wrapper's run::links in no reached unit
span-trace-coverage#11.1 · yes · the five other snapshot readers (client, run's collision check, hook twice, statusline arm) keep read_snapshot and write no state-recovered line, which the chunk re-reads (Needs, E5)
span-trace-coverage#12.1 · no · torn-line healing and the events.ndjson appender (Not needed), no append path changing
span-trace-coverage#13.1 · no · run.version_gate's ledger read (Not needed, not a client of an endpoint), with run.rs re-read only at its collision check
span-trace-coverage#14.1 · no · verify and plugin install are not reached, and the hidden capture arms in hook.rs are another unit of that file, a re-check every change owes
