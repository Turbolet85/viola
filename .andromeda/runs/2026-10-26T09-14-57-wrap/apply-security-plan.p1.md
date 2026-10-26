# Apply — security-plan · part 1 of 3

## Verdicts
- fanout-security-plan.md 1 · applied · .andromeda/security-plan.md:114#1
- fanout-security-plan.md 2 · applied · .andromeda/security-plan.md:439#1
- fanout-security-plan.md 3 · applied · .andromeda/security-plan.md:100#1
- fanout-security-plan--cascade-new-error-handling.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:8+#2
- fanout-security-plan--cascade.md 21 · applied · .andromeda/security-plan-detail/error-handling.md:8#1
- fanout-security-plan--cascade.md 22 · applied · .andromeda/security-plan-detail/error-handling.md:8+#1
- fanout-security-plan--cascade.md 1 · applied · .andromeda/security-plan-detail/threat-model-summary.md:54+#1

## Edits
- line: .andromeda/security-plan.md:114
  old: «the client verifies the server against the snapshot (Windows: an SQOS identification-level open, then the serving pid and start time; Unix: euid, and pid with start time where available; macOS: euid and the verified socket directory only, a known gap), never falling back to a default connect; the dated interim gaps stand until their two route entries land.»
  new: «the client verifies the server against its one snapshot read: on Linux and Windows the connected peer's pid (Windows over an SQOS identification-level open) and that process's start time, a mismatch or a check that cannot be made sending no frame, never falling back to a default connect; the snapshot read without the strict-modes check is a dated gap owed to "Home and code-bearing file integrity", and the client euid check and macOS (no peer pid; its euid and verified socket-directory checks) are owed to "Unix endpoint and home hardening".»
- line: .andromeda/security-plan.md:439
  old: «(the euid check never skipped), except the dated gaps (`hook.event`; the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames; `hook.dialog`; `viola mcp`'s tool frames) until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" land;»
  new: «(the euid check never skipped), except the dated gaps: `hook.event`, `hook.dialog`, the CLI `send`, `wait` / `last`, `answer`, `pause` / `release` frames and `viola mcp`'s tool frames verify the server against a snapshot read without the strict-modes check, owed to "Home and code-bearing file integrity", and every client's euid check and macOS's pid and start-time check (no peer pid) are owed to "Unix endpoint and home hardening";»
- line: .andromeda/security-plan.md:100
  old: «`Stream::peer_creds()`, `PipeStream::server_process_id()`)»
  new: «`Stream::peer_creds()`, its `pid()` also the client's read of the serving process: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-opened client end, none on macOS)»
- line: .andromeda/security-plan-detail/error-handling.md:8
  old: «the one-object parse, and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`).»
  new: «the one-object parse and, on Linux and Windows, the server verification (macOS the recorded gap), and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable` · `server-verify-failed`; strict-modes is owed to "Home and code-bearing file integrity" (Decisions Log `2026-09-28`).»
- after: .andromeda/security-plan-detail/error-handling.md:8
  anchor: «- `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human. As landed, `viola hook <event>` runs the stdin cap, the `VIOLA_NAME` / `VIOLA_DIR` shape checks and the one-object parse, and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`). Its hidden `--capture <DIR>` arm runs the stdin cap, the absolute, existing `<DIR>` check and the exclusive `create_private_new` claim, and writes the capture raw; with `--answers <DIR>` it also runs the `take(64)` closed-`ProbeAnswer` read and `probe_body`'s payload shape check, and prints one `decision_body` body to stdout only for a mapped answer. Its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` runs the same stdin cap and `<DIR>` check and claim, writes the payload raw and the closed shell verdict, and prints nothing on any path. On every failure it prints nothing, its stderr is always empty, and it exits 0 (Decisions Log `2026-09-28`, capture arm; the founder's live ruling of 2026-10-05 on `--answers`; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows («Run B + --settings», relayed verbatim by the operator, that chunk's `inputs#I1`), on the statusline form).»
  add: «- A `viola hook` event or dialog whose server check fails writes no frame and exits 0 with empty streams; its `hook-decision` line carries `server-verify-failed` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`), and a SessionEnd whose check fails is appended through the direct `append_session_end`.»
- after: .andromeda/security-plan-detail/error-handling.md:8
  anchor: «- `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human. As landed, `viola hook <event>` runs the stdin cap, the `VIOLA_NAME` / `VIOLA_DIR` shape checks and the one-object parse, and fails open with `hook-decision` detail `oversize-stdin` · `malformed-json` · `channel-unreachable`; server verification and strict-modes join at Epoch 6 (Decisions Log `2026-09-28`). Its hidden `--capture <DIR>` arm runs the stdin cap, the absolute, existing `<DIR>` check and the exclusive `create_private_new` claim, and writes the capture raw; with `--answers <DIR>` it also runs the `take(64)` closed-`ProbeAnswer` read and `probe_body`'s payload shape check, and prints one `decision_body` body to stdout only for a mapped answer. Its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` runs the same stdin cap and `<DIR>` check and claim, writes the payload raw and the closed shell verdict, and prints nothing on any path. On every failure it prints nothing, its stderr is always empty, and it exits 0 (Decisions Log `2026-09-28`, capture arm; the founder's live ruling of 2026-10-05 on `--answers`; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows («Run B + --settings», relayed verbatim by the operator, that chunk's `inputs#I1`), on the statusline form).»
  add: «- A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; it adds no error word and writes no log line.»
- after: .andromeda/security-plan-detail/threat-model-summary.md:54
  anchor: «      - Clients (`hook`, `send`, `mcp`) connect to the `endpoint` recorded in the snapshot and act on whatever server answers there. A process holding that name while the real wrapper is gone could receive `hook.dialog` payloads and send back dialog decisions.»
  add: «      - As built, on Linux and Windows every frame-writing client (CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s event and dialog paths; `viola mcp`'s `send` / `wait` / `last` / `answer` tools) compares the connected peer's pid and that process's start time with the snapshot's `pid` and `started_at` before its first frame, and sends no frame on a mismatch or a check it cannot make. The snapshot is read without the strict-modes check (owed to "Home and code-bearing file integrity"), and macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening").»

## Patterns
[[pattern]]
id = "sec1-01"
fixed = 'the dated interim gaps stand until their two route entries land'

[[pattern]]
id = "sec1-02"
fixed = 'until the Epoch 6 entries "Server verification before any frame"'

[[pattern]]
id = "sec1-03"
regex = 'server_process_id\(\)'

[[pattern]]
id = "sec1-04"
fixed = 'server verification and strict-modes join at Epoch 6'
