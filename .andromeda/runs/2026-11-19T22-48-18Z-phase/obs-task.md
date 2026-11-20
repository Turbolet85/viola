@task_prose part=chunk
Under viola run, a prompt that a CLI-scheduled wake submits is told apart from a prompt the human typed, so a fired wake leaves the wheel with its holder (capability v2-36).
After a wake is told as one, the driver's next send is confirmed and not refused human-typing, and the next question dialog is held for the driver; a prompt the human types still takes the wheel.
The means is this phase's to find: the CLI's UserPromptSubmit payload carried no source key on a fired wake in live readings, so a candidate means may rest on the wrapper's own pty (no byte from the human's side before the submit while a wake was scheduled), the turn end's scheduled_wakes count, or a capability ledger row with a verify probe; when it cannot tell, the origin is human.
It carries forward, without acting on them now: a third reading of the placeholder hint's cell attribute (founder's word), the rig rehearsal's intermittent red (acted on only at its next red), and the boot-timeout watch read at each run with no bound moved.
@task_item n=1 part=needs
The wheel's origin decision in viola run: where a submitted prompt's origin is decided from the UserPromptSubmit hook payload and the human's keys; it gains a scheduled-wake origin, and the conservative fallback is human.
@task_item n=2 part=needs
The wheel transition and its audit: a prompt told as a wake makes no wheel transition to human, so no pause or wheel product event, wheel log line or run.wheel_transition span is emitted for it, while a human-typed prompt still emits them as before.
@task_item n=3 part=needs
The wrapper's send path: after a fired wake the driver's next send passes the readiness gate and is confirmed (send-issued, send-confirmed) instead of a wrapper send-refused with detail human-typing; human-typing stays the detail when the human holds the wheel.
@task_item n=4 part=needs
The question dialog path: the next dialog after a fired wake is held for the driver for its hold window instead of being rendered for the human; the dialog-raised and dialog-answered lines and the hook decision keep their fields and corr.
@task_item n=5 part=needs
The hook payload parse for UserPromptSubmit in the claude agent adapter: the CLI schema declares an optional source field (schedule_wakeup, loop_wakeup) that live payloads did not carry; an absent field must not fail the parse, and an unrecognised value surfaces as unknown with a parse-rejected line (unknown-value) rather than a guess.
@task_item n=6 part=needs
The hook stays fail-open and silent: any new reading in the UserPromptSubmit hook keeps exit 0, writes no stderr, puts only the decision body on stdout, and stays inside the hook latency and spine hook perf budgets.
@task_item n=7 part=needs
The pty byte source (the pump that reads the human's keys) as a signal: the wrapper may note that human-side bytes arrived, but typed keys and the pty byte stream are never logged nor placed in any span field or detail code.
@task_item n=8 part=needs
The prompt text a wake submits is conversation content: it never reaches a home-level process log, a span or a detail word; only the instance detail file may hold content.
@task_item n=9 part=needs
The normalised event for a submitted prompt in viola-core and the turn end's scheduled_wakes count: any origin field added is a closed value, added to the events log line shape additively, and viola-core still emits no telemetry of its own.
@task_item n=10 part=needs
The capability ledger, if a relied row with a verify probe is part of the means: an older or unverified CLI, or an unreadable ledger, counts as unverified and gets the conservative human origin; ledger path and stamps never logged; verify lines carry no content; the verify probe runs against the fake agent in CI and the real CLI never in CI.
@task_item n=11 part=needs
Any new detail word or event value the means needs (an origin, a refusal or a parse-rejected cause) is a closed code with no path or pid, added to the diagnostic schema and event enum additively, never renaming an existing field.
@task_item n=12 part=needs
The live rig and the fake agent, where a wake would be rehearsed: no live start in this phase; any rehearsal or test waits on the log or event it expects, never on a sleep, and carries no retry rule.
@task_item n=13 part=needs
The rig rehearsal's intermittent red (box read empty after x and del): stays open with its cause unread; at its next red the printed rig summary and the kept after-del frame name the cause before the wait is mended, with no retry rule and no loosened entry.
@task_item n=14 part=needs
The boot-timeout watch (a boot that never gets ready times out and stops its supervisor, 10 s wall bound): read at each run, its bound not moved.
@task_item n=15 part=needs
The placeholder hint's cell attribute third reading: a fresh box, keys before any send, a start that makes no send after them, reading drawn hint cells past the cursor's by attribute only, never text; screen content is no telemetry source, and its start and keys are the founder's word.
@task_standing n=1
convention: every new log line goes through the obs_event macro with a closed event value and corr passed explicitly by the caller; no print or raw tracing macros
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: every line a new test makes a process emit is schema-checked in integration and E2E tests against the additive-only diagnostic schema
home: ci-integration
@task_standing n=3
convention: each new integration or E2E test keeps its home under the one test-home root, kept until the gate steps run
home: 9-ci-integration
@task_standing n=4
convention: each new logged field passes the default-deny source allow-list; content only in the instance detail file
home: 8-pii-scrubbing-compliance
@task_standing n=5
convention: each new span is declared manually with a static name, skips all arguments and lists its fields explicitly
home: 4-span-trace-coverage
@task_standing n=6
convention: each new recorded fixture is scrub-required and never a telemetry source
home: 1-obs-scope-summary
@task_standing n=7
convention: each new test waits on an observed line or event, never on a sleep, and no test gains a retry policy
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The MCP stdio server's tool frames, MAX_FRAME limits and MCP-side unreachable refusals: the chunk changes no MCP tool or frame.
@task_item n=2 part=not_needed
viola revive start preflight, revive --list and the agents probe of claude agents --json: the chunk touches no revive path.
@task_item n=3 part=not_needed
Link and unlink verbs, implicit link records and link names: the chunk changes no linking.
@task_item n=4 part=not_needed
Socket-directory, strict-modes and server-verification refusals and their exit-1 and exit-21 cause words: no connection or home-tree check changes.
@task_item n=5 part=not_needed
Start collision detail codes (already-live, squatted-name, stale-heartbeat, pinned-hash-mismatch, cwd-missing, no-session) and the run start span chain: the chunk changes no start.
@task_item n=6 part=not_needed
Heartbeat liveness, list liveness and stale thresholds: untouched.
@task_item n=7 part=not_needed
Snapshot replay and state-recovered lines (torn-line healing, unsupported v): untouched.
@task_item n=8 part=not_needed
A future error reporter, Sentry, OTel exporters, traceparent and network egress: the chunk adds no reporter or exporter.
@task_item n=9 part=not_needed
config.json keys, diagnostics level and home resolution order: the chunk adds no configuration.
@task_item n=10 part=not_needed
hyprctl probing and the harness logs and agent-run status outputs: the chunk does not reach them.
