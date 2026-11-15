@task_prose part=chunk
Removes viola's status-line interception whole: the user's status-line command is run by the CLI exactly as unwrapped, and viola writes no status-line override, passes no --settings for it, records no status-line command, has no hook statusline arm and runs no shell for any purpose.
The capability ledger loses its two interception rows (statusline-override, statusline-shell), so relied rows go from 23 to 21, and verify's Run B loses its status-line probe and capture.
Everything that existed only to build, test, record, document or log the interception leaves with it: the pure statusline module, the start's source read and override, the snapshot field, the fake agent's status-line modes, fixtures, schema, fuzz seeds, tests and log subjects.
It re-reads the decision that viola depends on no tmux or shell and closes what it states once the shell-out is gone; it re-measures whether a verify --record run can reach 0 fail without a live start, and the old-reader / new-file compatibility of the removed snapshot field.
@task_file n=1 path=src/cmd/hook/statusline.rs mark=edits
what: the hook statusline arm: the instance check, the replay read, the /bin/sh -c run under the statusline deadline, the stdout pass-through (the file leaves)
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the hook command's dispatch over its arms and the hidden --capture probe form — edits: the statusline module declaration, the statusline deadline and its five-seconds test, the statusline word branch, the hook statusline --capture probe form with its four tests, the --shell argument doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs mark=edits
what: the pure statusline module of the Claude agent crate: the /bin/sh argv, the user command read, the override document, the user settings path, the settings flag literal, the statusline word, and their tests (the file leaves)
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Claude agent crate's module list — edits: pub mod statusline
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt mark=edits
what: the proptest regression seeds of the statusline module (the file leaves)
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the per-behaviour CLI capability ledger: its rows, their ids, claims and probes — edits: the StatusLineOverride and StatusLineShell rows in ALL, their ids and claims, the ShellVerdict, StatusLineCapture, statusline_variants and StatusLine.<k> helpers, relied() from 23 rows to 21
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the capability ledger's test cases — edits: its status-line rows
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs mark=edits
what: the capability ledger's per-run test cases — edits: its status-line rows
@task_file n=9 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's typed probe runs, Run B's probe settings — edits: the status-line probe --settings whose status line is hook statusline --capture
@task_file n=10 path=src/cmd/verify/typed/runs.rs mark=edits
what: viola verify's five runs and their probe captures — edits: Run B's status-line probe and capture
@task_file n=11 path=src/run/version_gate.rs mark=edits
what: the version gate over the ledger's relied rows read from the stamps — edits: the statusline row group
@task_file n=12 path=src/cmd/run.rs mark=edits
what: the start path of viola run — edits: the statusline source read (statusline-source.json, the user settings statusLine for the default home), the instance settings.json override written every start, the snapshot recording of the status-line command, and their tests
@task_file n=13 path=src/run/mod.rs mark=edits
what: the wrapper run module and the child's argv — edits: the --settings flag constant, the statusline_command: None literal
@task_file n=14 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot record — edits: the statusline_command field and its tests
@task_file n=15 path=crates/viola-state/src/board.rs mark=edits
what: the board read model built from the snapshots — edits: the statusline_command: None constructor literal, the "statusline" literal
@task_file n=16 path=src/bin/viola-fake-agent/statusline.rs mark=edits
what: the fake agent's status-line mode (the file leaves)
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_modes.rs mark=edits
what: the fake agent's mode test cases — edits: the statusline-echo mode and statusline receipt kind rows
@task_file n=18 path=fixtures/claude/2.1.287/StatusLine.default-1.json mark=edits
what: a recorded StatusLine fixture of the claude CLI 2.1.287 (the file leaves)
@task_file n=19 path=fixtures/claude/2.1.302/StatusLine.default-1.json mark=edits
what: a recorded StatusLine fixture of the claude CLI 2.1.302 (the file leaves)
@task_file n=20 path=schemas/claude-statusline-fixture.v1.json mark=edits
what: the status-line fixture schema the fixture hygiene validation reads (the file leaves)
@task_file n=21 path=schemas/diag-line.v1.json mark=edits
what: the diagnostics line schema and its subject values — edits: the statusline-shell subject value
@task_file n=22 path=fuzz/corpus/hook_stdin/statusline-absent mark=edits
what: a hook stdin fuzz corpus seed for the status line (the file leaves)
@task_file n=23 path=fuzz/corpus/hook_stdin/statusline-epoch mark=edits
what: a hook stdin fuzz corpus seed for the status line (the file leaves)
@task_file n=24 path=tests/hook_statusline.rs mark=edits
what: the hook statusline integration test binary (the file leaves)
@task_file n=25 path=tests/support/statusline.rs mark=edits
what: the test support helper for the status line (the file leaves)
@task_file n=26 path=Cargo.toml mark=edits
what: the root crate manifest and its test targets — edits: the hook_statusline [[test]] entry
@task_file n=27 path=.config/nextest.toml mark=edits
what: the nextest configuration and its test filters — edits: the hook_statusline filter term
@task_item n=1 part=needs
The hook statusline arm leaves: the hook-to-kind map loses its hook statusline entry, and every viola hook statusline contract (stdout, exit status, refused instance check, --capture) and the statusline hook-output carve-outs leave with it; the other hook arms and the hidden hook <event> --capture and --answers probe forms stay.
@task_item n=2 part=needs
viola runs no shell for any purpose: the status-line shell-out was the only shell-out, so the rule that viola depends on no tmux or shell and the no-shell-out rule are re-read and closed with no exception left.
@task_item n=3 part=needs
The status-line command deadline, the statusline instance check before the log and the refused statusline check that leaves no trace all leave with the arm.
@task_item n=4 part=needs
The start no longer reads a statusline source (the user's settings statusLine for the default home, statusline-source.json in the viola home) and no longer writes the per-session instance settings.json override; the statusline source precedence and source-follows-home rules leave.
@task_item n=5 part=needs
The claude child's argv loses the --settings flag for the status line; the safe-pinned-path-only rule for the settings override leaves; the caller's arguments, --plugin-dir, --resume and --fork-session flags are unchanged.
@task_item n=6 part=needs
The user's status-line command and the user's global settings stay untouched: viola reads none, writes none, runs none, and adds no viola-written settings of its own (a later entry owns product-owned settings).
@task_item n=7 part=needs
The instance snapshot loses its statusline_command field; readers of an older snapshot or event log holding it keep reading by the tolerant, unknown-field-skipping own-format rule, never deny_unknown_fields; the old-reader / new-file direction is measured at P3.
@task_item n=8 part=needs
The version of every own format (snapshot envelope v, event line, stamps, diag line) is not decided here: whether the removal keeps v at 1 is the founder's fork, put on the away form with the measured compatibility facts.
@task_item n=9 part=needs
The capability ledger loses the statusline-override and statusline-shell rows, their probes, claims and helpers; relied() goes from 23 to 21; a CLI version is verified only when all 21 relied rows pass, and the version gate's strict stamps read drops the statusline row group; no ledger row is added.
@task_item n=10 part=needs
viola verify keeps its five runs; Run B keeps its other pastes and probes and its claude agents listing read and loses only the status-line probe settings and capture; the StatusLine.<k>.shell capture file and its verdict leave; P3 measures whether Run B passes --settings for anything else.
@task_item n=11 part=needs
Fixtures are recorded only from a clean verify at 0 fail and never hand-written: the box-only screen fixture CARRY waits on a named operator step (a live claude start under a per-chunk cap) and the chunk closes without it; whether --record can reach 0 fail now is P3's to measure without a live start or mark unmeasurable.
@task_item n=12 part=needs
The fake agent loses its StatusLine framing replay, its --settings and --statusline-stdin options, the statusline-echo mode and the statusline receipt kind, and the e2e harness loses boot --statusline-echo; the fake-agent drift check over StatusLine variants and the fixture hygiene validation of the status-line schema leave.
@task_item n=13 part=needs
The recorded StatusLine fixtures, the status-line fixture schema and the hook stdin fuzz corpus seeds for the status line leave; the fuzz corpus replay in CI keeps passing over the remaining seeds.
@task_item n=14 part=needs
Log subjects leave: the diag-line subject statusline-shell, the statusline span area, the contract-hook-statusline subject tag and the shell-spawn process-start and exit lines.
@task_item n=15 part=needs
The Claude flag literals stay in the agent crate: the settings flag literal leaves with the statusline module, and the agent crate keeps doing no file I/O; the chrono dependency leaves only if nothing else uses it, P3 reads its users.
@task_item n=16 part=needs
No orphan module may remain: the module declarations, the [[test]] entry and the nextest filter term of every removed file leave with it, and the build keeps formatting and clippy clean with warnings denied.
@task_item n=17 part=needs
Each test case that spells the interception is sorted at P3: it leaves where it tests the interception, or is rewritten where it tests something that stays; an acceptance probe over identifiers is checked against the plan's own test cases that must spell an old word, the old shapes living in fixture files outside the probed paths.
@task_item n=18 part=needs
The retired-words fixture stays; whether the removed words join it is decided by its rule, which P3 reads.
@task_item n=19 part=needs
A guard test's one-off remove-the-guard control runs through gate.py control under a kill deadline, its case the test's name alone, holding when that case ran and failed; no mutation run, local or dispatched.
@task_item n=20 part=needs
No install, re-pin or write under the live viola home or to the user's claude settings, and no live claude start or live desktop step; a live instance started by the pinned 0.1.0 binary keeps its own override until a re-pinned start, a named operator step.
@task_item n=21 part=needs
No new settings, flag, environment variable or ledger row; masters' and harness files' amendments (docs, rules, the verify wireframe rows and ledger order) are the wrap's, named as expected amendments.
@task_standing n=1
convention: each rewritten or new test runs in its own isolated viola home
home: established-decisions
@task_standing n=2
convention: each test runs against the fake agent in CI, never a live claude
home: ci-cd-approach
@task_standing n=3
convention: an OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The MCP tool surface (tools, results, refusals, stdin caps, exit codes): the removal reaches no MCP tool or its results.
@task_item n=2 part=not_needed
The wheel and human-take rules, send confirmation and paste rules: no send, wheel move or typed text changes.
@task_item n=3 part=not_needed
Link and unlink recording, channel link methods and their events: untouched.
@task_item n=4 part=not_needed
Dialog hold, answer responses and dialog hook decisions: the dialog arms stay as they are.
@task_item n=5 part=not_needed
The Hyprland window seam and its errors: no compositor call is touched.
@task_item n=6 part=not_needed
Revive refusals and revive --json error codes, frame-verb exit codes and the exit-21 documents: no refusal or exit code changes.
@task_item n=7 part=not_needed
Release, dist, deploy and the weekly advisory and fuzzing schedules: no release or workflow change.
@task_item n=8 part=not_needed
Endpoint naming, socket directory checks and peer euid admission: the channel and its endpoints are untouched.
@task_item n=9 part=not_needed
config.json keys, colour environment variables and the plugin install marketplace rewrite: none changes.
