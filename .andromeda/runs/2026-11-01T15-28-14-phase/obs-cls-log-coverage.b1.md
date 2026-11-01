# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · process-start fields are unchanged; the chunk adds no process-start line, and the wrapper refusal happens after process-start is already written
log-coverage#1.2 · yes · process-exit `detail` closed codes and `during:"connect"` for the new connect-time refusals, which sit beside server-verify-failed
log-coverage#1.3 · yes · channel-request `from` and `from_trust` stay self-reported under a verified euid, and a refused connection writes no channel-request
log-coverage#1.4 · yes · a refused connection writes no channel-response, so the `detail` list stays the three -32602 codes with no new value
log-coverage#1.5 · no · send-issued, send-confirmed and send-refused are written past a frame; the connect refusal is ahead of any frame and writes no send line
log-coverage#1.6 · no · dialog lifecycle and dialog fields are not moved; the hook connect reaches only hook-decision as a fail-open
log-coverage#1.7 · yes · hook connect refusal reaches hook only as a fail-open hook-decision `detail`
log-coverage#1.8 · no · no panic path is moved or added by the chunk
log-coverage#1.9 · yes · release-from-driver `from_trust` stays self-reported in any line, as the chunk's verified euid must not change it
log-coverage#1.10 · no · torn-line healing and snapshot replay (state-recovered) are not changed
log-coverage#1.11 · no · the chunk adds no parse-rejected parser or detail, and the euid check runs before any frame is parsed
log-coverage#2.1 · yes · exit 1 start refusal of a refused socket directory and the replaced `internal-error` path from `bind_endpoint` need closed codes in this list
log-coverage#2.2 · yes · exit 21 list gains the new client-side refusal codes, and server-verify-failed's Linux and Windows scope carries the macOS pid carve-out
log-coverage#2.3 · no · exit 20 wrapper-fault is not a cause the chunk adds or moves
log-coverage#2.4 · yes · the new hook connect refusal must take a code in this fail-open list, and no hook arm writes strict-modes-failed
log-coverage#2.5 · no · parse-rejected details per parser are unchanged; the chunk adds no parse-rejected line
log-coverage#2.6 · no · the mcp-stdin oversize parse-rejected is unrelated to the connect and home checks
log-coverage#2.7 · no · RefusalReason details are send refusals; the client connect refusal is not one of them
log-coverage#2.8 · yes · filesystem refusals: the home check's new strict-modes refusal from `board::scan`, the run and revive home-tree refusal lines, and the client refuse_untrusted and mcp frame-tool no-line rules
log-coverage#3.1 · no · spawn subjects and the process-start and process-exit spawn pairs are not moved; holder_gone's connect probe is not a spawn
