@task_prose part=session
A question (AskUserQuestion) first raised by the PermissionRequest hook, with no armed PreToolUse, is now held as a pending dialog like any other and answered: the hook replies PermissionRequest decision behavior allow with updatedInput holding the tool input plus the driver's answers and annotations.
The rule that left such a question null to the human (the unanswerable rule) is retired; a human key, an expired hold or an unverified CLI still leave it to the human, and a late answer or one after an expired hold is refused unknown-dialog (exit 13).
The capability ledger gains a 27th relied row, permission-request-question, with a sixth closed probe id permission-question-first-option and a fourth compiled prompt pasted last in verify's Run C; relied rows 26 to 27, verify step lines [NN/27], all-pass summary 27 pass 0 fail.
The recorded fixture set and the default CLI version of the fake agent and the test harness moved 2.1.287 to 2.1.328; only 2.1.328 is a stamped set, the older sets are replayed for drift only.
No IPC method, endpoint, socket, port, env var, config key, dependency or workspace crate added.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the Claude agent crate's dialog decision bodies, the hook reply a driver's answer becomes per hook and dialog kind
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: relied rows, their checks over captured payloads, verify's compiled dialog prompts and the closed probe answer ids
@task_file n=3 path=crates/viola-agent-claude/src/ledger
what: the ledger's unit cases (checks, framing, stamps) for the new row and probe
@task_file n=4 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: the insta snapshot of the PermissionRequest question decision body
@task_file n=5 path=src/run/dialog.rs
what: the wrapper's dialog hold: which raised dialogs are held for a driver answer and which are left to the human
@task_file n=6 path=src/cmd/verify.rs
what: viola verify, its relied-row step lines and summary line
@task_file n=7 path=src/cmd/verify/typed/runs.rs
what: verify's typed probe runs, Run C pasting the compiled dialog prompts
@task_file n=8 path=src/cmd/hook.rs
what: the hook command's test module
@task_file n=9 path=src/run/version_gate.rs
what: the version gate's test module over stamped relied rows
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig, its arm labels, kept stamp lines and floor rows
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the test harness boot, its default CLI version
@task_file n=12 path=crates/viola-e2e/src/harness/run.rs
what: the test harness run, its expected live verify output
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary, its default CLI version
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases
@task_file n=15 path=fixtures/claude/2.1.328
what: the recorded Claude CLI hook payload fixture set at 2.1.328, replaced and gaining the question-permission captures
@task_file n=16 path=fixtures/fake-scripts/path4.json
what: the fake agent script for the dialog answer path
@task_file n=17 path=fixtures/fake-scripts/path4-permission.json
what: the fake agent script for the PermissionRequest-raised question
@task_file n=18 path=tests/cli_answer.rs
what: integration tests of the answer verb over raised dialogs
@task_file n=19 path=tests/cli_verify.rs
what: integration tests of viola verify output and counts
@task_file n=20 path=tests/cli_verify/runs.rs
what: integration tests of verify's probe runs
@task_file n=21 path=tests/cli_verify/stamps.rs
what: integration tests of the stamps verify records
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped fixture set
@task_file n=23 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract over the recorded sets
@task_file n=24 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene contract
@task_file n=25 path=tests/support/verify.rs
what: test support for verify runs and captured shapes
@task_file n=26 path=tests/support/home.rs
what: test support home builder, its dialog rows of a stamp
@task_file n=27 path=tests/support/fake.rs
what: test support for the fake agent, its version literal
@task_file n=28 path=tests/support/start.rs
what: test support for starting an instance, its version literal
@task_file n=29 path=tests/chaos_revive.rs
what: chaos tests booted on the default fixture version
@task_file n=30 path=tests/cli_send.rs
what: integration tests booted on the default fixture version
@task_file n=31 path=tests/cli_list.rs
what: integration tests booted on the default fixture version
@task_file n=32 path=tests/cross_list_rows.rs
what: cross tests booted on the default fixture version
@task_file n=33 path=tests/tui_passthrough.rs
what: passthrough tests booted on the default fixture version
@task_file n=34 path=tests/run_cli.rs
what: integration tests of run booted on the default fixture version
@task_file n=35 path=tests/hook_events.rs
what: integration tests of hook events booted on the default fixture version
@task_file n=36 path=tests/cli_wheel.rs
what: integration tests booted on the default fixture version
@task_file n=37 path=crates/viola-e2e/tests/mcp_tools.rs
what: end-to-end tests booted on the default fixture version
@task_file n=38 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals, their screen-fact literals and expected verify counts
@task_item n=1 part=touches
decision/permission-request-question-stays-null is broken and retired: such a question is now held and answered allow with updatedInput, not left null to the human.
@task_item n=2 part=touches
contract/hook-contract/permission-request gains the question's decision form: behavior allow with updatedInput (tool input plus answers and annotations).
@task_item n=3 part=touches
contract/answer-refusals/unknown-dialog now covers a late answer to the PermissionRequest-raised question or one after its expired hold.
@task_item n=4 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/summary-line where they hold the relied count: [NN/26] to [NN/27], 26 pass 0 fail to 27 pass 0 fail.
@task_item n=5 part=touches
Rows holding the relied ledger row count or listing the relied rows (decision/cli-capability-ledger, decision/capability-ledger, decision/capability-ledger-gate, decision/version-verified-all-relied-rows, contract/ledger-stamps/rows): 26 to 27, the new row key permission-request-question in the stamp.
@task_item n=6 part=touches
Ledger rows with their probes (decision/browser-tools-prompt-ledger-row, decision/trust-record-ledger-row, decision/local-command-echo-probe, decision/no-probe-for-unrelied-shapes): a new relied row with its probe belongs beside them.
@task_item n=7 part=touches
decision/exit-plan-mode-allow-with-updated-input: the question now takes the allow-with-updatedInput form through PermissionRequest too, while ExitPlanMode stays PreToolUse-only.
@task_item n=8 part=touches
Dialog hold rules (decision/dialog-hold, decision/dialog-hold-bound, decision/one-pending-dialog, decision/viola-ends-the-dialog-wait, contract/dialog-hold-end/null-reply): one more raised case is now held instead of passed straight to the human.
@task_item n=9 part=touches
decision/human-wheel-renders-dialogs, decision/unverified-cli-transport-only and decision/partial-gate-on-unverified-cli: a human key or an unverified CLI still leaves this question to the human.
@task_item n=10 part=touches
contract/answer-response/question, contract/channel-answer/unknown-dialog and contract/mcp-tools/answer: an answer to a question raised by PermissionRequest is now accepted by id.
@task_item n=11 part=touches
contract/hook-to-kind-map/permissionrequest, contract/event-data/question and contract/normalised-event-kinds/question: a PermissionRequest AskUserQuestion raises a question dialog.
@task_item n=12 part=touches
decision/dialog-logged-exactly-once and contract/dialog-events/dialog-id: the PermissionRequest-raised question is logged once, woken and answered by id.
@task_item n=13 part=touches
contract/hook-tiers/sync-with-decision, contract/hook-contract/decision and decision/hook-output-one-crate: a new decision body produced by the agent crate.
@task_item n=14 part=touches
decision/permission-answer-without-suggestions: the permission-answer sibling, unchanged in itself; the question's PermissionRequest body is a separate allow form.
@task_item n=15 part=touches
decision/upstream-text-is-content, decision/untrusted-upstream-text and decision/mechanism-not-policy: viola carries the driver's answers into updatedInput unvetted.
@task_item n=16 part=touches
Verify rules over its probe runs (decision/verify-never-types-into-dialog, decision/verify-types-nothing-into-dialogs, decision/verify-child-set, decision/verify-box-wait-for-literal): Run C pastes a fourth compiled prompt last and settles after its Stop, its dialog answered through the hook, no key typed.
@task_item n=17 part=touches
contract/probe-capture/payload-file and decision/probe-capture-arms-inert: a new capture stem question-permission (PreToolUse, PermissionRequest, PostToolUse payloads).
@task_item n=18 part=touches
decision/recorded-fixtures-scrubbed and decision/verify-record-only-clean: the recorded set at 2.1.328 replaced and grown 28 to 31 files, hygiene-walked.
@task_item n=19 part=touches
decision/fake-agent-test-seams, decision/tests-against-fake-agent and decision/ci-ubuntu-fake-agent where a cell holds the fake agent's or harness's default CLI version: 2.1.287 to 2.1.328.
@task_item n=20 part=touches
decision/real-cli-probe-local-only and decision/real-claude-local-only: a live round against claude 2.1.328 stamped the new row locally.
@task_fact n=1 slug=established-decisions
fact: A question first raised by PermissionRequest with no armed PreToolUse is held as a pending dialog and the driver's answer is returned as PermissionRequest decision behavior allow with updatedInput (tool input plus answers and annotations); a human key, an expired hold or an unverified CLI leave it to the human.
@task_fact n=2 slug=established-decisions
fact: The permission-request-question relied ledger row and its probe: verify's Run C pastes a compiled prompt that raises the question through PermissionRequest and answers it with the closed probe id permission-question-first-option (first option labels), the row passing only when the answer lands in the tool's input.
@task_item n=1 part=untouched
The Hyprland compositor seam and window features.
@task_item n=2 part=untouched
Dependency policy, toolchain pin, MSRV, licences and cargo-deny bans: no dependency added.
@task_item n=3 part=untouched
Channel framing, protocol version, JSON-RPC fault codes and local socket endpoints: no method, endpoint or socket added.
@task_item n=4 part=untouched
config.json keys, environment variable names and the VIOLA_ namespace.
@task_item n=5 part=untouched
Trust writer code words.
@task_item n=6 part=untouched
Diagnostics line formats, process logs and role files.
@task_item n=7 part=untouched
Absences (daemon, database, ports, graphical frontend, hosting) and the crate dependency direction: no crate added or removed.
