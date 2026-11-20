@task_prose part=session
The session made a question that a PermissionRequest raises first answerable: the hook now answers it allow, with the tool input plus the driver's answers carried as updatedInput, where before such a question was left to the human.
The rule that held a PermissionRequest-raised question as unanswerable is retired; the question is now held like any dialog, and only a human key, an expired hold or an unverified CLI still leave it to the human. A late answer, or one after an expired hold, is refused unknown-dialog with exit 13.
viola verify gained a 27th relied ledger row, permission-request-question, with a sixth closed probe id; its step lines now count [NN/27] and its all-pass summary reads 27 pass  0 fail.
The recorded and default Claude CLI version moved from 2.1.287 to 2.1.328: a new recorded fixture set with three new question-permission captures, the only stamped set now 2.1.328, the older sets kept as drift-only.
No verb, flag, exit code, colour, glyph, column, IPC method, env var or config key was added or removed.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the Claude adapter's dialog decision bodies — the hook's answer to each dialog kind on PreToolUse and PermissionRequest
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger — the rows viola verify checks against a CLI's captures, its probe prompts and closed probe ids
@task_file n=3 path=src/run/dialog.rs
what: the wrapper's dialog holding — which dialogs are held for a driver's answer and which are left to the human
@task_file n=4 path=src/cmd/verify.rs
what: the viola verify verb — its step lines, row count and summary line
@task_file n=5 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs that paste the probe prompts into the CLI
@task_file n=6 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig (test-only) — its arms, labels, kept stamp and floor rows
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot — its default CLI version
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs
what: the test harness run — the expected live viola verify output
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent used by tests — its default CLI version
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the relied ledger's unit cases
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: the relied ledger's row-check unit cases
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: the relied ledger's probe-framing unit cases
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the relied ledger's stamp unit cases
@task_file n=15 path=src/cmd/hook.rs
what: the hook verb — its unit test module only
@task_file n=16 path=src/run/version_gate.rs
what: the CLI version gate that reads the verified stamp — its unit test module only
@task_file n=17 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: a new snapshot of the PermissionRequest question decision body
@task_file n=18 path=fixtures/claude/2.1.328
what: the recorded hook-capture fixture set of the Claude CLI 2.1.328, replaced and gaining three question-permission captures
@task_file n=19 path=fixtures/fake-scripts
what: the fake agent's Path 4 dialog scripts, plain and permission
@task_file n=20 path=tests/cli_answer.rs
what: the viola answer integration tests — Path 4, a PermissionRequest question answered by id
@task_file n=21 path=tests/cli_verify.rs
what: the viola verify integration tests — step lines and summary counts
@task_file n=22 path=tests/cli_verify/runs.rs
what: the viola verify typed-run integration tests
@task_file n=23 path=tests/cli_verify/stamps.rs
what: the viola verify stamp-record integration tests
@task_file n=24 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped set
@task_file n=25 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract tests over the recorded sets
@task_file n=26 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract tests
@task_file n=27 path=tests/support/verify.rs
what: test support for viola verify runs
@task_file n=28 path=tests/support/home.rs
what: test support for a home's verified stamp and its dialog rows
@task_file n=29 path=tests/support/fake.rs
what: test support for the fake agent — the recorded CLI version
@task_file n=30 path=tests/support/start.rs
what: test support for starting a wrapped session
@task_file n=31 path=tests/chaos_revive.rs
what: the revive chaos tests — version literal only
@task_file n=32 path=tests/cli_send.rs
what: the viola send integration tests — version literal only
@task_file n=33 path=tests/cli_list.rs
what: the viola list integration tests — version literal only
@task_file n=34 path=tests/cross_list_rows.rs
what: the cross-instance list row tests — version literal only
@task_file n=35 path=tests/tui_passthrough.rs
what: the terminal passthrough tests — version literal only
@task_file n=36 path=tests/run_cli.rs
what: the viola run integration tests — version literal only
@task_file n=37 path=tests/hook_events.rs
what: the hook event integration tests — version literal only
@task_file n=38 path=tests/cli_wheel.rs
what: the wheel (pause and release) integration tests — version literal only
@task_file n=39 path=crates/viola-e2e/tests/mcp_tools.rs
what: the mcp verb tools end-to-end tests — version literal only
@task_file n=40 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals — expected verify lines and screen facts of the default set
@task_item n=1 part=touches
viola verify step line: the step counter now reads [NN/27] where it read [NN/26]; any row holding the step total 26 is stale.
@task_item n=2 part=touches
viola verify summary and summary line: the all-pass summary now reads 27 pass  0 fail; any row holding 26 as the relied-row count is stale.
@task_item n=3 part=touches
The step counter as the one progress pattern: the verify counter it names now runs to 27.
@task_item n=4 part=touches
viola verify output rows naming relied ledger rows, the fail word or the trust-record row: a new relied row permission-request-question now prints among them.
@task_item n=5 part=touches
CLI verification state, verified and unverified-cli: verified now means 27 relied rows pass, including the PermissionRequest question row; the stamped CLI version is now 2.1.328.
@task_item n=6 part=touches
viola answer verb output: an answer to a question first raised by a PermissionRequest is now accepted and delivered, where before it was left to the human.
@task_item n=7 part=touches
Refusal hints for not-delivered unknown-dialog and exit code 13: a late answer, or one after an expired hold, to a PermissionRequest question is refused unknown-dialog with exit 13.
@task_item n=8 part=touches
Refusal hint for unverified-cli: an unverified CLI still leaves a PermissionRequest question to the human, the exception kept.
@task_item n=9 part=touches
Dialog-pending shown as a cocked strip and the flight-progress strip DIALOG field: a question first raised by a PermissionRequest is now held as a pending dialog, so it can show dialog-pending.
@task_item n=10 part=touches
viola wait output dialog form: a PermissionRequest-raised question now surfaces to the driver in the dialog form.
@task_item n=11 part=touches
The hook verb's contract (exit code, stdout): its decision body for a PermissionRequest question is now allow with updatedInput carrying the answers.
@task_item n=12 part=touches
Arrival amber for dialog-pending only: indirectly, the set of dialogs that can be pending widens to PermissionRequest-raised questions.
@task_fact n=1 slug=surface-cli
fact: A question first raised by a PermissionRequest is held as a pending dialog and answered by viola answer like any dialog; only a human key, an expired hold or an unverified CLI leave it to the human.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and the anti-patterns: no colour, glyph, font, motion or style rule changed.
@task_item n=2 part=untouched
Iconography and CLI glyphs, separators and transfer arrows: no glyph or separator added or changed.
@task_item n=3 part=untouched
viola list board and table, its columns, skip counts, NAME cut and row escaping: the board's output did not change.
@task_item n=4 part=untouched
viola run and revive start refusals, revive --json codes, exit 21 causes, strict-modes and socket-dir refusals: none moved.
@task_item n=5 part=untouched
viola link and unlink, viola last, send readback and the readback box mirror: no output form changed.
@task_item n=6 part=untouched
CLI colour decision order, colour depth and colour tokens, hints and streams: unchanged.
@task_item n=7 part=untouched
The web-spa surface absence, the rendered-page absence, command names and the self-validation protocol: no name taken or freed, no surface added.
