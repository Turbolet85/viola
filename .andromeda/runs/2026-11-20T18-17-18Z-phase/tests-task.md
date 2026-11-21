@task_prose part=chunk
Two `claude` behaviours the product relies on without a capability-ledger row become relied ledger rows in viola-agent-claude, each with a `viola verify` probe.
The harness prompt prefixes row: a second session sends into a verify child and the prompt's raw start is read to tell the harness-origin prefix that keeps a turn off the wheel.
The R8 identity floor row: the eleven measured CLAUDE* parent-identity environment names the child is always stripped of, captured as names and never as values.
The new rows are counted in a `viola verify` stamp from a live run in the rig's scratch home only, with the recorded fixture set re-recorded or recorded for the chosen CLI version and replayed by the fake agent in CI.
It re-reads what the tree shows of the standing relied-row count, the stamped fixture sets and the default-version constants.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of the claude agent: its rows, the relied-row list, each row's verify probe and the stamp's relied-id set — edits: the relied-row list, the ledger's rows
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the harness prompt prefixes: the compiled raw-prefix set by which a prompt is filed as a harness-injected turn that never moves the wheel
@task_file n=3 path=crates/viola-agent-claude/src/lib.rs
what: the identity floor: the eleven CLAUDE* parent-identity environment names the claude child's environment is always stripped of
@task_item n=1 part=needs
The capability ledger gains two relied rows, the harness prompt prefixes and the R8 identity floor, so the relied-row count, the stamp's relied-id set and the version gate that needs every relied row all move from 27.
@task_item n=2 part=needs
Every ledger row has a verify probe: each new row needs its own `viola verify` probe, held by the ledger probe contract suite.
@task_item n=3 part=needs
The harness-prefix probe: a second session sends into a verify child and the prompt's raw start is read to tell the harness-origin prefix; the harness-injected turn keeps the wheel and its prompt-submitted event carries origin harness.
@task_item n=4 part=needs
The identity-floor probe: the claude child environment strip of the eleven CLAUDE* names, the R8 strip, and the process-start claude-child record's env_kept, env_stripped_known and env_stripped_count; the floor is tested against a literal copy of its names.
@task_item n=5 part=needs
Environment names, never values, captured: no environment value may reach a fixture, stamp, log line, diagnostics file or screen read, and a failing test proves a captured value is refused; logs and diagnostics carry no secrets.
@task_item n=6 part=needs
The hidden `viola hook --capture` arm and its closed set of capture ids, which a new probe may extend.
@task_item n=7 part=needs
`viola verify` runs and its child set: the new probes' runs, the step line, the summary line, the stamped line and the last line count the new rows; a failing row is no refusal.
@task_item n=8 part=needs
`viola verify --record` and its scrub: home paths and usernames scrubbed, a dirty payload refuses the recording, recording only at zero failing rows, the unclean-fixture exit detail, and the handling of an environment name that itself carries private text.
@task_item n=9 part=needs
The recorded fixture set per CLI version: fixtures named by hook event, checked against their tolerant schema, walked for hygiene, synthetic text only, never from real prompts, refreshed only by live verify.
@task_item n=10 part=needs
The fake agent replays the new probes in CI byte for byte against the recorded fixtures; every CLI-version fixture set is replayed; stamps in tests come only from verify against the fake agent.
@task_item n=11 part=needs
Stamped live: the live rig's verify arm stamps the new rows in the rig's scratch home only; live verify never runs in CI, the real claude CLI is local-only, a live failing reading is recorded as a reading and never as verification, and tests never read the host's claude sessions.
@task_item n=12 part=needs
The unverified CLI gate: with two more relied rows, a CLI version is verified only when its stamp holds every relied row; stamps refuse a widened ledger.
@task_item n=13 part=needs
The default CLI version: the two default-version constants and the harness's one default CLI version matching the fake agent, read for 2.1.328 and 2.1.329 across compatibility tests over CLI versions.
@task_item n=14 part=needs
Every test verify start is tied to a test-owned home and runs under a test-owned user home, with the tied-verify kill witness and the per-test kill bound on verify-driven tests.
@task_item n=15 part=needs
Screen reads stay to compiled signatures: the probe's verdict comes from records and the prompt's raw start, never from parsing the rendered child screen, and no assertion goes beyond compiled literals.
@task_item n=16 part=needs
Test-started viola drops inherited environment and session variables, cleared environments re-add LLVM_PROFILE_FILE, and no test mutates the process-wide environment: the identity-floor probe's tests set the CLAUDE* names per child.
@task_item n=17 part=needs
The CI secret scan's claude-stripped hit class and content canary, which a captured environment value in any kept file must trip.
@task_standing n=1
convention: each new test is named by the test function naming rule and placed in its layer by binary prefix and nextest filterset
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each new test waits on logged events, with no sleep synchronisation and no retry
home: 2-test-strategy · 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: each new test creates its own private home and seeds its own data; product state comes from real runs, never hand-written
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=4
convention: each new test writes its oracle as a literal and asserts on the contract, not fixture internals or a Debug string
home: 11-test-anti-patterns-never-do-these
@task_standing n=5
convention: each new synthetic test input carries a canary and fixture text stays synthetic
home: e2e-test-strategy · 7-test-data-fixtures
@task_standing n=6
convention: each new log line is a structured JSON process-log line with the codes-only role-log fields and its corr where defined
home: log-format
@task_standing n=7
convention: each new code path stays inside the comprehensive-tier coverage floors and the zero-flakiness budget
home: 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
MCP server surfaces: tools/list, tool results, invalid-params fields and the MCP reply screen; the chunk adds or changes no MCP tool.
@task_item n=2 part=not_needed
viola revive: its refusal codes, exit codes and human or JSON output; the chunk touches no revive path.
@task_item n=3 part=not_needed
viola list: the human board, list --json rows and their control-character escaping.
@task_item n=4 part=not_needed
Dialog decision bodies for permission, plan and question answers, and viola answer's refusals.
@task_item n=5 part=not_needed
Link and unlink methods of the wrapper channel, their records and their survival across a wrapper restart.
@task_item n=6 part=not_needed
Heartbeat staleness classification and instance liveness states.
@task_item n=7 part=not_needed
Harness session record, status document, status states, boot readiness codes and cleanup document contracts.
@task_item n=8 part=not_needed
Fuzz targets, fuzz corpora and their replay.
@task_item n=9 part=not_needed
CI artifact names and retention, and the weekly advisories run; the chunk adds no workflow or artifact.
@task_item n=10 part=not_needed
Absences of a database, outbound HTTP, a browser suite, a deploy stage, and Windows or macOS support.
@task_item n=11 part=not_needed
Mutation run verdict contracts and the harness run document's mutants fields.
@task_item n=12 part=not_needed
The pid-file and session.json lifecycle contracts of the harness.
