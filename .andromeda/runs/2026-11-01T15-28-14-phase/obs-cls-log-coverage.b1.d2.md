# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the start refusal's order against the wrapper log open is a fork, and it decides whether process-start (with pid and endpoint_kind) is written
log-coverage#1.2 · yes · process-exit's detail and during:"connect" carry the new client refusal beside server-verify-failed, and the wrapper refusal's exit, with no path or pid
log-coverage#1.3 · yes · a refused connection writes no channel-request, and from_trust stays self-reported though the euid is verified
log-coverage#1.4 · yes · a refused connection writes no channel-response, and the three -32602 detail values are unchanged
log-coverage#1.5 · no · the client refusals fire ahead of any frame, so no send-issued, send-confirmed or send-refused line is written and the open detail string is untouched
log-coverage#1.6 · yes · a connect refusal on the hook.dialog path writes no dialog-raised line, so the hook trace is hook-decision alone
log-coverage#1.7 · yes · hook-decision's detail takes the new fail-open code that the hook connect refusal writes
log-coverage#1.8 · no · the chunk adds no panic site and no panic field
log-coverage#1.9 · yes · a verified euid leaves from_trust self-reported on release-from-driver
log-coverage#1.10 · no · state-recovered is torn-line healing and snapshot replay, which the chunk leaves unchanged
log-coverage#1.11 · no · refusals fire before the first frame, so channel-frame, hook-stdin and mcp-stdin never see them and no parse-rejected detail is added
log-coverage#2.1 · yes · the socket-directory refusal at run's start joins the pre-child exit-1 preflight list and needs a closed code
log-coverage#2.2 · yes · the peer euid mismatch and the refused socket directory at connect need closed exit-21 codes beside server-verify-failed
log-coverage#2.3 · yes · whether the wrapper-side euid refusal ends the wrapper with exit 20 is left to P3, so a wrapper-fault-only list is a fork
log-coverage#2.4 · yes · the hook connect refusal reaches hook only as a hook-decision fail-open detail from this list
log-coverage#2.5 · no · the chunk adds no parse-rejected detail, and its refusals precede any frame parse
log-coverage#2.6 · no · the mcp-stdin oversize line is untouched, and no frame-tool refusal writes a parse-rejected line
log-coverage#2.7 · no · the chunk adds no RefusalReason, and client refusals are not send refusals
log-coverage#2.8 · yes · making the home check one rule changes the strict-modes refusal pair and the no-line rule for list, run, client and mcp
log-coverage#3.1 · no · the chunk adds no child or shell spawn and no subject value
