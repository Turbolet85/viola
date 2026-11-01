# Classified — obs-plan · error-capture-reporting.b1

## Answers

error-capture-reporting#1.1 · no · the panic hook, its record and its catch sites are untouched (the session names none of them) and src/panic_frames.rs is not reached
error-capture-reporting#1.2 · no · check_diagnostics' folder and file rules hold: the report keeps diagnostics/ file modes and log locations unchanged, and the new socket-dir check never changes a mode
error-capture-reporting#2.1 · yes · contradict · the socket-directory refusal on the start arms of run and revive writes a process-exit line (log already open) at exit 1, so "a refused check on the start arm ... no process log opened" no longer holds for it; under --json it joins the exit-1 documents of refused_with
error-capture-reporting#3.1 · no · the pre-role-file window holds only the instance checks and refuse_untrusted, which are unchanged; the new socket-dir check runs inside dispatch after the cli role file opens, outside this window
error-capture-reporting#4.1 · no · the vt100 feed thread (src/run/gate.rs) is not reached and viola-pty was not edited
error-capture-reporting#4.2 · no · the forced-feed-panic passthrough test and its byte checks are not touched by the session
error-capture-reporting#5.1 · no · frame still runs check_instance before live_endpoint, returning strict-modes-failed with no frame and no line; the new socket-dir check sits after live_endpoint and is carried under #5.2
error-capture-reporting#5.2 · yes · add-to · F5 adds a socket-directory refusal in MCP's frame after live_endpoint, returned as UnreachableCause::SocketDir, which joins this per-refusal mapping
error-capture-reporting#5.3 · no · the oversize cut and its exit codes sit in src/main.rs and the stdin reader, neither reached; the MCP change is only the frame socket-dir check
error-capture-reporting#5.4 · no · the pre-initialize oversize line is on the stdin path, which the session did not change
error-capture-reporting#6.1 · no · sanitised_fault's five codes and fixed messages are unchanged; the session adds no peer fault code, and a refused euid takes server-verify-failed with no new word
error-capture-reporting#6.2 · no · is_code_word's pattern and the peer-reply closed list are unchanged; socket-dir-refused is a local exit 21 cause written by the client's own check, not a peer reply word
error-capture-reporting#6.3 · yes · add-to · F8 adds Refused::NotADirectory, a fixed-message variant naming no path, beside the fixed-message variants listed here; the nine thiserror derives stand, since Refused gains a variant and no derive
error-capture-reporting#6.4 · no · the exit-2 UNUSABLE paths of answer and send are not reached and the session did not touch them
error-capture-reporting#6.5 · yes · stale · the exit-21 closed causes move from five to six with socket-dir-refused, so "one of the five closed words" is a count the session moved; F5 adds that UnreachableCause::SocketDir has no serde path
