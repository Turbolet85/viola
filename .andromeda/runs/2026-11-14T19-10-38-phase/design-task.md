@task_prose part=chunk
viola stops intercepting the wrapped CLI's status line: the user's status-line command runs exactly as it does unwrapped, viola reads none, writes none, runs none.
It removes the hidden `hook statusline` arm and its `--capture`/`--shell` probe form, the pure status-line module and its shell-out, the start's settings override and `--settings` argument, the snapshot's recorded status-line command, the fake agent's status-line modes, the fixtures, schema, seeds and log subjects that served it.
The ledger loses its two interception rows (`statusline-override`, `statusline-shell`) and their probe in `viola verify`'s Run B, so the relied rows go from 23 to 21; Run B's other probes stay.
It re-reads the architecture decision on no tmux or shell dependency and closes what it states once the shell-out is gone; it re-measures whether a `viola verify --record` run can reach 0 fail without a live start, and the box-only screen fixture stays a carry unless a named operator step records it.
Each test case that spells the interception is sorted: it leaves, or it is rewritten to test only what stays.
@task_file n=1 path=src/cmd/hook/statusline.rs
what: the `hook statusline` arm — the instance check, the replay read, the shell run under its deadline, the stdout pass-through; leaves whole
@task_file n=2 path=src/cmd/hook.rs mark=edits
what: the hook verb's arm dispatch and its deadlines — edits: the `mod statusline` declaration, the `STATUSLINE_DEADLINE` constant, the `statusline::WORD` branch, the `statusline_deadline_is_five_seconds` test, the hidden `capture_statusline` probe form and its four tests, the `--shell` arg doc
@task_file n=3 path=crates/viola-agent-claude/src/statusline.rs
what: the pure status-line module — the `/bin/sh -c` argv, the user command read, the override document, the settings flag and word; leaves whole
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the agent crate's module list — edits: the `pub mod statusline` line
@task_file n=5 path=crates/viola-agent-claude/proptest-regressions/statusline.txt
what: the status-line module's proptest seeds; leave whole
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the verification ledger of relied rows — edits: the `StatusLineOverride` and `StatusLineShell` rows in `ALL`, their ids and claims, the `ShellVerdict`, `StatusLineCapture`, `statusline_variants` and `StatusLine.<k>` helpers
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases.rs mark=edits
what: the ledger's test cases — edits: the status-line rows' cases
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs mark=edits
what: the ledger's run test cases — edits: the status-line rows' cases
@task_file n=9 path=crates/viola-agent-claude/Cargo.toml mark=edits
what: the agent crate's manifest — edits: the `chrono` dependency, only if nothing else uses it
@task_file n=10 path=src/cmd/verify/typed.rs mark=edits
what: `viola verify`'s typed runs — edits: Run B's status-line probe `--settings` and its capture
@task_file n=11 path=src/cmd/verify/typed/runs.rs mark=edits
what: `viola verify`'s run definitions — edits: Run B's status-line probe and capture
@task_file n=12 path=src/run/version_gate.rs mark=edits
what: the version gate over the ledger's rows — edits: the statusline row group
@task_file n=13 path=src/cmd/run.rs mark=edits
what: the `viola run` start — edits: the status-line source read, the instance settings override written every start, the snapshot recording of the status-line command, and their tests
@task_file n=14 path=src/run/mod.rs mark=edits
what: the start's shared run module — edits: the `SETTINGS_FLAG` argument and the `statusline_command: None` literals
@task_file n=15 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot record — edits: the `statusline_command` field and its tests
@task_file n=16 path=src/bin/viola-fake-agent/statusline.rs
what: the fake agent's status-line replay; leaves whole
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_modes.rs mark=edits
what: the fake agent's mode test cases — edits: the `statusline-echo` mode and `statusline` receipt kind cases
@task_file n=18 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness's boot — edits: the `boot --statusline-echo` option
@task_file n=19 path=fixtures/claude/2.1.287/StatusLine.default-1.json
what: a recorded status-line fixture; leaves
@task_file n=20 path=fixtures/claude/2.1.302/StatusLine.default-1.json
what: a recorded status-line fixture; leaves
@task_file n=21 path=schemas/claude-statusline-fixture.v1.json
what: the status-line fixture schema; leaves with its hygiene validation
@task_file n=22 path=fuzz/corpus/hook_stdin/statusline-absent
what: a fuzz corpus seed for the hook's stdin reader; leaves
@task_file n=23 path=fuzz/corpus/hook_stdin/statusline-epoch
what: a fuzz corpus seed for the hook's stdin reader; leaves
@task_file n=24 path=schemas/diag-line.v1.json mark=edits
what: the diag-line log schema — edits: the `statusline-shell` subject value
@task_file n=25 path=tests/hook_statusline.rs
what: the `hook statusline` integration tests; leave whole
@task_file n=26 path=tests/support/statusline.rs
what: the test support for status-line cases; leaves whole
@task_file n=27 path=Cargo.toml mark=edits
what: the workspace manifest — edits: the `[[test]]` entry for the hook status-line tests
@task_file n=28 path=.config/nextest.toml mark=edits
what: the nextest config — edits: the status-line filter term
@task_file n=29 path=tests/tui_passthrough.rs mark=edits
what: the terminal passthrough tests — edits: `passthrough_with_a_statusline_source_adds_no_viola_bytes`
@task_file n=30 path=tests/cli_verify.rs mark=edits
what: the `viola verify` CLI tests — edits: the status-line probe and row cases
@task_file n=31 path=tests/hook_fail_open.rs mark=edits
what: the hook verb's fail-open tests — edits: the status-line cases
@task_file n=32 path=tests/path1_run_start.rs mark=edits
what: the `viola run` start tests — edits: the status-line override and snapshot cases
@task_item n=1 part=needs
The hook verb loses its status-line arm and hidden `--capture`/`--shell` probe form; every other hook arm, the dialog arms and their hidden `--capture`/`--answers` forms stay, and the hook verb's exit code, stderr and colour-decision rules keep binding what remains.
@task_item n=2 part=needs
The hook verb no longer passes a status-line command's stdout through: no hook arm writes to stdout for the status line, and viola runs no shell for any purpose.
@task_item n=3 part=needs
`viola run` and revive start the wrapped CLI with no status-line settings override and no `--settings` argument; the start stays silent while wrapped and its child-started output is unchanged.
@task_item n=4 part=needs
The terminal passthrough test that spoke of a status-line source keeps proving that a wrapped session's screen carries no viola bytes, now with no status-line source read at all.
@task_item n=5 part=needs
`viola verify` loses its two status-line ledger rows and Run B's status-line probe: its step lines and summary line count 21 relied rows, and no step line names a removed row; Run B's other probes keep their lines.
@task_item n=6 part=needs
`viola verify --record` writes only at 0 fail and refuses an unclean fixture; the box-only screen fixture is recorded only by that clean run on a named operator step, never hand-written.
@task_item n=7 part=needs
The instance snapshot loses its status-line command field; the board and every constructor stop carrying it, and `viola list`'s columns, rows and STATUS stay as they are.
@task_item n=8 part=needs
A snapshot or event log written by an older binary that still holds the status-line field keeps reading: the reader skips and counts the unknown field and the board's skip counts stay visible, never a refusal of the board.
@task_item n=9 part=needs
The removal adds no verb, flag, env var, setting or ledger row: the command name register keeps `hook` and every other verb, and `viola --help`'s verb groups are unchanged.
@task_item n=10 part=needs
The log subject `statusline-shell`, the span area `statusline`, the subject tag for the hook status-line contract and the shell-spawn process lines leave; the hook verb's stderr carries no line for the status line.
@task_item n=11 part=needs
The fake agent's status-line framing, `--statusline-stdin`, `statusline-echo` mode and `statusline` receipt kind leave; the removal is proved in CI by the fake agent and the code's own tests, never by a live start.
@task_item n=12 part=needs
A wrapped session is never treated as a target for its status line, as an unwrapped one never is: the user's status-line command and `~/.claude/settings.json` are untouched.
@task_item n=13 part=needs
A test case that only spells the interception while testing what stays is rewritten to keep testing that output against its contract's exact line, not dropped.
@task_standing n=1
convention: each rewritten test over CLI output asserts the contract's exact line form, on the right stream, ASCII only, with no colour where stdout is no terminal
home: surface-cli · surface
@task_item n=1 part=not_needed
The flight-progress strip, readback box, send readback, handoff and dialog-pending rows: the chunk changes no strip, field or readback a session shows.
@task_item n=2 part=not_needed
The colour palette, typography, spacing, depth and border-radius areas: the chunk adds no styled output and no colour token.
@task_item n=3 part=not_needed
The iconography area's glyph, separator and arrow rows: no glyph or column is added or moved.
@task_item n=4 part=not_needed
The send, wait, link, unlink, pause, release, answer and last verb outputs and their refusal hints: none of those verbs is touched.
@task_item n=5 part=not_needed
The exit-21 causes, wrapper-fault details, peer-reply screening and wrapper socket rows: the chunk touches no peer channel or instance-unreachable path.
@task_item n=6 part=not_needed
The revive refusal codes and run start refusals: no refusal is added, removed or reordered.
@task_item n=7 part=not_needed
The web-page surface and rendered-page absences: viola renders no page and the chunk adds none.
@task_item n=8 part=not_needed
The self-validation protocol's domain, squint, swap and readback-box checks: they judge a design's expression, and the chunk adds no expression.
