@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each worded by what the next boundary's audit reads.
It measures first, at P3, each from a test or a no-start probe: the chrono advisory's safe upgrade, the send slot's claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify's trace in a test-owned Claude config, and the boot supervisor test's timing.
Then it mends, each with a test: an end for a claimed line whose settle never comes, a bound of its own for verify's agents read, the boot test's binary copies out of its timed body with the abort deadline below the test's kill.
It adds killing tests for seven agent-claude and three paste-writer mutation survivors and reads the mutants profile's 15 s kill against a cold parallel baseline.
No live start, no mutation or witness run, no timeout raised, no retry; a new closed word, time bound value or user-seen behaviour is the operator's fork.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that holds a claimed line until it settles, the confirmed send's wait — edits: SendSlot::confirm, the Waiting | Claimed wait arm
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded claude agents read inside Run B — edits: the agents read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude agent's ledger of relied rows and environment names, its crate tests — edits: tests only, killing env_names_fault and the other ledger survivors
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the claude agent's screen signatures, its crate tests — edits: tests only, killing two survivors
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump and its paste writer, its crate tests — edits: tests only, killing three paste-writer survivors
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the boot supervisor test in the e2e harness lifecycle suite — edits: the test's timed body, binary copies moved out of it
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot supervisor — edits: ABORT_DEADLINE, held strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles — edits: the mutants profile's kill for contract_fake_agent_drift's two replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny advisory list — edits: the RUSTSEC-2026-0543 ignore and its reason line, re-read at each boundary
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the testing rule, at the phase's discretion — edits: its paths: list
@task_item n=1 part=needs
The send slot's claimed-line wait gets an end when the settle never comes under a failing snapshot write; the ended send's outcome is a confirmed-send path behaviour, and any new refusal or fault word it surfaces is the operator's fork.
@task_item n=2 part=needs
A send ended this way reaches the caller as a send refusal or a fault with its fixed form: a code word beside unable, one hint per code, no hint on a fault, one clean --json document.
@task_item n=3 part=needs
Verify's claude agents read gets a bound of its own, measured under a stalled host; when that bound fires the read still fails open, and verify's step line and exit stay as they are.
@task_item n=4 part=needs
Every new time bound (the claimed-line end, the agents read's bound, the boot abort deadline) falls on the driver only, never on the human, and is a named member of the a11y time-bound lists, carried without dropping another member's words.
@task_item n=5 part=needs
No timeout raised, no retry and no bound loosened without its measurement: the zero-flakiness rule binds the mends and the mutants profile's 15 s kill reading.
@task_item n=6 part=needs
Killing tests for the seven agent-claude survivors and three paste-writer survivors: the mutation audit's coverage of invariants held in crate tests, read again at the next boundary, with no mutation run inside the chunk.
@task_item n=7 part=needs
The paste-writer tests hold the existing paste behaviour: a human key still takes the wheel and is never blocked, terminal sequences do not move the wheel, zero viola bytes reach the terminal.
@task_item n=8 part=needs
The boot supervisor test: its binary copies leave the timed body and the boot abort deadline sits strictly below the test's kill, so a stall fails as an assertion, never as a timeout without one.
@task_item n=9 part=needs
The SIGKILL measurement of a verify under a test-owned home and test-owned Claude config: whether a project entry under the scratch root survives; if so a named residual or a mend; nobody reads or edits the operator's real config.
@task_item n=10 part=needs
Live readings never gate: no live start and no rig start of a real claude; each measurement is a test or a no-start probe giving a number or a closed word.
@task_item n=11 part=needs
The chrono advisory is measured first: a safe upgrade found removes the ignore with it; none found keeps the ignore and re-pins the re-read to the Epoch 5 boundary, the reason line saying it is re-read at each boundary.
@task_item n=12 part=needs
At discretion, one step function for verify's framing turns per-paste probes and a look at send_through's guarded section, each with its decision; no change to what verify or send prints.
@task_item n=13 part=needs
At discretion, the relied-row count read from one source and the stamped-set table held once, with no change to the relied rows themselves.
@task_standing n=1
convention: each new test runs inside the existing nextest suites, its failure a nextest failure, no separate a11y driver
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test is deterministic: zero flakiness, no retries, no soft SLO
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: the CI test gate allows no skips and requires coverage and doctest for each new test
home: 1-a11y-scope-summary · ci-integration
@task_standing n=4
convention: boundary tests over the PTY are sequenced on records, never on sleeps, and never inspect claude content
home: keyboard-test-harness
@task_standing n=5
convention: any fixed CLI string a new test asserts is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
The list board's colour, caption, stale-row dimming, NAME truncation and SGR rules: the chunk prints nothing new on the board.
@task_item n=2 part=not_needed
viola --help group labels, usage line and handoff row: no verb or help text changes.
@task_item n=3 part=not_needed
link and unlink, plugin install and revive outputs and refusals, revive --list: the chunk touches none of these verbs.
@task_item n=4 part=not_needed
Hook calls and MCP frame tools' socket-directory, strict-modes and server-verify refusals: no frame verb or hook path changes.
@task_item n=5 part=not_needed
Screen-reader targets and the manual Orca and keyboard passes: no manual pass is run or changed.
@task_item n=6 part=not_needed
Absences of a web or graphical frontend, a touch surface, internationalization and a rendered page: nothing in the chunk reaches them.
@task_item n=7 part=not_needed
Structured violation JSON and the ARIA-equivalent landmark decisions: no a11y artifact or output structure is added.
