@task_prose part=chunk
Adds a new CLI verb `viola adopt <session-id> --name <n> [--parent <p>]`: a plain claude session, named by a session-shaped id, is resumed under a viola wrapper with an operator-given name and optional parent, then driven like any wrapped session.
It is built on revive's start path: the resume argv crossing, the `claude agents --json` live-elsewhere preflight and its closed session-live refusal, and the start shared with run.
Conservative arm, held provisional: adopt never ends or signals a process it did not start; it resumes only a session live nowhere and refuses a live one; it never takes over a running process's terminal.
Folded CI item: re-measures the two env-capture security-negative cases against their kill line, and gives them a margin by their own class or reduced work. The kill line is never raised for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the revive verb's start path, which adopt is built on: the resume argv through the Claude adapter's resume args, the `claude agents --json` preflight with its closed session-live refusal, and the start shared with run
@task_file n=2 path=.config/nextest.toml mark=edits
what: the test runner's configuration, holding each test's per-case kill line and test classes — edits: the class or margin for the two env-capture security-negative cases
@task_item n=1 part=needs
The new CLI verb adopt and its input: a session id, a --name and an optional --parent, each taken across the CLI input trust boundary and checked before any frame, spawn or state read. Names go through the viola name check and the validated name type, and instance paths are built only from validated names.
@task_item n=2 part=needs
The session id's closed shape: it is checked by the existing closed session-id check before any home read beyond the strict ones. A malformed id is refused with a closed word, and a valid id crosses to the claude child only as one argv element, with no shell.
@task_item n=3 part=needs
Check order at start: the home tree's strict-modes check runs before any trusted read, log open, snapshot read or child spawn, and the socket-directory check runs before any bind. A strict-modes refusal leaves no trace, and an unstattable home is refused.
@task_item n=4 part=needs
The live-elsewhere refusal: a session that `claude agents --json` lists as live is refused with the closed session-live word before any child is spawned. The agents read is bounded and resolved from the caller's PATH, and names in its rows are never treated as targets.
@task_item n=5 part=needs
No reach into foreign processes: adopt sends no signal to a process viola did not start, ends none, and never takes over a running process's terminal or drives a foreign peer. Any wider arm counts as a boundary widening, and that decision belongs to the operator.
@task_item n=6 part=needs
The claude child start that adopt reuses: the --resume argv crossing, the launch pair handed on the child's argv, parent identity variables and inherited CLAUDE credentials stripped from the child environment, the pinned binary re-hashed before use, and the pinned folder first on the child's PATH.
@task_item n=7 part=needs
Name and parent as lineage: the operator-given parent is self-reported and authenticates nothing. Lineage and the first snapshot are kept to the snapshot and the event log, only the instance's own wrapper writes its snapshot, and the board shows lineage only when it re-parses.
@task_item n=8 part=needs
Collision outcomes: a second adopt of the same id and an adopt onto a name already held each end in a defined closed outcome. These are the start collision check's already-live, stale-heartbeat and squatted-name words, under the same --json codes revive carries.
@task_item n=9 part=needs
The cwd, model and effort of an adopted session where no snapshot recorded them: a recorded cwd is used only to spawn and is kept to the snapshot, and launch-pair flags and effort words are checked before use. Any value read from another process is a crossing, so it counts as content and is checked before it is trusted.
@task_item n=10 part=needs
adopt's refusals and errors: closed code words only, with no paths, pids or session ids in refusal or error output. Error messages interpolate nothing, human-mode output escapes control characters, and --json error documents carry closed codes and the CLI exit codes from the exit-cause table.
@task_item n=11 part=needs
viola writes nothing under the Claude home or in Claude's config while adopting, and reads no transcript content of the session it resumes.
@task_item n=12 part=needs
Every live reading of adopt is gated on the operator's grant: CI and tests use only the fake agent and never the real claude CLI, and no real session is read, listed, ended, adopted or resumed.
@task_item n=13 part=needs
The folded CI item: the env-capture security-negative cases, where an env name with the user word reads fail and records nothing, and a kept name reaches no stamp or record, keep their assertions whole. Only their wait class or work changes, and their measured time per case is read against their kill line.
@task_item n=14 part=needs
Env capture stays names-only: the recorded env fixture and the verify stamp's env probe hold names and counts only, and no CLAUDE value is logged. Any change to these cases' cost keeps that floor.
@task_item n=15 part=needs
Logging of the new verb: every new environment read or seam gets a log entry, and the process-start claude-child line carries env_kept and env_stripped_known names only. User content goes only to diagnostics, which are owner-only.
@task_standing n=1
convention: each new test runs in its own home under the fake agent and holds no secrets. Test-side removal touches only the dirs the test itself created, and test homes stay out of git.
home: secret-management
@task_standing n=2
convention: each new test seam is compiled only under the fake-agent feature and is compiled out of release builds
home: security-anti-patterns
@task_standing n=3
convention: each new seam or environment read adds a log entry, and no log line carries credentials or CLAUDE values
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: each new error writer is named, and its display message is fixed, with no upstream text, path or type name
home: error-handling
@task_standing n=5
convention: each new recorded fixture is scrubbed or refused whole and reviewed before commit, under the hygiene walk
home: data-protection
@task_item n=1 part=not_needed
The wrapper channel's frame methods and their contracts (send, answer, wait, last, pause, release, link, unlink): adopt adds no channel method and changes no frame.
@task_item n=2 part=not_needed
Paste text and dialog free-text character sets, send refusals and the human-typing wheel: adopt types nothing into a session.
@task_item n=3 part=not_needed
The hook contract (hook stdin, hook decisions, hook fail-open): the chunk changes no hook.
@task_item n=4 part=not_needed
Live rig arms and their kept-output rules, and verify's real-CLI probes and their residuals: this phase runs nothing live and adds no live probe arm.
@task_item n=5 part=not_needed
Release, signing, attestations and self-update rows: the chunk adds no release stage.
@task_item n=6 part=not_needed
Network-facing absences (hosting, public listener, webhook, websocket, database, file upload, remote access): adopt is a local CLI verb.
@task_item n=7 part=not_needed
The fleet console, switching a session's model and effort, and viola up's reboot reading: these are explicitly out of scope.
@task_item n=8 part=not_needed
link and unlink verbs and their link lines in the event log: adopt records lineage only, not links.
@task_item n=9 part=not_needed
Event log growth and rotation rows: the chunk adds no rotation and changes no log retention.
