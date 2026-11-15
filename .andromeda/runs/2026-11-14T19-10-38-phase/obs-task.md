@task_prose part=chunk
A removal: viola stops intercepting the wrapped claude CLI's status line; the user's own status-line command runs exactly as it does unwrapped.
Leaving: the hidden hook statusline arm and its capture probe form, the pure statusline module with its shell-out, the start's statusline source read, settings override and --settings argument, the snapshot's statusline_command field, the fake agent's status-line modes, the StatusLine fixtures, schema and fuzz seeds, the statusline-shell log subject and span area, and every test of the interception.
The capability ledger loses its two interception rows (status-line override and status-line shell), so its relied rows go from 23 to 21, and viola verify's Run B loses its status-line probe.
P3 re-reads the no-shell architecture decision, re-measures whether Run B passes --settings for anything else, whether chrono still has users, and the old-reader / new-file compatibility of disk state; no new settings, flag, env var, ledger row or log line is added.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the viola hook statusline arm — instance check, replay read, the /bin/sh -c run of the user's status-line command under the statusline deadline, and the stdout pass-through; deleted whole
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the viola hook dispatch and its hidden --capture probe forms — edits: the statusline module and dispatch branch, the STATUSLINE_DEADLINE constant and its five-second test, the hidden hook statusline --capture probe form (capture_statusline) and its four tests, the --shell argument doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs
what: the pure statusline module of the claude adapter — the /bin/sh shell argv, the user command read, the settings override document, the user-settings path and the --settings flag word; deleted whole
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude adapter crate's module list — edits: the pub mod statusline entry
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the proptest regression seeds of the statusline module; deleted
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the wrapped claude CLI and its relied rows — edits: the StatusLineOverride and StatusLineShell rows in ALL, their ids and claims, the ShellVerdict, StatusLineCapture, statusline_variants and StatusLine.<k> helpers, relied() from 23 to 21 rows
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's test cases — edits: the status-line interception rows
@task_file n=8 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's typed probe runs — edits: Run B's status-line probe (the --settings typed/statusline.json whose status line is hook statusline --capture)
@task_file n=9 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's run definitions — edits: Run B's status-line --settings and capture
@task_file n=10 path=src/run/version_gate.rs mark=edits
what: the start's version gate over the capability ledger — edits: the statusline row group
@task_file n=11 path=src/cmd/run.rs mark=edits
what: the viola run start — edits: the statusline source read (STATUSLINE_SOURCE, statusline-source.json, the default home's statusLine), the instance settings.json override written every start, the snapshot's status-line recording, and their tests
@task_file n=12 path=src/run/mod.rs mark=edits
what: the run module's child argv and snapshot constructors — edits: SETTINGS_FLAG (the --settings argument to the claude child), the statusline_command literal
@task_file n=13 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot.json shape and its replay — edits: the statusline_command field and its tests
@task_file n=14 path=crates/viola-state/src/board.rs mark=edits
what: the state crate's board of instances — edits: the statusline_command literal in its constructors, the "statusline" literal (P3 reads what it is)
@task_file n=15 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status-line mode (statusline-echo, receipt kind statusline); deleted whole
@task_file n=16 path=src/bin/viola-fake-agent/test_cases.rs mark=edits
what: the fake agent's test cases — edits: the StatusLine framing replay, --settings with --statusline-stdin and statusline-echo rows
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot command — edits: the boot --statusline-echo option
@task_file n=18 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded claude StatusLine fixture of CLI 2.1.287; deleted
@task_file n=19 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded claude StatusLine fixture of CLI 2.1.302; deleted
@task_file n=20 path=schemas/claude-statusline-fixture.v1.json
what: the claude status-line fixture schema checked by fixture hygiene; deleted
@task_file n=21 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook-stdin fuzz corpus seed of a status-line payload; deleted
@task_file n=22 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook-stdin fuzz corpus seed of a status-line payload; deleted
@task_file n=23 path=schemas/diag-line.v1.json mark=edits
what: the diag-line v1 schema of every process log line — edits: the statusline-shell value of the subject enum
@task_file n=24 path=tests/hook_statusline.rs
what: the integration test file of the hook statusline arm; deleted
@task_file n=25 path=Cargo.toml mark=edits
what: the root crate manifest — edits: the [[test]] entry of hook_statusline
@task_file n=26 path=.config/nextest.toml mark=edits
what: the nextest configuration — edits: the hook_statusline filter term
@task_file n=27 path=tests/support/statusline.rs
what: the test-support helpers for the status line; deleted
@task_file n=28 path=tests/tui_passthrough.rs mark=edits
what: the run's terminal pass-through tests — edits: passthrough_with_a_statusline_source_adds_no_viola_bytes
@task_file n=29 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene contract test — edits: the status-line fixture schema validation
@task_file n=30 path=tests/contract_fake_agent_drift.rs mark=edits
what: the fake agent drift check against recorded fixtures — edits: the drift check over StatusLine variants
@task_file n=31 path=tests/contract_ledger_probes.rs mark=edits
what: the capability ledger probe contract test — edits: its status-line cases
@task_file n=32 path=tests/contract_diag_schema.rs mark=edits
what: the diag-line schema contract test — edits: its statusline-shell subject cases
@task_file n=33 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open tests — edits: its status-line cases
@task_file n=34 path=tests/cli_verify.rs mark=edits
what: viola verify's CLI tests — edits: its status-line probe cases
@task_file n=35 path=tests/support/fake.rs mark=edits
what: the test-support driver of the fake agent — edits: its status-line modes
@task_file n=36 path=crates/viola-state/tests/state_replay.rs mark=edits
what: the state crate's replay tests of snapshot.json — edits: its statusline_command cases
@task_item n=1 part=needs
The hidden viola hook statusline arm leaves whole: no status-line hook invocation, no status-line hook-decision detail codes (deadline, malformed-json, oversize-stdin), no status-line hook log fields, spans or deadline, no stdout pass-through of a status line; the hook role's stdout carries only the decision body of the hook arms that stay.
@task_item n=2 part=needs
viola runs no shell for any purpose: the status-line shell-out and its process-start and process-exit spawn lines leave, the diag-line subject value statusline-shell leaves the subject enum, and the span area statusline, the budget span attribute statusline.shell_out and the subject tag contract-hook-statusline go with it.
@task_item n=3 part=needs
The start's interception leaves: viola run reads no statusLine source, writes no instance settings.json override, passes no --settings argument to the claude child and records no status-line command; the child's settings argument and the status-line read and settings-write steps no longer exist to be logged or kept out of telemetry.
@task_item n=4 part=needs
The status-line command string as a telemetry subject leaves: every rule confining it to diagnostics, keeping it out of home-level logs, or observing it by duration and exit only describes a surface this chunk removes; the plan names those rows as expected amendments, amended at the wrap, not in the phase.
@task_item n=5 part=needs
The snapshot loses its statusline_command field, so a snapshot rebuilt by replay no longer carries it among its live fields; a snapshot or event log written by an older binary still reads (unknown fields skipped and counted, never denied), and the old-reader / new-file direction is measured at P3.
@task_item n=6 part=needs
Removing the statusline-shell subject value from the diag-line schema is a schema change read against the rule that the diagnostic log schema is additive only and the rule that schema words no build can emit are removed; whether every own format's v stays 1 is the founder's fork, not decided here.
@task_item n=7 part=needs
The capability ledger's two interception rows leave and relied() goes from 23 to 21; viola verify's Run B loses only its status-line probe and capture (its other pastes, probes and the claude agents --json read stay), and the start's version gate loses its statusline row group, so verify's step lines and summary count change by those rows.
@task_item n=8 part=needs
The fake agent loses its StatusLine framing replay, its --settings with --statusline-stdin, the statusline-echo mode and the receipt kind statusline, and the e2e harness loses boot --statusline-echo; the removal is proved by the fake agent in CI and the code's own tests, never by a real-CLI verify in CI.
@task_item n=9 part=needs
The StatusLine recorded fixtures, the claude status-line fixture schema with its hygiene validation, the two hook-stdin fuzz corpus seeds and the drift check over StatusLine variants leave; old shapes that a test must still spell live in fixture files outside the paths an identifier probe reads.
@task_item n=10 part=needs
Each test case that spells the interception is sorted at P3: it leaves where it tests the interception, or is rewritten where it tests something that stays; an acceptance probe over identifiers is checked at P4 against the plan's own test cases that must spell an old word.
@task_item n=11 part=needs
A guard test's one-off remove-the-guard control runs through gate.py control under a kill deadline, its case the test's name alone, holding only when that case ran and failed; no mutation run, local or dispatched.
@task_item n=12 part=needs
The chrono dependency, stated as for the statusline module, leaves only if nothing else uses it; the obs plan's millisecond UTC timestamps and its direct chrono dependency for timestamps are users P3 reads before deciding.
@task_item n=13 part=needs
The box-only screen fixture CARRY needs a clean viola verify --record, a live claude start: a named operator step waiting on a per-chunk cap, never planned as granted; whether --record reaches 0 fail on this account after the removal is measured at P3 without a live start or marked unmeasurable.
@task_item n=14 part=needs
The live home ~/.viola/, its instance settings.json files and statusline-source.json, the pinned 0.1.0 binary and the user's ~/.claude/settings.json are not touched: no install, re-pin, or write there; a live instance keeps its own override until a named operator step re-pins it.
@task_item n=15 part=needs
No code beyond the removal: no new settings, flag, env var, ledger row, log event, detail code, span or field; a later entry owns any viola-written settings for its sessions.
@task_item n=16 part=needs
The cargo build and test runs research needs are each named with their wall time on a shared host, a baseline needing a build fired once, never in parallel.
@task_standing n=1
convention: every integration or e2e test the chunk rewrites that emits process log lines has those lines schema-checked against the diag-line schema by the schema conformance gate
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every rewritten test's home lives under the one integration-test root, kept until the panic, schema and secret-scan gates run, and the zero-panics gate holds over it
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: a claude fixture recorded by viola verify --record (the box-only fixture, if the operator's cap comes) is scrub-required and is no telemetry source
home: 1-obs-scope-summary · obs-scope-summary
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames, stdout and refusals: the status line never reached the MCP server.
@task_item n=2 part=not_needed
The channel's JSON-RPC lines, params, sender trust, conn and corr join keys: the chunk changes no channel method or line.
@task_item n=3 part=not_needed
The send, wait, last, wheel and dialog paths' spans, log fields and refusal detail words: none of them carries the status line.
@task_item n=4 part=not_needed
Panic capture, the panic line, panic outcome by role and the detail-file error line: the chunk adds no catch site and changes no panic handling.
@task_item n=5 part=not_needed
The future error reporter, OTel, exporters, network egress and the pulse-class receiver absences: the chunk adds no telemetry destination.
@task_item n=6 part=not_needed
Heartbeat ticks, liveness states and the list --json liveness contract: the chunk does not touch liveness.
@task_item n=7 part=not_needed
Metric coverage (counters, gauges, histograms, closed-enum grouping): the chunk adds no metric and derives none.
@task_item n=8 part=not_needed
The revive preflight order and revive process-exit detail codes, link and unlink methods and their events: no status-line step in them.
@task_item n=9 part=not_needed
The hyprctl probe, the PTY seam's spawn, paste and kill-fallback log events, and vt100 containment: the chunk changes neither the PTY nor the desktop probe.
@task_item n=10 part=not_needed
Socket-directory and server-verification refusals, their exit codes and stderr pairs: the chunk changes no refusal path.
@task_item n=11 part=not_needed
Service identity (service name, version constant) and the process-start line's os and pid: unchanged by the removal.
