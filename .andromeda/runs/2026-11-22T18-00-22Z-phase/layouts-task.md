@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
It measures first, at P3, each from a test or a no-start probe: the chrono advisory's safe upgrade, the send slot's claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify's trust-record residue, and the boot supervisor test's timing.
It then mends: an end for a claimed line whose settle never comes in the send slot's confirm, a bound of its own for verify's agents read, the boot supervisor test's binary copy out of its timed body with its abort deadline strictly below the test's kill, and the mutants profile's kill read against a cold parallel baseline.
It adds killing tests for the seven agent-adapter mutation survivors and the three paste-writer mutation survivors, tests only, no product change there.
At the phase's discretion only: one step function for the framing turns' per-paste probes, the relied-row count from one source, and the testing rule file's paths gap.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that holds a claimed line until its settle — edits: SendSlot::confirm, the Waiting or Claimed wait arm
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded claude agents --json read inside Run B — edits: the agents read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the agent adapter's ledger, env_names_fault among its mutation survivors — edits: tests only
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the agent adapter's screen reading, two mutation survivors — edits: tests only
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump's paste writer, three mutation survivors — edits: tests only
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the boot supervisor test of the e2e harness — edits: the binary copy moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot supervisor — edits: ABORT_DEADLINE, strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles — edits: the mutants profile's kill on the two drift replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny advisory list — edits: the RUSTSEC-2026-0543 ignore and its reason line
@task_item n=1 part=needs
The send's confirmation of a claimed line: a claimed line whose settle never comes (the snapshot write failing before the settle) now ends instead of waiting forever; which send outcome word that end carries is held by a test.
@task_item n=2 part=needs
Any send outcome the claimed-line end reports uses the existing send outcome and readback vocabulary; a new closed word is the operator's fork, never added by the chunk on its own.
@task_item n=3 part=needs
Verify's agents read gets a bound of its own, measured first under a stalled host; no retry, and no loosened bound without that measurement.
@task_item n=4 part=needs
When verify's bounded agents read runs out, verify reports it through its existing failing row, stop and exit lines; a new time bound's value or a changed line a user sees is the operator's fork.
@task_item n=5 part=needs
A new time bound is a named member of the a11y lists, whose cells are near full: its carriage is said in the fork block, never by dropping another member's words.
@task_item n=6 part=needs
The SIGKILLed verify measurement runs under a test-owned home and a test-owned Claude config; nothing reads or edits the operator's real Claude config; a project entry found under the scratch root becomes a named residual or a mend.
@task_item n=7 part=needs
Each measurement at P3 is a number or a closed word, taken from a test or a no-start probe: no live start, no rig start of a real claude, no mutation run, no witness run.
@task_item n=8 part=needs
The killing tests for the agent adapter and paste writer survivors are tests only: the adapter's screen and ledger readings and the paste writer's bounded paste keep what they do, held by their existing checks plus the new tests.
@task_item n=9 part=needs
No timeout is raised in the boot supervisor test or the mutants profile: the boot test's abort deadline sits strictly below the test's kill, and the mutants kill is only read against a cold parallel baseline.
@task_item n=10 part=needs
The chrono advisory stays ignored unless a safe upgrade exists; the reason line keeps saying it is re-read at each boundary.
@task_item n=11 part=needs
At discretion, a single step function for the verify framing turns' per-paste probes keeps the verify step lines and their order unchanged.
@task_standing n=1
convention: human output is a contract — any send or verify output line a mend reaches is held by a test of its exact text, plain and --json alike
home: surface-cli
@task_item n=1 part=not_needed
A web surface: viola renders no page, and the chunk adds none.
@task_item n=2 part=not_needed
The board of viola list, its columns, widths, truncation, colour depth and styling: the chunk changes no board rendering.
@task_item n=3 part=not_needed
CLI token rendering, colour roles and SGR styling rules: no styled output is added or changed.
@task_item n=4 part=not_needed
The MCP server and its tool results: no MCP tool or result changes.
@task_item n=5 part=not_needed
viola run and viola revive starts, their preflight and refusals: the chunk starts no instance and touches no start path.
@task_item n=6 part=not_needed
viola link, unlink, plugin install, wheel verbs and answer output: none of those verbs is reached.
@task_item n=7 part=not_needed
Help, usage and per-verb help output: no verb, flag or usage line is added.
@task_item n=8 part=not_needed
viola wait output and the frame verb exit-21 hints and details: the chunk changes no wait or reachability behaviour.
