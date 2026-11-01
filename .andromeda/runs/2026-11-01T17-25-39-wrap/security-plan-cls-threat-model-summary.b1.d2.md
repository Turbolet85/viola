# Classified — security-plan · threat-model-summary.b1

## Answers
threat-model-summary#1.1 · no · the session changes no event data, volume or retention, and no reached file alters what events.ndjson holds
threat-model-summary#1.2 · no · the tool-call, plan and dialog payload content and its channel are not reached by the session
threat-model-summary#1.3 · no · the config and snapshot fields, including the stored endpoint, are unchanged; the new check only reads the endpoint directory
threat-model-summary#1.4 · no · the credential environment and its strip list are not reached by the session
threat-model-summary#1.5 · no · snapshot and budget fields and their 0600 modes are unchanged, and the new refusal line holds no path, pid or uid
threat-model-summary#2.1 · yes · contradict · the session disproves the $TMPDIR-root socket file and the world-writable /tmp reading, adds the peer euid drop and client euid checks on Linux and macOS, and grows the closed causes from five to six with socket-dir-refused
threat-model-summary#3.1 · no · the send text rules and delivery confirmation are untouched, and the new checks refuse only before any frame, so the empty-text refusal still writes nothing
threat-model-summary#4.1 · no · the stdin classifier and the wheel rules are not reached; the session changes the start check and the accept-loop drop, not run's stdin
threat-model-summary#5.1 · yes · stale · the hook paths now also run the socket-directory check and the peer euid check, and a refused directory writes a hook-decision WARN line, so the enumerated hook checks and the no-line reading move
threat-model-summary#6.1 · yes · stale · MCP's frame now runs the directory check after live_endpoint and the euid check beside server_verified, and a directory refusal reads detail socket-dir-refused, not only strict-modes-failed
threat-model-summary#7.1 · no · the CappedLines stdin reader is not reached; the session changes the frame path, not MCP stdin reading
threat-model-summary#8.1 · yes · stale · run and revive's start now refuse a bad socket directory with exit 1, and the frame verbs add the socket-dir and peer euid checks at exit 21, so the enumerated CLI checks grow
threat-model-summary#9.1 · no · the check_home_tree path list and the readers' strict-modes checks are unchanged, and the new socket-dir check guards the endpoint directory, not the home tree the row enumerates
threat-model-summary#10.1 · no · the pinned binary placement and its close-before-name ETXTBSY handling are not reached
threat-model-summary#11.1 · no · the spawn list for list, mcp, revive's preflight, verify and run's --version is unchanged, and list's home read rule does not change when it spawns
threat-model-summary#12.1 · no · no dependency was added or bumped and cargo deny stayed green, so the supply-chain claims hold on the changed Cargo.toml and Cargo.lock
threat-model-summary#13.1 · no · the CI jobs and the per-OS test matrix are unchanged, and the new cfg(unix) test runs inside the existing per-OS test job
