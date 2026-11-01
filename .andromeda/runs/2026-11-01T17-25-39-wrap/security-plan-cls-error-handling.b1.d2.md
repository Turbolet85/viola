# Classified — security-plan · error-handling.b1

## Answers
error-handling#1.1 · yes · stale · macOS's server verification is no longer only the recorded gap since its euid check now holds (F17), and the hook's socket-dir refusal joins the fail-open details (F18)
error-handling#1.2 · yes · add-to · a server of another euid is refused on the hook seam too, as server-verify-failed with no frame and exit 0 (F17)
error-handling#1.3 · no · a replayed or absent snapshot carries no endpoint, so the new directory check (F3, F5) never runs there and channel-unreachable stands
error-handling#1.4 · no · a replayed snapshot has no endpoint for the directory check, and the statusline form sends no frame, so nothing it says moves
error-handling#1.5 · yes · add-to · the MCP frame's euid check beside server_verified makes a server of another euid a server-verify-failed refusal too (F17)
error-handling#1.6 · yes · add-to · the frame tool's directory check after live_endpoint adds socket-dir-refused to the detail list (F15)
error-handling#1.7 · no · the new refusals sit before any frame, and the wrapper's link and unlink fault mapping of replies is unchanged
error-handling#1.8 · no · sanitised_fault's five codes and the -32602 bodies are unchanged, and the new refusals are client-side and send no reply
error-handling#1.9 · no · the -32602 data-keeping bodies are unchanged and no new fault code or body is added
error-handling#1.10 · no · no fault code outside the five is added or read differently
error-handling#1.11 · no · is_code_word's 1-to-64-byte [a-z0-9-] grammar holds on the changed viola-core, and socket-dir-refused conforms to it (F15)
error-handling#1.12 · no · socket-dir-refused is an UnreachableCause with no RefusalReason (F15), so reason_of's unknown rule is not reached
error-handling#1.13 · no · ProtocolError::cause still gives the three -32602 causes and nothing for the other four, and no protocol error is added
error-handling#1.14 · no · the new negatives are forced refusals that send no frame, so no -32603 is driven and the absence claim stands
error-handling#1.15 · yes · add-to · the eight frame verbs also refuse a socket dir with exit 21 and detail socket-dir-refused, one process-exit at WARN and no channel-request (F15)
error-handling#1.16 · yes · stale · the exit-21 closed causes move from five to six with socket-dir-refused (F15)
error-handling#1.17 · no · UnreachableCause keeps no serde path (F15) and the new word is written by the same two clients
error-handling#1.18 · yes · add-to · the socket-dir refusal's hint names only the parsed ViolaName, as the two hints here do (F19)
error-handling#1.19 · no · the exit-2 paths of answer and send are untouched
error-handling#1.20 · yes · add-to · run's and revive's start also refuse a socket dir with exit 1 and one process-exit of detail socket-dir-refused (F16), where the strict-modes refusal writes no line
error-handling#1.21 · no · the new refusals are typed refusals, not the cli catch site's internal error, so the silent-exit claim stands
error-handling#1.22 · no · the directory check reads a recorded endpoint's parent (F3), which a stale wrapper's live_endpoint withholds, so instance-dead still reads
error-handling#1.23 · yes · add-to · Refused gains the fixed-message NotADirectory variant with no path, joining the no-path fixed Display messages (F19)
error-handling#1.24 · no · the thiserror derives stay nine and the list of types does not move, since Refused gains a hand-written variant and not a derive
error-handling#1.25 · no · Refused still diverges from the error-enum convention by its hand-written Display, so the divergence list stands
