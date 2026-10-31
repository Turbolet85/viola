# Classified — security · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · prompt and assistant text in events.ndjson and over wait/last; no event path changes and no code, document or hint carries it
threat-model-summary#1.2 · no · tool-call, plan and dialog text in events.ndjson and over wait and hook.dialog; the chunk's codes, documents and hints never carry it
threat-model-summary#1.3 · yes · snapshot endpoint, pid and pinned_bin, which client.rs's pre-check and server check read, and the pinned bin/ copy that pinned-hash-mismatch re-hashes, are integrity-sensitive
threat-model-summary#1.4 · no · CLAUDE* credentials in the wrapper's environment; the chunk neither reads nor stores them and the R8 strip is unchanged
threat-model-summary#1.5 · yes · snapshot pid and cwd are the operational metadata the new documents, hints and log lines must not hold (no path, no pid, no recorded directory)
threat-model-summary#2.1 · yes · same-user IPC trust boundary: peer pid and start-time check before the first frame, no frame on mismatch or failed check, macOS gap, the client's order
threat-model-summary#3.1 · no · bridge send paste framing and CR/LF rules are untouched; send.rs changes only as a caller of the client's order, which binds through 2.1
threat-model-summary#4.1 · no · the human terminal's stdin classifier is not a cause or code surface (named not needed by the task)
threat-model-summary#5.1 · yes · hook's strict-modes check before any read, exit 0 with no body or stderr on every failure, hook-decision as its only cause record
threat-model-summary#6.1 · yes · MCP strict-modes check before the liveness pre-check and server check; refusal is instance-unreachable with detail null, which the new codes replace in isError
threat-model-summary#7.1 · no · the MCP stdin CappedLines reader and its parse-rejected WARN are not edited; the chunk changes only mcp's refused home check
threat-model-summary#8.1 · yes · check_home_tree before run, revive start and verify's home reads (exit 1, no log) and check_instance on frame verbs (exit 21), with the --home and VIOLA_DIR surface
threat-model-summary#9.1 · yes · readers after the strict-modes check, a replayed snapshot yielding no endpoint or pid, and the bin/ and plugin/ artefacts that the pinned-hash, squatted-name and session-live causes read
threat-model-summary#10.1 · yes · pinned-hash-mismatch re-hashes the pinned copy that concurrent first starts place, so placement's close-before-name ordering must keep a partial copy from reading as a mismatch
threat-model-summary#11.1 · yes · claude agents --json spawns and PATH resolution: session-live from the agents read, a failed read fails open, claude not on PATH as internal-error, revive's spawn in the recorded cwd
threat-model-summary#12.1 · no · the chunk adds no dependency, lockfile, action or workflow change; cargo deny and SHA pins are untouched
threat-model-summary#13.1 · yes · the native-matrix tests job (fake agent, per OS) is where the 7 s wait_for margin tally reads its CI runs, and where the new table runs
