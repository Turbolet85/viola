# Classified — architecture · conventions.b1

## Answers
conventions#1.1 · no · the RefusalReason set, its details and the per-verb refusal order are unchanged, the CLI's text checks still run before any frame, and a server-verification failure is an unreachable outcome, not a refusal, so the claim still holds on the changed client code
conventions#2.1 · yes · add-to · F5 makes a CLI server-verification failure exit 21 with the unchanged instance-unreachable document under --json, reached after the client's own exit-13 refusals, and no code is added
conventions#3.1 · yes · add-to · F6 makes an MCP frame tool facing a server that is not the recorded wrapper return the instance-unreachable isError result (text refused: instance-unreachable), with no frame and no log line
