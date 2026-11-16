@task_prose part=chunk
Documents and pins by tests the wake condition of wait: which logged event kinds return a parked wait (CLI viola wait, wait --json, the MCP wait tool, the wrapper channel wait method) and which park it again, proved over the fake agent.
A turn end while a background task still holds the session wakes wait (the turn-ended line carrying its background task count, or the question it raises); a later log-only notification after a task-free turn end gives no false wake.
Re-reads at P3, from the code and the fake agent, what wait returns on today and whether a zero background-task turn end that still waits on a notification is documented behaviour, a defect, or the driver's to re-arm.
Takes the carried CI red: the stamped-home test fixture's unbounded viola verify run is either taken out of the tests' measured time (a home stamped once and shared or copied) or the ci profile's test kill is sized to its loaded time, chosen by measurement.
No live claude start without asking first (any granted start under a private config dir); no mutation run.
@task_file n=1 path=src/run/wait.rs mark=edits
what: the wrapper's wait feed over the event log — the wake kinds versus the log-only kinds — edits: the wake condition and its wait_each_wake_kind_wakes / wait_log_only_kinds_never_wake test cases
@task_file n=2 path=src/cmd/wait.rs mark=edits
what: the CLI viola wait verb and its --json output — edits: the wait verb
@task_file n=3 path=src/human/wait.rs mark=edits
what: the human-mode rendering of the CLI wait verb's result
@task_file n=4 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola mcp server's tool handlers — edits: the MCP wait tool
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: the core crate's shared types — edits: the event kinds of the event catalog (turn-ended, question, notification-class lines)
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent test binary and its test seams — edits: its turn scripts (turn end with and without a background task, a later notification)
@task_file n=7 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripted turn fixtures
@task_file n=8 path=tests/support/verify.rs mark=edits
what: the stamped-home test support that runs viola verify to stamp a test home — edits: the stamped_home fixture's verify call (within: None)
@task_file n=9 path=.config/nextest.toml mark=edits
what: the nextest test profiles — edits: the ci profile's slow-test kill for the stamped-home class
@task_item n=1 part=needs
The wrapper channel wait method and the MCP wait tool: their params (wait and last params, timeout_ms) and what a wait returns stay within their contracts while the wake condition is documented and pinned.
@task_item n=2 part=needs
The turn-ended event line's background task field: the Stop hook's background tasks are carried as a count only, and a wake read from it uses that count and nothing more of the hook payload.
@task_item n=3 part=needs
The question event a turn end raises wakes wait: question option text crossing into events is content, never a command, and the driver's read of it stays content.
@task_item n=4 part=needs
Log-only notification-class lines never wake a wait armed after a task-free turn end; the wake set is a closed set of event kinds, not a match on upstream text.
@task_item n=5 part=needs
The event log reader behind wait: bounded line reads, torn last lines tolerated, readers never write the state files they read.
@task_item n=6 part=needs
Event kinds stay closed protocol types in the core crate's shared validators; an unknown or new kind parses tolerantly to unknown and never wakes by default.
@task_item n=7 part=needs
Hook payload parsing that feeds turn-ended and notification events: tolerant parsing, security fields in closed enums, the hook failing open and silent.
@task_item n=8 part=needs
CLI wait output: the --json document carries closed values only, human-mode output escapes control characters, and no paths, pids or ids appear in its errors.
@task_item n=9 part=needs
The fake agent's turn scripts and any new seam stay test-only: compiled under the fake-agent feature only and out of release builds.
@task_item n=10 part=needs
A stamped-home test home shared or copied across tests keeps owner-only modes and passes the strict-modes check before use; tests use their own home and remove only the directories they created; stamps are written only by viola verify.
@task_item n=11 part=needs
The ci nextest profile change runs in CI with no secrets and no real claude CLI; the real CLI stays local only.
@task_item n=12 part=needs
A live claude start, if granted, runs under a private config dir: nothing written under the Claude home, credentials stripped from the child, the start count named before the card.
@task_item n=13 part=needs
Each new guard test carries a one-off remove-the-guard control instead of a mutation run; mutation testing stays at the epoch audit.
@task_item n=14 part=needs
New fake-script fixtures follow fixture hygiene: viola-format fixtures under the hygiene walk, scrubbed or refused whole, reviewed before commit.
@task_standing n=1
convention: each new test uses its own viola home, kept out of git, and removes only the directories it created
home: secret-management · bootstrap-phases
@task_standing n=2
convention: each new fixture is scrubbed, under the hygiene walk and reviewed before commit
home: data-protection
@task_standing n=3
convention: each new test seam or environment read lives only under the fake-agent feature and needs its log entry
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_standing n=4
convention: each new guard test is proved by a one-off remove-the-guard control
home: threat-model-summary
@task_item n=1 part=not_needed
Send and answer typing rules — bracketed paste, paste text character sets, delivery confirmation, the readiness race — send confirmation is another chunk's.
@task_item n=2 part=not_needed
Socket directory, socket mode and peer euid checks at bind and connect: the chunk changes no channel endpoint or client seam.
@task_item n=3 part=not_needed
revive, link, unlink, pause and release contracts and refusals: none of those verbs changes.
@task_item n=4 part=not_needed
Pinned binary placement and re-hash, plugin install and the marketplace tree: no pinned artefact or plugin file changes.
@task_item n=5 part=not_needed
Release workflow, artefact attestations, signed self-update and release-stage absences: there is no release stage in this chunk.
@task_item n=6 part=not_needed
rmcp feature set and banned HTTP or auth features: the MCP transport and dependency features stay as they are.
@task_item n=7 part=not_needed
Hyprland recorder, terminal colour facts and the viola list board scan: untouched by a wait change.
@task_item n=8 part=not_needed
Inherited credential stripping by name and the process-start env lines of the claude child: no child environment change on the fake-agent path.
@task_item n=9 part=not_needed
Key management, stored secrets, secret managers, accounts, databases, uploads and webhooks: absent and not reached.
