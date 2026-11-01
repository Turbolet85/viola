# Apply — obs-plan · part 2 of 5

## Verdicts
- fanout-obs-plan--cascade-g1.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:37#1
- fanout-obs-plan--cascade-g1.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:124#1, .andromeda/obs-plan-detail/obs-scope-summary.md:124#2
- fanout-obs-plan--cascade-g1.md 3 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:166#1
- fanout-obs-plan--cascade-g1.md 4 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:216#1
- fanout-obs-plan--cascade-g1.md 5 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:303#1
- fanout-obs-plan--cascade-g1.md 6 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:321#1
- fanout-obs-plan--cascade-g1.md 7 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:330#1
- fanout-obs-plan--cascade-g1.md 8 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:373#1, .andromeda/obs-plan-detail/obs-scope-summary.md:373#2
- fanout-obs-plan--cascade-g1.md 9 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:374#1, .andromeda/obs-plan-detail/obs-scope-summary.md:374#2
- fanout-obs-plan--cascade-g1.md 10 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:382#1
- fanout-obs-plan--cascade-g1.md 11 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:400+#1
- fanout-obs-plan--cascade-g1.md 13 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:460#1
- fanout-obs-plan--cascade-g1.md 14 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:491#1
- fanout-obs-plan--cascade-g1.md 15 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:492#1, .andromeda/obs-plan-detail/obs-scope-summary.md:492#2
- fanout-obs-plan--cascade-g1.md 16 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:493#1
- cascade-compare-obs-plan.md 1 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:191#1
- cascade-compare-obs-plan.md 2 · applied · .andromeda/obs-plan-detail/obs-scope-summary.md:205+#1

## Edits
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:37
  old: «exit 21's five cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`), with no serde path»
  new: «exit 21's six cause words (`strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`), with no serde path»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:124
  old: «the server check's peer-pid and start-time reads, whose failed check»
  new: «the server check's peer-euid (Unix), peer-pid and start-time reads, whose failed check»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:124
  old: «which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below).»
  new: «which of their refusals write a line is the strict-modes refusal-telemetry trigger's (below). The socket directory's check (`check_socket_dir`: viola's one `symlink_metadata`, off Unix no read) is such a call site, its refusal written under `socket-dir-refused` (MCP by no line), and the wrapper's read of a client's euid in `accept_loop` writes no line.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:166
  old: «The socket-directory cause and macOS's server check are the named carve-outs from this rule, both with the route entry "Unix endpoint and home hardening" (the Founder Direction 4 trigger below states them).»
  new: «A refused socket directory has its own code, `socket-dir-refused`, and writes its line; macOS compares no peer pid, the named carve-out from this rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone (the Founder Direction 4 trigger below states them).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:191
  old: «gate-2026-10-26-home-and-code-bearing-file-integrity.json`).»
  new: «gate-2026-10-26-home-and-code-bearing-file-integrity.json`); the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too, the rows under `SPINE_DEADLINE` and at or under that band with them (as measured at `.andromeda/runs/2026-11-01T16-38-56-implement/outcome-2026-11-01-unix-endpoint-and-home-hardening.md`).»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:205
  anchor: «  - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).»
  add: «  - A stream whose peer euid is refused is dropped on the accept thread (`accept_loop`, `peer_admitted`), before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no line, there being no frame to key a line on and `channel-response`'s `detail` being closed, and nothing on the terminal.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:216
  old: «`endpoint-lost`, `server-verify-failed`; the CLI's document less `v`»
  new: «`endpoint-lost`, `server-verify-failed`, `socket-dir-refused`; the CLI's document less `v`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:303
  old: «`.cmd`/`.bat` child, and for a start made by `viola revive` also»
  new: «`.cmd`/`.bat` child, `socket-dir-refused` (a refused socket directory, on `run` and on a start made by `viola revive`: one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind), and for a start made by `viola revive` also»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:321
  old: «`exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`), `duration_ms`»
  new: «`exit_code` (21 when the wrapper is killed mid-wait, `detail` `endpoint-lost`, `during:"call"`, or when a CLI `wait` meets a refused socket directory before any frame, `detail` `socket-dir-refused`, `during:"connect"`, no `channel-request`), `duration_ms`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:330
  old: «`server-verify-failed` on Linux and Windows: exit 21 on CLI `answer` with `during:"connect"`, fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint)»
  new: «`server-verify-failed` on Linux, Windows and macOS (on macOS an euid mismatch alone) and `socket-dir-refused`, each exit 21 on CLI `answer` with `during:"connect"` and fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint)»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:373
  old: «channel failure, server-verify mismatch, strict-modes failure, missing `VIOLA_NAME`.»
  new: «channel failure, server-verify mismatch, refused socket directory, strict-modes failure, missing `VIOLA_NAME`.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:373
  old: «the operator's answer at its planning superseded 2026-10-13).»
  new: «the operator's answer at its planning superseded 2026-10-13). A refused socket directory, though its check shares the same read verdict, is no such silent path: it writes its line under its own word.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:374
  old: «A server-verify mismatch (Linux and Windows) is one»
  new: «A server-verify mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) is one»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:374
  old: «path or endpoint in the line; macOS compares no peer pid (the recorded gap).»
  new: «path or endpoint in the line; a refused socket directory takes the same form with `detail` `socket-dir-refused`; macOS compares no peer pid (the named carve-out).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:382
  old: «`instance-dead` or `unwrapped-name`, and one whose server check fails returns `server-verify-failed`, each likewise with no frame.»
  new: «`instance-dead` or `unwrapped-name`, one whose socket directory is refused (`check_endpoint_dir`, after `live_endpoint`) returns `socket-dir-refused`, and one whose server check fails or whose server's euid is refused returns `server-verify-failed`, each likewise with no frame.»
