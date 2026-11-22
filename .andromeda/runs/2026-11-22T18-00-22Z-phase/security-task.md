@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
Measurements come first, each from a test or a no-start probe: whether a safe upgrade of chrono exists for the ignored advisory RUSTSEC-2026-0543 and how cargo-deny treats an ignored id once one does; the send slot's claimed-line wait under a failing snapshot write; verify's bounded claude agents read under a stalled host; whether a SIGKILLed viola verify leaves a project entry in a test-owned Claude config pointing under its scratch root; the boot supervisor test's timing.
Mends follow, each held by a test: an end for a claimed line whose settle never comes, killing tests for seven Claude-adapter and three paste-writer mutation survivors, a bound of its own for the agents read, the boot test's binary copy moved out of its timed body with its abort deadline below the test's kill, and the mutants profile's kill read against a cold parallel baseline.
The SIGKILL measurement ends in a named residual row or a mend; at the phase's discretion a framing-turns step function, one source for the relied-row count, and the testing rules' paths gap.
No live start, no real claude, no mutation or witness run, no timeout raised, no retry, and nothing reads or edits the operator's real Claude config.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that holds a claimed line until its settle, under the wrapper's send path and its snapshot update — edits: SendSlot::confirm, its Waiting or Claimed wait arm and the settle a failed snapshot update skips
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded claude agents --json read inside Run B, read fail-open — edits: the read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude adapter's ledger, holding the env names fault and the env names-only reads — edits: its tests only, the killing tests for env_names_fault and the other ledger mutation survivors
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the Claude adapter's screen reading against compiled literals — edits: its tests only, the killing tests for its two mutation survivors
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump's paste writer that types one bracketed paste within its bound and ends a cut paste — edits: its tests only, the killing tests for the three paste-writer mutation survivors
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the e2e harness lifecycle tests, holding the boot supervisor test — edits: the boot supervisor test, its binary copy moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot supervisor and its abort deadline — edits: ABORT_DEADLINE, held strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile's kill bound used at the epoch audit's mutation testing — edits: the mutants profile's kill as read for contract_fake_agent_drift's two replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny gate's configuration and its closed advisory ignore list — edits: the RUSTSEC-2026-0543 chrono ignore entry and its reason line, or its removal with the upgrade
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the project's testing rules, at the phase's discretion — edits: its paths: list
@task_item n=1 part=needs
The send slot's claimed line: a claimed line whose settle never comes because the snapshot update before it failed gets an end, measured first under a failing snapshot write and then held by a test; the change touches the wrapper's send path, its snapshot update and what a send caller sees when the line ends.
@task_item n=2 part=needs
Verify's claude agents read inside Run B gets a bound of its own, measured under a stalled host: the read stays bounded and fail-open, with no retry and no loosened bound without that measurement, and its log stays content-free.
@task_item n=3 part=needs
The chrono advisory RUSTSEC-2026-0543 stays on the closed advisory ignore list under its admitted grounds unless a safe upgrade is found; found, the ignore leaves with the upgrade under exact third-party pins and the cargo-deny gate still passes; none, its reason line keeps saying it is re-read at each boundary and the re-read is re-pinned to the Epoch 5 boundary.
@task_item n=4 part=needs
The SIGKILLed verify measurement: under a test-owned home and a test-owned Claude config, whether a viola verify ended by SIGKILL leaves a project entry pointing under its scratch root; this reads the trust record writer's guard and the verify probe directories' drop-guard cleanup, which a SIGKILL skips; the result is a named residual row or a mend, and nothing reads or edits the operator's real Claude config or writes under the real Claude home.
@task_item n=5 part=needs
The killing tests for the Claude adapter's survivors, env_names_fault first, prove the env reads keep names and counts only and never a credential value, and the screen survivors' tests read the screen only against compiled literals by signature rows.
@task_item n=6 part=needs
The killing tests for the paste writer's survivors hold the send as one bracketed paste, a cut paste ending the send as paste-cut, and the wheel unmoved.
@task_item n=7 part=needs
The boot supervisor test: the binary copy moves out of the timed body and the harness abort deadline stays strictly below the test's kill; no timeout is raised, and the test keeps its own home and removes only its own directories.
@task_item n=8 part=needs
The mutants nextest profile's kill on contract_fake_agent_drift's two replay tests is read against a cold parallel baseline; mutation testing stays at the epoch audit, no mutation run is made inside the chunk, and no bound is loosened without its measurement.
@task_item n=9 part=needs
No live start, no rig start of a real claude and no witness run: every measurement runs from a test or a no-start probe under the fake agent, CI holding no secrets and no real claude.
@task_item n=10 part=needs
A new time bound, a new closed word or a product behaviour a user sees is the operator's fork, written at P3's end; a new time bound is a named member of the bounded-readers census and the a11y lists.
@task_item n=11 part=needs
A failing snapshot write or a stalled agents read reached by a test goes through a test seam under the fake-agent feature, compiled out of release, and a new seam or environment read carries its log entry.
@task_item n=12 part=needs
At the phase's discretion: one step function for verify's framing turns per-paste probes and a look at send_through's guarded section; the relied-row count from one source and the stamped-set table held once; each named with its decision, none promised.
@task_standing n=1
convention: each new test runs under its own home and a test-owned Claude config, and removes only directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each new test seam sits only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns · input-validation
@task_standing n=3
convention: each new seam or environment read added for a test carries its log entry
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: each new fixture or test output holds no path, username, credential value or transcript byte, scrubbed or refused whole
home: data-protection
@task_standing n=5
convention: each new test runs in CI with no secrets and no real claude
home: threat-model-summary
@task_item n=1 part=not_needed
MCP server tools, MCP stdin line caps and MCP isError results: the chunk changes no MCP surface.
@task_item n=2 part=not_needed
Hook payload parsing, hook refusals and hook-decision details: the chunk touches no hook.
@task_item n=3 part=not_needed
Socket directory checks, peer euid checks and server verification before a frame: the chunk changes no channel connection.
@task_item n=4 part=not_needed
viola revive arguments, its --json codes and its list document: revive is not reached.
@task_item n=5 part=not_needed
link and unlink verbs, their exit codes and events link lines: not reached.
@task_item n=6 part=not_needed
Release, signing and deploy rows, dist attestations and self_update signatures: the chunk builds no release.
@task_item n=7 part=not_needed
Fuzz workspace, its nightly pin and its lockfile audit, and scheduled fuzzing: the chunk adds no fuzz target.
@task_item n=8 part=not_needed
rmcp feature sets and their banned HTTP and auth features: no rmcp change.
@task_item n=9 part=not_needed
Hyprland recorder and reply rows and terminal colour facts for list: not touched.
@task_item n=10 part=not_needed
Strict-modes refusal contracts of list, link, mcp start, mcp tool and plugin install: none of those verbs change.
@task_item n=11 part=not_needed
Absences of hosting, network listeners, outbound calls, accounts, databases, webhooks and secret managers: the chunk adds none.
@task_item n=12 part=not_needed
Live rig arms and their kept words: no live start or rig start happens in this chunk.
@task_item n=13 part=not_needed
Config.json keys and their bad-value handling: no config key changes.
