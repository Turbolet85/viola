# Apply — security-plan · part 3 of 3

## Verdicts
- fanout-security-plan--cascade.md 23 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:8#1
- fanout-security-plan--cascade.md 24 · applied · .andromeda/security-plan-detail/security-anti-patterns.md:8#2, .andromeda/security-plan-detail/security-anti-patterns.md:8#3, .andromeda/security-plan-detail/security-anti-patterns.md:8#4

## Edits
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «verifying the server's pid (Windows `server_process_id()`) or euid + pid (Linux/FreeBSD `peer_creds()`)»
  new: «verifying the server's pid (Windows: the connected stream's peer pid, interprocess `peer_creds().pid()`, which calls `GetNamedPipeServerProcessId` on the SQOS-adopted client end) or euid + pid (Linux/FreeBSD `peer_creds()`; the client-side euid check owed to the route entry "Unix endpoint and home hardening")»
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «The dated exceptions, until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" land:»
  new: «The dated exceptions hold only the strict-modes half — the reference values taken from a `snapshot.json` read without the strict-modes check — owed to the Epoch 6 entry "Home and code-bearing file integrity"; on Linux and Windows each frame below verifies the serving process first (macOS: the recorded gap):»
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «each after the liveness-only pre-check, and»
  new: «each after the liveness pre-check and the server check, and»
- line: .andromeda/security-plan-detail/security-anti-patterns.md:8
  old: «tool frames after the liveness-only pre-check (the eighth»
  new: «tool frames after the liveness pre-check and the server check (the eighth»

## Patterns
[[pattern]]
id = "sec3-01"
fixed = 'pid (Windows `server_process_id()`)'

[[pattern]]
id = "sec3-02"
fixed = 'until the Epoch 6 entries "Server verification before any frame"'

[[pattern]]
id = "sec3-03"
fixed = 'after the liveness-only pre-check'
