@task_prose part=chunk
Grows the closed cause set of the turn-ended event from the one word api-error to nine closed words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.
The Claude adapter keeps classifying a StopFailure's error value as it already does; only the resulting word now crosses into the turn-ended event, the event log, viola wait --json and the MCP wait tool.
The CLI's error text still crosses nothing: an unknown, absent or unmapped error value reads api-error, and every consumer that accepts api-error today accepts the nine and refuses any other text.
P3 re-measures where the word is produced, every consumer that matches on it, and each known CLI error value as measured from a recorded payload or read from the installed package's source.
@task_file n=1 path=src/cmd/run.rs mark=edits
what: the wrapper's run path, holding its check of a line appended to the event log before it is admitted — edits: the appended-line check that admits a turn-ended only with cause absent or a closed cause word
@task_item n=1 part=needs
The turn-ended event's cause field is a closed value set owned by the core crate; it grows from one word to nine, each a closed enum variant, and no new event kind is minted.
@task_item n=2 part=needs
Protocol and event fields stay closed types: the cause is parsed into the closed enum on every side and any word outside the nine is refused, never passed through as free text.
@task_item n=3 part=needs
API-error classes are kept inside the Claude adapter: the CLI's error matcher values are mapped there, no new Claude shape appears outside the adapter, and only the closed word leaves it.
@task_item n=4 part=needs
An API-error turn end carries no error text: last_assistant_message, error_details and the raw error value reach no event, snapshot, log line, reply or diagnostics line other than as one of the nine words.
@task_item n=5 part=needs
The StopFailure hook payload is untrusted content parsed tolerantly: an absent error, the CLI's unknown, or any value outside the map fails to the word api-error, and the hook still fails open and exits silently.
@task_item n=6 part=needs
Hook security fields are held in closed enums: the error value is classified at the parse and the upstream string is dropped there, never carried further in the hook.event frame.
@task_item n=7 part=needs
The wrapper re-validates every input: its check of an appended turn-ended line admits cause absent or one of the nine words and refuses any other text.
@task_item n=8 part=needs
Readers of the event log stay bounded and tolerant of torn lines while accepting the nine words and refusing any other cause text.
@task_item n=9 part=needs
The client screens the server's answer to closed values: viola wait under --json, the board and viola list read the cause only as one of the nine words.
@task_item n=10 part=needs
The MCP wait tool's structured content carries the cause as one closed word, and the MCP tool contract for wait lists the nine.
@task_item n=11 part=needs
Upstream text is content, never commands, and never logged: CLAUDE values and user content reach diagnostics only where already allowed, and the cause word carries no upstream text beside it.
@task_item n=12 part=needs
How a new, unmapped CLI error value is noticed later must be a count or a closed log word, never its text — or the plan states plainly that it is not noticed.
@task_item n=13 part=needs
Error messages and any new log or diagnostics line about the cause interpolate nothing upstream: fixed words only, no paths, no error strings.
@task_item n=14 part=needs
Proof rests on the fake agent and recorded payloads only: the real claude CLI is used only locally, CI holds no secrets and no real claude, and no live claude start is planned as proof.
@task_item n=15 part=needs
Any fake-agent seam the tests use to emit a StopFailure with a chosen error value lives only under the fake-agent test feature and is compiled out of the release build.
@task_item n=16 part=needs
Any recorded StopFailure payload used as a fixture is scrubbed of paths, usernames and emails and reviewed before commit, or refused whole.
@task_standing n=1
convention: each new recorded payload or fixture is scrubbed or refused whole and reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test runs under its own viola home, never the user's
home: secret-management
@task_standing n=3
convention: each new test seam lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · security-anti-patterns-never-do-these
@task_standing n=4
convention: each new log or diagnostics line honours the never-log floor and carries no upstream text
home: bootstrap-phases-derive-for-route-setup-project · bootstrap-phases
@task_item n=1 part=not_needed
Peer-euid checks, socket-directory checks and server verification before a frame: the chunk changes no channel access or seam check order.
@task_item n=2 part=not_needed
Send and answer text rules — paste character sets, control-character refusal, empty-text refusal, bracketed paste: the chunk types nothing into the agent.
@task_item n=3 part=not_needed
viola revive arguments, its --json codes and replayed snapshot handling: the chunk adds no revive behaviour.
@task_item n=4 part=not_needed
link and unlink methods, their event lines and exit codes: the chunk does not touch linking.
@task_item n=5 part=not_needed
Pinned binary placement, re-hashing, plugin files and the marketplace tree: the chunk changes no install or pinning.
@task_item n=6 part=not_needed
viola verify's probes, refusal codes, stamps and screen-signature reads: the chunk adds no verify run.
@task_item n=7 part=not_needed
CLAUDE credential stripping from the child environment and the identity floor: the chunk changes no child spawn.
@task_item n=8 part=not_needed
Instance-unreachable detail causes and strict-modes refusal contracts per verb: those closed sets are separate from the turn-ended cause and are not changed.
@task_item n=9 part=not_needed
Hyprland recorder allowlist and reply-text handling: the chunk reads nothing from the window manager.
@task_item n=10 part=not_needed
The human-typing wheel event and its cause human-input: a different event's cause, not changed here.
