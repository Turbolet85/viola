# Apply — obs-plan · part 1 of 5

## Verdicts
- fanout-obs-plan--cascade-g1.md 12 · applied · .andromeda/obs-plan.md:215#1
- fanout-obs-plan--cascade-g3.md 17 · applied · .andromeda/obs-plan.md:361#1, .andromeda/obs-plan.md:361#2
- fanout-obs-plan.md 3 · folded into fanout-obs-plan--cascade-g3.md 17
- fanout-obs-plan--cascade-g4.md 14 · applied · .andromeda/obs-plan.md:537#1
- fanout-obs-plan.md 4 · folded into fanout-obs-plan--cascade-g4.md 14
- fanout-obs-plan.md 1 · applied · .andromeda/obs-plan.md:311#1, .andromeda/obs-plan.md:311#2, .andromeda/obs-plan.md:311#3
- fanout-obs-plan.md 2 · applied · .andromeda/obs-plan.md:311#4
- fanout-obs-plan.md 6 · applied · .andromeda/obs-plan.md:479#1

## Edits
- line: .andromeda/obs-plan.md:215
  old: «a refused `diagnostics/` folder or file is a failed open, its line dropped.»
  new: «a refused `diagnostics/` folder or file is a failed open, its line dropped; a refused socket directory, read by the strict-modes read verdict, is no strict-modes failure: its line, where it has one, carries its own `detail`, `socket-dir-refused`.»
- line: .andromeda/obs-plan.md:361
  old: «on Linux and Windows a serving process that is not the recorded wrapper ends a CLI»
  new: «a serving process that is not the recorded wrapper (Linux and Windows) or runs under another euid (Linux and macOS) ends a CLI»
- line: .andromeda/obs-plan.md:361
  old: «(macOS compares no peer pid, the recorded gap); first»
  new: «(on macOS an euid mismatch alone, since macOS compares no peer pid, its named carve-out); before either comparison a refused socket directory ends a CLI `wait` / `last` before any frame, with one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN and no `channel-request` line, and MCP's returns `instance-unreachable` with `detail` `socket-dir-refused` and no line; first»
- line: .andromeda/obs-plan.md:537
  old: «the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows — is logged»
  new: «the liveness pre-check's, a frame tool's refused socket directory (made after it), or a frame tool's refused euid or server-verification mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) — is logged»
- line: .andromeda/obs-plan.md:311
  old: «squatted endpoint, a home tree another user could write or read:»
  new: «squatted endpoint, a socket directory `socket_dir_ready` refuses (Unix, exit 1, before any snapshot, child or `Server::bind`), a home tree another user could write or read:»
- line: .andromeda/obs-plan.md:311
  old: «called by `run` (7 sites)»
  new: «called by `run` (8 sites)»
- line: .andromeda/obs-plan.md:311
  old: «src` at chunk 2026-10-26-home-and-code-bearing-file-integrity»
  new: «src` at chunk 2026-11-01-unix-endpoint-and-home-hardening»
- line: .andromeda/obs-plan.md:311
  old: «adds three refusals to `run`'s five»
  new: «adds three refusals to `run`'s six»
- line: .andromeda/obs-plan.md:479
  old: «`params` and result bodies are never recorded, not even hashed.»
  new: «`params` and result bodies are never recorded, not even hashed. On Unix a peer the wrapper's `accept_loop` refuses by euid is dropped on the accept thread, before its worker thread and before any frame, and writes no line by rule (no frame to key a line on, `channel-response`'s `detail` closed); the client reads end-of-file.»

## Patterns
[[pattern]]
id = "obs1-01"
fixed = 'on Linux and Windows a serving process that is not the recorded wrapper'

[[pattern]]
id = "obs1-02"
fixed = 'compares no peer pid, the recorded gap'

[[pattern]]
id = "obs1-03"
fixed = 'server-verification mismatch on Linux and Windows'

[[pattern]]
id = "obs1-04"
fixed = '`run` (7 sites)'

[[pattern]]
id = "obs1-05"
fixed = '''refusals to `run`'s five'''
