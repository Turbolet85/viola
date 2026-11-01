# Classified — security · error-handling.b1

## Answers
error-handling#1.1 · yes · hook's exit 0, no body, no stderr and the closed hook-decision detail set, where a new refusal of this chunk reaches hook as a fail-open
error-handling#1.2 · yes · a failed client server verification sends no frame and writes server-verify-failed on hook, where a peer euid mismatch takes the same place
error-handling#1.3 · yes · hook's fail-open with a closed hook-decision cause, which the chunk's directory and peer refusals on hook must pick from
error-handling#1.4 · no · statusline without --capture over a replayed snapshot; the statusline_command read and shell-out are untouched and the scope names neither
error-handling#1.5 · yes · MCP frame tool on a failed server check: no frame, isError instance-unreachable with server-verify-failed and the refused text, which the peer euid mismatch takes on MCP
error-handling#1.6 · yes · check_instance runs before the liveness pre-check on MCP, the order the euid check must hold with the strict-modes check and the one snapshot read
error-handling#1.7 · no · link and unlink reply mapping for wrapper faults (-32603, exit 20, exit 1); the chunk's checks run before any frame and change no reply mapping
error-handling#1.8 · no · sanitised_fault reads -32xxx wrapper faults; the chunk's refusals are not fault replies
error-handling#1.9 · no · -32602 data bodies are frame content, and the chunk changes no frame shape
error-handling#1.10 · no · fault codes outside the five read as wrapper fault; a wrapper reply the chunk does not touch
error-handling#1.11 · yes · is_code_word gates the refusal detail in reply_of and of_reply, so a new cause must be a code word or it reads unknown or null
error-handling#1.12 · yes · a code word outside the closed list falls to exit 14, so each new closed refusal code must enter the list to map into exit 21
error-handling#1.13 · no · ProtocolError::cause and the channel-response schema cover protocol faults; the chunk's refusals write no response line before a frame
error-handling#1.14 · no · the no -32603 and failpoint rule in a scanned home; the chunk drives no -32603 and its peer-mismatch seam drives a refusal, not an internal error
error-handling#1.15 · yes · refuse_untrusted runs check_instance before any CLI frame, the order the client euid check joins, with exit 21 and the exact --json document
error-handling#1.16 · yes · exit 21's closed set of five detail causes, which the socket-directory and peer euid refusals take a word from or extend
error-handling#1.17 · yes · UnreachableCause has no serde path, so the new cause words are written only by unreachable_document and the MCP unreachable in result.rs
error-handling#1.18 · no · the rule names only the unwrapped-name and endpoint-lost hints; the new hint's text is governed by its own no-path rule
error-handling#1.19 · no · the exit-2 UNUSABLE lines of answer and send; the chunk changes no typed text and no exit-2 path
error-handling#1.20 · yes · run and revive's start run the home strict-modes check before the log opens, the place of the run refusal on a failed directory check and the Unix home check
error-handling#1.21 · yes · the catch-site internal-error line that bind_endpoint and create_private_dir's anyhow errors reach today, replaced by a closed refusal; run and hook stay silent
error-handling#1.22 · yes · collision_check's stale and live holder reading and instance-dead for frame verbs, the start arbiter the leftover socket takeover must sit beside
error-handling#1.23 · yes · fixed Display messages with no path for the strict-modes errors, the rule each new refusal's Display holds; run maps PinError::StrictModes
error-handling#1.24 · yes · a new thiserror refusal enum, if the chunk adds one, must be recorded in the nine-derive census in tests/contract_error_bodies.rs
error-handling#1.25 · no · names five existing error-enum divergences owned by a CARRY on the route entry; the chunk adds none
