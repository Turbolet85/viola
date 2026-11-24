@task_prose part=chunk
Changes the folder `viola adopt` starts its resumed claude child in, today the caller's current directory, so that the CLI finds the adopted session there.
Adds what `viola adopt` says and records when the resumed child ends at once right after the start: a closed refusal word, an exit code, a hint and an output line beside the existing refusal tables, an event and a log line, and whether the instance directory and the name are released after it.
If the fix moves where the R5 live-reading rig types `viola adopt`, the rig script changes with it and its rehearse form reaches the changed path with no claude.
Re-reads, without firing anything live: what the wrapper recorded of the two red children, where revive and up take their resume folder from, and what the capability ledger holds about a resume from another folder.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the viola adopt verb — the start that spawns the resumed claude child and its refusal and exit handling — edits: the spawn folder of the adopt start; the handling of a resumed child that exits at once (refusal word, exit code, hint, output line, event, log line, release of instance dir and name)
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the standing committed live rig for the gated live reading R5 of adopt, with its --rehearse form — edits: the folder the live arm types the command in, only if the fix changes where it must be typed
@task_item n=1 part=needs
The resume folder of an adopt start: the resumed claude child is spawned in the folder the CLI finds the adopted session from, no longer the caller's current directory; a test asserts the folder the child actually started in (as the fake agent's receipt records its cwd), never the product's own claim.
@task_item n=2 part=needs
How revive and up choose their resume folder — the instance snapshot's recorded cwd, read through the snapshot read-or-replay path, a replayed snapshot carrying no cwd — and that adopt alone has no such record; the adopt fix must take its folder from a source of its own and refuse, not guess, where that folder is missing, as revive's cwd-missing refusal does.
@task_item n=3 part=needs
A new closed refusal word for a resumed child that ends at once, added beside the existing start and revive refusal tables (run and revive exit 1 causes, revive refusal codes): its exit code, its own hint naming its cause, its human line and its --json failure document; a closed word changes only by decision.
@task_item n=4 part=needs
The early-exit detection itself: the child's exit is read on the process handle by waiting on it, never on PTY EOF, and an exit right after the start is told apart from a normal later end of the session.
@task_item n=5 part=needs
What an early exit of the adopted child records: the event log record and the process-exit log line, their fields and corr, each line JSON per line in the process log format, with no secret, no session transcript text and no upstream child output beyond what the contract allows.
@task_item n=6 part=needs
Whether the instance directory and the name are released after an early exit, so a second adopt or a run of the same name is not refused as already-live, squatted-name or stale-heartbeat afterwards; asserted on the files and liveness viola reads, not on a hand-written state file.
@task_item n=7 part=needs
The new hint and error body hold no path, no folder name and no upstream text, though the fix is about a folder; human output escapes control characters and carries no SGR without a terminal; one JSON document per verb run.
@task_item n=8 part=needs
The claude child spawn of adopt keeps the child environment strip and the process-start claude-child record (env kept, stripped counts) unchanged while its folder changes; the child is still given its viola name, instance dir and binary.
@task_item n=9 part=needs
Tests prove the fix only against the fake agent serving the claude boundary, with a fake agent that exits at once (its exit trigger) standing in for the real child's early exit; no real claude CLI in tests or CI, and a fake-agent run is no proof of real-CLI resume behaviour.
@task_item n=10 part=needs
Tests never read the host's claude sessions or real config; each adopt test runs under a test-owned user home and creates its own sessions, and the trust-start folders a test spawns the child in lie outside any repository.
@task_item n=11 part=needs
The R5 rig change, if any: the live arm types the command in the folder the fix requires; its --rehearse form reaches the changed path with no claude; the rig's seams stay rehearse-only, it runs clean of the firing session with its root outside every repository, and its screen read takes the compiled shapes only.
@task_item n=12 part=needs
Real adopt stays proven only by the gated live reading R5 of the adopt chunk; this chunk adds no live step, and a live reading is recorded as a reading, never as verification; the chunk claims no capability.
@task_item n=13 part=needs
The capability ledger as re-read for a resume from another folder: no row means UNMEASURED and is said so; the version gate and stamps are not widened by this chunk to claim a resume capability.
@task_item n=14 part=needs
viola adopt is tested as a subprocess, its exit codes and fixed strings pinned by literal asserts, waits keyed on logged events, with no sleep synchronisation and no retries.
@task_standing n=1
convention: every new test creates its own fresh private home and leaves the host's sessions unread
home: test-data-bootstrap
@task_standing n=2
convention: every new test waits on logged events, never sleeps, never retries, and runs in parallel without shared mutable state
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: every new CLI test asserts its human and --json outputs by literal oracles, its test function named by the plan's naming rule
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=4
convention: every new log line follows the process log line format, with its event and corr as the log-event corr table gives them
home: log-format
@task_standing n=5
convention: every new synthetic test input carries the canary and is free of real data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: every test that spawns viola ties it with a drop guard and drops inherited session environment variables
home: integration-test-strategy
@task_item n=1 part=not_needed
The MCP server and its tools, tools/list, MCP invalid-params fields and MCP reply screens: adopt is a CLI verb and the chunk adds no MCP tool or result.
@task_item n=2 part=not_needed
Dialog hooks, hook decision bodies, viola hook exec entry points and their deadlines: the chunk touches no hook path.
@task_item n=3 part=not_needed
viola send, answer, wait, last and release, send confirmation, paste framing and the wheel: none of these verbs or records change.
@task_item n=4 part=not_needed
The wrapper channel's link and unlink methods and JSON-RPC error codes: the early exit happens before any channel frame.
@task_item n=5 part=not_needed
Window placement, raise and re-open readings R3 and R4: they belong to other gated live readings, not to adopt.
@task_item n=6 part=not_needed
viola verify stamping, fixture recording and its process-exit details, and the fake agent's input keystroke and slash-command contracts: the chunk records no fixture and stamps nothing.
@task_item n=7 part=not_needed
Performance budgets, hyperfine timing, mutation-run verdicts and fuzz corpora: the chunk adds no hook path, parser or perf-timed surface.
@task_item n=8 part=not_needed
The harness session, status, boot, cleanup, gate and pre-push contracts: the chunk changes no harness command or its documents.
@task_item n=9 part=not_needed
The viola list board's columns and list --json items: the chunk adds no board column or list field.
