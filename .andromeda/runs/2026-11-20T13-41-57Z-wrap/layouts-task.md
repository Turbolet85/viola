@task_prose part=session
A question dialog first raised by a PermissionRequest hook, with no armed PreToolUse, is now held like any dialog and answered by viola answer: the hook replies allow with updatedInput carrying the tool input plus the answers and annotations.
The rule that left such a question to the human (unanswerable) is retired; a human key, an expired hold or an unverified CLI still leave it to the human, and a second or late answer is refused unknown-dialog, exit 13.
viola verify gains a 27th relied ledger row, permission-request-question, with a sixth closed probe id and a fourth compiled prompt pasted last in Run C; step lines read [NN/27] and the all-pass summary reads 27 pass  0 fail.
The recorded and default claude CLI version moved 2.1.287 to 2.1.328: the fixture set re-recorded (28 to 31 files), the stamped set is now 2.1.328 alone, older sets drift-only but still replayed.
Measured: the live round S1 green at 27/27; CI attempt 1 red on a wheel test (pause then release, exit 0 for 13), the one re-run green, its cause unread and left open.
No IPC method, endpoint, env var, config key, crate or dependency added; no command, status or verdict shape changed.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the hook decision body for each dialog kind and answer: the PermissionRequest question body (allow with updatedInput), the PreToolUse question body, the permission and plan bodies
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows viola verify checks, the dialog probe prompts, the closed probe answer ids and the probe bodies
@task_file n=3 path=src/run/dialog.rs
what: the wrapper's dialog hold: which raised dialog is held for an answer, which is left to the human, the hold expiry
@task_file n=4 path=src/cmd/verify.rs
what: viola verify: its step lines, the relied row count, the summary line and the exit
@task_file n=5 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs that paste the compiled dialog prompts into the CLI (Run C)
@task_file n=6 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig: its arms and labels, the kept stamp lines and the floor of relied rows a live round must pass
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot: the default CLI version a smoke session stamps at
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs
what: the harness run: the expected live viola verify pass output
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent standing in for claude in tests: its default CLI version
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger unit cases: shared capture shapes
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the ledger unit cases: each relied row's pass and fail checks
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the ledger unit cases: probe prompt framing and probe bodies
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger unit cases: the stamp record and its row keys
@task_file n=15 path=src/cmd/hook.rs
what: the hook verb that receives the CLI's hook events and replies the decision body (its test module)
@task_file n=16 path=src/run/version_gate.rs
what: the version gate between the running CLI version and the stamped sets (its test module)
@task_file n=17 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: the snapshot of the PermissionRequest question decision body
@task_file n=18 path=fixtures/claude/2.1.328
what: the recorded hook event fixture set of the claude CLI at the stamped version
@task_file n=19 path=fixtures/claude/2.1.328/PermissionRequest.question-permission-1.json
what: the recorded PermissionRequest hook event of a question raised with no armed PreToolUse
@task_file n=20 path=fixtures/claude/2.1.328/PreToolUse.question-permission-1.json
what: the recorded PreToolUse hook event of that question turn
@task_file n=21 path=fixtures/claude/2.1.328/PostToolUse.question-permission-1.json
what: the recorded PostToolUse hook event carrying that question's answers
@task_file n=22 path=fixtures/fake-scripts/path4.json
what: the fake agent's script for the dialog answer path (Path 4)
@task_file n=23 path=fixtures/fake-scripts/path4-permission.json
what: the fake agent's script for the dialog answer path on a PermissionRequest question
@task_file n=24 path=tests/cli_answer.rs
what: the viola answer CLI tests: answered, not delivered, unknown-dialog
@task_file n=25 path=tests/cli_verify.rs
what: the viola verify CLI tests: step lines, summary and exit
@task_file n=26 path=tests/cli_verify/runs.rs
what: the viola verify CLI tests of the typed runs
@task_file n=27 path=tests/cli_verify/stamps.rs
what: the viola verify CLI tests of the --record stamp
@task_file n=28 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped set
@task_file n=29 path=tests/contract_fake_agent_drift.rs
what: the contract check that the fake agent replays the recorded sets without drift
@task_file n=30 path=tests/contract_fixture_hygiene.rs
what: the contract check over the recorded fixture sets' names and contents
@task_file n=31 path=tests/support/verify.rs
what: the shared test support for viola verify runs
@task_file n=32 path=tests/support/home.rs
what: the shared test support for a viola home and its stamp
@task_file n=33 path=tests/support/fake.rs
what: the shared test support for the fake agent
@task_file n=34 path=tests/support/start.rs
what: the shared test support for starting a wrapped session
@task_file n=35 path=tests/cli_send.rs
what: the viola send CLI tests
@task_file n=36 path=tests/cli_list.rs
what: the viola list CLI tests
@task_file n=37 path=tests/hook_events.rs
what: the hook event tests
@task_file n=38 path=tests/cli_wheel.rs
what: the wheel verb CLI tests (pause, release)
@task_file n=39 path=crates/viola-e2e/tests/mcp_tools.rs
what: the viola mcp tools end-to-end tests
@task_file n=40 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals and their screen facts
@task_item n=1 part=touches
viola verify step line: the relied row count in the step line moved [NN/26] to [NN/27]; a row holding the 26 or naming the step line's denominator is touched.
@task_item n=2 part=touches
viola verify summary (both the summary and summary-line rows): the all-pass summary moved 26 pass  0 fail to 27 pass  0 fail; a no-dialog-replay stamp now reads 22 pass  5 fail.
@task_item n=3 part=touches
viola verify output exit 0, exit 1 and failing row: a new relied row id permission-request-question can now pass or fail and print as a failing row.
@task_item n=4 part=touches
viola verify --record output: the stamp gains the permission-request-question row key and records at 2.1.328.
@task_item n=5 part=touches
viola verify --help paragraph and the dialog probe runs: Run C now pastes a fourth compiled prompt (the permission question) last, with a settle after its Stop.
@task_item n=6 part=touches
viola answer output answered: a question dialog first raised by PermissionRequest is now answerable by viola answer, where it was left to the human before.
@task_item n=7 part=touches
viola answer output unknown-dialog: a second or late answer to a PermissionRequest question, or one after an expired hold, is refused unknown-dialog, exit 13.
@task_item n=8 part=touches
viola answer not-delivered hints: a human key, an expired hold or an unverified CLI still leave a PermissionRequest question to the human; a row saying such a question is never answerable is now stale.
@task_item n=9 part=touches
viola wait output dialog and dialog-expired: a PermissionRequest question is now held and reported as a dialog like a PreToolUse one.
@task_item n=10 part=touches
viola mcp tools answer and wait: the MCP answer tool now reaches a PermissionRequest question dialog.
@task_item n=11 part=touches
viola list board DIALOG column and the amber-means-dialog decision: a held PermissionRequest question now shows as a dialog on the board.
@task_item n=12 part=touches
CLI exit code 13: unknown-dialog for a late answer on a PermissionRequest question rides exit 13.
@task_item n=13 part=touches
viola verify refusals recorded fixture not clean and CLI version unreadable: the recorded fixture set and the stamped version moved to 2.1.328 alone; older sets are drift-only.
@task_item n=14 part=touches
Spoken phraseology for dialog verbs: answer and wait output now covers a question answered through the PermissionRequest hook.
@task_fact n=1 slug=surface-cli
fact: A question first raised by PermissionRequest with no armed PreToolUse is a held dialog that viola answer and the MCP answer tool answer; a human key, an expired hold or an unverified CLI still leave it to the human.
@task_item n=1 part=untouched
The web-spa surface and its absence of a web page: no web surface was reached.
@task_item n=2 part=untouched
The CLI token rendering rows (colour, spacing, radius, motion, roles) and the styling decisions (SGR module, no spinner, no banner, no footer, no glyphs): no styling changed.
@task_item n=3 part=untouched
viola link and unlink output: untouched.
@task_item n=4 part=untouched
viola run and viola revive start output, their refusals and their --json error codes: untouched.
@task_item n=5 part=untouched
viola send output, the readback mirror and the send refusals before any request: no send wording or outcome changed.
@task_item n=6 part=untouched
The frame verb exit-21 hints and the exit-21 --json details: no unreachable cause added or reworded.
@task_item n=7 part=untouched
viola plugin install output and refusals: untouched.
@task_item n=8 part=untouched
Wheel verb output for viola pause and viola release: no wording changed; the CI red on a wheel test moved no row.
@task_item n=9 part=untouched
viola --help usage line, verb group rows and per-verb usage, the flat verbs and global flags: no verb, flag or usage added.
@task_item n=10 part=untouched
The cursor byte offset, viola last output and the deep-links absence: untouched.
