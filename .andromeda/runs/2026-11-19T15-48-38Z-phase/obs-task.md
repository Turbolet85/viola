@task_prose part=chunk
First a test-only mend of the standing live rig: its kept record dir keyed by the CLI version, checked per version, not per label, so a later version's recorded set never meets an earlier one's.
Then one contained live start under the rig's private Claude config whose turn ends in the CLI's StopFailure hook, its payload recorded through the rig's scrub, never under the operator's real Claude or viola homes.
That payload's error word is read against the nine-word turn-end cause map: the turn end's cause is the mapped word, and no parse-rejected unknown-value line is written for any of the CLI's 13 words.
The StopFailure exception is settled on that reading by the founder: a capability-ledger row with its viola verify probe, or the exception kept on his word; the plan carries both shapes.
Each of the five carries is disposed by name; the placeholder-hint cell reading and the intermittent rehearsal red stay measured only as the scope says, the latter acted on only at its next red.
@task_item n=1 part=needs
The standing live rig, a test-only harness: its kept record directory is keyed by the CLI version and its existence check runs per version, so a record kept by an earlier chunk never reads as this start's record; no product behaviour changes with this mend.
@task_item n=2 part=needs
The viola hook entry for the CLI's StopFailure event: a hook invocation that must stay silent on stderr, fail open, exit zero and write only its decision body on stdout while the live payload passes through it.
@task_item n=3 part=needs
The hook stdin parser reading the StopFailure payload's error word: an unknown word surfaces as unknown with a parse-rejected unknown-value line, and none of the CLI's 13 known words may produce that line.
@task_item n=4 part=needs
The nine-word turn-end cause map: the turn end's cause field carries the word the payload's error word maps to, a closed enum value, never the payload's free text.
@task_item n=5 part=needs
The hook-invoked and hook-decision lines written for the StopFailure turn end: their required fields and corr, codes only, no payload body, no assistant message or last_assistant_message.
@task_item n=6 part=needs
One settlement shape: a capability-ledger row for StopFailure with its viola verify probe, which adds a verify spawn subject step whose log lines carry no content, whose ledger path and stamps are never logged, and where verify stays the only stamps writer.
@task_item n=7 part=needs
The verify probe's reach: a real-CLI verify never runs in CI and CI verify runs against the fake agent only, so a new probe needs its fake-agent arm for CI and its real arm only on the operator's host.
@task_item n=8 part=needs
The other settlement shape: the StopFailure exception kept as a named provisional exception on the founder's word, with no ledger row and no probe, and an unreadable or absent ledger still read as an unverified CLI.
@task_item n=9 part=needs
The recorded StopFailure payload is a Claude fixture: scrub-required before it is kept, and never a telemetry source for the product's own lines.
@task_item n=10 part=needs
Containment of the live start: one of two unmeasured means, a no-network namespace or the child's ANTHROPIC_BASE_URL set to a closed port; a value set into the claude child's environment is never logged, only env names and counts reach the process-start line of the claude child.
@task_item n=11 part=needs
The wrapped CLI's behaviour on a lost connection (retry, duration, which hook fires, whether a turn must be in flight) is read statically from the installed bundle and is not a contract the product may rely on.
@task_item n=12 part=needs
Credentials boundary: the rig reads the operator's credentials file once as an owner-only copy removed at teardown and stamps the operator's CLI config read-only; nothing is written under the operator's real Claude or viola homes and no secret or credential value reaches any log line or kept record.
@task_item n=13 part=needs
Every test start runs under a test-owned user home; the rehearsal under the fake agent reaches the containment means without any live start.
@task_item n=14 part=needs
No retry rule and no loosened wait: the intermittent rehearsal red is mended only after its cause is read from the printed rig summary and the kept after-del frame, and any test wait takes a bound under the 20 s lint rather than a sleep-based log wait.
@task_item n=15 part=needs
The placeholder-hint reading of the rig's kept frames: whether the hint is told apart from typed text by a cell attribute the screen model can read stays unmeasured unless a kept frame shows it; screen content yields no telemetry.
@task_item n=16 part=needs
Kept rig products (recorded sets, frames, rig summaries) are local run outputs, never uploaded as CI artifacts.
@task_item n=17 part=needs
Documentation gains no figure the code states (counts, member lists, versions) unless a master row holds it, so the cause-map word counts and ledger row counts stay out of prose.
@task_standing n=1
convention: each new integration or E2E test that starts a process runs under a test-owned home beneath the one integration-test home root, kept until the gate steps run
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: each line a new test or probe emits is schema-checked against the diagnostic line schema, closed event and detail words only
home: ci-integration
@task_standing n=3
convention: each new test that waits on a log line or state waits on a bounded condition, never a sleep
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: each new recorded fixture from the real CLI is scrubbed before it is kept and is never read as a telemetry source
home: 1-obs-scope-summary · pii-scrubbing-compliance
@task_standing n=5
convention: each new log line in product code goes through the one obs event macro with a closed event value and explicit corr and instance
home: 11-obs-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The MCP stdio server: its tools, frame-tool refusals, stdin frame bounds and isError results; the chunk adds no tool and touches no MCP frame.
@task_item n=2 part=not_needed
The channel JSON-RPC surface and the send, wait, last and wheel paths: their spans, log fields, refusal details and corr rules; no send, wait or wheel is changed.
@task_item n=3 part=not_needed
The dialog raise and answer path, the pause and release methods and their events; StopFailure is a turn-end hook, not a dialog.
@task_item n=4 part=not_needed
The revive, link and unlink verbs and their preflight and exit-detail codes.
@task_item n=5 part=not_needed
Heartbeat, liveness and list output, and the agent-run status and logs wrapper surfaces.
@task_item n=6 part=not_needed
Snapshot replay and the state-recovered line, torn-line healing and the events-log appender.
@task_item n=7 part=not_needed
Exit-1 and exit-21 detail codes for socket-directory, strict-modes and server-verification refusals, and the run start's collision, pin-copy and version-gate span attributes.
@task_item n=8 part=not_needed
Future error reporters, OTel exporters, span processors, Sentry and network telemetry receivers; nothing here exports.
@task_item n=9 part=not_needed
Panic capture by role and the panic line's keys and backtrace handling; no catch site changes.
@task_item n=10 part=not_needed
config.json keys, diagnostics level and home resolution order; no configuration is added.
