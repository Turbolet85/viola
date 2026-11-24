@task_prose part=chunk
Builds a new viola CLI verb that switches a running viola-wrapped session, named by its instance name, to a given model and effort word.
It records the new launch pair so a later resume (revive, up) replays the new pair, not the one named at launch.
It has no side effect on the user's settings: it writes nothing under the Claude home or a project's Claude settings, and it does not use the CLI's own model command as its channel.
It adds the verb's argument surface, closed words, exit codes, hints, output lines, --json document, an events-log record of the switch and its process-log lines beside the existing tables.
At P3 it re-reads how revive's pair override, the recorded launch pair, the closed model and effort sets, the wheel and the send gate stand. Nothing live runs in this phase.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive verb, which resumes a recorded session and takes --model / --effort to override the recorded launch pair on the resumed claude child's argv (given_pair)
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the Claude adapter crate, home of the effort-word check (is_effort_level, closed set low · medium · high · xhigh · max) and the compiled default launch pair DEFAULT_PAIR
@task_item n=1 part=needs
The new verb's input: the instance name goes through the viola name check and validated name type, and the model and effort words are checked against their closed sets (core model-word check, the adapter's effort-word check) before any state is read, any frame is sent or any child is spawned; a bad word is refused whole.
@task_item n=2 part=needs
No write under the Claude home or into a project's Claude settings: the switch must leave the user's settings byte-identical, so its channel cannot be the CLI's model command or any settings write.
@task_item n=3 part=needs
The recorded launch pair: the new pair is recorded where a resume replays it (the snapshot's launch record and the event log's launch lines), kept to the snapshot and the event log only, handed to the claude child as argv elements, authenticating nothing, and shown on the board only when it re-parses.
@task_item n=4 part=needs
State writes the switch makes: only the instance's own wrapper writes its snapshot; event-log appends heal a torn last line and readers tolerate torn lines; files are replaced through the one shared replace helper with owner-only modes set before any byte.
@task_item n=5 part=needs
The strict-modes check on the viola home and state tree comes before the verb reads the snapshot or any trusted state, before any log opens, and a strict-modes refusal leaves no trace; the new verb needs its own strict-modes refusal output beside the other verbs'.
@task_item n=6 part=needs
If the switch reaches the wrapper over the local channel: the client's check order (instance check first, socket-directory check, server verified before any frame, peer euid check), closed channel types, the frame cap, and a new channel method needing its process-log entry and its params, errors and channel-response detail rows.
@task_item n=7 part=needs
If the switch is typed into the live session: the send gate's rules — text typed as one bracketed paste, control-character and empty-text refusals, the wrapper re-checking text authoritatively — and human typing taking the wheel; what a switch mid-turn or under a held dialog does is measured at P3.
@task_item n=8 part=needs
If the switch restarts the session through the revive path: revive's checks before use (strict modes, socket-directory refusal, start collision check already-live / stale-heartbeat, pinned binary re-hash), the recorded cwd used only to spawn, and the child's environment with parent identity variables and inherited CLAUDE credentials stripped.
@task_item n=9 part=needs
Targets: the verb acts only on a viola-wrapped instance it can reach; an unwrapped session or agent name is never a target, and the verb acts on no process viola did not start.
@task_item n=10 part=needs
The verb's exit codes and refusal words beside the CLI exit-code table and the instance-unreachable details and causes (exit 21, its --json document and stderr hints), the --json error documents including internal-error, with no paths, pids or ids in any refusal.
@task_item n=11 part=needs
The verb's human and --json output: fixed error display messages that interpolate nothing and quote no upstream text, C0/C1 controls escaped in human output, full error detail only in diagnostics.
@task_item n=12 part=needs
The events-log record of the switch: its kind and data beside the existing event kinds, holding the closed model and effort words and no user content, CLAUDE value or path.
@task_item n=13 part=needs
The verb's process-log lines: closed details only, never logging CLAUDE values or user content, stderr honouring the never-log floor.
@task_item n=14 part=needs
No new flag, config value or environment variable the verb brings may switch off a control; configuration and environment are not a channel for the pair.
@task_item n=15 part=needs
If a fork settles an MCP tool for the switch: the MCP tool list and tool-input contracts, schema-checked raw arguments, the frame-tool refusals and isError structuredContent codes.
@task_item n=16 part=needs
The proof of no settings side effect: tests prove the user's settings byte-identical against a test-owned home and the fake agent, never the real Claude home, real claude CLI or a real session; the real CLI run on the pair is carried elsewhere.
@task_standing n=1
convention: each new test uses its own viola home and its own Claude home, never the user's, and CI runs no real claude CLI and holds no secrets
home: secret-management · threat-model-summary
@task_standing n=2
convention: each new environment read, seam, listener or channel method gets its process-log entry
home: security-anti-patterns-never-do-these
@task_standing n=3
convention: each new error or refusal output uses a fixed message and closed code words, with no paths, type names or upstream text
home: error-handling
@task_standing n=4
convention: each new recorded fixture is scrubbed, reviewed and under the hygiene walk, or refused whole
home: data-protection
@task_standing n=5
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_standing n=6
convention: each new validated input goes through the shared validators in the core crate
home: input-validation
@task_item n=1 part=not_needed
Window, focus and terminal-opening rows (Hyprland recorder, focus ancestry, viola open's argument allowlist): the switch raises, moves or opens no window, and the console's keyboard reach of it belongs to another chunk.
@task_item n=2 part=not_needed
Live rig arms, verify probe residuals and real-CLI fixture recording: nothing live runs in this phase and no real session, real Claude home or real board is read.
@task_item n=3 part=not_needed
Link, unlink and lineage rows, and pause and release with the driver-release guard: the switch neither links sessions nor moves the driver's hold.
@task_item n=4 part=not_needed
Hook contract rows (hook stdin, hook-decision details, hook fail-open): the switch is a CLI verb, not a hook the Claude CLI calls.
@task_item n=5 part=not_needed
Plugin install and marketplace tree rows: the switch installs no plugin and writes no marketplace file.
@task_item n=6 part=not_needed
Release, signing, deploy, attestation and self-update rows, and scheduled fuzzing or mutation testing at the epoch audit: the chunk adds no release stage and no audit run.
@task_item n=7 part=not_needed
Web-stack absences (hosting, public listener, database, file upload, webhook, websocket, user accounts): the chunk adds none of them.
