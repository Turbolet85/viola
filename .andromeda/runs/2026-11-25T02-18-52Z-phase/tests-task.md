@task_prose part=chunk
Changes the wrapper's window step so a placed window reaches its workspace while the compositor's active workspace stays unchanged (the operator's F4 «no focus change», v2-07), in a move form P3 measures from the compositor's own help, the seam's closed request set, its tap and its fake — never a second compositor seam.
Adds to v2-07's acceptance a clause a gate entry reads: the active workspace read before a placing open equals the one read after; v2-07 is claimed and proved again, its live half gated on the operator's grant and timing.
Every caller of the move — viola open, viola up, viola revive under --origin — takes the same answer (the same form or a named difference); the seam's tap, fake and recorded move fixture follow the form, the fixture re-recorded only through the operator's one granted live reading.
Folded item: says from the code where the automatic verify of a new CLI runs at a start and what it stamps when its own preconditions fail (an untrusted folder, a modal, a red it caused itself), and fixes that, its form a fork, with its own acceptance and a witness that cannot pass vacuously.
@task_file n=1 path=src/run/window.rs mark=edits
what: the wrapper's window step, which sends the window move to a workspace through the Hyprland seam — edits: the move-to-workspace request it sends
@task_file n=2 path=crates/viola-hyprland/src/lib.rs mark=edits
what: the Hyprland seam and its closed request set — edits: the move-to-workspace request's form
@task_file n=3 path=crates/viola-hyprland/src/reply.rs mark=edits
what: the Hyprland seam's reply reading for each request of its closed set — edits: the reply of the move request
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs mark=edits
what: the seam's test-only tap that records compositor exchanges into fixtures — edits: the move request it records
@task_file n=5 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake compositor stand-in on the test PATH that answers the seam's requests from recorded fixtures — edits: its reply set for the move
@task_file n=6 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json mark=edits
what: the recorded compositor fixture of the window move for Hyprland 0.56.2, re-recorded only by the operator's granted live reading
@task_file n=7 path=src/cmd/open.rs mark=edits
what: viola open, a caller of the window move that places an opened window on its workspace
@task_file n=8 path=src/cmd/up.rs mark=edits
what: viola up, a caller of the window move that places each fleet window on its workspace
@task_file n=9 path=src/cmd/revive.rs mark=edits
what: viola revive, whose --origin start is a caller of the window move
@task_file n=10 path=src/run/auto_verify.rs mark=edits
what: the automatic verify of a new CLI run at a start, and the stamp it leaves — edits: where it runs and what it stamps when its own preconditions fail
@task_file n=11 path=src/cmd/run.rs mark=edits
what: viola run's start, its verify and trust steps after the bind — edits: the start's call of the automatic verify
@task_file n=12 path=src/cmd/verify.rs mark=edits
what: viola verify, the only stamp writer — edits: the stamp writer
@task_file n=13 path=tests/contract_hyprland_seam.rs mark=edits
what: the Hyprland seam contract suite over the recorded fixtures and the closed request set
@task_file n=14 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the tests of the seam's recording tap
@task_file n=15 path=tests/cli_up.rs mark=edits
what: the CLI suite of viola up as a subprocess, its window placement over the fake compositor
@task_file n=16 path=tests/cli_revive.rs mark=edits
what: the CLI suite of viola revive as a subprocess, its --origin window placement over the fake compositor
@task_item n=1 part=needs
The window move's new form: a placed window reaches its workspace and the compositor's active workspace stays where it was, for open, up and revive under --origin alike (or a named difference per caller).
@task_item n=2 part=needs
The active-workspace clause as an acceptance predicate a gate entry reads: the active workspace read before a placing open equals the one read after.
@task_item n=3 part=needs
Real window placement, re-open and raise are proven only by a gated live reading; this chunk's live re-record and v2-07's live half are operator-granted, operator-fired legs held with rehearsal forms and never fired by the phase or by CI.
@task_item n=4 part=needs
A live reading that fails or reads only part of a promise is recorded as read, never as verification; v2-07 is not claimed verified on a placement-only reading.
@task_item n=5 part=needs
The seam stays one closed request set over the compositor; the move is changed inside it, and the compositor is reached in tests only through a real stub executable on a test-scoped PATH, never a mock library or a second seam.
@task_item n=6 part=needs
The fake compositor's reply set holds only what the real compositor answers, its replies replayed byte for byte from the recorded fixtures, and fixture drift is compared byte for byte.
@task_item n=7 part=needs
The re-recorded move fixture is made by the test-only tap, scrubbed of home paths, usernames and real data, checked by the fixture hygiene walk; a recording replaced by a new form is kept as a superseded fixture where the plan keeps relayed fixtures.
@task_item n=8 part=needs
The rehearsal-only seams of the live rig and the one standing committed live rig: the live legs of this chunk run through them, clean of the firing session and rooted outside every repository.
@task_item n=9 part=needs
The automatic verify at a start: where it runs (the start folder versus a trusted place) and the verify and trust steps of viola run after the bind; trust-start test folders stand outside any repository.
@task_item n=10 part=needs
The stamp on the automatic verify's own failure: a verify whose preconditions failed (an untrusted folder, a modal, a red it caused itself) must not leave a failing stamp that makes the CLI version read unverified for every later start; stamps are written only by verify, one stamp writer.
@task_item n=11 part=needs
The unverified CLI version gate as read through stamps: a version's verified or unverified reading after the fix, and a failing ledger row being no refusal.
@task_item n=12 part=needs
The folded item's witness cannot pass vacuously: a bounded remove-the-guard witness and the kill witness for a tied stamping verify, each tied to a test-owned home, runtime dir and user home.
@task_item n=13 part=needs
Every test verify or wrapper start tied to a test-owned home; no root test spawns viola untied; test-started viola drops inherited session and CLAUDE* env.
@task_item n=14 part=needs
The fake-foot stand-in's spawn record and the window step's records read by the up and revive suites, as the move's form changes what they observe.
@task_item n=15 part=needs
Seam replies and placement verdicts asserted on records and literals, never on the rendered screen; waits keyed on logged events, no sleeps or retries.
@task_item n=16 part=needs
External processes and faults are real (real stub executables, real filesystem), time through injected clocks; the compositor is never contacted live by any test.
@task_item n=17 part=needs
Mutation testing at the epoch boundary: this is the epoch's last entry, so the mutation gate's thresholds and verdict over the whole chunk bind its changed code.
@task_standing n=1
convention: each new test is named by the plan's test-function naming, runs under nextest in parallel with no retries or sleeps, and stands in the layer its binary prefix selects
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test creates its own private home under the test-home root and seeds its own data; on-disk product state is produced by viola, never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new or re-recorded fixture is synthetic or scrubbed, recorded by its tap or verify, never from real prompts, and passes the fixture hygiene walk
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: each new test asserts public behaviour by literal oracles, mocks nothing it does not own, and keeps no shared mutable state or process-wide env mutation
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new log line is a structured JSON process-log line with the plan's fields, codes only, carrying no secret
home: log-format
@task_standing n=6
convention: each change passes the CI jobs ending in the harness gate: coverage floors, lint, supply-chain and secret scan, with no gate skipped
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
MCP tool surfaces — tools/list, tool arguments, isError payloads, reply screens and MCP-CLI parity: the chunk adds no MCP tool and changes no MCP reply.
@task_item n=2 part=not_needed
The send path — send confirmation, paste framing, typed text, send refusals and the send event log: the chunk sends no text to the child.
@task_item n=3 part=not_needed
Dialog hooks and decision bodies — question, permission and plan bodies, dialog holds and expiry: the chunk touches no dialog.
@task_item n=4 part=not_needed
The wrapper channel's link and unlink methods, their event records and since times: the chunk links no instances.
@task_item n=5 part=not_needed
viola wait and viola last, their wake kinds, cursors and timeouts: the chunk changes neither.
@task_item n=6 part=not_needed
The wheel — human input, pause, release and the release verb: the chunk moves no wheel.
@task_item n=7 part=not_needed
Hook performance budgets and hyperfine timing of hooks: the chunk changes no hook path.
@task_item n=8 part=not_needed
The viola list board, its columns, liveness and skipped counts: the chunk changes no list output.
@task_item n=9 part=not_needed
Fuzz targets and their corpora: the chunk adds no parser surface to fuzz.
@task_item n=10 part=not_needed
Harness session, status and cleanup documents and their exit codes: the chunk changes no harness command.
