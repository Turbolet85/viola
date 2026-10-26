# Classified — arch · established-decisions.b1

## Answers
established-decisions#1.1 · no · standing S3 · new code on the run, hook, send or channel paths uses std threads and blocking I/O; only viola-mcp builds a runtime
established-decisions#2.1 · yes · the sideload fail-open (an absent, tampered or unloadable conpty companion falls back to the inbox ConPTY with no refusal) binds pin_companions and its new strict-modes set; the revive cwd read after the instance check binds that check's order
established-decisions#3.1 · yes · run passes SIGNATURES exactly when its version gate read cli_verified, so the gate read the chunk puts behind the strict check must record cli_verified:false on refusal and keep the partial gate
established-decisions#4.1 · yes · the client's empty-text refusal (refusal_of in src/cmd/send.rs, reached whole) must stay before any frame and keep its place among send's refusals once the home check is added; the wrapper's confirmation rules are untouched
established-decisions#5.1 · yes · the atomic snapshot and .lock-sibling rules bind the snapshot reads (live_endpoint, check_instance) and the stamps.json write update_stamps keeps (snapshot.rs and fs.rs reached whole)
established-decisions#6.1 · yes · replace_private and replace_private_shared (mode set before any byte; a failed replace counts as done on identical bytes) bind the pin_exe race of two first starts, the plugin rewrite and the conpty/ companions
established-decisions#7.1 · yes · the frame verbs' instance-unreachable outcome on a vanished endpoint (wait, answer, last, send, pause and release are reached whole) and the Windows SQOS identification connect the frame clients make bind the refused-check path
established-decisions#8.1 · yes · every frame-writing client's pid and start-time check against the snapshot before its first frame stays, and the strict check is added before the same snapshot read
established-decisions#9.1 · no · the macOS server-verification gap is owned by "Unix endpoint and home hardening" (Not needed), and server_verdict is not an edited unit of liveness.rs (only live_endpoint is)
established-decisions#10.1 · yes · every reader stays on read_snapshot until the move onto read_snapshot_or_replay, which the chunk's scope leaves as a P4 fork neither planned in nor out
established-decisions#11.1 · yes · the frame tools (send, wait, last, answer) in viola-mcp tools.rs: a refused check is the isError instance-unreachable result with detail null and no frame written
established-decisions#12.1 · yes · the new check on the hook path must fit inside SPINE_DEADLINE (1 s), CONNECT_DEADLINE (750 ms) and SessionEnd's ~1 s budget, and the per-start plugin rewrite keeps hooks.json's timeout keys
established-decisions#13.1 · yes · hook.rs's shared order (read_snapshot, Client::connect_by, verify_server) gains the strict check before read_snapshot, and a check that cannot be made sends no frame
established-decisions#14.1 · yes · a refused check in viola hook fails open (exit 0, no body, nothing on stderr, never exit 2), and the hidden --capture arms stay outside the instance check and the VIOLA_* reads
established-decisions#15.1 · no · the wrapper's dialog-expired append (WaitFeed::appending, in src/run, no reached unit) is untouched, and the wait.rs change adds only the home check before its read
established-decisions#15.2 · no · a recorded live measurement of the 671 s hold; the chunk re-measures no hold and its check runs before the hold
established-decisions#15.3 · yes · a dialog hook whose server check fails or cannot be made exits 0 with no body, so the refused strict check in hook.rs's dialog arms must take that same fail-open path
established-decisions#16.1 · no · a pending live reading of the 3600 s ceiling owed to "Version done-check" and "Windows-only live measurements"; the chunk changes no hold ceiling
