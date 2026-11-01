# Classified — obs-plan · error-capture-reporting.b1

## Answers
error-capture-reporting#1.1 · no · the panic hook, its one-line record and its catch sites are untouched per the report, and no file holding them is reached
error-capture-reporting#1.2 · no · check_diagnostics and the diagnostics/ folder and file modes are untouched per the report; the strict.rs change is the socket-dir check and the list home rule
error-capture-reporting#2.1 · yes · contradict · revive's start now refuses a socket directory through refuse_socket_dir and writes one process-exit socket-dir-refused (F2), so the reading that a refused start check opens no process log and writes no line no longer holds for that refusal
error-capture-reporting#3.1 · no · the pre-init window's refusals are the instance and home-tree checks, which still write no line and leave the mode as found; the new socket-dir refusal sits after the log opens and is not among them
error-capture-reporting#4.1 · no · the vt100 feed catch_unwind in src/run/gate.rs is not reached and the session changed no feed or poisoning path
error-capture-reporting#4.2 · no · the vt100 passthrough witness is not reached and the session changed no outer-terminal output path
error-capture-reporting#5.1 · yes · add-to · F5 adds a socket-directory reject in frame after live_endpoint, returning instance-unreachable with no frame and no line, beside the check_instance reject that this entity records
error-capture-reporting#5.2 · yes · add-to · F5 adds the SocketDir mapping in frame, joining the StrictModes, without_endpoint and ServerVerify mappings this entity lists
error-capture-reporting#5.3 · no · the MAX_FRAME stdin cut and its end-of-stream reading are untouched; the viola-mcp change is in the frame refusals, not the stdin path
error-capture-reporting#5.4 · no · the oversize-before-initialize test and its one parse-rejected record are untouched; the session changed the frame refusals only
error-capture-reporting#6.1 · no · sanitised_fault and its five known codes and fixed messages are unchanged, and the new socket-dir and euid refusals are client-side before any frame, adding no peer fault code
error-capture-reporting#6.2 · no · is_code_word screening of a peer's refusal word is unchanged, and socket-dir-refused is a client-written word that no peer reply carries
error-capture-reporting#6.3 · yes · add-to · F8 adds Refused::NotADirectory, a fixed-message variant of Refused, beside the derive list; derives stay 9 and no derive moves
error-capture-reporting#6.4 · no · the exit-2 UNUSABLE_RESPONSE and UNUSABLE_TEXT paths in answer.rs and send.rs are not reached and sit after any socket-dir check
error-capture-reporting#6.5 · yes · stale · the exit-21 closed causes move from five to six with socket-dir-refused (F5's SocketDir), so the five count and the five-word list go stale
