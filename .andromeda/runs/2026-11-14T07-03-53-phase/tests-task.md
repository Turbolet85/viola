@task_prose part=chunk
Builds viola's own Hyprland seam: the one place the binary talks to the compositor, carrying only the relied-on window queries and dispatches the later window verbs need, no general Hyprland client.
Each compositor reply is read capped at the frame cap and refused past it, parsed tolerantly into closed viola types as untrusted upstream output, and every seam call ends in a closed outcome code logged codes-only.
Outside Hyprland, with no compositor reachable, the seam answers a closed unsupported outcome, never a panic and never a guess.
A fake compositor replays replies recorded live on the founder's host and scrubbed like every fixture, so the seam's tests run green on the one ubuntu-latest CI leg where no compositor runs.
No verb is built and no capability is claimed: open, focus, up, adopt and the fleet console stay their own entries.
@task_item n=1 part=needs
The new Hyprland seam as a product seam: the single component that names a Hyprland request, reply shape or socket, as the pty and channel seams are the single users of their crates; nothing outside it reaches the compositor, and tests reach it only through that seam.
@task_item n=2 part=needs
The relied-on request set only: window queries for clients (window address, pid, workspace, position and size), the active window, workspaces and the active workspace, and monitors; dispatches that focus a window, show a workspace and place a window.
@task_item n=3 part=needs
The bounded reader of each compositor reply: read through take(MAX_FRAME + 1) and refused past the frame cap, the same cap every other external reader keeps.
@task_item n=4 part=needs
The tolerant reply parser: reply text is untrusted upstream output parsed into closed viola types, missing and unknown fields tolerated, never a panic on malformed input; as a new parser surface it owes property and fuzz-style coverage of its invariants.
@task_item n=5 part=needs
Reply content never escapes: window titles, classes and every other reply text are content, never a command, and reach no log line, error message, error body or refusal; error types interpolate nothing from the reply.
@task_item n=6 part=needs
Codes-only logging of the seam: each seam call ends in a closed outcome code (ok, unsupported, unreachable, malformed and the like) emitted through obs_event! as a structured JSON process log line, carrying no reply text, window address, pid, path or socket name.
@task_item n=7 part=needs
The closed unsupported outcome outside Hyprland: when no compositor is reachable the seam returns unsupported, never panics, never guesses; the process keeps the zero-panic rule.
@task_item n=8 part=needs
The fake compositor: a test double replaying recorded replies so the seam's tests run on ubuntu-latest without a compositor; it is a real stub process or listener standing in for an external process, held byte for byte to the recordings it replays, never to hand-written replies.
@task_item n=9 part=needs
The recorded compositor fixtures: replies recorded live on the founder's host, scrubbed of home paths, usernames and real data like every fixture, walked by the fixture hygiene check, and kept per recorded Hyprland version, the version read with each recording, in a record parallel to the Claude capability ledger.
@task_item n=10 part=needs
Live measurement is no test layer: the live compositor reads (read-only queries, focus and workspace dispatches with the locked state read first and a stop on locked, never an unlock) are named operator steps outside CI; CI never touches a real compositor, as it never touches the real agent CLI.
@task_item n=11 part=needs
The transport's spawn and socket rules: a compositor client resolved on PATH is spawned directly, with no shell and an absolute path, as viola run resolves its child; a request socket under the runtime directory would owe the same owner-only 0700 and owner checks as the channel's socket directory, refused and never repaired, or a recorded reason it does not.
@task_item n=12 part=needs
The new environment read of the Hyprland instance signature: a new environment read needs its own Decisions Log entry; tests feed it per child process and never mutate the test process's own environment.
@task_item n=13 part=needs
The seam's placement as a new sync workspace member or a module of the root binary: either way it builds without tokio, is wired into the module tree with no orphan module, and, as a new crate, takes its row in the crate table and its deny bans with their probes.
@task_item n=14 part=needs
Every test or control that spawns viola run, mcp, a wrapper or a compositor client carries a kill deadline and is held by a drop guard; waits key on logged events and outcomes, never on sleeps or retries.
@task_item n=15 part=needs
A guard's proof without a mutation run: no mutation run, local or dispatched, in this chunk; a guard test's one-off control is a bounded remove-the-guard witness under a kill deadline, holding only when the control's own case ran and failed with the guard removed.
@task_item n=16 part=needs
Linux alone: the window features are Hyprland only, tests run on Linux, and CI stays the single ubuntu-latest leg.
@task_standing n=1
convention: Each new test function is named by the test-naming rule, written as an inline unit test or a table-driven rstest case where it fits, and run by nextest.
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: Each new test creates its own private home and directories through tempfile and shares no mutable state with tests running in parallel.
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: Each new fixture is synthetic or scrubbed, recorded rather than hand-written, and passes the fixture hygiene walk, its hits reported by class only.
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: Each new log line is one structured JSON line with the process log line fields, codes-only, with any detail kept apart.
home: log-format
@task_standing n=5
convention: Each new test avoids the anti-patterns: no sleeps, no retries, no Debug-string comparison, literal oracles, no real network, no mocking of what viola does not own, no ignore parking.
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: Each new module and test counts toward the workspace coverage floor with no new exclusion, and any test failure fails the build.
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_item n=1 part=not_needed
MCP tool rows (tools/list, tool results, invalid-params fields, frame tools): the chunk adds no MCP tool and changes no MCP reply.
@task_item n=2 part=not_needed
CLI verb output rows (exit codes, --json documents, human lines, help table) for send, wait, last, answer, release, list and revive: no verb is built or changed.
@task_item n=3 part=not_needed
Wrapper channel JSON-RPC rows (error codes, link and unlink methods, wait method, channel-response details): the seam talks to the compositor, not to the wrapper channel.
@task_item n=4 part=not_needed
Hook rows (dialog decision bodies, statusline passthrough, hook deadlines and fail-open): the chunk touches no hook entry point.
@task_item n=5 part=not_needed
Budget, wheel and send-confirmation rows (pause thresholds, human typing, prompt-submitted confirmation): no driver input path is touched.
@task_item n=6 part=not_needed
Fake agent rows (its command line, receipts, turn scripts, fired events): the chunk's fake replays a compositor, not the claude agent.
@task_item n=7 part=not_needed
Hook performance budget rows (hyperfine budgets, spine deadline, perf job): the seam adds no timed hook path.
@task_item n=8 part=not_needed
Instance state rows (snapshot fields, heartbeat staleness, event log records, replay from the log): the seam writes no instance state.
@task_item n=9 part=not_needed
Harness command contract rows (boot, cleanup, status document, pre-push stages, session record, supervisor exit file): the chunk changes no harness command.
@task_item n=10 part=not_needed
PTY passthrough and screen rows (vt100 feed, outer PTY driver, terminal replies, screen signatures): the seam reads no child screen.
