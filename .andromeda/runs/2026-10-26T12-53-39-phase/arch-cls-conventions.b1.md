# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · the RefusalReason values and the send and answer refusal order; link and unlink answer no refusal, only the channel's -32602 and -32603 faults
conventions#2.1 · yes · the exit codes 0, 1, 2, 20 and 21 that the link and unlink verbs take (the Needs names them), and the cli role's internal-error rule for their dispatch arm
conventions#3.1 · yes · the link and unlink verbs are frame-writing clients, so their server identity check before the first frame exits 21 with the instance-unreachable document and line, as the other frame verbs do
conventions#4.1 · yes · the wrapper-fault and instance-unreachable document shape, stated as the same as CLI --json, which the link verbs print on exit 20 and 21; the MCP tool results bind nothing, since no MCP tool is built
conventions#5.1 · no · the MCP frame tools' refused instance-unreachable isError result (send, wait, last, answer); the chunk builds no MCP tool and reaches no viola-mcp file
