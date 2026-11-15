@task_prose part=chunk
Ends a turn the claude CLI ends on an API error: the CLI's signal (a StopFailure-class hook, re-measured at P3) is registered, read and logged as a turn-ended event carrying a cause code from a closed viola set, never the error text.
After that turn end the instance reads idle on the board, the wheel's turn-running state is cleared exactly as a Stop clears it, a wait on the turn end returns, and the driver's next send is accepted with no human pause and release.
Adds a cause key to the turn-ended event data contract, gives the fake agent the ability to emit the API-error end, and adds a test that fails on the tree before the fix.
Re-reads at P3 from the code which state reads turn-running while the board reads idle, and re-measures the ending mechanism from the code and the recorded fixtures.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the embedded Claude Code plugin's exec-form command hook registrations, each hook mapped to a viola hook subcommand — edits: the hook registration for the CLI signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot in the wrapper, holding the wheel's turn-running state that refuses a send mid-turn — edits: the turn-end path that clears the turn-running state
@task_file n=3 path=src/run/wait.rs mark=edits
what: the run pump's channel wait method, blocking until a driver-relevant event such as turn-ended — edits: the turn-end path a waiter wakes on
@task_item n=1 part=needs
A new hook registration in the plugin for the API-error turn-end signal: exec-form command hook, its hook tier (async, or the sync ordering spine with no reply body), its row in the hook-to-kind map beside Stop, and its hook subcommand name joining the register of hook names.
@task_item n=2 part=needs
The hook binary's behaviour on the new hook: fails open, never writes stderr, exits 0 with no body, exits silently with no trusted instance, stays inside the hook connect and spine deadline, and loses the event rather than the turn when the endpoint is lost.
@task_item n=3 part=needs
The API-error payload read in viola-agent-claude, the only crate that knows Claude shapes: parsed tolerantly, the CLI's error string mapped into the closed cause set, a value outside it reading a fixed fallback member (unparseable external values become unknown), and the crate doing no file I/O.
@task_item n=4 part=needs
The cause set's neutral home in viola-core: a closed set with every member named, no Claude-specific name, added only in viola-core per the agent-neutral event-kind rule, its wire values kebab-case enum strings, and the root bin consuming only the normalised event.
@task_item n=5 part=needs
The turn-ended event data contract: a new cause key beside last_assistant_message and background_tasks — a wire-contract change to Standard Contracts — under the event line shape, snake_case field names, absent optional fields omitted, and the normalised event kinds list.
@task_item n=6 part=needs
Mixed-version tolerance for the new key: readers skip and count unknown data keys, missing reads as null, so older and newer binaries keep reading the same events log; whether the change bumps the protocol version follows the protocol-version bump rule.
@task_item n=7 part=needs
The error text never leaves as data: last_assistant_message and error_details on this path reach no events.ndjson line, no snapshot and no process-log line — upstream text is untrusted content, process logs hold codes only, and content-bearing detail lives only in the instance detail file.
@task_item n=8 part=needs
The wheel and send path after the API-error end: the turn-running state cleared exactly as a Stop clears it, so the next driver send is not refused not-delivered turn-running (CLI exit 13) and a send is still refused, never queued, while a turn truly runs.
@task_item n=9 part=needs
The channel wait method: a wait on the turn end wakes and returns on the API-error turn-ended event, through the blocking wait (nothing polls), its start offset and timeout unchanged.
@task_item n=10 part=needs
The board (viola list and MCP list) and the instance snapshot read idle after the API-error end, with the snapshot replay agreeing with the live state and the instance snapshot carrying no error text.
@task_item n=11 part=needs
The fake agent's ability to emit the API-error end through its test seams, with recorded or synthetic payloads; a synthetic payload scrubbed like a recorded fixture and named as a probe capture payload file; no live claude in CI and tests run against the fake agent.
@task_item n=12 part=needs
A dependence on undocumented claude behaviour (whether the StopFailure-class hook fires live for every API-error end) is ruled by the capability ledger as the single gate for CLI behaviour, with a viola verify probe; whether a new row is added and how it is probed is decided at P3/P4.
@task_item n=13 part=needs
A live claude start is not given: real CLI probes run only locally, and any needed live reading is a counted fork asked before the card; a home given to verify --record goes through the recorded-fixture home helper, fixtures recorded only from a clean verify.
@task_item n=14 part=needs
The new guard test fails on the tree before the fix and carries a one-off remove-the-guard control under a kill deadline, named by the test's name alone; no mutation run, local or dispatched.
@task_item n=15 part=needs
A rate-limit or billing cause read on the turn end is a logged code only: it opens no usage-budget gate, no budget state, and no send paused or refused on usage.
@task_item n=16 part=needs
Error handling in the new code: typed thiserror errors per crate whose messages interpolate nothing, anyhow only in the root bin, error chains only in the detail file, events emitted only through the one obs event emitter.
@task_standing n=1
convention: each test runs in its own isolated viola home, cleaned by its owner, its evidence surviving a killed test
home: established-decisions · occupied-resources
@task_standing n=2
convention: a test's channel socket goes under the short /tmp root, outside the product endpoint namespace
home: occupied-resources
@task_standing n=3
convention: tests drive the fake agent, never a live claude in CI; fake-agent seams are test seams, never configuration, and test-support code stays out of product builds
home: occupied-resources · ci-cd-approach
@task_standing n=4
convention: a recorded or synthetic fixture is scrubbed and hygiene-walked, holding no home path, username, email or absolute path
home: occupied-resources
@task_standing n=5
convention: every new log line is a codes-only one-line JSON process-log line emitted through the one event emitter, with no print macros or tracing level macros
home: cross-cutting-patterns · build-system
@task_standing n=6
convention: end-to-end runs show zero panic lines, and root test waits share one bound
home: project-directory-structure · occupied-resources
@task_standing n=7
convention: an OS-gated test names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The MCP server's own transport, rmcp features, stdin line cap and tools/list shape: the chunk adds no MCP tool and changes no MCP framing.
@task_item n=2 part=not_needed
The Hyprland compositor seam, window features and the planned terminal fleet console: nothing in a turn end touches a window.
@task_item n=3 part=not_needed
Revive, the pinned binary copy, its re-hash and placement, and the start order and start refusals: the chunk starts and revives nothing.
@task_item n=4 part=not_needed
Link and unlink recording, the link set and its events: the turn end changes no link.
@task_item n=5 part=not_needed
Dialog handling — the dialog hold, answer, pending dialog, permission, plan and question events: an API-error end is no dialog.
@task_item n=6 part=not_needed
Send confirmation, the readiness gate, bracketed paste, typed-text normalisation and the confirmation windows: the scope puts send-confirmation whitespace and the readiness-gate race out.
@task_item n=7 part=not_needed
The wait wake condition for background-task questions: out of this chunk's scope.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the installed MCP-only plugin: the installed plugin carries no hooks, and the chunk changes only the wrapped-session plugin.
@task_item n=9 part=not_needed
config.json keys, configuration precedence and colour environment: the chunk adds no setting.
@task_item n=10 part=not_needed
Release tooling, crates.io publishing, licence, MSRV and toolchain pins: the chunk ships nothing and adds no dependency.
@task_item n=11 part=not_needed
Fuzz workspace, weekly fuzzing and the fuzz corpus replay: the chunk adds no fuzz target.
@task_item n=12 part=not_needed
Mutation testing at the epoch boundary and the mutation scratch and target directories: the directive runs no mutation.
@task_item n=13 part=not_needed
CLI exit-21 unreachable documents, socket-directory refusals and server verification before a frame: the chunk changes no client connect path.
