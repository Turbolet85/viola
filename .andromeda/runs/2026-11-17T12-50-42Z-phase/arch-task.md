@task_prose part=chunk
Turns the supply-chain CI gate green again by admitting the one chrono unmaintained advisory (RUSTSEC-2026-0543, no safe upgrade, brought in by rmcp and schemars) as a single ignore with its reason, under the founder's widened advisory rule.
Re-measures every config and job that reads advisories (the root graph, the fuzz lockfile audit, the weekly advisories run, the per-sync-crate deny runs) to see which read this id and which fail on it.
Closes the rest of v2-37: the in-test boots of the viola-e2e lifecycle and CLI tests are built tied, so their supervisors and fake agents end with their test, killed or not.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left, so the v2-37 narrowing can be lifted at the wrap.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny dependency policy for the root graph, read by the CI supply-chain job — edits: the advisories ignore list (one entry for RUSTSEC-2026-0543) and the comment above it
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the viola-e2e harness lifecycle end-to-end tests, booting a wrapper in-test — edits: the `options` BootOptions built with tied false
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the viola-e2e CLI end-to-end tests, booting a wrapper in-test — edits: the `booted` BootOptions built with tied false
@task_file n=4 path=fuzz/Cargo.lock mark=re-reads
what: the fuzz workspace lockfile audit (cargo deny over the fuzz manifest, advisories and sources), re-measured for whether it reads this advisory id
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly advisories audit run, re-measured for whether it reads and fails on this advisory id
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate cargo-deny runs, re-measured for whether they read and fail on this advisory id
@task_item n=1 part=needs
The cargo-deny dependency policy and its advisories section: one ignore for exactly one advisory id, its reason written beside it naming the upstream crates that bring chrono in and the re-read at each epoch boundary; no other advisory, skip or allow added.
@task_item n=2 part=needs
The CI supply-chain job as the gate this chunk turns green: the chunk's own pre-CI run must show supply-chain green on the pushed sha and the whole run green, and the plan names what would turn it red again (a new advisory id; the same id once a safe upgrade exists).
@task_item n=3 part=needs
The weekly advisory audit and the separately audited fuzz lockfile, and the fuzz workspace's place outside the root cargo-deny graph: which of these advisory readers see this id is re-measured, not assumed.
@task_item n=4 part=needs
The sync crates' own dependency policy runs (per-sync-crate deny config) as advisory readers re-measured for this id.
@task_item n=5 part=needs
chrono as Viola's timestamp crate stays: its own direct use in the root bin and viola-state is untouched, and the rmcp dependency with its exact features is untouched; the ignore covers only the upstream-brought unmaintained advisory.
@task_item n=6 part=needs
Gate steps fail closed: the gate is made green by an admitted, reasoned ignore, never by weakening or skipping the job or its step.
@task_item n=7 part=needs
The viola-e2e in-test wrapper boots (BootOptions) changed from untied to tied, so a supervisor and its fake agent end with their test even when the test is SIGKILLed, as the root tests' boots already do.
@task_item n=8 part=needs
The test-child tie through the HARNESS_TIED_TEST_CHILD and ROOT_TIED_TEST_CHILD names and the census built by the last chunk are used, not rebuilt; a site sweep covers every spelling of each constant.
@task_item n=9 part=needs
A new test kills the tied stamping viola verify and reads through the census that no process of it is left; viola verify as the one stamps writer and the removal of only its own probe dirs bound what the kill may leave behind.
@task_item n=10 part=needs
No test sends a kill at a process it does not own; the proof leaves nothing running; test evidence survives a killed test.
@task_item n=11 part=needs
Every test that waits on a channel or a thread takes a bound under the project's 20 s lint, and root test waits share one bound; a test socket goes under the short /tmp root, outside the product endpoint namespace.
@task_item n=12 part=needs
Each test in its own viola home, cleaned by its owner record only, on the dev-host tmpfs link where the e2e home base rules apply.
@task_item n=13 part=needs
Tests run against the fake agent only: no live claude start, in CI or in the tests; no mutation run in this chunk.
@task_item n=14 part=needs
Every acceptance predicate one that can fail, with the change that would turn it red; every test filter checked to select at least one test; every control fired with a named timeout.
@task_item n=15 part=needs
viola-e2e stays test-only and the test-support and fake-agent features stay out of product builds while the tied boot reaches the harness.
@task_item n=16 part=needs
The v2-37 capability-ledger narrowing (verified for named witnesses alone) is closed by what this chunk proves, never weaker than the note's text; the standing security-plan row for the widened advisory rule is the wrap's, so no master is edited here.
@task_standing n=1
convention: every new test that waits takes a bound under the 20 s lint and uses its own viola home, its sockets under the short /tmp root
home: occupied-resources
@task_standing n=2
convention: every new test runs against the fake agent, never a live claude, and is selected by at least one checked filter
home: ci-cd-approach
@task_standing n=3
convention: a new test that is OS-gated names its reason and names no other OS
home: cross-cutting-patterns
@task_item n=1 part=not_needed
The wheel, send, dialog and confirmation behaviour of the wrapper: the chunk changes how a test boot ends, not anything a wrapper does while it runs.
@task_item n=2 part=not_needed
The MCP tools, their results and the MCP stdin reader: rmcp is only the crate that brings chrono in; no tool or result changes.
@task_item n=3 part=not_needed
The channel methods, frames, fault codes and refusal details: no wire shape is added or changed.
@task_item n=4 part=not_needed
CLI verbs, exit codes, --json documents and human output lines: no verb or exit code changes.
@task_item n=5 part=not_needed
The event log, instance snapshot, ledger stamps envelope and config.json contents: no on-disk format changes.
@task_item n=6 part=not_needed
Hook tiers, hook-to-kind mapping and hook output: no hook behaviour changes.
@task_item n=7 part=not_needed
Release, install, plugin compilation and deployment rows: nothing is built for release or installed.
@task_item n=8 part=not_needed
The Hyprland seam, window features and the terminal fleet console: untouched.
@task_item n=9 part=not_needed
Revive, run start order, child argv and child environment rows: no start or revive path changes.
