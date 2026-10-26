# New groups — security-plan · authentication-authorization

## Verdicts
- fanout-security-plan--cascade.md 11 · distinct · line 24 states only the designed pid source and the macOS no-pid gap; no standing group names `Client::server_pid` / `TokioClient::server_pid` or `SERVER_PID_REPORTED`.
- fanout-security-plan--cascade.md 12 · distinct · line 24 states the designed pid + start-time comparison, consistent with this; the as-built `ServerVerdict` / `server_verdict` / `verify_server`, the one `read_snapshot` call and the `LiveEndpoint` shape stand nowhere.
- fanout-security-plan--cascade.md 13 · contradiction · lines 25–27 · «and checks neither the serving process's identity nor the snapshot's strict-modes» · the standing interim gaps say CLI `send`, `wait`, `last`, `answer`, `pause` and `release` write their frames after a liveness-only pre-check with no server identity check, while the proposal has them refuse on a mismatch or an unmakeable check with no frame sent (line 27's «On mismatch: CLI/MCP report `instance-unreachable`» already holds the exit-21 outcome; the hint text and the exit-13-first order are new).
- fanout-security-plan--cascade.md 14 · distinct · no standing group covers the exiting wrapper's Windows pipe window or its CARRY on "Windows-only live measurements".
- fanout-security-plan--cascade.md 15 · distinct · line 24 records the macOS known gap (euid + 0700 directory, no pid check), but not the 0755 exit-21 case, the client euid CARRY on "Unix endpoint and home hardening" or the negatives compiling out on macOS.
