# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · the RefusalReason set and the send and answer refusal orders hold: the wheel, budget and hook paths are untouched, and link and unlink answer a result, -32602 or -32603, never a refusal
conventions#2.1 · yes · add-to · link and unlink's strict-modes refusal joins exit 21 (not revive's exit 1), exit 20 covers a self-pair's wrapper -32602, and an unexpected reply is exit 1 (F5, F7)
conventions#3.1 · yes · add-to · link and unlink are built CLI frame verbs whose server identity check runs through cmd::client::connect, so exit 21 on a server that is not the recorded wrapper now also covers them
conventions#4.1 · no · the MCP tool set, result shapes and error mapping are unchanged, the change reaches no MCP file, and link and unlink stay CLI-only, so no MCP handler receives their errors
conventions#5.1 · no · the frame tools' server check and instance-unreachable result are unchanged, the change reaches no MCP file, and link and unlink add no MCP frame path
