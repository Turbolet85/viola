@task_prose part=session
An AskUserQuestion question first raised by a PermissionRequest hook is now held as a dialog and answered by the driver: the hook decision body is allow with updatedInput carrying the tool input, the answers and the annotations.
The rule that left such a question to the human (unanswerable) is retired; a human key, an expired hold or an unverified CLI still leave it to the human, and a late answer or one after an expired hold is refused unknown-dialog, exit 13.
viola verify gains a 27th relied ledger row, permission-request-question, with a sixth closed probe id (permission-question-first-option); verify step lines read [NN/27] and the all-pass summary 27 pass  0 fail.
The recorded and default claude CLI version moved 2.1.287 to 2.1.328: the fixture set re-recorded (three new question-permission captures), the stamped list narrowed to 2.1.328, older sets kept as drift-only and still replayed.
Measured: a live verify round green at 27/27 with no key into any CLI dialog; pre-push green; CI attempt 1 red on a wheel-suite test (a send into a running turn accepted where turn-running exit 13 was expected), the one allowed re-run green, the red left open with its cause unread.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the hook decision bodies the plugin hook prints for a dialog answer; edited, the PermissionRequest question form added
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the relied ledger rows, verify probe prompts and closed probe answer ids the verify run checks; edited, a 27th row and sixth probe id added
@task_file n=3 path=src/run/dialog.rs
what: the wrapper's dialog holding, the dialog hold and who answers a pending dialog; edited, the leave-to-the-human rule for PermissionRequest questions removed
@task_file n=4 path=src/cmd/verify.rs
what: viola verify and its step lines and summary output; edited, step count 26 to 27
@task_file n=5 path=src/cmd/verify/typed/runs.rs
what: viola verify's interactive runs that paste the probe prompts; edited, a fourth prompt in Run C
@task_file n=6 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig that runs a real claude under the kept stamp; edited, new PQ labels and floor rows
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: the agent harness boot and its default CLI version; edited
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs
what: the harness run and its expected live verify output; edited
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary and its default CLI version; edited
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases; edited
@task_file n=11 path=crates/viola-agent-claude/src/ledger/
what: the ledger's unit cases (row checks, framing, stamps); edited
@task_file n=12 path=src/cmd/hook.rs
what: the viola hook command's test module; edited
@task_file n=13 path=src/run/version_gate.rs
what: the CLI version gate's test module; edited
@task_file n=14 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: the snapshot of the PermissionRequest question decision body; new
@task_file n=15 path=fixtures/claude/2.1.328/
what: the recorded claude hook capture set the boundary tests are sequenced on; replaced, three captures added
@task_file n=16 path=fixtures/fake-scripts/path4.json
what: the fake agent script for the dialog-pending-then-answer path; edited
@task_file n=17 path=fixtures/fake-scripts/path4-permission.json
what: the fake agent script for the permission dialog path; edited
@task_file n=18 path=tests/cli_answer.rs
what: the viola answer suite, the dialog answer critical path; edited, the PermissionRequest question answered by id
@task_file n=19 path=tests/cli_verify.rs
what: the viola verify output suite, step line and summary literals; edited
@task_file n=20 path=tests/cli_verify/runs.rs
what: the viola verify interactive run cases; edited
@task_file n=21 path=tests/cli_verify/stamps.rs
what: the viola verify stamp record cases; edited
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped set; edited
@task_file n=23 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract over the recorded sets; edited
@task_file n=24 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene contract; edited
@task_file n=25 path=tests/support/verify.rs
what: the shared verify test support; edited
@task_file n=26 path=tests/support/home.rs
what: the shared test home and its dialog rows; edited
@task_file n=27 path=tests/support/fake.rs
what: the shared fake agent test support; edited, CLI version literal
@task_file n=28 path=tests/support/start.rs
what: the shared session start test support; edited, CLI version literal
@task_file n=29 path=tests/chaos_revive.rs
what: the revive chaos suite; edited, CLI version literal only
@task_file n=30 path=tests/cli_send.rs
what: the viola send suite; edited, CLI version literal only
@task_file n=31 path=tests/cli_list.rs
what: the viola list board suite; edited, CLI version literal only
@task_file n=32 path=tests/cross_list_rows.rs
what: the list rows cross suite; edited, CLI version literal only
@task_file n=33 path=tests/tui_passthrough.rs
what: the TUI passthrough boundary suite; edited, CLI version literal only
@task_file n=34 path=tests/run_cli.rs
what: the viola run suite; edited, CLI version literal only
@task_file n=35 path=tests/hook_events.rs
what: the hook event suite; edited, CLI version literal only
@task_file n=36 path=tests/cli_wheel.rs
what: the wheel suite, human takes the wheel and pause and release; edited, CLI version literal only
@task_file n=37 path=crates/viola-e2e/tests/mcp_tools.rs
what: the MCP server tools suite; edited, CLI version literal only
@task_file n=38 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals and their screen-fact literals; edited
@task_item n=1 part=touches
The dialog-pending-then-answer critical path now covers a question first raised by PermissionRequest: it is held and answered by the driver, no longer left to the human.
@task_item n=2 part=touches
The viola verify step line: its count moved [NN/26] to [NN/27] and the all-pass summary 26 pass to 27 pass; any row holding the step count or the summary literal is reached.
@task_item n=3 part=touches
Fixed CLI strings pinned by literal: the verify step and summary literals were re-pinned at 27.
@task_item n=4 part=touches
The dialog hold bounds the driver and time bounds fall on the driver, never on the human: the hold now also governs a PermissionRequest question, and an expired hold leaves it to the human.
@task_item n=5 part=touches
config.json dialog_hold_s and the dialog-expired event line now apply to the PermissionRequest question dialog too.
@task_item n=6 part=touches
Human key takes the wheel and human keystrokes never blocked: a human key on a held PermissionRequest question still leaves it to the human.
@task_item n=7 part=touches
CLI exit code 13: a late answer, or one after an expired hold, to a PermissionRequest question is refused unknown-dialog, exit 13; the CI red was a send that got exit 0 where turn-running exit 13 was expected.
@task_item n=8 part=touches
Zero flakiness, no retries, and the CI test gate: CI attempt 1 went red on a wheel-suite test and the one allowed re-run went green, with the red left open and its cause unread.
@task_item n=9 part=touches
Boundary tests sequenced on records: the recorded capture set moved to 2.1.328, three captures added, and older sets kept as drift-only and still replayed.
@task_item n=10 part=touches
The boundary check never inspects claude content, and peer content is not vetted: the decision body carries the driver's answers in updatedInput unvetted.
@task_item n=11 part=touches
Plugin and IPC channel not assertable: the plugin hook's decision body gained a new form, asserted by literal and snapshot unit cases; no IPC method, endpoint or socket was added.
@task_item n=12 part=touches
Live readings never gate: a live verify round under a new rig label PQ read 27/27 and did not gate.
@task_item n=13 part=touches
A11y assertions inside existing suites: the new cases were added to the existing unit, integration and e2e suites, with no new suite.
@task_fact n=1 slug=1-a11y-scope-summary
fact: The dialog-pending-then-answer critical path includes an AskUserQuestion question first raised by PermissionRequest; it is held and answered by the driver like any dialog, and left to the human only on a human key, an expired hold or an unverified CLI.
@task_item n=1 part=untouched
SGR and colour rules (amber on the DIALOG word, bold NAME, dim stale rows, no SGR off a terminal, word beside every SGR span): no output colour or styling changed.
@task_item n=2 part=untouched
viola link and viola unlink contracts and viola plugin install output: not reached.
@task_item n=3 part=untouched
viola --help contracts (usage line, group labels, served verbs, handoff row): no verb or help text changed.
@task_item n=4 part=untouched
Screen-reader targets and setups (Orca with GNOME Terminal, Orca with foot) and the manual keyboard and screen-reader passes: not reached.
@task_item n=5 part=untouched
Absences (touch surface, web or graphical frontend, internationalization, rendered page, user time limit): none taken back.
@task_item n=6 part=untouched
Log identity tags, structured violation JSON rows and the a11y artifact and driver decisions in CI integration: unchanged.
@task_item n=7 part=untouched
Frame verb, hook call and instance-unreachable refusals over the socket directory, strict modes and server verify: no refusal word or detail added.
