# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · a re-check every change owes: the scope never names the panic hook's line, and the chunk's content-bearing detail fields are chain and drift_report, not panic_payload or backtrace
error-capture-reporting#1.2 · yes · the chain line the chunk writes to detail-<process>.ndjson must pass check_diagnostics, which drops it on refusal, and the owner-readable detail-file leg of v1-47 rests on that guard
error-capture-reporting#2.1 · yes · the cli catch site's process-exit with detail internal-error and exact `error: internal error`, and the channel Drop guard's -32603 channel-response, are the fault path the scope names
error-capture-reporting#3.1 · yes · the link/unlink instance-check refusal's --json document {"v":1,"error":"instance-unreachable","detail":null} with no line, the form every verb's --json error document carries
error-capture-reporting#4.1 · yes · the G2 seam exemption: chaos_feed_panic's home is outside target/e2e-home, so the secret scan the v1-47 leg re-reads after every e2e test does not read it
error-capture-reporting#4.2 · no · standing S5 · no telemetry on stdout on any role, a convention the task lists
error-capture-reporting#5.1 · yes · the frame tools' check_instance refusal returns isError instance-unreachable with detail null and writes no frame and no line
error-capture-reporting#5.2 · yes · a stdin line past MAX_FRAME before initialize completes ends serve with ServeError::Initialize (exit 11), the cap-driving test the scope adds, its one record parse-rejected{parser:"mcp-stdin", detail:"oversize"}
