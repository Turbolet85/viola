# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · no client line the scope names is process-start, and the claude-child fields are the task's Not-needed E2
log-coverage#1.2 · yes · the CLI verbs' new exit-21 process-exit carries subject self, exit_code 21, detail server-verify-failed, during connect and duration_ms
log-coverage#1.3 · yes · client-side channel-request is written only for a frame made, with conn on every call, and a refused connect writes none
log-coverage#1.4 · yes · client-side channel-response carries conn and its result fields on each call made, and a refused connect makes none
log-coverage#1.5 · yes · a send refused before any frame by a failed verification is send-refused with side client, codes only and no sent text
log-coverage#1.6 · no · dialog-raised and dialog-answered are the wrapper's lines from frames it receives, a refused hook sends none, and no field changes
log-coverage#1.7 · yes · the hook's fail-open hook-decision carries detail server-verify-failed at WARN, and hook-invoked follows the hook.dialog response
log-coverage#1.8 · no · standing S6 · home-level lines stay path-free and content-free; content and chains go only to detail-<process>.ndjson
log-coverage#1.9 · no · the scope changes release only before its frame, and the wrapper's release-from-driver refusal line is unchanged
log-coverage#1.10 · yes · E5: state-recovered comes only from read_snapshot_or_replay, so the chunk's clients stay on read_snapshot and write no state-recovered line
log-coverage#1.11 · no · the chunk adds no parse-rejected parser or detail, since the verification reads a peer identity rather than a frame, and the ledger-stamps and claude-agents-json rows are Not-needed
log-coverage#2.1 · yes · run's collision check, re-read at P3, is what states already-live for a live or stale holder and the no-line takeover of a gone holder
log-coverage#2.2 · yes · the CLI verbs' mismatch line uses detail server-verify-failed, which this exit-21 set holds
log-coverage#2.3 · no · exit 20 wrapper-fault is the wrapper's fault code, and the scope maps every verification outcome to exit 21
log-coverage#2.4 · yes · the hook's fail-open hook-decision on a mismatch uses detail server-verify-failed, which this fail-open set holds
log-coverage#2.5 · no · the chunk adds no parse-rejected line or detail, and the ledger-stamps strict-modes row is the run version gate's read, which is Not-needed
log-coverage#2.6 · yes · the client-side send-refused detail server-verify-failed is not a RefusalReason, so the §6 refusal-detail list must name it
log-coverage#2.7 · yes · the exit-21 and hook-decision writers this chunk adds must keep the rule that no arm writes strict-modes-failed unless P4 takes the strict-modes half into the clients
log-coverage#3.1 · no · the chunk spawns no child or shell, and the statusline shell-out and the agents-probe read are Not-needed
