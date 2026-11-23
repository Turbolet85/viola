@task_prose part=chunk
Builds the verb `viola open`: a closed flag allowlist (name, parent, role, cwd, workspace, model and effort, json, home; after `--` the word claude alone) that starts a foot terminal directly, no shell, running the pinned viola's `run` for a new session beside its parent.
The started wrapper takes two hidden checked flags (origin cli or agent, and the target workspace), finds its own window through the Hyprland seam by its terminal ancestor's pid, issues the one closed move dispatch, and records a new log-only event kind `window` and a snapshot `window?`, replayed last line winning; `list --json` and MCP `list` items gain `window?` and `origin?`, the six-column board unchanged.
Open waits at most 30 s for the instance to be live (else exit 1 not-live), gives the window 5 s polled at 100 ms (else unplaced), refuses already-live and stale-heartbeat before any spawn, and adds exit 1 terminal-not-found and a fixed refusal line for a word after `--` other than claude.
Also adds tests for the hypr tap recorder's three untested record-move codes (move-refused, move-unseen, window-left) over stub variants, lands the 5 s compositor read bound as a time-bound row, and adds a test-only one-command live reading script (R2) that gates v2-07 and is not run in CI.
@task_file n=1 path=Cargo.toml mark=edits
what: the root package manifest and its dependency lists — edits: the Hyprland seam crate's entry, moved from dev-dependencies to a product dependency
@task_file n=2 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the test-only E2E crate's tests of the hypr tap recorder over planted stub executables — edits: new tests for the record-move failure codes move-refused, move-unseen and window-left
@task_file n=3 path=scripts/r2-open.sh
what: a new test-only live-reading script: one command that starts a parent wrapper over sleep in its own foot window, runs one `viola open … -- claude` under a scratch home, reads the verdict and closes both windows on every exit path
@task_item n=1 part=needs
The new CLI verb open, tested as a subprocess: its closed allowlist refused with exit 2 before anything starts, a word after `--` other than claude refused by one fixed line, the leading-slash and argv vector negatives that apply to its arguments.
@task_item n=2 part=needs
No shell line is ever built from open's arguments: the terminal and the run it starts are spawned directly by argv, whatever characters the name, cwd or other arguments hold.
@task_item n=3 part=needs
Open's exit codes and JSON documents: exit 1 terminal-not-found, exit 1 not-live, already-live and stale-heartbeat read before any spawn, exit 1 strict-modes-failed before any log is opened or read, no exit 11; one JSON document per run in the v1 ok/error shape with a null detail.
@task_item n=4 part=needs
Human output of open and the grouped help verb table, whose setup group gains open; refusal lines are fixed literals pinned by literal asserts, with no SGR without a terminal and control characters escaped.
@task_item n=5 part=needs
The new event kind window (address, workspace, placement, origin), log-only: the event line's kinds, keys and round trip, and an older reader counting it as unknown and skipping it rather than failing.
@task_item n=6 part=needs
The instance snapshot gains window, written only by the wrapper (the single writer of the snapshot and its start lines, never open), and the replay from the log recovers it with the last window line winning; the replayed field set grows by it.
@task_item n=7 part=needs
viola list --json items and the MCP list tool items gain window and origin; CLI and MCP keep identical payloads and the list JSON keeps mirroring the channel; the human board's six columns stay unchanged.
@task_item n=8 part=needs
The wrapper's start takes two new hidden checked flags from open, origin and target workspace, on the started run's argv; a bad value is refused as a start refusal before the child.
@task_item n=9 part=needs
Placement through the Hyprland seam: a bounded clients read until a window's pid is the wrapper's terminal ancestor, then the one closed move dispatch to the target workspace; no parent window gives unplaced, the seam's unsupported and unreachable skip placement with the reason recorded, no environment read for it.
@task_item n=10 part=needs
The parent's window is its recorded one, else an ancestry walk of at most 8 steps held in memory; the parent defaults to the caller's VIOLA_NAME.
@task_item n=11 part=needs
New time bounds (open's 30 s liveness wait, the 5 s window appearance at 100 ms polls, the 5 s compositor read bound) each land as a row of their own beside the existing time-bound lists, and tests drive them through the injected clock or a test parameter, never by sleeping, staying under the per-test kill lines.
@task_item n=12 part=needs
Liveness read by open goes through the instance liveness classification (live, stale, gone) and the heartbeat staleness rules, reading the snapshot through read-or-replay.
@task_item n=13 part=needs
The fake agent side gains a stand-in foot executable and a clients reply keyed to the stand-in's pid; tests reach the terminal and the compositor only through real stub executables on a test-scoped PATH, never the operator's live compositor or terminal.
@task_item n=14 part=needs
The hypr tap recorder's record-move failure codes move-refused, move-unseen and window-left, each reached with no live window over a planted stub variant.
@task_item n=15 part=needs
The window line and every open log line are codes-only: no session name, path, title or argument text in the logs, detail kept apart.
@task_item n=16 part=needs
The claude child started through open keeps the agent identity environment strip and the child env rules of viola run; open passes no claude flag.
@task_item n=17 part=needs
Spawned stand-in terminals and wrappers in tests are tied to the test (drop guard, tied spawn thread) under a test-owned home and test-owned user home; no untied viola spawn.
@task_item n=18 part=needs
The R2 live reading is local only, never in CI: one real claude start under a scratch home outside every repository with a private config dir, recorded as a reading, never as verification; tests never read the host's claude sessions.
@task_item n=19 part=needs
The Hyprland seam crate becomes a product dependency of the root binary: the sync-crate and tokio bans and the release-build-viola-only rule hold over the new product graph.
@task_standing n=1
convention: Each new root integration test binary is prefixed by its surface, and each test function follows the test naming rule.
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: Each new test creates its own fresh private home, seeds its own data and holds no shared mutable state or process-wide env mutation.
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=3
convention: Each new test synchronises on logged events or bounded polls, never sleeps or retries.
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: Each new test asserts against literal oracles of the contract, never Debug strings or fixture internals.
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: Each new stub or fixture is a real executable or synthetic scrubbed data, never mocking what viola does not own.
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=6
convention: Each new security control (allowlist, no shell) gets one negative test.
home: e2e-test-strategy · 6-e2e-test-strategy
@task_standing n=7
convention: Each new log line follows the process log line format and its corr rules.
home: log-format
@task_item n=1 part=not_needed
Hook dialog decision bodies, dialog answer mapping and dialog hold rules: open raises and answers no dialog.
@task_item n=2 part=not_needed
Send, paste framing and send confirmation, the wheel and release verbs, and the wait and last verbs: the chunk sends no text and moves no wheel.
@task_item n=3 part=not_needed
The wrapper channel's link and unlink methods and its JSON-RPC error codes: open adds no channel method.
@task_item n=4 part=not_needed
Revive-only refusal codes and preflight (no-session, cwd-missing, session-live, squatted-name as revive reads it): open starts a new session, never revives one.
@task_item n=5 part=not_needed
The vt100 screen feed, terminal replies, outer-PTY passthrough and screen signature reading: the chunk reads no child screen.
@task_item n=6 part=not_needed
Hook performance budgets and hyperfine perf rows: the chunk touches no hook path.
@task_item n=7 part=not_needed
The agent-run harness status document, boot readiness codes, cleanup document and supervisor rows: the chunk changes no harness verb.
@task_item n=8 part=not_needed
Recorded Claude hook payload fixtures, the fake agent's turn script, receipts and input handling: the chunk records no new CLI-version fixture.
@task_item n=9 part=not_needed
The absences of a database and a browser suite: nothing here reaches them.
