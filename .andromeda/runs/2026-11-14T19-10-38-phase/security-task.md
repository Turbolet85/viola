@task_prose part=chunk
A REMOVAL: viola stops intercepting the wrapped CLI's status line; the user's own status-line command runs exactly as unwrapped, and viola reads none, writes none and runs none.
Leaving whole: the hidden `hook statusline` arm with its `/bin/sh -c` shell-out, deadline and `--capture`/`--shell` probe form; the pure statusline module; the start's statusline source read, instance settings override and `--settings` argument; the snapshot's `statusline_command` field; the fake agent's status-line modes; the StatusLine fixtures, schema, fuzz seeds, tests and the `statusline-shell` log subject.
The ledger loses `statusline-override` and `statusline-shell` (relied rows 23 to 21), with their Run B probe in viola verify and the version gate's statusline row group.
It re-reads the decision that viola has no tmux or shell dependency and closes what it states once the shell-out is gone; it re-measures whether Run B passes `--settings` for anything else, whether `chrono` has other users, and both compatibility directions of the removed field (new reader on old files, old reader on new files).
@task_file n=1 path=src/cmd/hook/statusline.rs mark=edits
what: the hook statusline arm: the instance check, the replay read, the statusline shell-out under the statusline deadline and the stdout pass-through (deleted whole)
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the viola hook dispatch over its arms, deadlines and hidden capture probe forms — edits: the statusline module line, the statusline deadline and its five-second test, the statusline dispatch branch, the statusline capture arm and its four tests, the `--shell` arg doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the pure statusline module: the shell argv, the user-command read, the override document, the user settings path and the settings flag (deleted whole)
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the agent-claude crate root — edits: the public statusline module declaration
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt mark=edits
what: the statusline module's proptest seeds (deleted whole)
@task_file n=6 path=crates/viola-agent-claude/Cargo.toml mark=edits
what: the agent-claude crate's dependency list — edits: the chrono dependency, only if nothing else uses it
@task_file n=7 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the CLI ledger of relied rows and their probes — edits: the StatusLineOverride and StatusLineShell rows in ALL, their ids and claims, the ShellVerdict, StatusLineCapture, statusline_variants and StatusLine helpers, relied()
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the ledger's test cases — edits: the statusline rows
@task_file n=9 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's typed probes in Run B — edits: the status-line probe `--settings` and its capture
@task_file n=10 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's runs — edits: Run B's status-line probe and capture
@task_file n=11 path=src/run/version_gate.rs mark=edits
what: viola run's version gate over the verify stamps — edits: the statusline row group
@task_file n=12 path=src/cmd/run.rs mark=edits
what: viola run's start — edits: the statusline source read, the instance settings override written every start, the snapshot recording of the statusline command, and their tests
@task_file n=13 path=src/run/mod.rs mark=edits
what: the run module's child argv and snapshot constructors — edits: the settings flag, the statusline_command literal
@task_file n=14 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot format — edits: the statusline_command field and its tests
@task_file n=15 path=crates/viola-state/src/board.rs mark=edits
what: the board read over instance snapshots — edits: the statusline_command literal, the board's "statusline" literal
@task_file n=16 path=src/bin/viola-fake-agent/statusline.rs mark=edits
what: the fake agent's status-line mode: the StatusLine replay, the settings and statusline-stdin options, the statusline-echo receipt (deleted whole)
@task_file n=17 path=src/bin/viola-fake-agent/test_cases.rs mark=edits
what: the fake agent's test cases — edits: the statusline rows
@task_file n=18 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot — edits: the boot statusline-echo option
@task_file n=19 path=fixtures/claude/2.1.287/StatusLine.default-1.json mark=edits
what: a recorded StatusLine screen fixture of the real CLI (deleted whole)
@task_file n=20 path=fixtures/claude/2.1.302/StatusLine.default-1.json mark=edits
what: a recorded StatusLine screen fixture of the real CLI (deleted whole)
@task_file n=21 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the statusline fixture schema checked by the fixture hygiene validation (deleted whole)
@task_file n=22 path=fuzz/corpus/hook_stdin/statusline-absent mark=edits
what: a hook stdin fuzz corpus seed for the statusline payload (deleted whole)
@task_file n=23 path=fuzz/corpus/hook_stdin/statusline-epoch mark=edits
what: a hook stdin fuzz corpus seed for the statusline payload (deleted whole)
@task_file n=24 path=tests/hook_statusline.rs mark=edits
what: the hook statusline contract tests (deleted whole)
@task_file n=25 path=Cargo.toml mark=edits
what: the workspace manifest's test targets — edits: the hook_statusline test target
@task_file n=26 path=.config/nextest.toml mark=edits
what: the nextest profile filters — edits: the hook statusline filter term
@task_file n=27 path=tests/support/statusline.rs mark=edits
what: the test support helpers for the status line (deleted whole)
@task_file n=28 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics line schema — edits: the statusline-shell subject value
@task_file n=29 path=tests/contract_diag_schema.rs mark=edits
what: the diagnostics line schema contract tests — edits: the statusline cases
@task_file n=30 path=tests/tui_passthrough.rs mark=edits
what: the terminal passthrough tests — edits: the passthrough with a statusline source test
@task_file n=31 path=tests/contract_fixture_hygiene.rs mark=edits
what: the recorded fixture hygiene walk — edits: the statusline schema validation and statusline cases
@task_file n=32 path=tests/contract_fake_agent_drift.rs mark=edits
what: the fake agent drift check against recorded fixtures — edits: the drift check over StatusLine variants
@task_file n=33 path=tests/contract_ledger_probes.rs mark=edits
what: the ledger probe contract tests — edits: the statusline cases
@task_file n=34 path=tests/cli_verify.rs mark=edits
what: the viola verify CLI tests — edits: the statusline cases
@task_file n=35 path=tests/path1_run_start.rs mark=edits
what: the viola run start tests — edits: the statusline cases
@task_file n=36 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negative tests — edits: the statusline cases
@task_file n=37 path=tests/hook_fail_open.rs mark=edits
what: the hook fail-open tests — edits: the statusline cases
@task_file n=38 path=tests/contract_bounded_readers.rs mark=edits
what: the bounded readers census check — edits: the statusline cases
@task_file n=39 path=crates/viola-state/tests/state_replay.rs mark=edits
what: the state replay tests over old snapshots and events — edits: the statusline cases
@task_file n=40 path=crates/viola-mcp/tests/handlers.rs mark=edits
what: the MCP tool handler tests — edits: the statusline_command literal in snapshot constructors
@task_item n=1 part=needs
The status-line shell-out leaves: after the chunk viola runs no shell for any purpose, so every rule that names the statusline command as the one shell-out, its deadline, its trust through strict modes or its strict home loses its subject, and the no-shell-dependency decision is re-read and closed.
@task_item n=2 part=needs
The hidden statusline capture probe form and the statusline payload's silent-failure handling leave with the hook arm; the hook's other arms, the dialog arms and the hidden event capture and answers probe forms stay as they are.
@task_item n=3 part=needs
viola run's start no longer reads a statusline source (the user's Claude settings statusLine or a source file), writes no instance settings override and passes no settings argument to the child; nothing is written under the Claude home or to the user's settings file.
@task_item n=4 part=needs
The settings override that existed only for a plain pinned path leaves; the chunk adds no settings of its own (a later entry owns product-owned settings).
@task_item n=5 part=needs
The snapshot loses its statusline_command field: a replayed snapshot still runs nothing, and a snapshot or event log written by an older binary keeps reading because viola's readers skip and count unknown fields; the old-reader direction on new files is measured.
@task_item n=6 part=needs
The ledger loses the status-line override and shell rows (21 relied rows left) and viola verify's Run B loses its status-line probe settings and capture; Run B's other pastes, probes and its claude agents read stay, and verify still reads only signatures and spawns its children directly.
@task_item n=7 part=needs
The version gate's statusline row group leaves; the gate still reads the stamps only through the strict-modes check.
@task_item n=8 part=needs
The fake agent's StatusLine replay, settings and statusline-stdin options, statusline-echo mode and its receipt kind leave, with the harness's boot option; the remaining fake-agent seams stay behind the test-only feature and out of the release build.
@task_item n=9 part=needs
The recorded StatusLine fixtures, their schema and the drift check over StatusLine variants leave; the fixture hygiene walk keeps covering every remaining recorded fixture, and no fixture is hand-written.
@task_item n=10 part=needs
The hook stdin fuzz seeds for the statusline payload leave; the hook stdin parser stays in fuzz scope with its other seeds.
@task_item n=11 part=needs
The log subjects of the interception leave: the diagnostics subject statusline-shell, the statusline span area, the hook statusline contract tag and the shell-spawn process-start and exit lines; the never-log floors over the statusline command lose their subject and every other never-log rule holds.
@task_item n=12 part=needs
The bounded readers census loses the statusline replay read and the shell child output read; every remaining product reader stays bounded or named.
@task_item n=13 part=needs
The rule that hook, MCP and statusline commands use an absolute viola path loses its statusline clause; hook and MCP commands keep it.
@task_item n=14 part=needs
The chunk adds no new settings, flag, environment variable, ledger row or seam; no configuration or flag gains a way to switch a control off.
@task_item n=15 part=needs
The chrono dependency leaves only if nothing else uses it; the dependency graph stays on exact pins from crates.io.
@task_item n=16 part=needs
Each test case that spells the interception is sorted: it leaves if it tests the interception, or is rewritten if it tests something that stays; a rewritten case that must spell an old word keeps that word in fixture files outside the probed paths, and the retired-words fixture decides whether the removed words join it.
@task_item n=17 part=needs
A guard test's one-off remove-the-guard control runs through the gate's control under a kill deadline, its case the test's name alone, and it holds only when that case ran and failed.
@task_item n=18 part=needs
No install, re-pin or write under the live viola home, and no live claude start: the pinned binary and the live instances' existing overrides are untouched, and the box-only screen fixture recording is a named operator step that waits.
@task_item n=19 part=needs
The version field of every own format whose field or subject is removed is not decided here: it is the founder's fork, carried with the measured compatibility facts in both directions.
@task_standing n=1
convention: tests run against their own home, never the user's live viola home or Claude settings
home: secret-management
@task_standing n=2
convention: test seams live only under the fake-agent feature and are compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: recorded fixtures stay under the hygiene walk, scrubbed or refused whole
home: security-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The MCP tools and their inputs, structured content and isError results: the chunk changes no MCP tool.
@task_item n=2 part=not_needed
The wrapper channel methods and frames (send, answer, wait, last, pause, release, link, unlink) and their error codes: no channel method or frame changes.
@task_item n=3 part=not_needed
Socket directory, socket mode, peer euid and server verification rules and their refusal contracts: the channel endpoint is untouched.
@task_item n=4 part=not_needed
Paste text, dialog free text and send refusal character-set rules: typed text handling is untouched.
@task_item n=5 part=not_needed
viola revive's arguments and json codes and the revive list document: revive changes only by a dropped snapshot literal, not by behaviour.
@task_item n=6 part=not_needed
Plugin install, marketplace tree and live plugin-install round rows: the plugin surface is untouched.
@task_item n=7 part=not_needed
The Hyprland recorder and hyprctl rows: no window-manager surface is touched.
@task_item n=8 part=not_needed
GitHub Actions pinning, workflow permissions, workflow audit, release attestation and release-stage rows: no workflow or release stage changes.
@task_item n=9 part=not_needed
The rmcp feature set and its advisory ignores: no MCP server dependency changes.
@task_item n=10 part=not_needed
The absences (hosting, public listener, outbound network, database, webhook, file upload, accounts, secrets, regulated personal data): none of them is introduced or touched.
@task_item n=11 part=not_needed
The parent-identity and CLAUDE credential environment strip and the pre-push empty-environment launch: the child environment is unchanged.
@task_item n=12 part=not_needed
Link lines in the event log and the link and unlink verbs' exit codes: linking is untouched.
