@task_prose part=session
The session built the not-resumed end of viola adopt: when the resumed child ends and the instance's real snapshot holds no agent_session_id, adopt logs the child's process-exit, then a self process-exit exit 1 with detail not-resumed, writes a fixed unable/hint pair on stderr (or under --json the not-resumed error document), and exits 1.
A resumed child (or a replayed, absent or unreadable snapshot) still ends exit 0; a pump worker panic still ends internal-error, exit 1, with no document.
The start's pump was split: a new pump_end runs the pump and logs the child's exit without writing a self exit; pump_child keeps its signature and mapping for run and revive, now written over pump_end; adopt alone calls pump_end.
viola adopt's own refusals grow from two to three (strict-modes-failed, session-live, not-resumed); its --json refusal codes from eight to nine; the process-exit detail enum gains not-resumed.
The fake agent gains an argv-only option --resume-absent (exits 1 before any SessionStart when resuming), and the R5 adopt rig gains a start= field on r5a and a rehearse-only r5c line.
Measured, not changed: the CLI resumes a session by id from a folder other than the one it was made in, so adopt's spawn folder was left as it was; no capability was claimed, no IPC method, event kind, socket, port or env var was added.
@task_file n=1 path=src/cmd/run.rs
what: the start path of viola run and its pump of the wrapped child (pump_child, now over the new pump_end), shared by run, revive and adopt
@task_file n=2 path=src/cmd/adopt.rs
what: the viola adopt verb: its own refusals and their fixed pairs and --json documents, and its end after the child exits
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema, holding the closed process-exit detail enum
@task_file n=4 path=src/bin/viola-fake-agent.rs
what: the fake agent test binary standing in for the claude CLI, and its argv-only options
@task_file n=5 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing unit tests
@task_file n=6 path=tests/cli_adopt.rs
what: the integration tests of the viola adopt verb
@task_file n=7 path=tests/cross_exit_causes.rs
what: the cross-check table of every exit-1 cause and its process-exit detail
@task_file n=8 path=tests/contract_error_bodies.rs
what: the contract table of every CLI --json error document body
@task_file n=9 path=scripts/r5-adopt.sh
what: the R5 adopt rig script, its live arms and its rehearse-only arms over the fake agent, and its verdict lines
@task_item n=1 part=touches
contract/cli-json-output/adopt-refusal-codes: the listed codes go from eight to nine and adopt's own from two to three; not-resumed joins strict-modes-failed and session-live.
@task_item n=2 part=touches
decision/no-document-after-child-held-terminal: an exception is added for adopt's not-resumed end only, which writes its pair or its --json document after the child released the terminal; run and revive keep the rule; a pump error still prints no document.
@task_item n=3 part=touches
decision/run-terminal-child-only: indirectly, through the shared start path; adopt now writes its own stderr pair once the child has ended and released the terminal.
@task_item n=4 part=touches
contract/cli-exit-codes/1: exit 1 now also covers adopt's not-resumed end, after the child ended, not a start refusal.
@task_item n=5 part=touches
The process-exit detail enum rows (contract/process-exit-detail/*): the closed enum gains not-resumed, which no row names yet.
@task_item n=6 part=touches
decision/refusal-details-closed-set and contract/code-word/format: a new code word not-resumed enters the closed set.
@task_item n=7 part=touches
decision/adopt-refuses-live-session and contract/cli-verbs/adopt: adopt's own refusals are now three, the third reached after the child ends rather than before it starts.
@task_item n=8 part=touches
contract/human-stderr/refusal: a new fixed unable/hint pair, the hint naming the session id by the literal word only, never the id itself.
@task_item n=9 part=touches
contract/cli-json-error-document/shape and contract/cli-json-output/error: a new document with error not-resumed and detail null, stderr empty.
@task_item n=10 part=touches
contract/revive-json/pump-error: pump_child's mapping is unchanged for revive and run; the pump error row still holds, but the pump now runs through pump_end, which adopt calls directly.
@task_item n=11 part=touches
decision/silent-roles-at-internal-error-catch: adopt's worker-panic end still writes internal-error with no document; its catch site moved from pump_child to adopt's own match over pump_end.
@task_item n=12 part=touches
contract/instance-snapshot/agent-session-id: adopt now reads it once after the child ends to decide whether the session resumed; none on a real snapshot means not resumed.
@task_item n=13 part=touches
decision/snapshots-read-through-replay, decision/replayed-snapshot-liveness and contract/snapshot-envelope/unreadable-snapshot: adopt's resumed check reads through read_snapshot_or_replay, and a replayed, absent or unreadable snapshot counts as resumed.
@task_item n=14 part=touches
contract/diag-line/format: the diagnostics line schema's process-exit detail enum gains one word.
@task_item n=15 part=touches
decision/fake-agent-test-seams and name/binary/viola-fake-agent: the fake agent gains the argv-only option --resume-absent, which closes its hooks and exits 1 before any SessionStart when resuming.
@task_item n=16 part=touches
decision/real-cli-probe-local-only, decision/real-claude-local-only and decision/no-live-claude-in-ci: the R5 rig's new r5c line runs only under rehearse over the fake agent; no real claude ran.
@task_item n=17 part=touches
decision/child-cwd-explicit, decision/spawn-directory-confined-to-snapshot and contract/claude-child-argv/resume: unchanged by choice; the premise that adopt must spawn its child in the session's original folder was measured false, so the spawn folder and the --resume argv stay as they were.
@task_item n=18 part=touches
decision/verify-run-e-resumes-run-b-first-session: its probe, resuming from a different folder than the session was made in, is one of the three readings that measured the resume folder immaterial.
@task_fact n=1 slug=established-decisions
fact: viola adopt ends not-resumed when its child exits and the instance's real snapshot holds no agent_session_id: the child's process-exit is logged, then a self process-exit exit 1 detail not-resumed, then its fixed pair or --json document, exit 1; a replayed, absent or unreadable snapshot reads as resumed and ends exit 0.
@task_fact n=2 slug=standard-contracts
fact: viola adopt --json on a not-resumed end prints the error document with error not-resumed and detail null, stderr empty, written after the child released the terminal; this is the one named exception to no document once the child held the terminal, scoped to adopt's not-resumed end and to nothing of run or revive.
@task_fact n=3 slug=occupied-resources
fact: the process-exit detail enum holds not-resumed, adopt's own exit-1 detail, logged on the self subject after the claude-child process-exit.
@task_fact n=4 slug=standard-contracts
fact: adopt's not-resumed human pair is fixed: unable: <name>'s session did not resume, then hint: check the session id - claude --resume <id> must open it by hand, under the same claude config, with <id> the literal word and the session id in no line.
@task_fact n=5 slug=established-decisions
fact: adopt does not start its child in the folder the session was made in; the claude CLI was measured to resume a session by id from an unrelated folder, so adopt's spawn folder stays as revive's start path sets it.
@task_item n=1 part=untouched
The wheel, its claim, take and release, and the human-always-wins rules.
@task_item n=2 part=untouched
Send, paste, confirmation windows and send refusal details.
@task_item n=3 part=untouched
Dialog hold, hook tiers, hook exits and the hook-to-kind map.
@task_item n=4 part=untouched
MCP tools, MCP results and the MCP server's stdin rules.
@task_item n=5 part=untouched
Channel methods, frames, fault codes and protocol versioning; no IPC method was added.
@task_item n=6 part=untouched
Normalised event kinds and event data; no event kind was added.
@task_item n=7 part=untouched
viola list, the board and the agents listing.
@task_item n=8 part=untouched
viola verify, the ledger stamps and the capability ledger; no capability was claimed.
@task_item n=9 part=untouched
Build, CI, dependency policy, toolchain and crate dependency direction; no crate or dependency changed.
@task_item n=10 part=untouched
Hyprland, focus, open and up, and window placement.
@task_item n=11 part=untouched
config.json keys, environment variables and endpoint sockets; none added.
