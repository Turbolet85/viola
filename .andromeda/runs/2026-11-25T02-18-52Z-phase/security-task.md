@task_prose part=chunk
Changes the wrapper's window step so that a placed window reaches its workspace while the compositor's active workspace stays where it was (the operator's «no focus change»), using a move form taken from the Hyprland seam's closed request set, never a second compositor seam.
Every caller of the move (viola open, viola up, viola revive under --origin) takes the same answer, or a named difference; the seam's tap, fake and recorded move fixture follow the form, the fixture re-recorded only through the operator's one live grant.
Adds an acceptance clause a gate entry reads: the active workspace read before a placing open equals the one read after; v2-07 is claimed and proved again, its live half gated on the operator.
Folded item: says where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused), with a fix form chosen at a fork, its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step that moves only its own terminal's window to its workspace through the Hyprland seam — edits: the move request it sends (Request::MoveToWorkspace)
@task_file n=2 path=crates/viola-hyprland mark=edits
what: the Hyprland seam crate: its closed request set, its hyprctl child, its reply parse, its test-only tap (viola-hypr-tap) and its fake — edits: the move request form, the tap and the fake's reply set
@task_file n=3 path=fixtures/hyprland/0.56.2 mark=edits
what: the recorded Hyprland reply fixtures for the compositor version, scrubbed and allowlisted — edits: the move fixture, re-recorded only under the operator's grant
@task_file n=4 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new CLI that a start runs, and the stamp it writes — edits: where it runs and what it stamps when its own preconditions fail
@task_file n=5 path=src/cmd/run.rs mark=edits
what: viola run's start: its version gate, its trust record and its call of the automatic verify — edits: the start's call of the automatic verify
@task_item n=1 part=needs
The window move's behaviour changes: a placed window lands on its workspace and the compositor's active workspace is the same after the move as before; the move remains a request of the seam's closed request set and a single dispatch, never a new or second compositor seam.
@task_item n=2 part=needs
The window step still moves only its own terminal's window: the new move form must target the address of the window the wrapper itself opened, never a window chosen by another reading.
@task_item n=3 part=needs
The hyprctl child that carries the new move (and the active-workspace reads) stays bounded by its deadline, is spawned directly with closed argv, and its reply text is dropped at the parse; nothing from the compositor's answer reaches output beyond closed values.
@task_item n=4 part=needs
The callers of the move (viola open, viola up, viola revive under --origin) each act on the new form; a recorded window origin authenticates nothing and is used only as the window step already uses it, and the window record stays kept to the snapshot and the event log.
@task_item n=5 part=needs
The seam's fake replies only with what the real compositor answers to the new form; any fake or test seam stays test-only and compiled out of release.
@task_item n=6 part=needs
The move fixture's re-record is one operator-granted, operator-fired live leg (one terminal window, no claude, the host unlocked) with a rehearsal form: the recorder moves only its own terminal's window, keeps strings by its closed allowlist, and the recording is scrubbed or refused whole and reviewed before commit.
@task_item n=7 part=needs
The new acceptance clause reads the active workspace before and after a placing open as a gate-entry predicate through read-only compositor queries; no check in CI dispatches to a real compositor or runs a real claude.
@task_item n=8 part=needs
The phase measures the move form only from the compositor's own help, version and state through named read-only queries: no live start, no window opened, moved, focused or closed, no key sent, nothing read of the operator's real Claude home or viola home.
@task_item n=9 part=needs
The automatic verify's place: a real-CLI verify needs a prepared, trusted folder and never answers an import, trust or other dialog; it must not run where the start happened in a folder the CLI does not trust and stamp what that folder made it read.
@task_item n=10 part=needs
The automatic verify's stamp on its own failure: what is stamped when its own preconditions fail (an untrusted folder, a modal, a red it caused itself) changes so a version is not kept unverified by a failure verify caused; only viola verify writes the stamps file, stamps are checked before and under lock, in closed words.
@task_item n=11 part=needs
The unverified-CLI gate and viola run's version gate still read the stamps through the strict-modes check first; the fix must not let a start pass the gate on a stamp it did not earn nor switch the gate off by config, env or flag.
@task_item n=12 part=needs
A start's trust handling stays as it is bound: it skips the home folder and trusted folders, writes the trust record by flag only through the guarded writer, and writes nothing under the Claude home; any folder the automatic verify uses for its probes stays its own, owner-only, and is removed by its drop guards.
@task_item n=13 part=needs
Any new refusal or failure line the folded item adds uses a fixed message and a closed code word, carries no path, pid, id or upstream text, and keeps full detail to diagnostics only.
@task_item n=14 part=needs
The folded item's witness cannot pass vacuously: it runs against the fake agent in its own test home, never a real claude, and fails if the verify runs in the untrusted start folder or keeps a self-caused failing stamp.
@task_standing n=1
convention: each new test runs under its own viola home, never the operator's, and removes only the directories it created
home: secret-management
@task_standing n=2
convention: each new or re-recorded fixture is scrubbed of home paths, usernames, emails and absolute paths, kept by its closed allowlist and reviewed before commit
home: data-protection
@task_standing n=3
convention: each new test seam or fake reply lives only under the test-only feature and is compiled out of the release build
home: security-anti-patterns
@task_standing n=4
convention: a new seam request or environment read is entered in the log the plan keeps for them
home: security-anti-patterns-never-do-these
@task_standing n=5
convention: each CI job holds no secrets and runs no real claude CLI; real-CLI and live compositor legs stay local and operator-fired
home: threat-model-summary
@task_item n=1 part=not_needed
The IPC channel's socket, peer euid checks, server verification and frame verbs: the chunk adds no channel method and sends no frame.
@task_item n=2 part=not_needed
The MCP server's tools, stdin cap, structuredContent and error codes: the chunk touches no MCP surface.
@task_item n=3 part=not_needed
Paste and send text validation, the bracketed paste, typed-text character sets and the send or answer refusal words: the chunk types nothing into a pty.
@task_item n=4 part=not_needed
The hook contract, hook stdin, hook decision details and hook fail-open rules: the chunk changes no hook.
@task_item n=5 part=not_needed
link and unlink, their event lines and exit codes: the chunk does not reach links.
@task_item n=6 part=not_needed
The board (viola list) rendering of lineage, launch pair, window and origin, and its colour facts: the chunk changes no board output.
@task_item n=7 part=not_needed
Credential stripping and the inherited CLAUDE environment of the claude child: the chunk changes no child environment.
@task_item n=8 part=not_needed
Release, signing, self_update, attestations and the advisory and licence lists: the chunk builds no release and changes no dependency policy.
@task_item n=9 part=not_needed
The pause, release and switch methods and driver-originated release refusals: the chunk changes no wheel or switch behaviour.
@task_item n=10 part=not_needed
Event log rotation and growth, and the snapshot writer's own rules beyond the window record: the chunk changes no log rotation.
