@task_prose part=session
The session built the not-resumed end of viola adopt: when the resumed claude child ends before its session starts (the wrapper's snapshot holds no agent session id), adopt exits 1 with a fixed unable/hint pair, or under --json the refused document with error not-resumed and detail null.
That end writes its pair or document after the child released the terminal, and logs a self process-exit with exit_code 1 and detail not-resumed after the child's process-exit{subject:"claude-child"}; a resumed child, or a replayed, absent or unreadable snapshot, still ends exit 0.
The child pump was split: a new pump end returns the child's end without writing a self exit; the run and revive pump keeps its signature and its mapping (exit 0, or internal-error exit 1) on top of it, and adopt no longer calls it.
The fake agent gained an argv-only option --resume-absent: with --resume it fires SessionEnd with no SessionStart, closes its hooks, ignores SIGHUP and exits 1.
The R5 adopt rig gained a start= field on its r5a verdict line and a rehearse-only third arm r5c over the fake agent; the session measured nothing live and no real claude ran, so the not-resumed end is proven on the fake agent alone.
It retired nothing; viola adopt's own refusals went from two to three, the adopt --json refusal codes from eight to nine, and the process-exit detail enum gained one word.
@task_file n=1 path=src/cmd/run.rs
what: the viola run start and its child pump, the pump that waits on the claude child and logs its process-exit, shared by run, revive and now adopt through a pump end that writes no self exit
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt verb, its start of the resumed claude child, its own refusals (strict-modes-failed, session-live, now not-resumed) with their fixed pair and --json refusal document, and its read of the instance snapshot through read-or-replay
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema the diagnostics schema conformance check validates against, holding the closed detail enum of process-exit
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the fake agent viola-fake-agent standing in for claude, its argv-only modes and options, its fired hook events and its exit triggers
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's inline unit cases for its command-line option parsing
@task_file n=6 path=tests/cli_adopt.rs
what: the root integration suite for viola adopt run as a subprocess in a test-owned home, its CLI outputs pinned by literal asserts
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-surface exit-1 cause table, each exit cause with its own hint and its writer named
@task_file n=8 path=tests/contract_error_bodies.rs
what: the error-body surface witnesses, the literal table of CLI --json failure documents
@task_file n=9 path=scripts/r5-adopt.sh
what: the standing live rig for the gated live reading R5 (real adopt), its verdict lines r5a and r5b, its rehearse-only arms over the fake agent and its own process teardown
@task_item n=1 part=touches
decision/fake-agent-argv-only: the fake agent's argv-only option list gains --resume-absent.
@task_item n=2 part=touches
contract/fake-agent-cli/argv-only-options: the modes and options of the fake agent command line gain --resume-absent; no per-option row exists for it yet, while --stop-crons, --stop-reading, --transcript and --wake-after-ms each have one.
@task_item n=3 part=touches
contract/fake-agent-exit/exit-triggers: a new exit trigger, --resume-absent with --resume, fires SessionEnd and exits 1 before any SessionStart.
@task_item n=4 part=touches
contract/fake-agent-events/session-start: under --resume-absent with --resume the fake agent fires no SessionStart.
@task_item n=5 part=touches
decision/fake-agent-exit-ignores-sighup: the new resume-absent exit also closes its hooks and ignores SIGHUP before exit 1, as the fake's other exits do.
@task_item n=6 part=touches
decision/adopt-proven-live and decision/fake-agent-not-proof-of-real-cli: adopt's not-resumed end is proven over the fake agent alone; what the real CLI does with an id it cannot resume stays the gated live reading R5's, and no capability is claimed.
@task_item n=7 part=touches
decision/revive-json-no-document-after-terminal and contract/revive-json/pump-child-error: the child pump now sits over a pump end; revive's mapping is unchanged and a pump error still prints no document, but adopt's not-resumed end now writes its pair or document after the child released the terminal, an adopt-scoped exception to writing nothing past the child.
@task_item n=8 part=touches
decision/each-exit-cause-own-hint and decision/hint-names-its-cause: a new exit cause not-resumed with its own hint (check the session id, claude --resume with the id must open it by hand under the same claude config).
@task_item n=9 part=touches
decision/error-writers-each-named, decision/error-bodies-held-by-surface-witnesses and decision/contract-tables-are-literal: the exit-1 cause table gains one row and the CLI document witness table gains one body, for not-resumed.
@task_item n=10 part=touches
contract/cli-json-failure/document and decision/cli-json-one-document: viola adopt --json gains a ninth refusal code, not-resumed, written as one refused document with detail null and stderr empty.
@task_item n=11 part=touches
decision/diagnostics-schema-conformance and decision/closed-enums-need-decisions-log: the closed process-exit detail enum of the diagnostics line schema gains the word not-resumed.
@task_item n=12 part=touches
contract/log-event-corr/process-exit and contract/process-start/claude-child: a not-resumed adopt logs process-start and process-exit of the claude child, then a self process-exit exit_code 1 detail not-resumed.
@task_item n=13 part=touches
decision/cli-fixed-strings-pinned-by-literal and decision/cli-outputs-pinned-by-literal: the not-resumed unable and hint pair is pinned by a literal in the adopt suite.
@task_item n=14 part=touches
decision/snapshot-read-through-replay and decision/replayed-snapshot-has-no-endpoint: adopt's resumed check reads the instance snapshot through read-or-replay once, and treats a replayed, absent or unreadable snapshot as resumed; only a real snapshot without an agent session id is not resumed.
@task_item n=15 part=touches
decision/child-exit-read-on-the-process-handle and decision/exit-detected-on-handle: the pump end that reports the child's end is the same handle wait, now returned to the caller before any self exit is written.
@task_item n=16 part=touches
decision/standing-live-rig, decision/refusal-rehearsal-reads-own-effects and decision/kr-seams-rehearse-only: the R5 rig gains a rehearse-only arm r5c reading the refusal's own effects (not-resumed, child started, child ended) and a start= field on r5a read from the last self process-exit detail.
@task_item n=17 part=touches
decision/logs-carry-no-secrets and decision/secrets-never-in-logs: the not-resumed pair and lines hold the instance name only; the session id is asserted absent from every line.
@task_item n=18 part=touches
decision/wrapper-drop-guard and decision/wrapper-spawned-with-guard: the not-resumed cases cannot use the readiness-waiting test wrapper and run viola adopt piped in a test-owned home outside any terminal.
@task_item n=19 part=touches
decision/real-claude-local-only, decision/no-real-cli-in-ci and decision/live-verify-never-in-ci: the session ran no real claude and the R5 rig only under its rehearsal.
@task_fact n=1 slug=7-test-data-fixtures
fact: The fake agent option --resume-absent (argv-only): given with --resume, the fake fires SessionEnd with no SessionStart, closes its hooks, ignores SIGHUP and exits 1, at both launch sites (a trusted start and the trust dialog's accept).
@task_fact n=2 slug=6-e2e-test-strategy
fact: viola adopt exit 1 cause not-resumed: the child ended without its session starting, so adopt writes the unable line naming the instance and its own hint on stderr, or under --json the refused document not-resumed with detail null, after the child released the terminal, and logs a self process-exit detail not-resumed after the child's process-exit.
@task_fact n=3 slug=e2e-test-strategy
fact: Adopt's not-resumed end is proven only over the fake agent; the real CLI's end for an id it cannot resume is read only by the gated live reading R5.
@task_fact n=4 slug=6-e2e-test-strategy
fact: The R5 adopt rig's arm r5c runs only under rehearse over the fake agent, and r5a's verdict line ends start= the last self process-exit detail or ok, its green rule unchanged.
@task_fact n=5 slug=integration-test-strategy
fact: A test whose start must end before a session starts runs the verb piped outside a terminal, never through the readiness-waiting test wrapper, reading stdout and stderr apart.
@task_item n=1 part=untouched
The harness contract, agent-run status, pre-push, gate, coverage, mutation and perf rows: no harness command, gate, threshold, floor or budget moved.
@task_item n=2 part=untouched
CI integration and supply-chain rows: no workflow, action, dependency, tool version or cache changed.
@task_item n=3 part=untouched
MCP server, MCP tool and wrapper channel JSON-RPC rows: no IPC method, endpoint, event kind, socket or MCP tool was added or changed.
@task_item n=4 part=untouched
viola send, wait, last, answer, release, link and list rows: none of those verbs or their refusals moved.
@task_item n=5 part=untouched
Hook exec, hook decision body and dialog rows: hook behaviour and decision bodies are unchanged.
@task_item n=6 part=untouched
Socket directory, strict-modes, home-read and pinned-exe security control rows: no security check or its order changed.
@task_item n=7 part=untouched
Window placement, raise and re-open rows and their live readings R3 and R4: nothing of focus or windows moved.
@task_item n=8 part=untouched
The instance-unreachable and exit 21 cause rows of the frame verbs: no unreachable cause was added.
