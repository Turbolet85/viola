@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, covering only the relied-on window queries and dispatches the later window entries need, with no general Hyprland client and no verb.
Each compositor reply is read capped at MAX_FRAME (refused past it), treated as untrusted upstream output and parsed tolerantly into closed viola types; window titles, classes and all reply text are content, never a command.
Each seam call ends in a closed outcome code (ok, unsupported, unreachable, malformed, ...) logged codes-only through obs_event!; outside Hyprland the seam answers unsupported, never a panic or a guess.
A fake compositor replays replies recorded live on the founder's host and scrubbed like every fixture, so the seam's tests run green on ubuntu-latest; the relied-on requests are measured live once at named operator steps.
Placement (new workspace crate or root-bin module) and transport (hyprctl -j spawned child, or Hyprland's request socket, with a new HYPRLAND_INSTANCE_SIGNATURE environment read) are left to P3/P4.
@task_item n=1 part=needs
The new seam component: the single place a Hyprland request, reply shape or socket is named, with closed outcome types (ok, unsupported, unreachable, malformed and the like) as its only result words.
@task_item n=2 part=needs
Compositor replies are untrusted upstream output: parsed tolerantly, a parse failure becomes a closed malformed or unknown outcome rather than a fault, the way the claude agents output and its parse failures are handled.
@task_item n=3 part=needs
Reply reads are bounded at MAX_FRAME and an oversize reply is refused, with an oversize parse-rejected style outcome and no reply bytes in the line.
@task_item n=4 part=needs
Reply text (window titles, classes, workspace names) is content: it reaches no log line, span field, error message or refusal; telemetry is never taken from window or screen content.
@task_item n=5 part=needs
Codes-only logging: every seam call emits one closed event and detail code through the obs_event macro only, never a raw tracing or print macro, never a stringified field.
@task_item n=6 part=needs
No window address, pid, path, socket name or instance signature in any home-level line, detail code or span field; paths and ids appear only where the plan lets them, as typed fields in the instance detail file.
@task_item n=7 part=needs
Outside Hyprland the seam returns a closed unsupported outcome: no panic, no guess, and errors at the seam end in one logged line with a fixed Display message that interpolates nothing.
@task_item n=8 part=needs
If the transport spawns hyprctl: a direct spawn, absolute path, no shell, observed at viola's call site with its process-start/process-exit lines, duration and exit status, and a spawn subject from the closed subject enum (a new subject is an additive schema word).
@task_item n=9 part=needs
If the transport is the request socket: the socket path and its directory checks are never logged, a refusal names nothing, and its outcome is a closed code like the socket-directory refusals.
@task_item n=10 part=needs
The new environment read of the instance signature: environment names only, never values, in any line; environment variables are not a configuration channel; a new env read needs its own Decisions Log entry.
@task_item n=11 part=needs
Recorded compositor fixtures are scrub-required and are no telemetry source; the Hyprland version read with each recording is kept with the fixture, not in logs.
@task_item n=12 part=needs
Any new event, detail or subject word the seam emits is an additive change to the diagnostic line schema and must match the closed event enum; no word the build cannot emit is added.
@task_item n=13 part=needs
If the seam is a new crate: synchronous instrumentation without Tokio, obs dependencies Tokio-free and C-free and passing cargo deny, no tracing-subscriber outside the root binary, any third-party log target off.
@task_item n=14 part=needs
Spans the seam declares: manual spans with static area.operation names, skip all arguments with an explicit field allow-list, no err or ret options, no endpoint, path or env value in a span field.
@task_item n=15 part=needs
Seam tests on ubuntu-latest drive the fake compositor; every test or control that spawns a compositor client carries a kill deadline; lines the tests emit are schema-checked and the zero-panics gate still holds.
@task_item n=16 part=needs
Linux is the only OS target; the window features are Hyprland only and the seam reports unsupported elsewhere rather than branching per OS.
@task_standing n=1
convention: each new log line goes through obs_event with a closed event, closed detail codes and only allow-listed typed fields, codes-only, no content, path or pid
home: 11-obs-anti-patterns-never-do-these · logging-stack
@task_standing n=2
convention: each new diagnostic event or detail word is an additive, schema-conformant entry in the diagnostic line schema and its closed enums
home: 6-log-coverage · log-coverage
@task_standing n=3
convention: each new span is declared manually with a static name, skip all and an explicit field allow-list
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=4
convention: each new error type has a fixed Display message and ends in one logged line at its seam
home: 7-error-capture-reporting · error-capture-reporting
@task_standing n=5
convention: each new recorded fixture is scrubbed and is never a telemetry source
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_standing n=6
convention: each new integration test keeps its home under the shared test-home root, its emitted lines schema-checked and its panic count at zero
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The MCP stdio server rows: its stdout frames, tools and isError results; the seam adds no MCP tool and changes no MCP output.
@task_item n=2 part=not_needed
The dialog and hook paths (hook stdout, fail-open, statusline arm and its shell-out, dialog raised and answered lines); the seam touches no hook or dialog.
@task_item n=3 part=not_needed
The send, wait, last and wheel paths with their span chains, log fields and refusal details; no channel method or verb is built here.
@task_item n=4 part=not_needed
The budget governor, heartbeat file and liveness rows, and config.json keys; the seam reads no budget, heartbeat or config key.
@task_item n=5 part=not_needed
viola verify, the Claude capability ledger and its stamps; the ledger stays Claude-only and the seam's live measurements ride a separate record.
@task_item n=6 part=not_needed
Revive, link and unlink verbs and their process-exit details; no such verb changes.
@task_item n=7 part=not_needed
The future error reporter, OTel, network egress and exporter absences; the seam adds no reporter, exporter or network path.
@task_item n=8 part=not_needed
The events.ndjson product audit stream and snapshot replay rows; the seam writes no product event and reads no snapshot.
@task_item n=9 part=not_needed
Hook performance budgets and the hyperfine perf job; the seam is not on a hook path and no perf gate is added.
@task_item n=10 part=not_needed
The harness and agent-run logs output rows (merged sources, filters, status); the seam adds no reader of the diagnostics files.
