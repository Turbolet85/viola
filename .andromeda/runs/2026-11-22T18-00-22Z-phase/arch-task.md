@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
Measurements come first, each from a test or a no-start probe and each a number or a closed word: the chrono advisory, the send slot's claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify's trust record, the boot supervisor test.
Mends follow, each with the test that holds it: an end for a claimed send line whose settle never comes, a bound of its own for verify's agents read, the boot test's binary copy out of its timed body with its abort deadline below the test's kill, the mutants profile's kill read against a cold parallel baseline.
Killing tests are added for seven capability-ledger and screen-signature mutation survivors and three paste-writer survivors, tests only; at discretion a step function for verify's framing turns, the relied-row count from one source, and the test rules file's paths.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot holding a send's claimed line through its confirmation and settle — edits: SendSlot::confirm, its Waiting | Claimed wait arm and the claimed-line settle skipped when the snapshot update fails
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded claude agents --json read inside Run B — edits: the agents read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the per-behaviour CLI capability ledger and its environment-strip name check (env_names_fault) — edits: its tests only, killing tests for the ledger's mutation survivors
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the compiled screen signatures read by the vt100 readiness gate — edits: its tests only, killing tests for the screen's mutation survivors
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump's bracketed-paste writer cut at the compiled paste bound — edits: its tests only, killing tests for the three paste-writer mutation survivors
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the end-to-end harness lifecycle tests, among them the boot supervisor test — edits: the boot supervisor test, its binary copy moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the end-to-end harness's boot supervisor — edits: ABORT_DEADLINE, held strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles, among them the mutants profile used by mutation runs — edits: the mutants profile's kill for contract_fake_agent_drift's two replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny dependency policy and its advisory ignores — edits: RUSTSEC-2026-0543's ignore and its reason line
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the project's testing rules — edits: its paths: list (at discretion)
@task_item n=1 part=needs
A claimed send line whose settle never comes gets an end: under a failing snapshot write the send slot's confirm no longer waits forever on the claimed line; measured first, then mended, a test holding it, and the send still confirmed after the fact or reported unconfirmed, never left hanging.
@task_item n=2 part=needs
The claimed-line end acts inside the send time bound and the send post-gate section under the wrapper flight lock; it adds no retry, no queue and no loosened bound, and the instance snapshot writer's failure is what the test injects.
@task_item n=3 part=needs
Verify's agents read in Run B gets a bound of its own, measured under a stalled host; never a retry nor a loosened bound without that measurement; the agents listing still degrades rather than errors, and verify's bounded child set stays bounded.
@task_item n=4 part=needs
Killing tests for the capability-ledger survivors, the environment-strip name check first (the strip reads names only, CLAUDE names stripped from the child's environment), and for the screen-signature survivors; the agent crate stays pure, with no file I/O, tests only, no product change.
@task_item n=5 part=needs
Killing tests for the paste-writer survivors: the atomic bracketed-paste send cut at the compiled paste bound, the cut mark and its paste-cut refusal, held by tests only with no product change.
@task_item n=6 part=needs
The boot supervisor test: its binary copy leaves the timed body, and the supervisor's abort deadline stays strictly below the test's kill; no timeout raised; the cause of its stalled-host timeout is read, not guessed.
@task_item n=7 part=needs
The mutants nextest profile's kill on the fake-agent drift contract's two replay tests is read against a cold parallel baseline; mutation testing stays at the epoch boundary, no mutation run inside this chunk, and the profile keeps its own target directory.
@task_item n=8 part=needs
The chrono advisory: measure whether a safe chrono upgrade exists and how cargo-deny treats an ignored advisory once one does; found, the ignore leaves with the upgrade under the workspace's single version pin; none, the re-read is re-pinned to the Epoch 5 boundary and the reason line keeps saying it is re-read at each boundary; chrono RFC 3339 UTC millisecond timestamps unchanged.
@task_item n=9 part=needs
A SIGKILLed viola verify is measured under a test-owned home and a test-owned Claude config for a project entry left pointing under its scratch root: verify writes and removes its own trust record, its probe folders removed by drop guard, and a leftover becomes a named residual in the masters or a mend; nobody reads or edits the operator's real config.
@task_item n=10 part=needs
At discretion, each named with its decision: one step function for verify's framing turns' per-paste probes and a look at the send path's guarded section under the flight lock; the relied-row count from one source and the stamped-set table held once, a CLI version verified only when every relied row counts.
@task_item n=11 part=needs
Every measurement is a number or a closed word from a test or a no-start probe; a new closed word, a new time bound's value, a product behaviour a user sees, or a split of this chunk is the operator's fork, written to the founder forks at P3's end.
@task_item n=12 part=needs
A new time bound is a named member of the a11y lists, its carriage said in its block, never by dropping another member's words.
@task_item n=13 part=needs
No live start, no rig start of a real claude, no witness run: real claude CLI probes run only locally and are not run here, tests run against the fake agent, no live claude in CI.
@task_item n=14 part=needs
The new tests reaching killed or frozen processes keep their evidence: test evidence survives a killed test, test homes removed by owner record only, test-side removals outside the working directory, root tests freezing a wrapper through one SIGSTOP guard.
@task_standing n=1
convention: each new test runs in its own viola home against the fake agent, with test seams never a configuration channel and test support kept out of product builds
home: occupied-resources
@task_standing n=2
convention: root test waits share one bound and no new test raises a timeout
home: occupied-resources
@task_standing n=3
convention: an OS-gated test names its reason and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: new code and tests pass formatting and clippy with warnings denied, with print macros and tracing level macros banned
home: build-system
@task_standing n=5
convention: insta snapshots in check mode only and zero panic lines in end-to-end runs
home: project-directory-structure
@task_standing n=6
convention: coverage floors enforced by the harness gate and gate steps fail closed
home: ci-cd-approach
@task_item n=1 part=not_needed
The MCP server's tools, tool results, stdin cap and exit codes: the chunk changes no MCP surface.
@task_item n=2 part=not_needed
Hook tiers, the hook contract, the hook-to-kind map and dialog hold rules: no hook or dialog path changes.
@task_item n=3 part=not_needed
Revive: its program resolution, refusals, json error codes and exit codes are untouched.
@task_item n=4 part=not_needed
Link and unlink methods, link events and the link set: the chunk records no link.
@task_item n=5 part=not_needed
The Hyprland seam, window features and the terminal fleet console: no compositor or view work.
@task_item n=6 part=not_needed
Plugin install, the marketplace tree, the plugin's .mcp.json and release tooling through dist: nothing is installed or released.
@task_item n=7 part=not_needed
config.json keys, configuration precedence and colour environment rules: no configuration key is added or read.
@task_item n=8 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing: no fuzz target changes.
@task_item n=9 part=not_needed
The design-philosophy and project-intent absences (usage budget gate, graphical frontend, mobile view, hosting, database, broker): none is approached.
@task_item n=10 part=not_needed
Wheel claim, human take and wheel release decisions: the chunk does not move the wheel.
