@task_prose part=chunk
Turns the supply-chain CI job green again by admitting one cargo deny advisory ignore, RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in by rmcp and schemars), under the founder's widened advisory rule, its reason and epoch-boundary re-read written beside it.
Re-words the comment above the advisories ignore list so it states both grounds: the rmcp disabled-feature ground and the widened unmaintained ground; Viola's own direct chrono use stays.
Re-measures every config and job that reads advisories (root graph, the fuzz lock audit, the weekly nightly advisories run, the per-sync-crate deny runs) for which read this id and which fail on it.
Ties the in-test boots of the viola-e2e lifecycle and cli tests so their supervisors end with their test, killed or not, and adds a test that kills the tied stamping viola verify and reads that nothing of it is left.
Closes what the narrowed v2-37 ledger note names, using the existing harness tie and process census; acceptance is the chunk's own pre-CI run green with supply-chain green.
@task_file n=1 path=deny.toml mark=edits
what: the cargo deny config of the root dependency graph, read by the supply-chain CI job — edits: the advisories ignore list (one entry for RUSTSEC-2026-0543) and the comment above it
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the viola-e2e harness lifecycle integration tests that boot a wrapper in-test — edits: the `options` BootOptions boot, untied today
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the viola-e2e CLI integration tests that boot a wrapper in-test — edits: the `booted` BootOptions boot, untied today
@task_file n=4 path=.github/workflows/ci.yml mark=re-reads
what: the supply-chain CI job's cargo deny advisories check over the root graph and the fuzz lock audit, re-read against the failing run's log
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly scheduled cargo deny advisories run, re-measured for whether it reads this advisory id
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the cargo deny config of the per-sync-crate runs, re-measured for whether it reads this advisory id
@task_file n=7 path=fuzz/Cargo.toml mark=re-reads
what: the fuzz workspace manifest whose lock the advisories and sources audit reads, re-measured for this advisory id
@task_item n=1 part=needs
The obs dependencies must still pass cargo deny: the chunk's change admits exactly one advisory id, no other advisory, no skip and no allow, and the ignore is no wider than RUSTSEC-2026-0543.
@task_item n=2 part=needs
A cargo deny failure is a failure and CI gates fail closed: the supply-chain gate turns green only by the admitted ignore, and the plan names what would turn it red again (a new advisory id; the same id once a safe upgrade exists).
@task_item n=3 part=needs
Viola's own direct chrono dependency for timestamps stays as it is; the chunk replaces no chrono use and changes no timestamp format.
@task_item n=4 part=needs
The supply-chain CI job reads the advisory from the advisory database; its red is caused there, not by a manifest, lock, deny config or workflow change, and P3 re-reads that claim from the failing run's log.
@task_item n=5 part=needs
Every in-test boot of the e2e lifecycle and cli tests builds its boot options tied, so the booted wrapper's supervisor and its fake agent end with their test, killed or not, as the root tests' boots already do.
@task_item n=6 part=needs
A new test SIGKILLs the tied stamping viola verify (which runs only against the fake agent in CI) and reads, through the existing process census, that no process of it is left.
@task_item n=7 part=needs
viola verify is the only stamps writer; the kill test drives that writer and must leave no running process and no process it does not own signalled.
@task_item n=8 part=needs
CI verify runs only against the fake agent and the real-CLI verify never runs in CI: the kill test and the retied boots start no live claude.
@task_item n=9 part=needs
The OS-gated test census holds the new and changed tests: the tie constants are swept under every spelling, and every filter the plan names selects at least one test.
@task_item n=10 part=needs
A test that waits on a channel or a thread takes a bound under the project's 20 s lint; no sleep-based log waits and no retry policies.
@task_item n=11 part=needs
Integration-test homes live under one root that the gates scan, and are kept until the gate steps run; a test socket goes under the short /tmp root.
@task_item n=12 part=needs
The zero-panics gate and the schema conformance gate still run over the homes the new and changed tests leave; a killed process leaves no unlogged panic line.
@task_item n=13 part=needs
Every acceptance predicate can fail, with the change that would turn it red named; every control names its timeout and fires with it; no mutation run.
@task_standing n=1
convention: every new integration test keeps its test home under the one scanned e2e home root, kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: every new test that waits on a process, channel or log does so with a bound, never a sleep-based wait and never a retry
home: slo-invariants-telemetry-budgets · 10-slo-invariants-telemetry-budgets
@task_standing n=3
convention: every diagnostics line a new test's processes emit stays schema-checked and panic-free under the CI gates
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Metric coverage: counters, gauges, histograms and quantiles derived from logs; the chunk adds no metric and no derived count.
@task_item n=2 part=not_needed
The MCP stdio server tools and their refusal results and stdin frame bounds; the chunk changes no MCP surface.
@task_item n=3 part=not_needed
Send, wait, dialog, wheel and confirm-window span chains and their span attributes; the chunk adds no span and touches no send or dialog path.
@task_item n=4 part=not_needed
Correlation by event, conn and rpc id across channel, dialog, hook and send lines; no correlated line is added or changed.
@task_item n=5 part=not_needed
Error capture and reporting: the panic outcome per role, the detail-file error line and any future opt-in error reporter; no product error path changes.
@task_item n=6 part=not_needed
The agent-run logs and status output for snapshot paste, and the heartbeat ticks and liveness thresholds; the chunk reads neither.
@task_item n=7 part=not_needed
PII scrubbing of payload types, veil redaction and the content-bearing detail-file line fields; no logged field or payload type is added.
@task_item n=8 part=not_needed
Exit detail codes and refusal stderr pairs of run, revive, link, list and the CLI frame verbs; no exit cause or refusal word changes.
