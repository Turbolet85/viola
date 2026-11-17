@task_prose part=chunk
Turns the supply-chain CI gate green again: one cargo-deny advisory ignore for RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in by upstream crates), under the founder's widened advisory rule, its reason and its epoch-boundary re-read written beside it.
Re-words the comment above the advisories ignore list so it states both grounds: the rmcp disabled-feature ground and the widened unmaintained ground.
Re-measures every config and job that reads advisories (the root graph, the fuzz lock audit, the weekly advisories run, the per-sync-crate runs) for which of them read this id and which fail on it.
Closes the rest of v2-37: the in-test boots of the viola-e2e harness-lifecycle and CLI tests are built tied, so their supervisors and fake agents end with their test, killed or not.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny supply-chain config of the root graph — the advisories ignore list the supply-chain gate reads — edits: the advisories ignore list (one entry for RUSTSEC-2026-0543) and the comment above it
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the viola-e2e harness lifecycle tests that boot a harness session in-test — edits: the in-test boot options, built tied instead of untied
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the viola-e2e tests of the harness CLI that boot a session in-test — edits: the booted boot options, built tied instead of untied
@task_file n=4 path=.github/workflows/ci.yml mark=re-reads
what: the CI supply-chain job running cargo deny over the root graph and the fuzz lock audit (cargo deny check advisories sources on the fuzz manifest), re-read for which advisories it fails on
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly advisories run, re-read for whether it reads and fails on this advisory id
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate cargo-deny config whose runs read advisories, re-read for whether they read this advisory id
@task_file n=7 path=fuzz/Cargo.toml mark=re-reads
what: the separate fuzz workspace whose lock the supply-chain audit checks for advisories and sources, re-read for whether it reaches this advisory id
@task_item n=1 part=needs
The cargo-deny supply-chain gate and its fail-closed CI job: green again on the chunk's pushed sha, the whole run green; the plan says what would turn it red again (a new advisory id; the same id once a safe upgrade exists, if the tool reads that).
@task_item n=2 part=needs
The no-advisory-ignored rule, under the founder's widened carve-out: an ignore admitted only for an unmaintained advisory with no safe upgrade whose crate an upstream dependency brings in, its reason written beside it, every such ignore re-read at each epoch boundary; the ignore no wider than the one id, no other advisory admitted, no skip or allow added.
@task_item n=3 part=needs
The weekly advisories run and the fuzz-workspace advisories audit: each re-measured for whether it reads this id and fails on it.
@task_item n=4 part=needs
The tokio-banned per-sync-crate deny runs: re-measured for whether they read advisories and this id.
@task_item n=5 part=needs
The rmcp feature limit (server and transport-io) as the first ground stated in the ignore-list comment, kept beside the widened ground.
@task_item n=6 part=needs
The tied test child: the in-test harness boots of the viola-e2e tests take the tie (harness and root tied test child) so a supervisor and its fake agent end with their test, killed or not, as the root tests already do; no test spawns viola or a supervisor untied.
@task_item n=7 part=needs
The tied stamping viola verify run by harness boot: a test kills it and reads that nothing of it is left, through the process census the last chunk built.
@task_item n=8 part=needs
Kill safety: no test sends a kill at a process it does not own; a kill checks pid and start time first; the proof leaves nothing running.
@task_item n=9 part=needs
A site sweep over every spelling of the tie constant, so no in-test boot is left untied.
@task_item n=10 part=needs
The v2-37 ledger narrowing: the chunk closes what its dated note names (verified for named witnesses alone), claimed so the wrap can lift it, never weaker than its text.
@task_item n=11 part=needs
Wait bounds: a test that waits on a channel or a thread takes a bound under the project's 20 s kill line, waits keyed on logged events, never on sleeps or retries.
@task_item n=12 part=needs
Test endpoints: a test socket goes under the short /tmp root, its directory made private.
@task_item n=13 part=needs
Controls and acceptance: every control names its timeout and fires with it, a control id within 16 characters, every acceptance predicate one that can fail with the change that turns it red, every filter selects at least one test.
@task_item n=14 part=needs
Harness cleanup and supervisor lifecycle: the supervisor spawned only by boot, cleanup removes the home, the session record and the supervisor, and the killed test leaves no process of its session.
@task_item n=15 part=needs
No live claude start and no mutation run in this chunk; the widened rule as a standing plan row is the wrap's, no master edited.
@task_standing n=1
convention: every new test follows the test naming, runs in parallel on its own fresh home under the e2e home root, and is held to no sleep and no retry
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: every new test that spawns a wrapper or a supervisor holds it in a drop guard and spawns it tied
home: integration-test-strategy
@task_standing n=3
convention: every new test is selected by its layer filterset and binary prefix and is covered by the per-test kill line
home: 5-command-implementation · bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: every new test asserts against literal oracles on cross-process behaviour, and its synthetic inputs carry no real data
home: 11-test-anti-patterns-never-do-these
@task_item n=1 part=not_needed
The MCP tool, MCP schema and MCP reply contracts: the chunk changes no MCP surface.
@task_item n=2 part=not_needed
The wrapper channel, JSON-RPC error and link or unlink contracts: no channel method or body changes.
@task_item n=3 part=not_needed
The viola send, answer, wait, last, release and list contracts and their exit codes: no product verb changes.
@task_item n=4 part=not_needed
Hook decision bodies, hook deadlines and the performance budgets: no hook code or perf run is touched.
@task_item n=5 part=not_needed
Fake-agent command-line, input, screens and receipt contracts: the fake agent is used as it is.
@task_item n=6 part=not_needed
Fixture recording, scrubbing and hygiene rows: no fixture is recorded or added.
@task_item n=7 part=not_needed
Coverage floors and the mutation gate, its verdict and its cadence: the chunk runs no mutation and moves no threshold.
@task_item n=8 part=not_needed
Fuzz corpora, fuzz targets and property-test generators: no parser surface changes.
@task_item n=9 part=not_needed
The pre-push gate, secret scan and CI artifact contracts: no artifact or pre-push stage changes.
@task_item n=10 part=not_needed
Revive, run start and strict-modes refusal contracts: no start path or refusal code changes.
@task_item n=11 part=not_needed
The standing live rig and real-claude local-only rows: no live claude start in this chunk.
