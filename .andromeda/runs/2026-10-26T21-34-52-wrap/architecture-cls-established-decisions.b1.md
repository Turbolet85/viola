# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the session changed no dependency or crate (Cargo.toml, Cargo.lock and deny.toml untouched), and the tools.rs edit adds no Tokio use, so the Tokio-only-in-mcp rule still holds on the changed files
established-decisions#2.1 · yes · add-to · the companions' owner/DACL refusal is a new fallback cause, `strict-modes-failed` (the word list goes from four to five, F14), and revive's recorded-directory reading now shares one snapshot read with its collision check (F12)
established-decisions#3.1 · no · the readiness gate and screen model sit in the unreached src/run/gate.rs and viola-agent-claude, no ledger row changed, and the reached send.rs edit only adds an instance check ahead of the frame
established-decisions#4.1 · no · the empty-typed-text refusal (refusal_of in send.rs) still comes before any frame, since send reads its text, then runs refuse_untrusted, then writes the frame, so the claim holds on the changed file
established-decisions#5.1 · yes · add-to · the state-recovered line now also comes from the snapshot replay, which state.snapshot_recover places under run.start before run.collision_check and under hook.handle (F11), beside the healing line this entity names
established-decisions#6.1 · yes · add-to · the pinned copy is now pinned by pin_bytes from bytes already checked, with the pin key named once by pin_key (F13), beside the replace_private_shared list that names the pinned copy
established-decisions#7.1 · no · wait's blocking, cursor and timeout rules, the endpoint naming and the Windows SQOS connect are unchanged (no wire shape changed), and a vanished endpoint still returns instance-unreachable
established-decisions#8.1 · no · the peer-pid and start-time comparison every frame-writing client makes before its first frame is unchanged (the server identity check is untouched), and the session only adds an instance check ahead of it
established-decisions#9.1 · no · server_verdict is unchanged (Matches with no comparison on macOS, the peer-pid gap stays), and its owed entry, Unix endpoint and home hardening, is not the one this session closed
established-decisions#10.1 · yes · stale · the claim that every reader stays on read_snapshot is no longer true: read_snapshot has no product caller under src/ (3 → 0), and live_endpoint, hook connect, statusline serve and collision_check now read the replay (F11)
established-decisions#11.1 · no · the MCP tool set, the isError mapping and the CLI-only release, pause, link and unlink are unchanged in tools.rs, and a refused instance check is the existing instance-unreachable code, not a new refusal class
established-decisions#12.1 · yes · add-to · the SessionEnd ladder starts only after the instance check, so a refused or absent instance makes no SessionEnd append, not even the direct try_lock one (F8)
established-decisions#13.1 · yes · stale · the connect's first step is no longer read_snapshot: hook connect reads the replay and treats a replay or absence as unreachable (F11), after the instance check
established-decisions#14.1 · yes · add-to · a refused or absent instance joins the exit-0-at-once cases, with no config.json read, role file, stdin read, line or frame, and the --capture arms still stay ahead of the check (F8)
established-decisions#15.1 · no · the dialog-expired append is the wrapper's WaitFeed in src/run, which the session does not reach, and a refused instance never opens a dialog that could expire
established-decisions#15.2 · no · it records a live measurement of a 671 s held dialog hook on 2.1.287, which this session neither re-measures nor changes
established-decisions#15.3 · no · a dialog hook whose server check fails still sends no hook.dialog and exits 0 with no body, the same fail-open that the new refusal and the replay-unreachable paths take
established-decisions#16.1 · no · the hold ceiling and its owed live readings are untouched, since the session's hook.rs changes are the instance check ahead of the event path and the connect's replay read
