# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook's line and raw backtrace frames are not touched by the socket, euid or home checks
error-capture-reporting#1.2 · no · the rule is over `check_diagnostics` on the diagnostics folder and detail file in `obs.rs`, a unit the chunk neither edits nor names
error-capture-reporting#2.1 · yes · the start-arm refusal rule (stderr pair, exit 1, no process log opened) governs the wrapper's new socket-directory refusal at `run` and `revive` start, pre-child
error-capture-reporting#3.1 · yes · the pre-log window holds the instance and home-tree checks the new client and wrapper refusals sit beside: each writes no line and leaves the directory mode as found
error-capture-reporting#4.1 · no · the vt100 feed-thread panic catch is not reached by the chunk
error-capture-reporting#4.2 · no · the post-feed-panic passthrough of the vt100 test is not reached by the chunk
error-capture-reporting#5.1 · yes · MCP frame tools run the instance check before `live_endpoint` with no line on refusal, which the new client-side refusals must follow through `frame`
error-capture-reporting#5.2 · yes · `frame` maps refused `check_instance` to `StrictModes` and failed `server_verified` to `ServerVerify`, the mapping the new client refusals must join
error-capture-reporting#5.3 · no · the oversize MCP stdin line past `MAX_FRAME` is not reached by the chunk
error-capture-reporting#5.4 · no · the oversize MCP stdin line before `initialize` is not reached by the chunk
error-capture-reporting#6.1 · no · `sanitised_fault` reads a peer's fault codes, and the chunk's refusals are client-side, not peer replies
error-capture-reporting#6.2 · no · the `is_code_word` screening and `reason_of` exit mapping apply to peer replies, not to the chunk's own refusal codes
error-capture-reporting#6.3 · yes · the recorded count of 9 `thiserror::Error` derives goes stale once the chunk adds a fixed-message typed error at the crate seam for `bind_endpoint`
error-capture-reporting#6.4 · no · the exit-2 `UNUSABLE_RESPONSE` and `UNUSABLE_TEXT` lines of `answer` and `send` are not reached by the chunk
error-capture-reporting#6.5 · yes · exit 21's closed five-word `UnreachableCause` list and its single `unreachable_document` writer, reached through `refuse_untrusted`, govern how the new client refusals are recorded
