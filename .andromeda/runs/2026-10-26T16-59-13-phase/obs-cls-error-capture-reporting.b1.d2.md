# Classified — obs · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · yes · its detail-<process>.ndjson write is a diagnostics open the chunk moves behind check_instance and the 0600 / Windows-DACL rule (Needs), so the panic line must still land on a passed check
error-capture-reporting#2.1 · yes · the chunk's new check paths (hook's fail-open exit 0, the refused CLI and run exits) must stay inside main's catch_unwind and role dispatch, per Needs "no new check path ends a role through an uncaught unwind"
error-capture-reporting#3.1 · yes · the chunk's new strict-modes checks run before any role file opens, so a refused check falls in this pre-step-4 window: no file exists and the panic hook writes nothing (Needs init-order bullet)
error-capture-reporting#4.1 · no · the vt100 feed catch and its one parse-rejected line are unchanged (Not needed: vt100 feed parse-rejected; src/run/gate.rs and tests/chaos_feed_panic.rs unreached), and its outside-scan test-home clause is S4's home convention
