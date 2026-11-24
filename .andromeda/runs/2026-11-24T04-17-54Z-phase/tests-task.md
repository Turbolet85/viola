@task_prose part=chunk
Adds a new human CLI verb `viola up`: the fleet re-opened after a reboot, consuming only normalised state and events.
It scans the board's instances under the home, selects those that read as live before the host went down, orders them parents before children by the recorded lineage, skips every one whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing revive start path, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance naming the instance and a closed outcome word, and under --json one document of codes and closed words only.
It adds the tests that prove the selection, the order, the skip, the lines and the argv handed to each start, over the fake agent and the Hyprland fakes under a test-owned user home; it re-measures at HEAD the lineage and launch lines at every start, viola open's window and its window record under --origin, and revive resuming in place.
@task_file n=1 path=src/cmd/ mark=edits
what: the root bin's CLI verb dispatch (clap) — the verb table and help rows, each verb's exit codes and exit causes, and the --json machine contract — edits: a new verb `up` and its arm
@task_file n=2 path=src/human/ mark=edits
what: the human-mode output module — the CLI human lines each verb prints, control-character escaping, no SGR without a terminal — edits: the per-instance line of `viola up`
@task_item n=1 part=needs
A new CLI verb joins the verb table: the grouped verb table of viola --help, its help row, its exit codes and exit causes, each exit cause with its own hint; help carries no bold or underline.
@task_item n=2 part=needs
Under --json the verb prints exactly one JSON document per run on stdout, of codes and closed words only, beside the existing CLI --json success and failure document shapes; error bodies hold no path or upstream text.
@task_item n=3 part=needs
One human line per instance naming the instance and a closed outcome word: instance names escaped (every control character shown, never raw), no escape or SGR bytes without a terminal, human output pinned by literal asserts in the tests; an unknown refusal word stays visible.
@task_item n=4 part=needs
The board read model: the board scan of instances under the home, the snapshot read through read-or-replay (a broken snapshot replayed from the event log, the replay writing no file and carrying no endpoint or cwd), the tolerant event log reader that counts and skips unknown or torn lines, and tailing that ignores symlinks and invalid names; the board held to the home read rule.
@task_item n=5 part=needs
The skip of a live instance: instance liveness live, stale and gone, read from the snapshot pid plus start time and the heartbeat; heartbeat staleness classification by age and by pid or start-time mismatch, through the pure classifier that takes its instant; a pid reused by another process is never read live.
@task_item n=6 part=needs
The selection of what was live before the host went down versus what ended on purpose, read from snapshot, heartbeat and event log; runtime-dir sockets and locks may not survive a reboot (the endpoint is read only from the snapshot and a gone endpoint is not liveness).
@task_item n=7 part=needs
Parents before children: the order is taken from the recorded lineage line {parent, role} of each start; a missing, never-live or cyclic parent must still yield one outcome line per instance and never hang or panic.
@task_item n=8 part=needs
Each re-open goes through revive's start arm: its preflight order (the agents list via claude agents --json, the session-live reading that fails open, the agents list that reads unknown when unreadable), and its closed refusal codes (already-live, session-live, no-session, cwd-missing, stale-heartbeat, squatted-name, socket-dir-refused, pinned-hash-mismatch, strict-modes-failed, internal-error), each surfacing as that instance's outcome rather than ending the fleet run.
@task_item n=9 part=needs
Revive's exit codes and exit 1 causes, its human refusal line and its --json refusal document, as the outcome `up` reads per instance and maps to its own closed words; no revive document once a child holds the terminal.
@task_item n=10 part=needs
Strict-modes checks on the home tree before any read or open: a home readable by another user refused, a widened home tree refused, the socket directory checked and refused, never repaired or chmod-ed; the strict-modes refusal exit and its per-verb rows apply to the new verb.
@task_item n=11 part=needs
The terminal window each re-open gets is viola open's terminal spawn, and the wrapper's window record {address, workspace, placement, origin} under --origin; the Hyprland seam's closed request set is used, never widened; a recorded compositor address does not survive a reboot.
@task_item n=12 part=needs
Real window placement and every clause needing a real host restart or a real resumed session are proven only by a gated live reading the operator grants; the live reading is recorded as a reading, never as verification, and a fake-agent run is no proof of real-CLI behaviour.
@task_item n=13 part=needs
Every start the verb makes records the launch pair {model, effort} and the lineage line {parent, role}; these are re-measured at HEAD in the new tests, not copied.
@task_item n=14 part=needs
The started child's environment: the CLAUDE* environment strip, the child given its viola name, instance dir and binary, and holding nothing else of viola's; its process-start record of env kept and stripped counts.
@task_item n=15 part=needs
The argv handed to each start is asserted through the fake agent's command line and receipt (argv, cwd, env, start), the fake agent configured by argv alone and placed on a test-scoped PATH; the Hyprland side through real stub executables on PATH.
@task_item n=16 part=needs
A per-instance obs event in the event catalog: a process log line in structured JSON per line with its event, instance and corr fields, codes only with detail apart; the NEVER-log floor holds: no address, pid or workspace in a process-log line, no secret in logs or diagnostics.
@task_item n=17 part=needs
CLI-only, no MCP tool: the MCP tools/list tool set is unchanged and there is no control tool over MCP; re-opening the fleet is never reachable from a driver session's MCP surface.
@task_item n=18 part=needs
Tests run viola as a subprocess under a test-owned user home and runtime dir: test-started viola drops inherited env and session variables, every spawned viola and wrapper is tied (drop guard, tied spawn thread), and tests never read the host's claude sessions or the real claude CLI.
@task_item n=19 part=needs
Waits in the new tests key on logged events with bounded waits inside the existing wait bounds (the WITHIN bound, root waits below the kill lines, the per-test kill for stamped-home tests); no new time bound is added while its fork waits.
@task_item n=20 part=needs
Revive's resume in place and the start events' order (run start events, the first wheel record of a start, child spawn after events) as the existing start path gives them, re-measured at HEAD for a revived instance.
@task_standing n=1
convention: each new root integration test binary is prefixed by its surface and selected into its layer by that prefix; test functions follow the naming rule; shared helpers are reused, never copied
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test creates its own private home, seeds its own data, and produces on-disk product state by running viola, never by hand-writing it
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new test runs and revives under a test-owned user home and runtime dir, its home under the one test-home root
home: test-data-bootstrap
@task_standing n=4
convention: each new test asserts literal oracles on the contract, uses no sleep or retry, no process-wide env mutation, no shared mutable state, and runs in parallel
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new fake or double is a real stub executable or a seam trait, never a mock of what is not owned, and nothing is mocked in E2E
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=6
convention: new code counts toward the comprehensive-tier coverage floors and the mutation gate at the epoch boundary; no #[ignore] parking
home: 10-quality-gates-coverage-targets · quality-gates-coverage-targets
@task_standing n=7
convention: each new process log line is one JSON object per line through the tracing subscriber, with the harness-grepped fields and corr where defined
home: log-format
@task_standing n=8
convention: each OS- or host-gated test is registered with its reason and kept in the binary's root file
home: test-strategy
@task_item n=1 part=not_needed
The wrapper channel's JSON-RPC methods, link and unlink, MCP frame tools and their invalid-params fields: `up` opens no frame to a running wrapper and changes no channel method.
@task_item n=2 part=not_needed
The send, answer, release, wait and last verbs and their contracts, dialog hook decision bodies, the wheel and human-input passthrough: the chunk sends no text, answers no dialog and moves no wheel.
@task_item n=3 part=not_needed
Hook exec-form entry points, hook deadlines and the performance budgets of hooks: the chunk adds no hook and times no hook path.
@task_item n=4 part=not_needed
viola verify, stamps, ledger rows, fixture recording and the per-CLI-version fixture replay: the chunk stamps nothing and records no fixture.
@task_item n=5 part=not_needed
The harness's own command surfaces (boot, cleanup, status, supervise, schema-check and run documents, their exit codes and reason words, pre-push detail): the chunk changes no harness command.
@task_item n=6 part=not_needed
The rendered child screen, vt100 feed, outer-PTY passthrough and terminal-reply readings: the chunk reads no screen and drives no passthrough.
@task_item n=7 part=not_needed
Workflow files and their CI artifacts, retention, SHA pinning and permissions: the chunk adds no CI job and edits no workflow.
@task_item n=8 part=not_needed
Fuzz targets, corpora and nightly fuzzing: the chunk adds no fuzz target.
@task_item n=9 part=not_needed
Absences of Windows and macOS support, a database, outbound HTTP, a browser suite and a deploy stage: nothing of the chunk touches them.
