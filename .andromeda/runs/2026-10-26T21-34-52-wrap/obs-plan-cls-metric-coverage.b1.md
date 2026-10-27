# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the counter counts any event, detail or refusal value the role files hold, and the diag-line schema change adds only one enum word, so the claim holds on the changed file
metric-coverage#1.2 · no · the claim is generic over each event's duration_ms samples, and the refusals the session adds write no line to sample
metric-coverage#1.3 · no · the gauge's harness readings keep their sources, and a replayed snapshot carries no endpoint, so liveness reads unreachable rather than live
metric-coverage#2.1 · no · the §10 re-measure with the instance check in the timed path stays under SPINE_DEADLINE and the report leaves that claim unmoved, so this derivation and its max gate hold
metric-coverage#2.2 · no · no channel method changed (the §5 claim is untouched), and a refused check sends no frame, so no channel-response is added
metric-coverage#2.3 · no · the send window starts at the wrapper's arrival in src/run/send.rs, which is not reached, and the CLI's pre-frame check sits outside it
metric-coverage#2.4 · no · the new CLI refusal writes no send-confirmed or send-refused line, and the wrapper's send path is not reached
metric-coverage#2.5 · no · a wait refused by the new check writes no channel-response, and the served wait outcome the counter reads is unchanged
metric-coverage#2.6 · yes · add-to · four more product readers (hook connect, statusline serve, run's collision_check, live_endpoint) now reach the state-recovered event this counter derives from, with its event and detail words unchanged
metric-coverage#2.7 · no · the skipped count is list's own output from the board read, and board.rs is unchanged
metric-coverage#2.8 · no · a refused-instance hook skips its config.json read and so writes no parse-rejected line, which the per-parser counter does not promise
metric-coverage#2.9 · no · no -32602 path or channel method changed (the §5 claim is untouched), and the new refusals are instance-unreachable documents that neither counter reads
metric-coverage#2.10 · no · the process-exit source is unchanged, and a refused run or verify writes no exit line for the counter to read
metric-coverage#2.11 · no · the panic writer is unchanged (the §7 claim is untouched), so event:panic still records each logged panic, and the widened pre-init window is a §7 and §10 exemption this row does not state
metric-coverage#3.1 · no · a wait refused by the new check writes no frame and no line, as the note's first sentence already says, and the report moves no line the server-verify refusal writes
metric-coverage#4.0 · no · the snippet reads unchanged hook-decision fields from diagnostics/hook-*.ndjson, and a refused hook writes no line it would count
