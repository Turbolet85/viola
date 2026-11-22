@task_prose part=chunk
One corrective chunk over the Epoch 4 boundary audit's ranked defects, in the audit's order, each item worded by what the next boundary's audit will read.
Measurements come first, each from a test or a no-start probe, each a number or a closed word: the chrono advisory (a safe upgrade or none, and how cargo-deny treats an ignored id), the claimed-line wait under a failing snapshot write, verify's agents read under a stalled host, a SIGKILLed verify under a test-owned home and Claude config, and the boot supervisor test.
Mends follow, each held by a test: an end for a send's claimed line whose settle never comes; a bound of its own for verify's agents read; the boot test's binary copy out of its timed body with its abort deadline strictly below the test's kill; the mutants profile's kill read against a cold baseline.
Killing tests for seven mutation survivors in the Claude agent adapter (env names fault first) and three in the PTY paste writer; at discretion one step function for the framing turns' per-paste probes, the relied-row count from one source, and the testing rule's paths gap.
No live start, no mutation run, no witness run, no timeout raised, no retry; nothing reads or edits the operator's real Claude config.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the send slot that holds a claimed line until the send is confirmed and settles it — edits: SendSlot::confirm, the Waiting or Claimed wait arm and the settle a failed snapshot update skips
@task_file n=2 path=src/run/agents.rs mark=edits
what: verify's bounded claude agents --json read inside Run B, the agents probe — edits: the read's bound
@task_file n=3 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude agent adapter's capability ledger, its environment-name fault and stamp handling — edits: its tests only, killing tests for the env_names_fault survivors and the other ledger survivors
@task_file n=4 path=crates/viola-agent-claude/src/screen.rs mark=edits
what: the Claude agent adapter's screen reading — edits: its tests only, killing tests for two mutation survivors
@task_file n=5 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump's paste writer, the bounded paste write — edits: its tests only, killing tests for three paste-writer mutation survivors
@task_file n=6 path=crates/viola-e2e/tests/harness_lifecycle.rs mark=edits
what: the e2e harness lifecycle tests — edits: the boot supervisor test, its binary copy moved out of the timed body
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs mark=edits
what: the e2e harness boot supervisor — edits: ABORT_DEADLINE, held strictly below the test's kill
@task_file n=8 path=.config/nextest.toml mark=edits
what: the nextest profiles, the mutants profile's per-test kill — edits: the mutants profile's kill on the contract_fake_agent_drift replay tests
@task_file n=9 path=deny.toml mark=edits
what: the cargo-deny configuration, its advisory ignore list — edits: the RUSTSEC-2026-0543 chrono ignore and its reason line
@task_file n=10 path=.claude/rules/testing.md mark=edits
what: the project's testing rule — edits: its paths: list, at the phase's discretion
@task_item n=1 part=needs
The send path's claimed line gets an end when its settle never comes because the snapshot update fails; the send's issued, confirmed and refused log lines, their corr and their closed detail words stay as they are unless the operator's fork admits a new word, and the error the failed snapshot write raises ends in one logged line with a fixed message.
@task_item n=2 part=needs
The snapshot write on the send path, its state.snapshot_write span and its failure, measured first under a failing write by a test before any mend.
@task_item n=3 part=needs
Verify's claude agents --json read gets a bound of its own: the agents-probe child spawn subject, its process-start and process-exit lines with child_exit_status and duration_ms, and a parse failure or a stall reading as unknown, never as a verdict.
@task_item n=4 part=needs
No retry and no retry-once on the agents read, the claimed-line wait or any test; no timeout raised and no bound loosened without its measurement; a new time bound's value is the operator's fork.
@task_item n=5 part=needs
The measurement of a verify ended by SIGKILL: whether the trust record written for its scratch root is left in a test-owned Claude config under a test-owned home; the trust-record write and removal stay uninstrumented, and the start's trust-record log line and verify's interrupted process-exit detail are what such a run can leave behind.
@task_item n=6 part=needs
A trust-record residual found by that measurement becomes a named residual or a mend; nobody reads or edits the operator's real Claude config, and verify logs only inside an instance.
@task_item n=7 part=needs
The killing tests for the env_names_fault survivors hold that environment names, never values, are logged and that stripped CLAUDE values never reach a line; the ledger's stamps and path stay out of every line and an unreadable ledger counts as unverified.
@task_item n=8 part=needs
The killing tests for the screen survivors hold that screen content is no telemetry source and that a vt100 feed panic stays contained.
@task_item n=9 part=needs
The killing tests for the paste-writer survivors hold the PTY seam's paste log event and the paste-cut send refusal, and that the PTY byte stream and the pasted text are never logged.
@task_item n=10 part=needs
The boot supervisor test: the binary copy leaves the timed body, the abort deadline sits strictly below the test's kill, and the test waits on its log without sleeping.
@task_item n=11 part=needs
The mutants nextest profile's 15 s kill on the fake-agent drift replay tests, read against a cold parallel baseline; obs code stays mutation-tested like product code, and no mutation run happens inside the chunk.
@task_item n=12 part=needs
The chrono advisory: a safe chrono upgrade measured first; found, the ignore leaves with the upgrade; none, the ignore stays with a reason line saying it is re-read at each boundary; chrono stays the direct dependency for millisecond UTC timestamps and obs dependencies keep passing cargo deny, a cargo deny failure being a failure.
@task_item n=13 part=needs
Every measurement runs from a test or a no-start probe against the fake agent: no live start, no rig start of a real claude, real-CLI verify never in CI.
@task_item n=14 part=needs
At discretion, one step function for verify's framing turns' per-paste probes: verify's lines carry no content and its spawn subjects and step lines stay as they are.
@task_item n=15 part=needs
At discretion, the relied-row count read from one source and the stamped-set table held once: verify stays the only stamps writer and the ledger-stamps parse rejection stays as it is.
@task_standing n=1
convention: each new integration or e2e test's emitted diagnostics lines are schema-checked against the line schema
home: ci-integration
@task_standing n=2
convention: each new test's home lives under the scanned e2e home root and is kept until the gate steps run
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new test that waits on a log line waits without sleeping
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: each new OS-gated test is counted in the OS-gated test census
home: ci-integration
@task_item n=1 part=not_needed
The MCP stdio server's tools, frames, stdin bounds and refusal results: the chunk touches no MCP path.
@task_item n=2 part=not_needed
The dialog hook path: hook decisions, fail-open, hook latency budgets and hook stdout arms; no hook code changes.
@task_item n=3 part=not_needed
The channel's JSON-RPC lines, peer credential and server-verification checks and the socket-directory refusals: the chunk changes no channel code.
@task_item n=4 part=not_needed
The link, unlink, revive and list verbs, their stderr pairs and exit detail codes: none is touched.
@task_item n=5 part=not_needed
A future error reporter, Sentry, OpenTelemetry exporters, OTLP receivers and network egress: the chunk adds no telemetry sink.
@task_item n=6 part=not_needed
Heartbeat liveness, staleness thresholds and list liveness: no liveness code changes.
@task_item n=7 part=not_needed
The wheel's pause and release, its spans and human-typing lines: the chunk leaves the wheel unmoved.
@task_item n=8 part=not_needed
Derived metrics, quantiles, gauges and histograms over the ndjson: the chunk adds no metric.
@task_item n=9 part=not_needed
The config.json keys, the diagnostics level and home resolution order: no configuration changes.
@task_item n=10 part=not_needed
The CLI's --json refusal documents and exit 21 cause words: no CLI frame verb changes.
