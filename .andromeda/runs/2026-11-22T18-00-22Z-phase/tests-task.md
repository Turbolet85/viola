@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
Measurements come first, each from a test or a no-start probe and each a number or a closed word: the chrono advisory's upgrade path, the send slot's claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify's trust-record residue, the boot supervisor test's timing.
Mends follow, each held by a test: an end for a claimed send line whose settle never comes, a bound of its own for verify's agents read, the boot test's binary copies moved out of its timed body with its abort deadline below its kill, and the mutants profile's kill on two fake-agent drift replay tests read against a cold baseline.
Killing tests for the seven agent-crate mutation survivors (env_names_fault first) and the three paste-writer survivors; at discretion a framing-turns step function, one source for the relied-row count, and the testing rule's paths gap.
No live start, no mutation run, no witness run, no timeout raised and no retry; a new closed word, a new time bound's value or a split is the operator's fork.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that claims and settles a sent line and waits on its confirmation — edits: SendSlot::confirm, the Waiting or Claimed wait arm, the claimed-line settle after the snapshot update
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded read of the claude agents listing (agents --json) inside Run B — edits: the read's time bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude agent ledger of relied rows, its env-name checks and probes — edits: inline unit tests only, killing tests for env_names_fault and the other surviving mutants
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the screen read for compiled signatures of the Claude agent — edits: inline unit tests only, killing tests for its two surviving mutants
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump and its bounded paste writer — edits: inline unit tests only, killing tests for the three paste-writer surviving mutants
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the harness lifecycle tests of boot, supervisor and cleanup — edits: the boot supervisor test, its binary copies moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the harness boot command that builds, stamps, starts wrappers and supervises readiness — edits: ABORT_DEADLINE, held strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles and per-test kill lines — edits: the mutants profile's kill on contract_fake_agent_drift's two replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny supply-chain configuration, its advisory ignore list — edits: the RUSTSEC-2026-0543 chrono ignore entry and its reason line
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the testing rule file for agents — edits: its paths: front matter
@task_item n=1 part=needs
A claimed send line whose settle never comes (the snapshot update fails before the settle) gets an end: send confirmation no longer waits forever on a claimed line; measured first under a real failing snapshot write, then mended, a test holding it.
@task_item n=2 part=needs
The send result and its records when a claimed line ends without a settle: the send confirmation window, the unconfirmable or not-delivered outcome and the send log records the ended claim must yield, and the snapshot single-writer rule the failing write runs under.
@task_item n=3 part=needs
Verify's bounded agents listing read gets a bound of its own, measured under a stalled host; never a retry and never a loosened bound without that measurement; an unreadable or timed-out agents listing still reads unknown.
@task_item n=4 part=needs
Every external reader stays bounded and keeps its cap: the agents read's new bound is a named time bound, and a new time bound's value is the operator's fork, carried as a named member of the bound lists without dropping another member's words.
@task_item n=5 part=needs
Killing tests for the seven agent-crate mutation survivors: the ledger's env-name fault check (env names of the recorded Claude fixtures and the agent identity env strip), the ledger's relied-row and probe lines, and the screen signature read lines; tests only, no product change.
@task_item n=6 part=needs
Killing tests for the three paste-writer survivors of the PTY pump: the bounded paste write, a cut paste ending the send paste-cut with the wheel unmoved, and the paste framing; tests only, no product change.
@task_item n=7 part=needs
The surviving mutants are read by the next boundary's mutation tier: mutation gate thresholds and the survived count, mutants tested by their owning package, host-unreachable mutants recorded not caught; no mutation run inside this chunk.
@task_item n=8 part=needs
The boot supervisor test: binary copies out of the timed body, the harness boot abort deadline strictly below the test's kill, root waits bounded below the kills; the boot deadline is not a performance budget; measured as a number, never by raising the kill.
@task_item n=9 part=needs
The mutants nextest profile's 15 s kill on the fake-agent drift replay tests read against a cold parallel baseline: kill lines stay below cargo-mutants' timeout, per-test kill bounds and the WITHIN bound for other waits; no kill raised without that measurement.
@task_item n=10 part=needs
A viola verify ended by SIGKILL, measured under a test-owned user home and a test-owned Claude config: whether it leaves a project trust entry pointing under its scratch root; found, a named residual or a mend; the kill witness for a tied verify, trust-start folders outside any repository; nobody reads or edits the operator's real config.
@task_item n=11 part=needs
The chrono advisory RUSTSEC-2026-0543 stays ignored unless measured otherwise: whether a safe chrono upgrade exists and how cargo-deny treats an ignored id once one does; found, the ignore leaves with the upgrade; none, the re-read is re-pinned to the next boundary and the reason line keeps saying it is re-read at each boundary; the carve-out from the no-ignored-advisory rule.
@task_item n=12 part=needs
The cargo-deny supply-chain gate and the weekly advisory scan the advisory entry is read by: the gate stays fail-closed and every ban keeps its negative probe whether the ignore stays or leaves.
@task_item n=13 part=needs
Every measurement is a test or a no-start probe giving a number or a closed word: no live start, no rig start of a real claude, no witness run, no mutation run; real claude never in CI and never in this chunk; a new closed word changes only by the operator's decision.
@task_item n=14 part=needs
No test retry, no sleep synchronisation and no raised timeout: the zero-flakiness budget holds for every stall the measurements expose; waits key on logged events or handles, never on time alone.
@task_item n=15 part=needs
Every verify started by a test of this chunk is tied to a test-owned home and a test-owned runtime dir, under a test-owned user home; no root test spawns viola untied and no viola-e2e test boots a harness session untied.
@task_item n=16 part=needs
At discretion, each named with its decision: one step function for the framing turns' per-paste probes in verify and a look at the send path's guarded section; the relied-row count from one source and the stamped-set table held once (every ledger row keeps its verify probe, the version gate needs every relied row); the testing rule's paths gap.
@task_item n=17 part=needs
Where a discretionary item restructures code (framing turns step function, relied-row count source), what that code does stays as it is: the framing turns' per-paste probes, the relied-row count and the stamped set keep their meaning, held by the existing verify and ledger probe tests.
@task_standing n=1
convention: each new test function is named by the test-function naming rule and, where a table of cases, written as rstest cases; unit tests inline in the module they test
home: 2-test-strategy
@task_standing n=2
convention: each new test seeds its own data in its own fresh home and temp directories, never shared state across parallel tests
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new test asserts on literal oracles and public observable behaviour, never on a Debug string or fixture internals, with no sleep, no retry and no ignore parking
home: 11-test-anti-patterns-never-do-these
@task_standing n=4
convention: each new fault in a test is a real filesystem or process fault and time is faked only through the injected clock seam; doubles only on owned seam traits
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=5
convention: each new test that spawns viola or a wrapper holds it tied in a drop guard and spawns it on a thread that outlives the child
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=6
convention: each new log line follows the structured JSON process log format with codes only and no secrets
home: log-format
@task_item n=1 part=not_needed
MCP server tools, tools/list, MCP tool results, invalid-params fields and the MCP reply screen: the chunk changes no MCP surface.
@task_item n=2 part=not_needed
Hook dialog decision bodies, dialog answer mapping, dialog hold expiry and hook deadline perf budgets: no hook or dialog path is touched.
@task_item n=3 part=not_needed
Wrapper channel JSON-RPC error codes, link and unlink methods and their event data: the channel protocol is unchanged.
@task_item n=4 part=not_needed
viola list board columns, revive refusal codes and exit causes, run start refusals and run start events: no start or list behaviour changes.
@task_item n=5 part=not_needed
CLI help table, human-mode lines, SGR and control-character escaping of CLI output: no CLI wording changes.
@task_item n=6 part=not_needed
Fuzz targets, fuzz corpus replay and nightly fuzzing; property-test parser invariants: the chunk adds no parser.
@task_item n=7 part=not_needed
Hyperfine performance budgets for hooks and the perf CI job: no hook path is timed or changed.
@task_item n=8 part=not_needed
CI artifacts, artifact retention, secret-scan hit classes and uploads, and the pre-push gate's details and stages: no workflow or gate tool changes.
@task_item n=9 part=not_needed
Harness status document, status state, status polled fields and the agent-run logs stream: the harness status surface is unchanged.
@task_item n=10 part=not_needed
Heartbeat staleness classification, event log tailing and torn-append healing: no on-disk liveness or log reading changes.
@task_item n=11 part=not_needed
The live rig's KR arms, standing live rig and live readings: the chunk starts nothing live.
@task_item n=12 part=not_needed
Absences of a browser suite, Windows and macOS support, a database, outbound HTTP and a deploy stage: nothing here reaches them.
