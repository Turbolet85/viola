@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, as the pty and channel seams are the one place their libraries are used; nothing outside it names a Hyprland request, reply shape or socket.
It carries only the relied-on window queries and dispatches the later window verbs need (open, focus, up, the fleet console), no general Hyprland client, and builds no verb itself.
Each reply is read capped at the shared frame cap and treated as untrusted upstream output, parsed tolerantly into closed viola types; each call ends in a closed outcome code logged codes-only, and outside Hyprland the seam answers a closed unsupported outcome.
A fake compositor replays replies recorded live on the founder's host, scrubbed like every fixture, so the seam's tests run green in CI where no compositor runs; the relied-on requests are measured live once each in named operator steps.
@task_item n=1 part=needs
A new seam: the Hyprland seam is a new place the binary reaches an outside program or socket, and a new seam or a new environment read needs its own Decisions Log entry before it lands.
@task_item n=2 part=needs
A new environment read: the compositor's instance signature (HYPRLAND_INSTANCE_SIGNATURE, and the runtime directory under which the request socket sits) is read by the binary for the first time; it is a lookup, never configuration, and no variable may switch a control off.
@task_item n=3 part=needs
The relied-on request set only: window queries (clients with window address, pid, workspace, position and size; the active window; workspaces and the active workspace; monitors) and dispatches (focus a window, show a workspace, place a window); no general client surface.
@task_item n=4 part=needs
Every compositor reply is read through the one shared external read cap: read at most the cap plus one byte and refused past it; the seam joins the bounded-readers census as a bounded external reader.
@task_item n=5 part=needs
Compositor replies are untrusted upstream output: parsed tolerantly into closed viola types, an unknown or unparseable reply failing to a closed malformed outcome, under the default JSON depth limit, never an unbounded depth.
@task_item n=6 part=needs
Upstream text is content, never a command: window titles, classes and every other reply text are never executed, never matched as instructions, and reach no log line, error message or refusal.
@task_item n=7 part=needs
Logging codes-only: each seam call ends in a closed outcome code (ok, unsupported, unreachable, malformed and the like) emitted through the shared observability event macro; no reply text, window address, pid, path or socket name in any line.
@task_item n=8 part=needs
Error and refusal output of the seam: fixed messages that interpolate nothing, no absolute paths, pids, ids or parser error chains outward; full detail, if any, only where diagnostics allow it.
@task_item n=9 part=needs
Outside Hyprland, with no compositor reachable, the seam answers a closed unsupported outcome: never a panic, never a guess.
@task_item n=10 part=needs
If the transport is the compositor's command-line client resolved on PATH: a direct spawn with no shell and an absolute resolved program path, as run resolves its child; the child's output bounded and parsed tolerantly; the child trusts PATH as other children do.
@task_item n=11 part=needs
If the transport is the compositor's request socket under the per-user runtime directory: the same owner-only directory and owner checks the channel's socket directory has (0700, owner, no chmod by the checker), or a recorded reason it does not; a client connection only, no listener.
@task_item n=12 part=needs
Placement of the seam: either a new workspace crate with its own crate-table row, its sync standing free of tokio, and the deny and ban checks over the root graph, or a module of the root binary; any dependency it adds comes from crates.io, exactly pinned, licence-allowed and building no C.
@task_item n=13 part=needs
A fake compositor for tests: a test-only replayer of recorded replies, reachable only under a test-only feature and compiled out of the release build, so the seam's tests run on ubuntu-latest where no compositor runs.
@task_item n=14 part=needs
Recorded compositor fixtures: recorded live on the founder's host, scrubbed of home paths, usernames, absolute paths and emails (or refused whole), with window titles and other user content scrubbed, and reviewed before commit.
@task_item n=15 part=needs
The measured-live record: a record parallel to the Claude capability ledger, which stays Claude-only; the Hyprland version read with each recording and a fixture directory per version.
@task_item n=16 part=needs
Live-desktop steps on the operator's host: read-only queries once each; a dispatch that focuses, moves or shows only after the compositor's lock state is read, stopping on locked and never unlocking; no typed key and no live claude start.
@task_item n=17 part=needs
Every test or guard control that spawns a compositor client, the fake compositor, viola run, mcp or a wrapper carries a kill deadline; a guard control holds only when its own case ran and failed with the guard removed; no mutation run.
@task_item n=18 part=needs
CI stays on ubuntu-latest with no secrets and no real compositor or real claude CLI; the seam's tests read only the replayed fixtures there.
@task_standing n=1
convention: each new test uses its own temporary home, kept out of git, and removes only the directories it created.
home: secret-management
@task_standing n=2
convention: each new recorded fixture is scrubbed of paths, usernames, emails and user content, or refused whole, and reviewed before commit.
home: data-protection
@task_standing n=3
convention: each new log line carries closed codes only: no user content, upstream text, path, pid or identity value.
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: each new error or refusal message is fixed text that interpolates nothing and names no path.
home: error-handling
@task_standing n=5
convention: each new test-only seam or replayer lives under the test-only feature and is compiled out of release.
home: security-anti-patterns
@task_standing n=6
convention: each new external reader is bounded by the shared frame cap or named in the readers census, and parses tolerantly.
home: input-validation
@task_standing n=7
convention: each new third-party dependency is exactly pinned, from crates.io, licence-allowed, builds no C and passes the deny gate.
home: dependency-security
@task_item n=1 part=not_needed
MCP tool contracts, MCP stdin line rules and MCP structured-content codes: the chunk adds no MCP tool and touches no MCP input.
@task_item n=2 part=not_needed
Wrapper channel methods, channel frame params, channel JSON-RPC error codes and channel refusals: the chunk adds no channel method and sends no frame.
@task_item n=3 part=not_needed
Statusline command, statusline sources, budget file and config file keys: the seam reads none of them.
@task_item n=4 part=not_needed
Hook stdin, hook decision details and hook dialog rows: the chunk touches no hook path.
@task_item n=5 part=not_needed
Paste and send text character sets, bracketed paste, empty-text refusals and the human-typing wheel: the chunk types no text into any session.
@task_item n=6 part=not_needed
revive codes and arguments, link and unlink rows, pause and release-from-driver rows: no verb of these is touched.
@task_item n=7 part=not_needed
verify's refusal codes, verify probe directories, dialog and screen-read rules and real-CLI transcripts: the chunk runs no claude verification.
@task_item n=8 part=not_needed
Pinned binary and plugin folder rows, plugin install rounds and marketplace rules: the seam places and re-hashes nothing.
@task_item n=9 part=not_needed
rmcp feature sets and HTTP or auth bans, self-update signatures and release attestations: the chunk adds no MCP transport and no release stage.
@task_item n=10 part=not_needed
Inherited CLAUDE credential stripping and the claude child's process-start line: those rule over the claude child, not a compositor client.
