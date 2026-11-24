@task_prose part=chunk
Builds a new CLI verb `viola adopt` that takes a session-shaped id, an operator-given name and an optional parent, and starts a viola wrapper whose claude child resumes that plain claude session, so it stands on the board as a named instance with its lineage, driven like any wrapped session.
It is built on revive's start path: the resume argv, the `claude agents --json` live-elsewhere preflight with its closed session-live refusal, and the start shared with run; a session live in another claude is refused and no running process's terminal is taken over.
Under the conservative arm adopt never ends or signals a process it did not start: it resumes only a session live nowhere and refuses a live one with a hint to end it by hand; every live reading is gated on the operator.
Folded CI item: re-measures the two security-negative env-capture cases (one killed at the 20 s line on CI attempt 1, its sibling passing at 16.9 s) for what they wait on and what they cost locally and under coverage, and gives them a margin by their kill class or reduced work, never by raising the kill line for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the revive verb's start path — the resume argv, the claude agents preflight and its session-live refusal, the start shared with run — the component adopt is built on
@task_file n=2 path=.config/nextest.toml mark=edits
what: the nextest runner profiles and per-test kill lines (the 20 s kill line for stamped-home and verify-driven tests, the kill classes for cases that wait by design) — edits: the kill class or override that covers the two env-capture security-negative cases
@task_item n=1 part=needs
A new CLI verb adopt with its argument shape (a session-shaped id, --name, optional --parent, --json): its row in the grouped verb table of viola --help, its exit codes and refusal words, its human lines and its one --json document per run.
@task_item n=2 part=needs
The session id's shape is checked by the existing closed session-id check after the strict-modes check and before any other home read; a malformed id is refused with a closed word and spawns nothing.
@task_item n=3 part=needs
The live-elsewhere refusal: a session listed live by the claude agents listing is refused with the session-live word before any child is spawned, in the revive preflight order; adopt's behaviour when the listing is unreadable follows the agents-list rule (an unreadable listing reads unknown) and the session-live reading's fail-open rule.
@task_item n=4 part=needs
Adopt never ends, signals or takes over the terminal of a process it did not start; the refusal of a live session carries a hint to end it by hand, and each exit cause has its own hint with no release suggested.
@task_item n=5 part=needs
The start shared with run and revive: the run and revive exit-1 start refusals (already-live, stale-heartbeat, squatted-name, socket-dir-refused, strict-modes-failed, pinned-hash-mismatch) and the run start events order apply to an adopt start.
@task_item n=6 part=needs
The resumed claude child gets the resume argv through the agent crate's resume-args builder, the claude child environment strip, and its viola name, instance dir and binary in its environment; its process-start claude-child record carries env_kept and the stripped counts.
@task_item n=7 part=needs
The given name and parent land as the instance's first snapshot (pinned path, pids, endpoint, started_at) and as the lineage record the board reads; viola list shows the adopted instance as a running wrapped instance.
@task_item n=8 part=needs
A second adopt of the same session id, and an adopt onto a name already held, each end in a defined closed outcome (the exact words are measured at P3).
@task_item n=9 part=needs
The adopted session's cwd, model and effort take a defined source where no snapshot ever recorded them; a missing cwd follows the cwd-missing refusal; a replayed snapshot carries no endpoint or cwd.
@task_item n=10 part=needs
The event log and process-log catalogs: the first record of an adopt start (the start wheel record), process-start records, and every new refusal or start log line in the codes-only, corr-carrying JSON line format.
@task_item n=11 part=needs
Tests of adopt run against the fake agent resuming a transcript and the agents-list stub on a test-scoped PATH, under a test-owned user home; no test reads, lists, ends or resumes the host's real claude sessions.
@task_item n=12 part=needs
Every live reading of adopt (it ends an operator session and spends real claude starts) is local-only and gated on the operator; a clause the fakes cannot carry is planned gated, and a fake-agent run is never recorded as proof of real-CLI behaviour.
@task_item n=13 part=needs
Error bodies and human output of adopt are sanitized: no path or upstream text in error bodies, control characters escaped in human lines, no SGR without a terminal.
@task_item n=14 part=needs
Folded CI item: the two env-capture security-negative cases (env name with the user word reads fail and records nothing; kept name reaches no stamp or record) are timed per case against their kill line locally and under the coverage-instrumented run, with what each waits on said from the code.
@task_item n=15 part=needs
Folded CI item: the margin comes from the tree's own form for a case that waits by design — the case moved into a kill class (as the verify-window 80 s class) or its work reduced — never the 20 s per-test kill line raised for every test, no retries, no sleeps; acceptance is a measured time per case against its kill, not a green run.
@task_item n=16 part=needs
Kill lines stay ordered: a per-test kill class stays below the mutants' build timeout and the harness abort wait stays below the CI kill, and root waits stay bounded below the kills.
@task_standing n=1
convention: every new test creates its own fresh private home under the e2e home root and seeds its own data; on-disk product state is produced by viola, never hand-written
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=2
convention: every test-started viola drops the inherited and session environment, re-adds LLVM_PROFILE_FILE where it clears the env, and is spawned tied to a drop guard
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=3
convention: tests wait on logged events within a bound, never on sleeps or retries, and never parse the rendered child screen
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: CLI fixed strings and human-mode output asserted by literal oracles, one --json document per verb asserted against the contract
home: 5-integration-test-strategy · integration-test-strategy
@task_standing n=5
convention: test function naming, inline unit tests, rstest case tables and root integration binaries prefixed by surface
home: 2-test-strategy · test-strategy
@task_standing n=6
convention: external processes in tests are real stub executables on a test-scoped PATH; mocks only at product seams
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=7
convention: one negative test per security control, each new refusal witnessed before any child spawn
home: e2e-test-strategy · 6-e2e-test-strategy
@task_standing n=8
convention: synthetic test inputs carry a canary and fixtures carry no real data
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=9
convention: every new process log line is structured JSON with its event, corr and codes only, detail apart
home: log-format
@task_item n=1 part=not_needed
Hook decision bodies, dialog raising, holding and answering (permission, plan, question) and their insta snapshots: adopt raises and answers no dialog.
@task_item n=2 part=not_needed
Send confirmation internals — paste framing, trailing trims, /clear and /compact confirmation, the send window — and the fake agent's input contracts for them: adopt adds no send path.
@task_item n=3 part=not_needed
Hook performance budgets, hyperfine invocation and the spine deadline: adopt is no hook.
@task_item n=4 part=not_needed
The vt100 screen feed, compiled screen signatures and the live rig's KR arm: adopt reads no screen.
@task_item n=5 part=not_needed
The harness's own surfaces — status document and states, supervisor exit file, cleanup document, pre-push stages, details and reasons: adopt changes none of them.
@task_item n=6 part=not_needed
Wrapper channel JSON-RPC error codes, MAX_FRAME and frame faults: adopt adds no channel method.
@task_item n=7 part=not_needed
Workflow audit, SHA-pinned Actions, CI artifact names and retention: the chunk edits no workflow.
@task_item n=8 part=not_needed
Windows and macOS support, a browser suite, a database, outbound HTTP and a deploy stage: absent from the product.
