# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · no accounts or passwords exist in v1, so the chunk has no password input to bound
authentication-authorization#1.2 · yes · each of the nine served methods and hook.event rests on its own params check, which the chunk's closed method and params checks (-32602) build
authentication-authorization#1.3 · no · the listener's protected SDDL and accept_remote(false) are Not needed (owned by Unix endpoint and home hardening), and the chunk's server.rs edit is the frame read, not the bind
authentication-authorization#1.4 · no · the Unix 0700 socket directory and the socket chmod are Not needed (owned by Unix endpoint and home hardening), and the chunk's frame read leaves the bind alone
authentication-authorization#1.5 · yes · the chunk's server-side frame read must come after the accept-time peer_creds euid drop, so no frame is read before it (the check itself is Not needed)
authentication-authorization#1.6 · no · the verify-before-first-frame gate sits in the connect that the chunk's hook and send paths only call, the SQOS open and macOS gap are Not needed, and the client euid check is owed to Unix endpoint and home hardening
authentication-authorization#1.7 · yes · a release whose from is not a string must return -32602 with data null, a closed-type params check the chunk builds (the release-from-driver refusal is decision policy, Not needed)
authentication-authorization#1.8 · yes · every bounded home read the chunk adds or moves (snapshot, events, stamps, budget, config, statusline source, heartbeat) must come after its entry point's strict-modes check
authentication-authorization#1.9 · yes · the closed pinned-key shape the chunk validates must precede the bin/<key> and plugin/<key> joins that check_home_tree makes, and the tree check must still precede the log
authentication-authorization#2.1 · no · the Windows remote-client witness covers the protected listener, Not needed (owned by Unix endpoint and home hardening), and no bounded input is involved
authentication-authorization#2.2 · no · the Windows anonymous-open DACL witness covers the protected listener, Not needed (owned by Unix endpoint and home hardening), and no bounded input is involved
authentication-authorization#3.1 · no · the observed server pid is read by a transport call the chunk leaves unchanged, and that read writes and logs nothing
authentication-authorization#3.2 · no · the ServerVerdict and verify_server live in viola-state liveness, which the chunk does not reach; it only calls connect and leaves the verdict unchanged
authentication-authorization#3.3 · yes · the chunk's ViolaName::try_new on link and unlink's driver and driven must run before the instances/<driven> join this check makes, the check's own order being unchanged
authentication-authorization#3.4 · yes · the chunk's send and answer text read must follow refuse_untrusted and precede any frame, the order this entity fixes for the frame verbs
authentication-authorization#3.5 · no · a CARRY owed to Windows-only live measurements, which the task's Not needed excludes from this chunk
authentication-authorization#3.6 · no · the macOS server-verification cases are a CARRY owed to Unix endpoint and home hardening, and the server-identity negatives compile out on macOS
authentication-authorization#3.7 · yes · the existing channel link and unlink name witnesses (-32602 for bad, foreign and self-pair names, no line written) are the witnesses the chunk's name validation must keep green
