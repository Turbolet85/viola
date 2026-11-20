@task_prose part=session
A question dialog first raised by the PermissionRequest hook is now held for the driver and answered, where before it was always left to the human.
The hook's decision body for that question is behaviour allow with updatedInput: the CLI's tool input plus the driver's answers and annotations; the PreToolUse question form is unchanged.
The retired rule held such a question unanswerable; a human key, an expired hold or an unverified CLI still leave it to the human, and a late answer is refused unknown-dialog.
viola verify gains a 27th relied ledger row, permission-request-question, proved by a sixth closed probe id and a fourth synthetic prompt pasted last in Run C; the stamp gains that row key.
The recorded claude fixture set and the default CLI version moved to 2.1.328 (24 fixtures replaced, 3 new), the stamped set narrowed to that one version, the older sets kept as drift-only replays.
The live round ran one start under a new rig label PQ, no key into any CLI dialog, nothing written under the real Claude or viola homes; CI attempt 1 went red on a wheel test, attempt 2 green, that red left open.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the Claude adapter's dialog answer mapping, the hook decision body written back to the CLI for a question, permission or plan dialog
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the verify ledger of relied rows, its closed capture-answer probe ids, the synthetic probe prompts and the checks over hook captures
@task_file n=3 path=src/run/dialog.rs
what: the wrapper's dialog hold, which holds a raised dialog for the driver's answer and hands it back to the human on a key, an expiry or an unverified CLI
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, its step lines over every relied row and its all-pass summary
@task_file n=5 path=src/cmd/verify/typed/runs.rs
what: verify's typed runs, the compiled probe prompts pasted into the real claude child in Run C
@task_file n=6 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig, its arms and labels, its kept stamp lines and its floor of relied rows
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot, its default recorded CLI version
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness run, the expected live verify output
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent, its default recorded CLI version
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: unit cases of the fake agent's options
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the verify ledger's capture shapes
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger's row checks, pass and fail
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit cases of the ledger's probe bodies and turn framing
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamp rows written by verify
@task_file n=15 path=src/cmd/hook.rs
what: viola hook, the hook entry reading the CLI's payload; its test module only
@task_file n=16 path=src/run/version_gate.rs
what: viola run's version gate over the verify stamp; its test module only
@task_file n=17 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: a recorded snapshot of the question decision body on PermissionRequest
@task_file n=18 path=fixtures/claude/2.1.328
what: the recorded claude hook fixtures, scrubbed and under the fixture hygiene walk
@task_file n=19 path=fixtures/fake-scripts/path4.json
what: a fake agent script for the dialog path
@task_file n=20 path=fixtures/fake-scripts/path4-permission.json
what: a fake agent script for the permission dialog path
@task_file n=21 path=tests/cli_answer.rs
what: integration tests of viola answer on held dialogs
@task_file n=22 path=tests/cli_verify.rs
what: integration tests of viola verify's step lines and summary
@task_file n=23 path=tests/cli_verify/runs.rs
what: integration tests of verify's typed runs
@task_file n=24 path=tests/cli_verify/stamps.rs
what: integration tests of the stamps written by verify
@task_file n=25 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped fixture set
@task_file n=26 path=tests/contract_fake_agent_drift.rs
what: the drift contract between the fake agent and the recorded fixture sets
@task_file n=27 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene check
@task_file n=28 path=tests/support/verify.rs
what: test support for verify, the recorded capture shapes
@task_file n=29 path=tests/support/home.rs
what: test support for test homes and their seeded stamps
@task_file n=30 path=tests/support/fake.rs
what: test support for the fake agent, its recorded version
@task_file n=31 path=tests/support/start.rs
what: test support for starting a wrapped instance
@task_file n=32 path=tests/chaos_revive.rs
what: chaos tests of revive, a version literal only
@task_file n=33 path=tests/cli_send.rs
what: integration tests of viola send, a version literal only
@task_file n=34 path=tests/cli_list.rs
what: integration tests of viola list, version literals only
@task_file n=35 path=tests/tui_passthrough.rs
what: passthrough tests, a version literal only
@task_file n=36 path=tests/run_cli.rs
what: integration tests of viola run, a version literal only
@task_file n=37 path=tests/hook_events.rs
what: integration tests of hook event lines, a session id literal only
@task_file n=38 path=tests/cli_wheel.rs
what: integration tests of the wheel, pause and release, a version literal only
@task_file n=39 path=crates/viola-e2e/tests/mcp_tools.rs
what: e2e tests of the MCP tools, version literals only
@task_file n=40 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: rehearsals of the live rig's arms, their screen-fact literals
@task_item n=1 part=touches
decision/capture-answers-closed-probe-ids: the closed probe ids grow from five to six, the new id permission-question-first-option answering a PermissionRequest question with the allow body.
@task_item n=2 part=touches
decision/live-rig-kept-stamp-owner-only: the rig gains the label PQ (and an unspent PQ-2) for the permission-question round; its kept stamp now prints the permission-request-question row.
@task_item n=3 part=touches
decision/driver-permission-policy-outside-viola: the driver's answers now reach a PermissionRequest question and are carried into updatedInput unvetted, as the founder's reading holds; check its wording still covers this form.
@task_item n=4 part=touches
The fail-open and never-block-the-human rules (decision/hook-fails-open, decision/hooks-fail-open, decision/refusals-never-block-human): the rule that left every PermissionRequest-first question to the human is retired; such a question is now held, a human key, an expired hold or an unverified CLI still giving it back to the human.
@task_item n=5 part=touches
decision/unverified-cli-gate and decision/dialog-answers-need-verify-stamp: answering a PermissionRequest question now needs the CLI verified with the new relied row; the stamp gains the permission-request-question row key.
@task_item n=6 part=touches
decision/answer-behavior-closed-enum: the PermissionRequest decision for a question carries behaviour allow plus updatedInput (tool input, answers, annotations); other answer bodies keep their shape.
@task_item n=7 part=touches
decision/question-option-text-crosses-boundary and decision/hook-payload-is-content: the question's tool input from the CLI, with the driver's answers and annotations, now crosses back to the CLI inside updatedInput.
@task_item n=8 part=touches
contract/wrapper-channel-methods/answer, contract/wrapper-channel-methods/hook-dialog and contract/mcp-tools/answer: an answer now reaches a question raised by PermissionRequest; a second or late answer is refused unknown-dialog, exit 13.
@task_item n=9 part=touches
decision/probe-prompts-synthetic and decision/verify-probe-captures-transient: verify gains a fourth synthetic prompt asking for an AskUserQuestion call, pasted last in Run C with a settle after its Stop.
@task_item n=10 part=touches
The verify-never-answers rules (decision/verify-never-answers-dialogs, decision/verify-writes-no-dialog-byte, decision/verify-screen-read-against-literals): the new probe answers through the hook decision body, never a key into the dialog; check the rows still hold.
@task_item n=11 part=touches
decision/hook-capture-arm and decision/capture-arms-isolated: the capture turns gain the question-permission stem.
@task_item n=12 part=touches
The fixture rows (decision/fixture-recording-scrubbed, decision/fixture-scrub-and-refusal, decision/fixtures-reviewed-before-commit, decision/fixture-hygiene): the recorded set at 2.1.328 replaced 24 fixtures and added 3, the hygiene check extended to them.
@task_item n=13 part=touches
decision/stamps-written-only-by-verify and decision/version-gate-strict-stamps-read: the stamped fixture set narrowed to 2.1.328 alone, older sets kept as drift-only replays; the stamp holds 27 relied rows.
@task_item n=14 part=touches
The live round rows (decision/live-round-rig-keys-own-pty, decision/real-cli-local-only, decision/nothing-written-under-claude-home): one live start under PQ with no key into any CLI dialog and the live stamp unmoved; any row holding the rig's arm list or floor of relied rows (now 23) moves.
@task_item n=15 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake agent's default recorded version moved to 2.1.328; no seam added.
@task_fact n=1 slug=threat-model-summary
fact: A question first raised by the PermissionRequest hook is held for the driver like any dialog and answered allow with updatedInput carrying the tool input plus the driver's answers and annotations; a human key, an expired hold or an unverified CLI still leave it to the human.
@task_fact n=2 slug=authentication-authorization
fact: The driver's answer to a PermissionRequest question is relied on only where the verify stamp passes the permission-request-question ledger row.
@task_item n=1 part=untouched
Dependency security: no crate, feature, pin, advisory ignore, licence or toolchain changed.
@task_item n=2 part=untouched
The rmcp feature set and API security: no MCP tool, method or feature added.
@task_item n=3 part=untouched
Socket directory, peer euid, server verification and strict-modes refusal rows: no channel check or refusal contract moved.
@task_item n=4 part=untouched
The absences: no listener, endpoint, port, env var, outbound call, secret or workspace crate added.
@task_item n=5 part=untouched
Secret management's credential strip, identity floor and canary scan: no environment read or stripped name changed.
@task_item n=6 part=untouched
Error-handling contracts: no exit code, detail word or error document added or reworded.
@task_item n=7 part=untouched
CI workflow rows: no workflow, action pin or permission changed; the open CI red is a test fault, no security rule.