- after: .andromeda/obs-plan-detail/obs-scope-summary.md:400
  anchor: «  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in three places: the `ledger-stamps` read in `run`'s version gate, on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; and a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`), which leaves only verify's own `process-exit` (with an instance; exit 1, `detail` `strict-modes-failed`). Every other strict-modes refusal leaves no line: `viola revive`'s instance check, `run`'s and a revived start's home-tree check (`check_home_tree`, exit 1, before the wrapper log opens), `viola verify`'s home-tree check (exit 1), the instance check of every `viola hook` arm but the two `--capture` arms (exit 0), the instance check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`refuse_untrusted`, exit 21), `viola link` / `viola unlink`'s instance check of the driven instance (made before any role file opens; exit 21), the `viola mcp` frame tools' instance check (`instance-unreachable`), the `diagnostics/` opens (`check_diagnostics`: a refusal is a failed open), the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), and `viola list`'s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list).»
  add: «  - A refused socket directory (`check_socket_dir`, which shares the read verdict) is outside that count, writing under its own word `socket-dir-refused`: one `process-exit` on a start (exit 1) and on a CLI frame verb (exit 21, `during:"connect"`), one `hook-decision` on `viola hook` (exit 0), and no line on MCP's `frame`.»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:460
  old: «the server check's peer-pid read: Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so the verdict is `Matches` with no comparison and no `server-verify-failed` line is written there: the recorded gap, owned by "Unix endpoint and home hardening").»
  new: «the server check's reads: the peer pid, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so that verdict is `Matches` with no comparison: the named carve-out, owned by no route entry); the peer euid, Unix `peer_creds` (a mismatch reads `server-verify-failed`, on macOS its one meaning) vs Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison the controls).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:491
  old: «`strict-modes-failed`, `server-verify-failed`) and exit 1 (`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and `viola revive`'s `no-session`»
  new: «`strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`) and exit 1 (`already-live`, `stale-heartbeat`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, `socket-dir-refused`, and `viola revive`'s `no-session`»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:492
  old: «The server-verify fail is written on Linux and Windows as `process-exit{»
  new: «The server-verify fail is written on Linux, Windows and macOS (on macOS an euid mismatch alone) as `process-exit{»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:492
  old: «path or endpoint; macOS writes none (no peer pid, the recorded gap).»
  new: «path or endpoint; macOS compares no peer pid (the named carve-out).»
- line: .andromeda/obs-plan-detail/obs-scope-summary.md:493
  old: «Two causes stand outside the distinct-code rule, both with the route entry "Unix endpoint and home hardening": the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check. OPEN — the socket-directory cause has no code · owner: leaving its code to that entry.»
  new: «The socket-directory cause has its code, `socket-dir-refused`, whatever failed: a refused check, a failed create or a lost race reads that one word. One carve-out stands outside the distinct-code rule: macOS compares no peer pid, named and owned by no route entry.»

## Patterns
[[pattern]]
id = "obs2-01"
fixed = 'five cause words'
ignore_case = true

[[pattern]]
id = "obs2-02"
fixed = 'the named carve-outs from this rule, both with the route entry'

[[pattern]]
id = "obs2-03"
fixed = '`server-verify-failed` on Linux and Windows'

[[pattern]]
id = "obs2-04"
fixed = 'server-verify mismatch (Linux and Windows)'

[[pattern]]
id = "obs2-05"
fixed = 'macOS compares no peer pid (the recorded gap)'

[[pattern]]
id = "obs2-06"
fixed = 'no `server-verify-failed` line is written there'

[[pattern]]
id = "obs2-07"
fixed = 'the recorded gap, owned by "Unix endpoint and home hardening"'

[[pattern]]
id = "obs2-08"
fixed = 'fail is written on Linux and Windows as'

[[pattern]]
id = "obs2-09"
fixed = 'macOS writes none (no peer pid, the recorded gap)'

[[pattern]]
id = "obs2-10"
fixed = 'the socket-directory cause has no code'

[[pattern]]
id = "obs2-11"
fixed = '`server-verify-failed`) and exit 1'

[[pattern]]
id = "obs2-12"
fixed = '`endpoint-lost`, `server-verify-failed`; the CLI'
