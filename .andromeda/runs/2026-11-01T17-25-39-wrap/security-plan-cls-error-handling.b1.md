# Classified — security-plan · error-handling.b1

## Answers
error-handling#1.1 · yes · add-to · the hook's refused directory fails open as hook-decision detail socket-dir-refused (F18), joining this detail list
error-handling#1.2 · yes · add-to · the euid refusal on the hook seam reads server-verify-failed on the dialog path (F17), joining this row
error-handling#1.3 · no · a replayed or absent snapshot has no endpoint, so no directory check runs and the channel-unreachable fail-open holds
error-handling#1.4 · no · statusline over a replayed snapshot reads no endpoint, so no check runs and nothing prints as before
error-handling#1.5 · yes · add-to · the MCP frame's euid check beside server_verified refuses a foreign-euid server with server-verify-failed (F17)
error-handling#1.6 · yes · add-to · the MCP frame's directory check after live_endpoint is a further strict-check refusal: no frame, no line, isError the record
error-handling#1.7 · no · link and unlink reply mapping (exit 20, exit 1) is unchanged; the session adds only a pre-frame refusal, which is not a wrapper reply
error-handling#1.8 · no · sanitised_fault and its reply readers are untouched; the client.rs edit adds the seam checks before connect only
error-handling#1.9 · no · the two -32602 bodies that keep data are untouched by the session
error-handling#1.10 · no · the wrapper-fault mapping for codes outside the five is untouched
error-handling#1.11 · no · is_code_word is unchanged, and socket-dir-refused fits its 1 to 64 byte [a-z0-9-] rule
error-handling#1.12 · no · socket-dir-refused has no RefusalReason, so the closed reason list and reason_of's 14 do not move
error-handling#1.13 · no · channel-response's detail enum is not one of the two schema counts the session moved (process-exit, hook-decision)
error-handling#1.14 · no · the new negatives drive refusals before any frame and no -32603 path, so the absence holds
error-handling#1.15 · yes · add-to · the frame verbs' directory refusal is exit 21 with detail socket-dir-refused and one process-exit WARN line (F15), extending this refusal set
error-handling#1.16 · yes · stale · exit 21's closed causes move from five to six with socket-dir-refused (F15)
error-handling#1.17 · yes · add-to · the new cause SocketDir, word socket-dir-refused with no serde path, joins the cause words the two clients write (F15)
error-handling#1.18 · no · the two named hints and their ViolaName naming are unchanged; the new socket-dir hint is neither of them
error-handling#1.19 · no · the exit-2 paths of answer and send (src/cmd/answer.rs, src/cmd/send.rs) are not reached by the session
error-handling#1.20 · yes · add-to · run and revive's start refuse a socket directory with exit 1 and one process-exit detail socket-dir-refused (F16), extending this refusal set
error-handling#1.21 · no · json_asked and the internal-error catch site are unchanged, and the new refusals are not internal errors
error-handling#1.22 · no · the directory check follows live_endpoint, so a stale instance still reads instance-dead and stale-heartbeat is unchanged
error-handling#1.23 · yes · add-to · the new refusal's fixed wording names only the parsed ViolaName with no path, pid or uid (F19), joining this fixed-message list
error-handling#1.24 · no · the thiserror derive count stays nine and the list of types does not move
error-handling#1.25 · no · Refused stays a non-derive divergence; its new NotADirectory variant is a fixed message, not a derive
