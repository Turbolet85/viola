@task_prose part=chunk
Builds a new viola CLI verb that switches a running viola-wrapped session, a named instance, to a given model and effort.
The verb records the new pair, so a later resume through revive or up replays the new pair and not the one named at launch.
The verb has no side effect on the user's own settings. It writes nothing under the user's claude home or a project's claude settings, and it never uses the agent CLI's own model command as its channel.
It adds the verb's argument surface, closed words, exit codes, hints, human lines, --json document, its event-log record and its process-log lines next to the existing tables.
Nothing is run live: no real session is read, switched, ended or resumed, and no real settings file is read.
@task_file n=1 path=src/cmd/revive.rs
what: the revive verb, which resumes a recorded session and whose --model and --effort override the recorded model and effort pair on the resumed child's argv (the given pair). It is the existing start path a restart-based switch may ride.
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent component holding the closed effort set (low, medium, high, xhigh, max), its effort check, and the compiled default model and effort pair.
@task_item n=1 part=needs
A new CLI verb in the verb table: it takes a named wrapped instance and a model and effort, switches that session to the pair for its next turn, and returns one outcome per run.
@task_item n=2 part=needs
The recorded pair a resume replays: the switch leaves a record (a launch model and effort line in the instance event log, and the launch field the snapshot holds and replay recovers) so that revive and up start the child on the new pair. Snapshot readers go through read-or-replay, and the snapshot and its log keep their single writers.
@task_item n=3 part=needs
No side effect on the user's settings: the verb writes nothing to the user's claude home files or to a project's claude settings files. Tests prove those files are byte-identical before and after a switch, in a test-owned user home.
@task_item n=4 part=needs
The agent CLI's own model command is not the switch channel, because it writes the user-scope model key. The verb never types or sends it into the session.
@task_item n=5 part=needs
Closed words of the argument surface: model words are checked by the core model-word check and effort words by the agent's closed effort set. The compiled default pair is the fallback. Any new closed word is changed only by a recorded decision.
@task_item n=6 part=needs
The verb's exit codes and exit-cause catalogue entries: each cause has its own hint, and no hint suggests release to a driver.
@task_item n=7 part=needs
Human-mode output lines of the verb: they are pinned as literals, they escape control characters, they carry no SGR or other escape bytes without a terminal, and their error lines hold no path or upstream text.
@task_item n=8 part=needs
The verb's --json output: one JSON document on stdout per run, with an ok document and a failure document (refusal, instance-unreachable, internal-error) shaped like the other verbs' documents.
@task_item n=9 part=needs
The events-log record of a switch: a new event kind in the events catalogue, written as one complete event line with the standard keys and ts. Readers that do not know the kind count it and skip it; they never fail on it.
@task_item n=10 part=needs
The verb's process-log lines: structured JSON lines through the one tracing install, codes only with detail kept apart, corr where always defined, closed details added to the observability plan's closed-detail list, and no secrets, prompt text or session names.
@task_item n=11 part=needs
Indirect, through the instance it targets: the verb checks the instance first, reaches only the recorded wrapper at the endpoint read from the snapshot, and verifies the server's identity, the socket directory and strict modes before any frame or read. Its unreachable outcomes take the existing instance-unreachable causes and exit 21.
@task_item n=12 part=needs
Indirect: an unwrapped agent-list row name is never a switch target, and a name that is not a running wrapped instance is refused with the existing cause words.
@task_item n=13 part=needs
Indirect, through the existing start path: if the switch restarts the session on the new pair, it goes through revive's preflight order and its refusal codes (session-live, already-live, stale-heartbeat and the rest). A start appends exactly one launch model and effort record, and the pair reaches the claude child's argv. Tests read that argv from the live child's cmdline.
@task_item n=14 part=needs
Indirect, through the claude child: a restarted child keeps the child environment strip, so nothing of viola's or of the user's CLAUDE* identity leaks into it. The child runs under the test-owned user home in tests.
@task_item n=15 part=needs
The verb's tests run against the fake agent standing in for claude, as a subprocess, in a fresh test-owned home. They never touch the real claude CLI, the host's claude sessions, or the operator's real claude home or board.
@task_item n=16 part=needs
Error payloads of the verb are sanitized: thiserror messages interpolate nothing, and every new error writer is named.
@task_standing n=1
convention: every new test creates its own isolated, private home and seeds its own data; on-disk product state comes from real viola runs and is never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=2
convention: new unit tests sit inline, case tables use rstest, test functions follow the naming rule, and a new integration binary is prefixed by its surface
home: 2-test-strategy
@task_standing n=3
convention: every new test is free of retries and sleeps, waits key on logged events, and the test shares no mutable state and mutates no process-wide environment
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: every test oracle is a literal; no comparison by Debug string, and only public behaviour is asserted
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: every new fixture or test input is synthetic, carries the canary, and holds no real prompt, path or user name
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=6
convention: external processes in new tests are real stub executables (the fake agent on a test-scoped PATH); only owned seams are mocked, and time is faked through the clock seam
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=7
convention: every viola process a new test starts is tied to a drop guard, drops inherited session variables and runs under a test-owned user home
home: integration-test-strategy
@task_item n=1 part=not_needed
Hook dialog decision bodies and their insta snapshots (permission, question, plan approve or revise answers): the switch answers no dialog and writes no decision body.
@task_item n=2 part=not_needed
The wrapper channel's link and unlink methods, link records and link-since rules: the switch links and unlinks no driver pair.
@task_item n=3 part=not_needed
Window placement, raise and re-open readings proven live (R3, R4), and the live rig's KR arm and hint-cell readings: the chunk opens no window and runs nothing live. The console's keyboard reach of the switch belongs to the console actions chunk.
@task_item n=4 part=not_needed
The test harness's own command contracts (session record, status document, supervisor, cleanup document, boot readiness codes, pre-push detail): the chunk adds no harness command or field.
@task_item n=5 part=not_needed
Performance budgets for hook deadlines and the hyperfine perf job: the chunk adds no hook path and changes no hook deadline.
