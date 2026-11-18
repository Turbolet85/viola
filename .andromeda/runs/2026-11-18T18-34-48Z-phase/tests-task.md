@task_prose part=chunk
Builds a new recorded fixture set for claude 2.1.325, recorded whole by a clean live viola verify --record under the rig's private config, every file through the recorder's scrub, none hand-edited.
Re-records the MCP tools/list exchange through the test-only MCP tap on 2.1.325 and places it in the same set.
Re-reads the ledger probes' stamped reading and the fixture drift suites against the new set, the fake agent replaying it in CI.
Measures live beside the recording: a StopFailure payload inside a started turn, the /context post-condition, the placeholder hint of the box text, whether Run E starts, and the remove-the-guard control for the browser-tools row on a filtered run.
Disposes each of the entry's eleven carries by name: built, measured, or leaving with its reason and next reader; the pinned product binary is not rebuilt or re-pinned.
@task_file n=1 path=fixtures/claude/2.1.325/
what: the recorded Claude fixture set for CLI version 2.1.325 — every hook payload, screen and transcript the live verify recorder writes, replayed by the fake agent in CI
@task_file n=2 path=fixtures/claude/2.1.325/Mcp.tools-list-1.json
what: the recorded MCP tools/list exchange for CLI version 2.1.325, captured by the test-only MCP tap from a live claude -p start
@task_item n=1 part=needs
The recorded fixture set per CLI version: a new 2.1.325 set beside 2.1.283, 2.1.287, 2.1.288 and 2.1.302, produced only by a live verify recording and landed whole, never hand-written or hand-edited.
@task_item n=2 part=needs
Recording only at zero failing rows: the live verify that records the set must read every relied ledger row passing (24 relied rows), or the recording is refused.
@task_item n=3 part=needs
Recorder hygiene through the recorder's scrub: no host path, user name, email, token or status-line text in any recorded file; a dirty payload refuses the recording whole and is never scrubbed by hand; screens kept by the run stay local.
@task_item n=4 part=needs
Record homes never named home: the live rig's record home comes from a helper like the tests' record home helper and holds no path component named home, since a recorder refuses a payload holding /home/.
@task_item n=5 part=needs
The MCP exchange recorded by the test-only tap, which refuses a dirty exchange whole; the re-recorded tools/list reads the list tool's description without the usage-budget wording, and the tool set the server advertises is what the recording holds.
@task_item n=6 part=needs
The fake agent replays recorded bytes for the new version, and its transcripts are compared byte for byte against the recordings: the drift suites and every-version replay must run over 2.1.325 in CI.
@task_item n=7 part=needs
Recorded fixtures checked against their tolerant schema and named by hook event, at the recorded payload path the fixture contract names; the run-time fixture hygiene walk reaches the new set.
@task_item n=8 part=needs
The ledger probes' stamped reading: viola verify's stamped line and summary read against the new set, every ledger row with its verify probe, the version gate needing every relied row, and stamps written only by verify.
@task_item n=9 part=needs
The StopFailure hook, registered with no ledger row and no verify probe: a live StopFailure payload recorded inside a started session's turn, its error word read against the nine-word cause map with no parse-rejected unknown-value line for any of the CLI's 13 words; the exception settled as a ledger row with its probe or kept on the founder's word.
@task_item n=10 part=needs
The 22nd relied row local-command-echo leaves every earlier stamp unverified until verify runs again: the re-stamp is live, under the founder's cap word.
@task_item n=11 part=needs
Local commands confirmed by their post-condition only: /context's post-condition measured live, and whether /mcp disable of an absent server writes to the user's real Claude config; a local command with no measured post-condition stays unconfirmable.
@task_item n=12 part=needs
The screen read only for compiled signatures: the box text holds-text reading, provisionally the CLI's placeholder hint, measured for whether a cell attribute the screen model reads tells the hint apart from typed text; the rendered child screen is never parsed beyond that.
@task_item n=13 part=needs
The browser-tools prompt: viola never answers or seeds it; on a host where it was never answered a real verify reads 8 of 24 and the version stays unverified; whether viola ever answers or seeds it is left to the founder.
@task_item n=14 part=needs
Run E's resume start: the clean recording records Run E, so its record reads whether Run E starts on a SessionStart capture the resume target accepts.
@task_item n=15 part=needs
The bounded remove-the-guard witness for the browser-tools row: the bt-absent control fired on a filtered run (a case filter or no fail-fast) so the named case that verifies the absent browser-tools row is counted is read red without the guard.
@task_item n=16 part=needs
Real claude only locally, never in CI: every live start runs on the local host through the local-live path, names the 2.1.325 install by its path rather than a bare claude, and each start's count needs the founder's cap word.
@task_item n=17 part=needs
A live reading is recorded as read, never as verification: nothing is claimed for 2.1.326 or 2.1.327, and readings made under the rig's private config are not claimed for the live home.
@task_item n=18 part=needs
The pinned product binary that drives the builder is not rebuilt or re-pinned before or by this chunk's verify; a binary built from the tree reads earlier 22- or 23-row stamps unverified.
@task_item n=19 part=needs
Each of the entry's carries is disposed by name, none dropped or widened; no mutation run and no scheduled wake or timer belong to this chunk.
@task_standing n=1
convention: each new test follows the test function naming rule and is table-driven in rstest cases where it is a unit case
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test creates its own private home under the test homes root and never reads the host's claude sessions
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: each new or changed test synchronises on logged events, with no retries and no sleeps
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: each new fixture is synthetic text only, recorded by verify or the tap and walked for hygiene, and is never edited after recording
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: each new test's oracle is a literal, asserting on the contract rather than fixture internals
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: each new verify-driven test is tied to a test-owned home and stays inside the per-test kill bound
home: integration-test-strategy · ci-integration
@task_item n=1 part=not_needed
The wrapper channel, its JSON-RPC error bodies, link and unlink methods and channel wait: the chunk records fixtures and changes no channel behaviour.
@task_item n=2 part=not_needed
Revive, run and start refusal codes, exit-1 causes and their JSON and human documents: no start or revive refusal is added or changed.
@task_item n=3 part=not_needed
The send path and its confirmation, refusal order, paste framing and send result contracts, beyond the local-command post-condition named above.
@task_item n=4 part=not_needed
MCP frame tools' invalid-params fields, reply screen and instance-unreachable details: only the tools/list recording is touched.
@task_item n=5 part=not_needed
Hook dialog decision bodies, dialog hold and answer mapping: no dialog behaviour changes.
@task_item n=6 part=not_needed
Heartbeat staleness, instance liveness, the instance snapshot and its replay from the log.
@task_item n=7 part=not_needed
Socket directory checks, strict-modes refusals, server identity and peer euid admission.
@task_item n=8 part=not_needed
The viola list board and list --json rows, columns and skipped counts.
@task_item n=9 part=not_needed
Performance budgets and the hyperfine perf job: no hook path is timed or changed.
@task_item n=10 part=not_needed
Mutation run verdicts, mutants kill lines and host-excluded mutants: the chunk runs no mutation.
@task_item n=11 part=not_needed
Fuzz targets, fuzz corpora and property-test seeds: no parser surface changes.
@task_item n=12 part=not_needed
Harness session record, boot readiness, cleanup document, status document and pid-file contracts of the test harness.
@task_item n=13 part=not_needed
Process log line fields and per-event corr contracts: no log line is added.
@task_item n=14 part=not_needed
Chaos tests and torn-append healing of the event log.
