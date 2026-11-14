@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, as the pty and channel seams are the one place their crates are used; nothing outside it names a Hyprland request, reply shape or socket.
It carries only the relied-on window queries and dispatches the later window entries need (open, focus, up, the fleet console), no general Hyprland client, and builds no verb.
Each reply is read capped and parsed tolerantly as untrusted upstream output into closed viola types; each call ends in a closed outcome code logged codes-only; outside Hyprland the seam answers a closed unsupported outcome.
A fake compositor replays replies recorded live on the founder's host (scrubbed like every fixture) so the seam's tests run green on ubuntu-latest, where no compositor runs.
The reply shapes are measured live in named operator steps: read-only queries, then relied-on dispatches with the compositor's locked state read first.
@task_item n=1 part=needs
The window features target Hyprland alone on Linux alone: the seam is the single boundary to the compositor, and no request name, reply shape or socket appears outside it.
@task_item n=2 part=needs
Only the relied-on requests are built: the window queries (clients with window address, pid, workspace, position and size; the active window; workspaces and the active workspace; monitors) and the dispatches (focus a window, show a workspace, place a window), closed against the later window entries' acceptance; no probe or request for a shape nothing relies on.
@task_item n=3 part=needs
Every compositor reply is read through a bounded reader capped at the channel's MAX_FRAME (take MAX_FRAME + 1) and refused whole past it.
@task_item n=4 part=needs
Compositor replies are external payloads parsed tolerantly into closed viola types: missing and null read the same, unparseable values become unknown, never a panic.
@task_item n=5 part=needs
Window titles, classes and every other reply text are upstream content, never a command, and reach no log line, error message or refusal: error surfaces stay sanitised and error messages interpolate nothing.
@task_item n=6 part=needs
Each seam call ends in a closed outcome code (ok, unsupported, unreachable, malformed and the like) emitted through obs_event! as a codes-only process log line, carrying no reply text, window address, pid, path or socket name.
@task_item n=7 part=needs
With no compositor reachable the seam answers a closed unsupported outcome, never a panic and never a guess.
@task_item n=8 part=needs
The seam's own error type: a hand-written closed error enum for its crate or module, with anyhow only at the binary edge.
@task_item n=9 part=needs
The seam's placement, a new flat workspace crate or a module of the root bin: a new crate takes its row in the crate table, a viola- name, its dependency direction, the sync, Tokio-free standing and the dependency-ban checks; a module joins the root bin's module tree.
@task_item n=10 part=needs
The transport is the hyprctl program with JSON output or Hyprland's request socket under the per-user runtime directory; either needs the instance signature, a new environment read the binary does not make today, which needs its own Decisions Log entry and stays outside the VIOLA_ namespace and outside configuration.
@task_item n=11 part=needs
A program resolved on PATH would be spawned directly, with no shell and an absolute path, resolved the way viola run resolves its child; no shelling out.
@task_item n=12 part=needs
A compositor socket would need the same owner-only 0700 and owner checks the channel's socket directory has, or a recorded reason it does not; the seam binds and listens on nothing.
@task_item n=13 part=needs
The fake compositor is a test-only replayer of recorded replies, kept out of product and release builds and never read by the product as configuration; the seam's tests run in CI on ubuntu-latest against it, never against a live compositor.
@task_item n=14 part=needs
The replayed replies are recorded fixtures: recorded live on the founder's host, scrubbed of paths, user names, home paths and emails, and hygiene-walked like every recorded fixture.
@task_item n=15 part=needs
Measured live rides a record parallel to the Claude capability ledger, which stays Claude-only in the Claude agent crate: the Hyprland version read with each recording and a fixture set per version.
@task_item n=16 part=needs
Live-desktop steps are named operator steps only: read-only compositor queries recorded once each, then each relied-on dispatch once with the locked state read first, stopping on locked and never unlocking; no typed key and no live claude start.
@task_item n=17 part=needs
Every test or control that spawns viola run, mcp, a wrapper or a compositor client carries a kill deadline, and the seam's waits stay bounded.
@task_item n=18 part=needs
No mutation run, local or dispatched; a guard test's one-off control goes through the gate's control step under a kill deadline and holds only when its own case ran and failed with the guard removed.
@task_standing n=1
convention: each new test runs in its own isolated viola home
home: established-decisions
@task_standing n=2
convention: each new test's homes are removed by owner record only and its evidence survives a killed test
home: occupied-resources
@task_standing n=3
convention: each new OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: each new module is reached from the crate's module tree, and the new code passes formatting, clippy with warnings denied and the print, dbg and tracing level-macro bans
home: build-system
@task_standing n=5
convention: each new test counts toward the coverage floors the harness gate enforces
home: ci-cd-approach
@task_standing n=6
convention: each new test run leaves zero panic lines, and insta snapshots stay in check mode only
home: project-directory-structure
@task_item n=1 part=not_needed
The wheel, send, bracketed paste, typed-text normalisation and the screen readiness gate: the chunk types nothing into any session.
@task_item n=2 part=not_needed
Hook contracts, hook tiers, the hook-to-kind map and the dialog hold: the chunk adds no hook and answers no dialog.
@task_item n=3 part=not_needed
MCP tools, their results and the stdio MCP server: the chunk adds no MCP tool.
@task_item n=4 part=not_needed
CLI verbs, their arguments, exit codes and --json documents: open, focus, up, adopt and the console are their own entries.
@task_item n=5 part=not_needed
The wrapper channel's methods, params, fault codes and responses: the seam talks to the compositor, not to a wrapper.
@task_item n=6 part=not_needed
Normalised event kinds, event data and the events log: the chunk appends no event to any instance log.
@task_item n=7 part=not_needed
The budget governor, budget.json and budget readings: untouched by the seam.
@task_item n=8 part=not_needed
Pinned copies, the plugin and its install, the statusline wrapper and its settings override: untouched.
@task_item n=9 part=not_needed
Revive, the board and list payloads, session liveness and instance snapshots: no instance state is read or written.
@task_item n=10 part=not_needed
Release tooling, the release workflow and crates.io publishing: nothing is released by this chunk.
@task_item n=11 part=not_needed
config.json keys and the configuration precedence: the chunk adds no configuration key.
