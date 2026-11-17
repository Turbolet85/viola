@task_prose part=chunk
Turns the supply-chain CI gate green again by admitting one reasoned advisory ignore, for RUSTSEC-2026-0543 (chrono unmaintained, no safe upgrade, brought in by upstream rmcp and schemars), under the founder's widened ignore rule, and re-words the deny config comment to state both ignore grounds.
Re-measures every config and job that reads advisories (root graph and its CI job, the fuzz lockfile's own audit, the weekly scheduled advisory run, the per-sync-crate deny runs) for which read this id and which fail on it; re-reads the claim that the red comes from the advisory database, not from repository history.
Ties the in-test wrapper boots of the e2e harness lifecycle test and the e2e CLI test, so their supervisors end with their test, killed or not, as the root tests' already do.
Adds a test that kills the tied stamping viola verify and reads that nothing of it is left, closing the narrowed v2-37 ledger note so the wrap can lift it.
@task_file n=1 path=deny.toml mark=edits
what: the cargo-deny config for the root dependency graph that the supply-chain gate reads — edits: the advisories ignore list (one new entry for RUSTSEC-2026-0543) and the comment above that ignore list
@task_file n=2 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the e2e harness lifecycle test that boots a wrapper in-test through the harness boot options — edits: the options boot, from untied to tied
@task_file n=3 path=crates/viola-e2e/tests/cli.rs mark=edits
what: the e2e CLI test that boots a wrapper in-test through the harness boot options — edits: the booted boot, from untied to tied
@task_file n=4 path=.github/workflows/ci.yml mark=re-reads
what: the supply-chain CI job, the cargo-deny advisories gate on the root graph, and the fuzz lockfile's own advisories and sources audit; re-read for whether it reads this advisory id and fails on it
@task_file n=5 path=.github/workflows/nightly.yml mark=re-reads
what: the weekly scheduled advisory run; re-read for whether it reads this advisory id and fails on it
@task_file n=6 path=deny-sync.toml mark=re-reads
what: the per-sync-crate cargo-deny config its deny runs read; re-read for whether those runs read this advisory id and fail on it
@task_file n=7 path=fuzz/Cargo.toml mark=re-reads
what: the fuzz workspace whose lockfile is audited on its own for advisories and sources; re-read for whether that audit reads this advisory id and fails on it
@task_item n=1 part=needs
The cargo-deny gate on the root graph and its supply-chain CI job: the chunk's change must leave that gate reading green on the pushed sha, with the whole CI run green, as the operator entry's acceptance.
@task_item n=2 part=needs
A reasoned advisory ignore: exactly one new entry, for RUSTSEC-2026-0543 alone, its reason written beside it naming the upstream crates that bring chrono into the graph (rmcp and schemars through viola-mcp) and its re-read ground at each epoch boundary; no wider than this one id, no other advisory admitted.
@task_item n=3 part=needs
The admission ground for advisory ignores: until now only an advisory in a disabled rmcp feature; the founder's widened rule adds an unmaintained advisory with no safe upgrade whose crate an upstream dependency brings in. The config comment above the ignore list states both grounds; the standing plan row for the widened rule is the wrap's expected amendment, and no master is edited here.
@task_item n=4 part=needs
No skip and no allow entry added to the deny config, and no licence, source or ban rule loosened by the way; the ignore list is the only list that grows.
@task_item n=5 part=needs
Viola's own direct chrono use (in viola and viola-state) stays as it is: no dependency replaced, upgraded or re-pinned, the exact third-party pins and the manual dependency update review untouched.
@task_item n=6 part=needs
The acceptance predicate must be able to fail: the plan names what turns the gate red again, a new advisory id, or the same id once a safe upgrade exists where the tool reads that.
@task_item n=7 part=needs
The fuzz lockfile audited on its own, the weekly scheduled advisory run and the per-sync-crate deny runs are re-measured against this advisory id from the code and the failing run's log, so each config or job that fails on it is named and handled under the same one-id ignore.
@task_item n=8 part=needs
The tied test child: the e2e in-test boots build their boot options tied, using the tie the last chunk built (the harness and root tied-test-child constants), so the wrapper's supervisor and its fake agent end with their test when the test is SIGKILLed.
@task_item n=9 part=needs
A new test kills the tied stamping viola verify and reads, through the process census the last chunk built, that nothing of it is left running; the verify run it kills is one it booted itself.
@task_item n=10 part=needs
No test sends a kill at a process it does not own: every kill targets only a process the test spawned and identified, and the proof leaves nothing running after it.
@task_item n=11 part=needs
A site sweep for the tie runs over every spelling of a constant, so no untied in-test boot is left under another spelling.
@task_item n=12 part=needs
Verify probe directories and captures left by a killed verify run: what verify creates in its own directory must not outlive the killed test, the drop-guard cleanup and the confinement of probe effects to its own directory holding when the process is killed rather than dropped.
@task_item n=13 part=needs
Test-side removal only of directories the test created, under its own home and e2e home, following only a safe linked base; viola never touches the e2e-home backing.
@task_item n=14 part=needs
A test socket goes under the short per-test root under /tmp, never a Unix socket at the temp root itself; the per-user owner-only socket directory rules hold for booted wrappers in tests.
@task_item n=15 part=needs
Every test that waits on a channel or a thread takes a bound under the project's 20 s lint.
@task_item n=16 part=needs
Booted wrappers run the fake agent, never the real claude CLI: no live claude start in this chunk and none in CI, and no mutation run.
@task_item n=17 part=needs
Test seams the new or changed tests use stay under the fake-agent test-only feature, compiled out of the release build; no new seam forces a security refusal.
@task_item n=18 part=needs
The v2-37 ledger note, verified for its named witnesses only, is closed by a claim never weaker than its text: the booted e2e tests and the stamping verify are the witnesses this chunk adds.
@task_item n=19 part=needs
Every control the plan fires names its timeout and fires with it, its id within 16 characters, and every test filter it uses is checked to select at least one test.
@task_standing n=1
convention: each new test uses its own viola home, never the user's, and test homes stay out of git
home: secret-management · bootstrap-phases
@task_standing n=2
convention: each new test removes only directories it created itself, following only a safe linked base for the e2e home
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_standing n=3
convention: each new test seam or fake-agent hook it relies on is test-only, under the fake-agent feature, and compiled out of release
home: input-validation · security-anti-patterns
@task_standing n=4
convention: each new test socket sits in an owner-only per-test directory, never at the temp root
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_item n=1 part=not_needed
The MCP tool contracts and the MCP stdio server's input checks, line cap and error results: the chunk adds no MCP tool and changes no MCP input or output.
@task_item n=2 part=not_needed
The wrapper channel method contracts, channel frame params and the send, answer and paste text character-set rules: the chunk adds no channel method and types no text.
@task_item n=3 part=not_needed
The hook stdin, hook decision detail and hook fail-open contracts: the chunk changes no hook path.
@task_item n=4 part=not_needed
The revive and list JSON codes, the CLI exit codes and the instance-unreachable detail contracts: no CLI verb's output or exit changes.
@task_item n=5 part=not_needed
The release build, dist attestations, signed self-update and release-check rules: the chunk has no release stage and touches no release workflow.
@task_item n=6 part=not_needed
The CI artifact canary scan classes and the inherited-credential strip from the claude child: the chunk adds no upload and no child environment handling.
@task_item n=7 part=not_needed
Human-output control escaping and terminal colour facts: no human-mode output changes.
@task_item n=8 part=not_needed
The events link lines and the link and unlink method replies: no link state is touched.
@task_item n=9 part=not_needed
The config.json value contracts (claude_env_keep, diagnostics_level, dialog_hold_s): no configuration value is added or changed.
