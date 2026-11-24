@task_prose part=chunk
Adds a new viola CLI verb that switches a running viola-wrapped session, a named instance, to a given model and effort.
The verb records the new pair, so a later resume through revive or up replays the new pair and not the one named at launch.
It has no side effect on the user's own settings: it writes nothing to the user's claude home or to a project's settings files, and it never uses the CLI's own model command as its channel.
It adds the verb's arguments, closed words, exit codes, hints, output lines, --json document, the events-log record of the switch and its process-log lines, each placed beside the existing tables.
Nothing runs live: no real session is read, switched, ended or resumed, and no real claude home is read.
@task_file n=1 path=src/cmd/revive.rs
what: the revive verb, which resumes a gone instance in place through the start path; its --model and --effort override the recorded launch pair on the resumed child's command line
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate, which holds the closed effort set check and the compiled default launch pair
@task_item n=1 part=needs
The new verb is a CLI subcommand on a named instance, added beside the CLI verbs table and the command names. It follows the CLI flag and subcommand style, the clap parser, and the rule that new capabilities arrive as subcommands or channel verbs.
@task_item n=2 part=needs
The verb's arguments are the model and the effort. They are checked against the closed model-word format and the closed effort set (low, medium, high, xhigh, max). The compiled default launch pair and the configured default model and effort stay what they are.
@task_item n=3 part=needs
The switch records the new pair as the pair a resume replays: the launch event kind and its data, the wrapper's launch append, and the instance snapshot's launch field. Revive and up then hand the recorded pair on the child's command line, which the launch-pair flags passed to the child carry.
@task_item n=4 part=needs
The events-log record of the switch is a normalised event kind. It is added only in viola-core, follows the event line's shape, source and data, and is appended as one write per line under the versioned, tolerant own-format rules. The snapshot replay still recovers the pair from log-derived fields.
@task_item n=5 part=needs
No side effect on the user's settings: the switch writes no settings layer, passes no --settings at start, leaves the global statusline config untouched and writes no user-scope model key. The CLI's own model command, which writes that key, is not the channel.
@task_item n=6 part=needs
A switch that restarts the session rides the revive start path. That path resumes in place, refuses a session a live claude holds, never starts over a live or stale instance, refuses a held name and resolves the child program itself. It ends the running session and resumes it, which shows in session liveness and in the session-end and session-start records.
@task_item n=7 part=needs
The ways a running session's model or effort can change, and which of them writes where, are read only from the capability ledger's recorded rows. The ledger is the single gate for CLI behaviour, and local commands are classified by it. Anything the ledger does not hold is unmeasured and is not relied on.
@task_item n=8 part=needs
A switch asked for mid-turn, mid-send or while a dialog is held must obey the wheel and the send gate. The human always wins the wheel, a send mid-turn is refused and never queued, and there is at most one pending dialog with a dialog hold per instance.
@task_item n=9 part=needs
If the verb reaches the running wrapper over the channel, it is a frame-writing client. It runs the strict-modes check, the socket-directory check and the server-identity check in their one order before the first frame. It reports unreachable causes through exit 21 and its documents. A new channel method follows the channel methods table and the protocol-version bump rule.
@task_item n=10 part=needs
The verb's exit codes are drawn from the CLI exit codes table and the exit-cause catalogue, and a retired code is never reused. Its refusals come from the closed set of refusal details, only the first refusal is returned, and the closed words use kebab-case wire values.
@task_item n=11 part=needs
The verb's output prints human text by default and a --json document on request, in declared key order with snake_case field names and absent optional fields omitted. Errors use the CLI --json error document and the human error line, both sanitised. Hint lines are short-lived CLI stderr lines.
@task_item n=12 part=needs
The verb's process-log lines are codes-only lines in its role's home-level diagnostics file (the cli role file). They are emitted only through the one event emitter, and error context chains reach only the detail file.
@task_item n=13 part=needs
The board's model and effort fields of a list item show the recorded pair, so after a switch they read the new pair.
@task_item n=14 part=needs
Claude-specific flag literals and pair shapes stay in the claude agent crate, which does no file I/O. The root bin consumes only normalised events.
@task_standing n=1
convention: Every new test runs against the fake agent in its own isolated viola home. No live claude runs in CI, and real-CLI probes run only locally.
home: established-decisions · occupied-resources
@task_standing n=2
convention: Every new error variant joins its crate's one thiserror enum, and its message interpolates nothing. anyhow appears only at the root bin edge.
home: conventions · cross-cutting-patterns
@task_standing n=3
convention: Every new module joins the module tree, with no orphan modules, no print or dbg macros and no tracing level macros. It passes fmt and clippy with warnings denied.
home: build-system · project-directory-structure
@task_standing n=4
convention: Every new test respects the coverage floors, zero panic lines in end-to-end runs, and insta snapshots in check mode only.
home: ci-cd-approach · project-directory-structure
@task_standing n=5
convention: Every new test seam or knob uses the fake-agent test seam variables or the AGENT_RUN_ harness prefix. These are never a configuration channel the product reads.
home: conventions · occupied-resources
@task_item n=1 part=not_needed
The Hyprland window seam, its window features and its seam errors, plus the focus and open verbs: the switch spawns, finds and focuses no window.
@task_item n=2 part=not_needed
The terminal fleet console and its keyboard reach: the console leg of the switch belongs to another chunk.
@task_item n=3 part=not_needed
Dialog answering: the answer verb, the permission, plan and question answer responses, and the hook contract's decision bodies.
@task_item n=4 part=not_needed
Driver-to-driven links: link, unlink, from, lineage labels and the link set.
@task_item n=5 part=not_needed
The machinery of viola verify: its probe folders, trust record, Run E and fixture recording, and the ledger stamps writer. The switch writes no stamps.
@task_item n=6 part=not_needed
Plugin installation and the marketplace tree, and the pinned-copy placement of the binary and plugin.
@task_item n=7 part=not_needed
up's selection and ordering of the instances it re-opens, and adopt's acceptance of an unwrapped session.
@task_item n=8 part=not_needed
Release tooling, crates.io publishing, the deploy stage, and container or hosting absences.
