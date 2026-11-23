@task_prose part=chunk
Every session viola starts names a model and an effort: `viola run` and the start arm of `viola revive` hand the claude child the pair the operator gave, else viola's configured default pair, on every spawn.
The pair is recorded per instance as a closed set of values, beside the lineage record in the events log and the snapshot, and `viola revive` replays the recorded pair on its resume.
The CLI channel that carries the pair to claude (a flag, an environment name or a settings key), where and in what shape the pair is recorded, and what a resume does with model and effort today are measured at P3 first, never assumed; an undocumented claude behaviour becomes a capability ledger row with a verify probe.
Whether the pair is shown on `viola list`, its --json and the MCP list tool, and whether the default pair lives in config.json, are forks of the operator's; nothing is written to the user's own settings.
@task_item n=1 part=needs
The `viola run` and `viola revive` argument surface gains model and effort inputs: CLI input crosses the trust boundary and is checked as a closed set of values before anything is read, spawned or framed; a bad value is refused whole, never stripped or narrowed.
@task_item n=2 part=needs
The claude child spawn carries the pair on every start: the values go to the child as argv elements with no shell, resolved by the same child resolution that run and revive use today, and the pair is a closed value, never free text from upstream.
@task_item n=3 part=needs
If the CLI channel measured at P3 is an environment name, the child environment rules bind it: parent identity variables and inherited CLAUDE credentials stay stripped, the environment is not a configuration channel for viola itself, a new environment read or seam needs its log entry, and the process-start claude-child line records env names only, never CLAUDE values.
@task_item n=4 part=needs
A default pair in config.json, if the fork lands there: the config is parsed tolerantly, a bad value falls back by the bad-config-values rule rather than reaching the child, and no config value may switch off a control.
@task_item n=5 part=needs
The per-instance record of the pair in the events log and the snapshot: kept to those two stores as the lineage record is, written only by the instance's own wrapper, owner-only modes, plaintext protected by OS permissions, a closed set of values so readers tolerate torn lines and unknown values.
@task_item n=6 part=needs
Revive's replay of the recorded pair: the snapshot is read only after the strict-modes check of the home and the snapshot, the replayed value is re-checked against the closed set and falls back by the replayed-snapshot fallbacks when it does not parse, and a replayed snapshot still runs nothing and frames nothing beyond the spawn.
@task_item n=7 part=needs
The start arm of revive and run keep their existing check order: strict modes and the socket-directory check before any bind, snapshot or child; the new inputs add refusals that come after the instance and home checks, not before them.
@task_item n=8 part=needs
New refusals of a bad model or effort value on run and revive: fixed messages that interpolate nothing, no paths, pids or ids, sanitised for human output, and a closed code on the revive --json error document if one is added.
@task_item n=9 part=needs
If the pair is shown on `viola list`, list --json or the MCP list tool: the board shows a recorded value only when it re-parses to the closed set, human output escapes control characters, and the client screens the server's answer to closed values.
@task_item n=10 part=needs
Nothing is written under the Claude home or into the user's own settings: the pair reaches the child only through the measured CLI channel, keeping v2-31's promise that the user's settings stay byte-identical reachable.
@task_item n=11 part=needs
The capability ledger row for an undocumented claude behaviour, with its verify probe: the probe reads signatures only, compares against compiled literals, answers no dialog and keeps its effects in its own directory; this phase runs no live start, no rig start of a real claude and reads nothing of the operator's real Claude home.
@task_item n=12 part=needs
The P3 measurements run without a real claude in CI: the real claude CLI is local only, CI holds no secrets, and any fixture recorded from the child is scrubbed and reviewed before commit.
@task_item n=13 part=needs
Tests of the new pair handling reach the child through the fake agent: test seams live only under the fake-agent feature and are compiled out of the release build.
@task_standing n=1
convention: each new test runs in its own viola home and never touches the user's real home or Claude home
home: secret-management
@task_standing n=2
convention: each new test seam sits only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: each new recorded fixture is scrubbed of paths, names and emails and reviewed before commit
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: each new log line holds no user content and no CLAUDE values; full detail goes to diagnostics only
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
The wrapper channel methods and their frames (send, answer, wait, last, pause, release, link, unlink, hook.event, hook.dialog): the chunk adds no channel method and changes no frame.
@task_item n=2 part=not_needed
Typed text and dialog answers: the paste text and dialog free-text character sets, bracketed paste delivery and the human-typing wheel; the chunk types nothing into a session.
@task_item n=3 part=not_needed
The hook contract: hook stdin, hook payload parsing, hook decision details and the hook's silent exit; the chunk adds no hook and changes no hook event.
@task_item n=4 part=not_needed
Peer authentication on the local socket: peer euid checks on client and server, server verification before a frame, and socket-file modes; the chunk opens no new listener and connects to no peer.
@task_item n=5 part=not_needed
The pinned viola binary and plugin folders: their placement, re-hash and rewrite at start; the chunk changes neither.
@task_item n=6 part=not_needed
The live rig arms and the live plugin-install rounds: this phase runs no live start and no rig start of a real claude.
@task_item n=7 part=not_needed
The release, signing and attestation stage and the scheduled fuzzing workspace: the chunk adds no release step and no parser to fuzz scope.
@task_item n=8 part=not_needed
The Hyprland recorder and terminal colour facts: the chunk reads no window manager state and changes no colour choice.
@task_item n=9 part=not_needed
Switching a running session's model and effort: that is a later entry, not built here; nor the inheritance of the pair by open, up and adopt, verbs that do not yet exist.
