@task_prose part=chunk
Removes viola's interception of the wrapped CLI's status line whole: the user's status-line command runs untouched.
Leaves: the `hook statusline` arm and its shell-out, the start's settings override and statusline source read, the snapshot's statusline command field, the two ledger rows statusline-override and statusline-shell (relied rows 23 to 21), the fake agent's status-line modes, fixtures, schema, fuzz seeds, tests and log subjects.
Rewrites every test case that only spells the interception while testing something that stays; adds no code beyond the removal, no settings, flag, env var or ledger row.
Re-reads the architecture rule that viola runs no shell and closes it once the shell-out is gone; measures old-reader / new-file and new-reader / old-file compatibility of the snapshot and event log.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the `hook statusline` arm — instance check, replay read, `/bin/sh -c` run under the status-line deadline, stdout pass-through — deleted whole
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the viola hook exec-form entry points and their dispatch — edits: the statusline module declaration, the STATUSLINE_DEADLINE constant, the statusline word dispatch branch, the statusline_deadline_is_five_seconds test, the hidden capture_statusline probe form and its four tests, the --shell argument doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs
what: the pure statusline module of the claude agent crate — BIN_SH, shell_argv, user_command, override_document, USER_SETTINGS, SETTINGS_FLAG, WORD and their tests — deleted whole
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the claude agent crate's module list — edits: the public statusline module declaration
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the committed proptest seeds of the statusline module, deleted
@task_file n=6 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's typed probe runs, Run B — edits: the status-line probe settings document and its capture
@task_file n=7 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's typed run definitions — edits: Run B's status-line probe and capture
@task_file n=8 path=src/run/version_gate.rs mark=edits
what: the unverified-CLI version gate over the relied ledger rows — edits: the statusline row group
@task_file n=9 path=src/cmd/run.rs mark=edits
what: the viola run start — edits: the statusline source read, the instance settings.json status-line override written every start, the snapshot's statusline command recording, and their tests
@task_file n=10 path=src/run/mod.rs mark=edits
what: the run module's start constants and snapshot constructors — edits: SETTINGS_FLAG, the statusline_command literal
@task_file n=11 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot.json shape — edits: the statusline_command field and its tests
@task_file n=12 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status-line mode (statusline-echo, --statusline-stdin, StatusLine framing replay), deleted whole
@task_file n=13 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: the recorded StatusLine fixture of CLI version 2.1.287, deleted
@task_file n=14 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: the recorded StatusLine fixture of CLI version 2.1.302, deleted
@task_file n=15 path=schemas/claude-statusline-fixture.v1.json
what: the schema of the recorded status-line fixtures, deleted with its hygiene validation
@task_file n=16 path=fuzz/corpus/hook_stdin/statusline-absent
what: a hook stdin fuzz corpus seed for the status line, deleted
@task_file n=17 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a hook stdin fuzz corpus seed for the status line, deleted
@task_file n=18 path=tests/hook_statusline.rs
what: the hook statusline integration test binary, deleted whole
@task_file n=19 path=Cargo.toml mark=edits
what: the root package manifest — edits: the hook_statusline test target entry
@task_file n=20 path=.config/nextest.toml mark=edits
what: the nextest configuration and its test-layer filtersets — edits: the hook_statusline filter term
@task_file n=21 path=tests/support/statusline.rs
what: the shared test helper for the status line, deleted whole
@task_file n=22 path=tests/support/fake.rs mark=edits
what: the shared fake agent test helpers — edits: their statusline cases
@task_file n=23 path=tests/support/home.rs mark=edits
what: the shared per-test home helpers — edits: their statusline cases
@task_file n=24 path=tests/support/mod.rs mark=edits
what: the shared test support module list — edits: the statusline helper module
@task_file n=25 path=tests/support/verify.rs mark=edits
what: the shared verify test helpers — edits: their statusline cases
@task_file n=26 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics line schema — edits: the subject value statusline-shell
@task_file n=27 path=tests/tui_passthrough.rs mark=edits
what: the passthrough tests through an outer PTY — edits: the passthrough_with_a_statusline_source_adds_no_viola_bytes case
@task_file n=28 path=crates/viola-state/tests/state_replay.rs mark=edits
what: the snapshot replay tests of the state crate — edits: their statusline command cases
@task_file n=29 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI tests — edits: their status-line probe cases
@task_file n=30 path=tests/contract_ledger_probes.rs mark=edits
what: the ledger probe contract suite — edits: the statusline-override and statusline-shell probe cases
@task_file n=31 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene walk tests — edits: the statusline fixture schema validation
@task_file n=32 path=tests/contract_fake_agent_drift.rs mark=edits
what: the fake agent drift check against recorded fixtures — edits: the drift check over StatusLine variants
@task_file n=33 path=tests/contract_diag_schema.rs mark=edits
what: the diagnostics schema conformance tests — edits: their statusline-shell subject cases
@task_file n=34 path=tests/cli_fake_agent.rs mark=edits
what: the fake agent command line tests — edits: their statusline mode and receipt cases
@task_file n=35 path=tests/path1_run_start.rs mark=edits
what: the run start critical path tests — edits: their settings override and statusline source cases
@task_file n=36 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open tests — edits: their statusline cases
@task_file n=37 path=tests/contract_os_gated_tests.rs mark=edits
what: the OS-gated test register check — edits: its statusline entries
@task_file n=38 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded external readers check — edits: its statusline reader cases
@task_file n=39 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negative tests — edits: their statusline command cases
@task_file n=40 path=fixtures/viola-0.1.0/retired-words.json
what: the retired-words committed fixture, which the plan decides whether the removed words join
@task_item n=1 part=needs
The `viola hook statusline` entry point leaves whole: no dispatch arm, no status-line deadline, no hidden statusline capture probe form; every other hook arm and the hidden event capture and answers probe forms stay as they are.
@task_item n=2 part=needs
viola runs no shell for any purpose: the `/bin/sh -c` status-line shell-out, its bound and its frame cap leave, and the rule that viola has no shell dependency is closed once it is gone.
@task_item n=3 part=needs
The start writes no status-line override into the instance settings file, passes no settings argument for it and reads no status-line source (neither the user's settings statusLine nor a statusline source file); the plugin is still rewritten on start; the user's own settings file is never written.
@task_item n=4 part=needs
The instance snapshot loses its statusline command field and every constructor literal of it; a snapshot or event log written by an older binary keeps reading because readers skip and count unknown fields, never deny them; the snapshot replay field set no longer holds the field.
@task_item n=5 part=needs
The ledger loses its statusline-override and statusline-shell rows: relied rows go from 23 to 21, the version gate needs exactly the 21 relied rows, every remaining ledger row keeps its verify probe, and stamps keyed to the old ledger are read under the stamps-refuse-widened-ledger rule.
@task_item n=6 part=needs
viola verify's Run B keeps its other pastes and probes and its agents listing read; only its status-line probe settings document and capture leave; whether Run B still passes a settings argument for anything else is measured.
@task_item n=7 part=needs
The fake agent loses its status-line surface: the StatusLine framing replay, the settings and statusline-stdin options, the statusline-echo mode, the statusline receipt kind and its status line command execution; the harness boot loses its statusline-echo option.
@task_item n=8 part=needs
The recorded StatusLine fixtures of both CLI versions, their fixture schema and its hygiene validation, and the fake agent drift check over StatusLine variants leave; every remaining CLI-version fixture set is still replayed and still matched byte for byte.
@task_item n=9 part=needs
The hook stdin fuzz corpus loses its two status-line seeds; the corpus replay of that target must stay non-vacuous.
@task_item n=10 part=needs
Log subjects leave: the diagnostics subject statusline-shell, the statusline span area, the contract-hook-statusline subject tag and the shell-spawn process-start and process-exit lines; diagnostics still conform to the diagnostics line schema.
@task_item n=11 part=needs
Every test case that spells the interception is sorted: it leaves if it tests the interception, or is rewritten if it tests something that stays; the hook statusline test binary leaves with its manifest target and nextest filter term, and the shared helpers lose their status-line parts.
@task_item n=12 part=needs
The statusline command gates (strict home, after strict modes, never over unsafe state) and the statusline settings source rules leave with the command they gated.
@task_item n=13 part=needs
A guard test's one-off remove-the-guard control runs bounded under a kill deadline, its case named by the test's name alone, reaching the case past fail-fast; it holds only when that case ran and failed; no mutation run, local or dispatched.
@task_item n=14 part=needs
An acceptance probe over the removed identifiers is checked against the plan's own test cases that must still spell an old word; old shapes live in fixture files outside the probed paths, and the retired-words fixture may take the removed words.
@task_item n=15 part=needs
The own-format version field of the snapshot, the event log and the diagnostics line is not decided by this chunk: it is the founder's fork, laid with the measured compatibility in both directions.
@task_item n=16 part=needs
No live claude start, no install, no re-pin, no write under the live viola home or to the user's settings file; a box-only screen fixture is recorded only by a clean verify record run at zero failing rows, never hand-written, as a named operator step waiting on a per-chunk cap; the chunk closes without it.
@task_item n=17 part=needs
The box-only screen fixture, if recorded, holds the input box's rule and prompt rows as literals with no footer literal, is scrubbed, and is replayed by the fake agent in CI.
@task_item n=18 part=needs
The chrono dependency leaves only if nothing but the removed module uses it; any dependency removal stays inside the supply-chain gate.
@task_item n=19 part=needs
Test-plan rows that rule over the interception (statusline passthrough, shell-out cap, instance settings override, statusline source, fake agent statusline receipt and hook, harness boot statusline-echo) are expected amendments the plan names; their amendment is the wrap's.
@task_standing n=1
convention: every new or rewritten test function follows the test function naming rule
home: 2-test-strategy
@task_standing n=2
convention: every test creates its own private home and seeds its own data, never reading the host's claude sessions
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: test waits key on logged events, with no retries and no sleep synchronisation
home: 2-test-strategy
@task_standing n=4
convention: test oracles and human-mode expected output are literals, never compared by Debug string
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each test helper exists once in the shared support, never copied
home: test-strategy
@task_standing n=6
convention: fixtures carry synthetic text only, free of real data and recorded rather than hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_item n=1 part=not_needed
MCP tool surfaces: tools/list, MCP invalid-params fields, MCP tool results and refusal bodies, MCP stdin line cap; the chunk touches no MCP tool.
@task_item n=2 part=not_needed
The wrapper channel's JSON-RPC errors, link and unlink methods and their event data; no channel method changes.
@task_item n=3 part=not_needed
Send confirmation, driver send, local-command matching and send refusal order; sending is untouched.
@task_item n=4 part=not_needed
Dialog hook decision bodies, dialog answer mapping, dialog hold and expiry; the dialog arms stay as they are.
@task_item n=5 part=not_needed
Revive refusal codes, revive exit codes and revive JSON documents.
@task_item n=6 part=not_needed
Heartbeat staleness classification and instance liveness.
@task_item n=7 part=not_needed
The wheel: human input taking the wheel, release, pause and the harness-injected turn.
@task_item n=8 part=not_needed
viola wait and viola last, their wake kinds and timeouts.
@task_item n=9 part=not_needed
The pre-push gate's stages, details and reasons, and the secret-scan hit classes and outputs.
@task_item n=10 part=not_needed
Harness status document, status states and session.json pid fields.
@task_item n=11 part=not_needed
Performance budgets of the spine, pre-tool-use and session-end hooks and the hyperfine invocation.
@task_item n=12 part=not_needed
The viola list board's columns and list JSON fields.
