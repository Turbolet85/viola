@task_prose part=chunk
Adds a `viola focus <name>` CLI verb: resolve the home, validate the name, read the instance state under the strict-modes checks, find the session's window and issue the Hyprland dispatches that show its workspace and bring the window forward.
Window identity has two sources: the window record the wrapper keeps in the snapshot for a session started with --origin (viola open's), re-parsed through the seam's address validator, and for a hand-started viola run the wrapper's process ancestry matched against the compositor's client list.
Every no-window outcome is a named refusal, and outside Hyprland the verb refuses as unsupported.
Tests run on the fake compositor (recorded Hyprland 0.56.2 replies) and the fake agent; the help, exit-cause and verb tables gain the new verb's entries.
No live start, no real window opened or focused, no real dispatch or key, and no read of the host's real claude state in this phase.
@task_file n=1 path=src/cmd/focus.rs
what: the new viola focus CLI verb of the root bin — the focus command: name validation, instance state read under strict modes, window lookup by recorded identity or ancestry, Hyprland workspace and window focus dispatches, named refusals and the unsupported refusal
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the root bin's CLI verb set and dispatch, the grouped verb table of viola --help — edits: the verb list and its help verb-table entry for focus
@task_item n=1 part=needs
A new CLI verb, viola focus, tested as a subprocess of the real viola binary in a fresh per-test home, its outputs pinned by literal asserts.
@task_item n=2 part=needs
The verb's name argument is validated as a viola name before any read; an invalid or control-character name is refused, never used as a target, and human output escapes control characters.
@task_item n=3 part=needs
The instance's state is read under the strict-modes checks before any read or open: a home or instance another user may read is refused with the strict-modes refusal, as for every other CLI verb.
@task_item n=4 part=needs
The snapshot is read only through the read-or-replay path; a replayed snapshot carries no endpoint and no window, and the verb must act on that (the window record is read only from the snapshot the wrapper wrote).
@task_item n=5 part=needs
Window identity arm (a): the window record the wrapper keeps under --origin (address, workspace, placement, origin), its address re-parsed through the seam's address validator before any dispatch.
@task_item n=6 part=needs
Window identity arm (b): for a hand-started viola run with no window record, the wrapper's process ancestry (a bounded in-memory walk of at most 8 parent pids) matched against the compositor's client list.
@task_item n=7 part=needs
Hyprland dispatches only through the hyprland seam's closed request set: a workspace-focus dispatch and a focus-window dispatch, recorded for Hyprland 0.56.2 and replayed by the fake compositor; no other request is sent.
@task_item n=8 part=needs
Named refusal for every no-window outcome (no window found, recorded window gone, no ancestry match, instance not wrapped or not live) and an unsupported refusal outside Hyprland, each a distinct exit cause with its own hint, in both human and --json modes, one JSON document per run.
@task_item n=9 part=needs
CLI exit codes and exit-cause table gain the focus refusals; the --json failure document and human refusal lines carry them, error bodies sanitized with no path or upstream text.
@task_item n=10 part=needs
viola --help grouped verb table gains the focus verb; help carries no bold or underline and no SGR without a terminal.
@task_item n=11 part=needs
Tests reach the compositor only through the fake (recorded replies, a real stub process or seam double), never a live Hyprland; the real window placement and focus are proven only live and that clause is planned gated on the operator's live reading.
@task_item n=12 part=needs
Test-started viola drops inherited session and VIOLA_* environment variables, and the Hyprland signature environment the verb reads is set per child, never by process-wide env mutation.
@task_item n=13 part=needs
Tests never read the host's claude sessions or real home; viola list or focus runs only against the fake agent on a test-scoped PATH, with no real claude in CI.
@task_item n=14 part=needs
CLI-only: focus is not added to the MCP tool set (no control tools over MCP); the MCP tools/list stays as it is.
@task_item n=15 part=needs
No new time bound: any wait on the compositor reply stays inside the existing bounded readers and wait bounds, no sleeps or retries in tests.
@task_item n=16 part=needs
Fixtures for the compositor replies are recorded, scrubbed of real data (no addresses, usernames or home paths of the host) and byte-compared; the fake replays them byte for byte.
@task_item n=17 part=needs
Error sanitization and log lines: any process log or diagnostics line the verb writes is structured JSON, codes-only, carrying no session names or secrets.
@task_standing n=1
convention: each new test function follows the test naming, runs in parallel with its own fresh home, and waits only on logged events, never sleeps or retries
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new root integration binary for a CLI surface is prefixed by its surface and selected by nextest filterset
home: 2-test-strategy
@task_standing n=3
convention: each new security control the verb adds (name validation, strict modes, address re-parse) gets one negative test
home: 1-test-scope-summary
@task_standing n=4
convention: each new fixture is synthetic or recorded and scrubbed, walked by the hygiene check, and carries no real host data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: each new double sits only at a product seam trait or is a real stub executable; nothing owned by others is mocked and nothing is mocked in E2E
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=6
convention: each new test that spawns viola ties the child with a drop guard and drops inherited env while keeping LLVM_PROFILE_FILE
home: integration-test-strategy
@task_item n=1 part=not_needed
The wrapper channel and its JSON-RPC methods, link and unlink, and the channel error tables: focus sends no frame to the wrapper.
@task_item n=2 part=not_needed
Send, answer, wait, last, release and the wheel rules, dialogs and hook decision bodies: focus neither types nor answers nor moves the wheel.
@task_item n=3 part=not_needed
The viola hook exec entry points, hook deadlines and their performance budgets: focus adds no hook.
@task_item n=4 part=not_needed
viola run, revive and verify start sequences, their start refusals, stamps and pinned-exe checks: focus starts nothing.
@task_item n=5 part=not_needed
The test harness commands (boot, status, cleanup, supervise, gate, pre-push, mutants verdict) and their documents: the chunk changes no harness command.
@task_item n=6 part=not_needed
The fake agent's command-line options, receipts and screens: the chunk uses the fake agent as it stands and adds no option to it.
@task_item n=7 part=not_needed
Fuzzing and the vt100 screen feed, passthrough TUI rendering and the live rig's screen read: focus parses no child screen.
@task_item n=8 part=not_needed
CI artifacts, workflow permissions and action pinning: the chunk adds no CI job or artifact.
