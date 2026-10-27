# Classified — tests · test-strategy.b1

## Answers
test-strategy#1.1 · yes · the chunk's absent-copy arm and its fs.rs private-write rule need unit cases inline in pin.rs and fs.rs, and the all-three-OS unit run is where the Windows exe arm's OS-only check is verified
test-strategy#1.2 · yes · the narrower concurrent-pin control is a crate-level On-disk contract case over a real tempfile home, since the race rests on disk state across processes
test-strategy#1.3 · yes · Path 1's start sequence and tampered-exe case, Path 7's workaround and the two-instance race witness are the E2E cases the chunk reaches (Paths 2–6 are not)
test-strategy#1.4 · no · the chunk adds or changes no parser, and the pinned-exe hash is not a parser surface
test-strategy#1.5 · no · the hook deadlines the perf arm gates are the hook path's, which the chunk leaves unchanged since its fix sits on the start path
test-strategy#1.6 · yes · two starts racing one pinned-copy write is a process and disk fault the chaos tier names, so the race witness is a chaos case that must stay deterministic with no sleep or retry
test-strategy#1.7 · yes · the per-chunk remove-the-guard rule binds because the red-before-fix control lets two product starts run and so needs a bounded witness through Running with WITHIN, the mutation gate itself staying at the epoch boundary
test-strategy#1.8 · no · the chunk changes no fake-agent replay, recorded fixture, hook stdin or recorded CLI behaviour, and the contract drift checks it leaves unchanged
test-strategy#2.1 · yes · the race witness's rig is the events.rs two-instance boot and its session-record wait, which the scope names, so the witness takes a root layer prefix and uses the shared helpers rather than copying them
test-strategy#3.1 · no · the chunk changes no IPC frame, endpoint or server-identity check, since the pinned copy is a file and not a channel
test-strategy#3.2 · no · paste validation and the typed text of send are untouched by a pinned-copy write
test-strategy#3.3 · no · the hook's stdin parser is unchanged, and the race only spawns the hook without altering what it parses
test-strategy#3.4 · no · the chunk changes no MCP tool, frame or recorded exchange
test-strategy#3.5 · yes · run's and verify's pinned-copy paths change and the scope names their exits (exit 1 before any log for a widened pinned copy or a hash mismatch, exit 1 for a tampered exe), which are CLI integration cases
test-strategy#3.6 · yes · the absent-copy write is a filesystem write under the home, so the integration and chaos cases bind, including the umask-000 owner-only negative that the fix's mode-before-bytes rule must keep
test-strategy#3.7 · yes · the agent's direct spawn of the absolute pinned path is the race's signal and Path 1's pinned copy must precede the child spawn, so the process-boundary integration cases bind while Scenario E2 does not
test-strategy#3.8 · no · the chunk adds no dependency or CI job, the release-check stage is release packaging owed to Version done-check, and the measurement re-reads its profile and target dir only as local evidence
test-strategy#3.9 · no · the chunk adds or changes no parser
test-strategy#3.10 · yes · the race witness is process and disk fault injection, which the chaos-test trigger places in the root chaos suite
test-strategy#3.11 · no · no hook deadline or heartbeat budget moves, since the fix sits on the start path
test-strategy#3.12 · no · the chunk changes no cross-surface CLI or MCP coordination and leaves Scenario E3 untouched
test-strategy#3.13 · yes · the chunk's Windows exe arm is an OS-only check read on the Windows CI runner, so the multi-OS CI matrix binds as the rule on where that code is verified
test-strategy#3.14 · no · the multi-version trigger concerns claude CLI versions in viola-agent-claude and its fixtures, which the chunk does not touch
test-strategy#3.15 · no · the fake agent's contract against recorded fixtures is unchanged, and the chunk only uses the fake agent and reads its receipts
test-strategy#3.16 · no · the error-sanitization and NEVER-log floor are owed by every change that adds a log line or error alike, and the chunk's verdict reads existing events, receipts and exit codes
