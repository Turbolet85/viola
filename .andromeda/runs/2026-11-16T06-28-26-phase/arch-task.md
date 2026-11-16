@task_prose part=chunk
Makes a driver's send of a local slash command that the CLI runs read as delivered (ok), confirmed by an observable echo of that command, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command waits for a prompt-submitted that never fires and ends not-delivered / no-prompt-submitted although it ran; a listed command with no measured post-condition reads ok, unconfirmed, detail unconfirmable.
Re-measures from the code and the recorded fixtures what echo the CLI gives for a local command (a hook event, a screen line the readiness model reads, or another signal) and which commands it covers, before the plan states it.
Re-reads what a send of a local command reads today: the listed post-conditions, unconfirmable, and the unlisted command's no-prompt-submitted, on the send path, the CLI send human mirror and the MCP send result.
Stands on the fake agent and its local-command mode; no live claude start is planned without asking first, and an echo relied on becomes a ledger row with a verify probe.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path: the post-gate send, its confirmation after the fact and its ok / not-delivered result — edits: how a local command's send is confirmed
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classification of typed text by the ledger's local-command list into what confirms it (prompt-submitted or a local command's post-condition) — edits: the classification of a local command, today Confirms::Prompt for an unlisted one
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send's confirmation slot the wrapper waits on inside the confirmation window
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the per-behaviour CLI capability ledger and the local commands listed with delivery post-conditions — edits: the local-command list LOCAL_COMMANDS, a new ledger row if the echo is relied on
@task_file n=5 path=src/cmd/verify/ mark=edits
what: viola verify, the probes that measure and stamp each relied ledger row — edits: the probe for the local command's echo
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: the version gate that reads the ledger stamps strictly and degrades a send on an unverified CLI
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the CLI send human mirror of the send result, with unconfirmable's fixed note
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent the tests and CI run against — edits: its --local-command-mode
@task_file n=9 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripted sessions the tests drive
@task_file n=10 path=fixtures/claude/ mark=re-reads
what: the recorded CLI payload fixtures per claude version, read back to measure what echo a local command gives
@task_item n=1 part=needs
Every send is confirmed after the fact: a local command that runs reads ok only on an echo the product observes, and an unconfirmed delivery still reads not-delivered.
@task_item n=2 part=needs
Local commands are classified by the ledger: the local-command list and its delivery post-conditions decide what confirms a send, and an unlisted command the CLI runs must no longer wait for a prompt-submitted that never fires.
@task_item n=3 part=needs
The send results a local command can end in: ok, ok with the unconfirmable detail and confirmed false, and not-delivered with no-prompt-submitted; refusal details stay a closed set.
@task_item n=4 part=needs
An echo the send relies on is an undocumented CLI behaviour: a capability-ledger row with a viola verify probe, measured and stamped; where no row is relied on, no probe is added.
@task_item n=5 part=needs
On an unverified CLI or with an unstamped row the send stays honest about what it cannot confirm: the partial gate and transport-only degrade, with a version verified only when every relied row passes.
@task_item n=6 part=needs
Claude shapes stay in the agent crate: what counts as a local command's echo is read in viola-agent-claude, which does no file I/O, and the send path in the root bin consumes only a normalised fact or event.
@task_item n=7 part=needs
If the echo needs a new normalised event kind or event data, it is added only in viola-core with agent-neutral kind names and kebab-case wire values.
@task_item n=8 part=needs
Upstream text is content: a local command's output is never read as a viola command or config and crosses no log, event or error.
@task_item n=9 part=needs
The wrapper-appended send records (send-issued, send-confirmed, send-refused), their event data and the send-confirmed order in the events log, as a local command's confirmation writes them.
@task_item n=10 part=needs
The confirmation wait stays inside its compiled window and the send time bound, under the send flight lock; nothing polls.
@task_item n=11 part=needs
If the echo is a screen line: run never reads screen content and the vt100 model is a pre-send readiness gate only, screen signatures are compiled literals, and screen row text is read only by verify.
@task_item n=12 part=needs
The CLI send human mirror and the MCP send result carry the local command's outcome only as they already read the wrapper's reply; unconfirmable's fixed note stays fixed.
@task_item n=13 part=needs
The verify probe for the echo: verify's bounded child set, it types into no CLI dialog, it is the one stamps writer, the ledger stamps envelope rows it writes, and fixtures recorded only from a clean verify.
@task_item n=14 part=needs
Any live claude reading runs only locally, never in CI, under a private CLAUDE_CONFIG_DIR inside a started session's turn, with its count of starts named and asked before the review card.
@task_item n=15 part=needs
The fake agent's local-command mode and its test seams carry the echo for the tests; test seams are no configuration channel and viola never reads harness variables.
@task_standing n=1
convention: each new test runs against the fake agent in its own isolated viola home, a stamped home coming from the shared stamp's copy
home: established-decisions
@task_standing n=2
convention: each test that waits on a channel or a thread takes the shared bound and cannot hang past a failure
home: occupied-resources
@task_standing n=3
convention: each test channel socket goes under the short /tmp root, outside the product namespace
home: occupied-resources
@task_standing n=4
convention: each recorded fixture is scrubbed and hygiene-walked, keeping only signature rows for screens
home: occupied-resources
@task_standing n=5
convention: each new log line is codes-only, emitted through obs_event!, with content-bearing detail only in the instance
home: occupied-resources · cross-cutting-patterns
@task_standing n=6
convention: each new JSON field is snake_case and each new wire enum value kebab-case, absent optional fields omitted
home: conventions
@task_standing n=7
convention: each end-to-end run ends with zero panic lines
home: project-directory-structure
@task_item n=1 part=not_needed
The wheel's rules (human take, byte-source claim, release, harness prompts): the scope leaves them untouched.
@task_item n=2 part=not_needed
wait's wake rule and the channel wait method's params, wake kinds and pending dialog: untouched by this chunk.
@task_item n=3 part=not_needed
The dialog paths: dialog hold, hook.dialog, answer and its permission, plan and question responses.
@task_item n=4 part=not_needed
The MCP server's transport and tool-surface shape: rmcp, stdin reader and line cap, tools/list, exit codes.
@task_item n=5 part=not_needed
Revive and run start: start order, child argv and environment, pinned copies, start refusals.
@task_item n=6 part=not_needed
The board and list: agents listing, list payload, session liveness and snapshots read through replay.
@task_item n=7 part=not_needed
Link, unlink, pause and release methods, their CLI verbs and exit codes.
@task_item n=8 part=not_needed
Hyprland seam, window features and the planned fleet console.
@task_item n=9 part=not_needed
Plugin install, the plugin's marketplace tree and .mcp.json.
@task_item n=10 part=not_needed
Release tooling, deploy, fuzz workspace and weekly advisory or fuzz jobs; no mutation run is planned.
@task_item n=11 part=not_needed
Configuration keys, colour environment and diagnostics level.
