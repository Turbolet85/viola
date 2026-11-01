# Apply — security-plan · part 1 of 4

## Verdicts
- fanout-security-plan--cascade-g1.md 35 · applied · .andromeda/security-plan.md:100#1
- fanout-security-plan--cascade-g1.md 36 · applied · .andromeda/security-plan.md:114#1
- fanout-security-plan.md 1 · folded into fanout-security-plan--cascade-g1.md 36
- fanout-security-plan--cascade-g3.md 10 · applied · .andromeda/security-plan.md:413#1
- fanout-security-plan.md 3 · folded into fanout-security-plan--cascade-g3.md 10
- fanout-security-plan--cascade-g3.md 13 · applied · .andromeda/security-plan.md:442#1
- fanout-security-plan.md 2 · folded into fanout-security-plan--cascade-g3.md 13
- fanout-security-plan.md 4 · applied · .andromeda/security-plan.md:70#1
- fanout-security-plan.md 5 · applied · .andromeda/security-plan.md:23#1
- fanout-security-plan--cascade-g1.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:43#1
- fanout-security-plan--cascade-g1.md 2 · applied · .andromeda/security-plan-detail/threat-model-summary.md:53#1
- fanout-security-plan--cascade-g1.md 3 · applied · .andromeda/security-plan-detail/threat-model-summary.md:56#1
- fanout-security-plan--cascade-g1.md 4 · applied · .andromeda/security-plan-detail/threat-model-summary.md:60#1, .andromeda/security-plan-detail/threat-model-summary.md:60#2
- fanout-security-plan--cascade-g1.md 5 · applied · .andromeda/security-plan-detail/threat-model-summary.md:100#1
- cascade-compare-security-plan.md 5 · applied · .andromeda/security-plan-detail/threat-model-summary.md:123+#1

## Edits
- line: .andromeda/security-plan.md:100
  old: «none on macOS), windows-sys 0.61.2»
  new: «none on macOS; its `euid()` the Unix read of the peer on both sides, the wrapper reading a client and a client reading the server, none on Windows), windows-sys 0.61.2»
- line: .andromeda/security-plan.md:114
  old: «the client euid check and macOS (no peer pid; its euid and verified socket-directory checks) are owed to "Unix endpoint and home hardening".»
  new: «on Linux and macOS the client also checks the recorded endpoint's socket directory before the connect, a refusal reading `socket-dir-refused`, and past the connect admits only a peer of its own euid, an unreadable peer refused, a refusal reading `server-verify-failed`, each sending no frame; macOS compares no peer pid or start time, the named carve-out, owned by no route entry, so there `server-verify-failed` means an euid mismatch alone.»
- line: .andromeda/security-plan.md:413
  old: «`endpoint-lost` · `server-verify-failed`, `wrapper-fault` with»
  new: «`endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, `wrapper-fault` with»
- line: .andromeda/security-plan.md:442
  old: «except the dated gaps: every client's euid check and macOS's pid and start-time check (no peer pid) are owed to "Unix endpoint and home hardening"; no other frame or process may borrow them.»
  new: «except the named carve-out: macOS's pid and start-time check (no peer pid), owned by no route entry, its euid and verified socket-directory checks standing; no other frame or process may borrow it.»
- line: .andromeda/security-plan.md:70
  old: «Unix domain sockets (`$TMPDIR` or `/tmp`); no Linux abstract namespace.»
  new: «Unix domain sockets (a verified per-user 0700 directory, never the `/tmp` or `$TMPDIR` root); no Linux abstract namespace.»
- line: .andromeda/security-plan.md:23
  old: «its names are predictable in shared namespaces (Windows pipes, `/tmp`).»
  new: «its names are predictable in the shared Windows pipe namespace, the Unix socket standing in a per-user 0700 directory.»
- line: .andromeda/security-plan-detail/threat-model-summary.md:43
  old: «socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix.»
  new: «socket file `<dir>/viola-<h12>.sock` on Unix, `<dir>` the per-user 0700 directory (§ Authentication & Authorization, IPC access control (Unix)).»
- line: .andromeda/security-plan-detail/threat-model-summary.md:53
  old: «On Unix with `TMPDIR` unset, the socket sits in world-writable `/tmp`, so another local user can create the path first.»
  new: «On Unix the socket sits in the per-user 0700 directory, not in world-writable `/tmp`; a directory another user owns, a loosened one or a symlink is refused by the socket-directory check, with no bind on `run` and no frame on a client.»
- line: .andromeda/security-plan-detail/threat-model-summary.md:56
  old: «macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening").»
  new: «ahead of that comparison, on Linux and macOS, each runs the socket-directory check on the recorded endpoint before connecting (a refusal `socket-dir-refused`, no frame) and the peer euid decision past the connect (a refusal `server-verify-failed`, no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.»
- line: .andromeda/security-plan-detail/threat-model-summary.md:60
  old: «one of five closed words»
  new: «one of six closed words»
- line: .andromeda/security-plan-detail/threat-model-summary.md:60
  old: «`server-verify-failed` (off macOS).»
  new: «`server-verify-failed` (Linux, Windows and macOS, on macOS an euid mismatch alone); `socket-dir-refused` (the socket directory of the recorded endpoint refused).»
- line: .andromeda/security-plan-detail/threat-model-summary.md:100
  old: «a refusal is `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line.»
  new: «a refusal is `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line. Past the liveness pre-check `frame` runs the socket-directory check on the recorded endpoint, a refusal `instance-unreachable` with `detail` `socket-dir-refused`, and past the connect the peer euid decision beside the server check, a refusal with `detail` `server-verify-failed`, each with no frame and no line.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:123
  anchor: «    - Before its first read or open under the home, `run`, `revive`'s start and `verify` hold the home tree to the strict-modes check (`check_home_tree`; a refusal exit 1, no log opened), and the frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` hold the instance to it (`check_instance`; a refusal exit 21), so a state tree chosen by `--home` or `VIOLA_DIR` that another user can write, or whose home, instance directory or `diagnostics/` another user can read, is refused.»
  add: «    - `viola list`'s board scan holds the home to the read rule on every OS (`check_read`) and `instances/` to `check_path`, so a home another user may read is refused whole, with or without an instance; the scan keeps its check order and its no-open, and `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home.»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = 'the client euid check and macOS (no peer pid'

[[pattern]]
id = "sec1-02"
fixed = '`endpoint-lost` · `server-verify-failed`,'

[[pattern]]
id = "sec1-03"
fixed = 'start-time check (no peer pid) are owed to'

[[pattern]]
id = "sec1-04"
fixed = 'Unix domain sockets (`$TMPDIR` or `/tmp`)'

[[pattern]]
id = "sec1-05"
fixed = 'predictable in shared namespaces (Windows pipes, `/tmp`)'

[[pattern]]
id = "sec1-06"
fixed = '`$TMPDIR/viola-<h12>.sock` (else `/tmp`)'

[[pattern]]
id = "sec1-07"
fixed = 'the socket sits in world-writable `/tmp`'

[[pattern]]
id = "sec1-08"
fixed = 'stays a recorded gap (owed to'

[[pattern]]
id = "sec1-09"
fixed = 'one of five closed words'

[[pattern]]
id = "sec1-10"
fixed = '`server-verify-failed` (off macOS)'
