@task_prose part=session
The session added the 29th relied capability-ledger row, cross-session-prefix, reading pass or fail only; the relied count moved 28 to 29 (verify step lines [NN/29], summary 29 pass 0 fail) and a 28-row stamp now reads unverified.
viola verify's Run B gained a cross-session turn: a print-mode claude sender child, new spawn subject verify-sender, sends to the wrapped claude by a composed uds address (the runtime dir, cc-socks, the child pid, .sock), and Run B now runs under a settings argument holding an accept literal for inbound cross-session messages.
verify now reads XDG_RUNTIME_DIR once, holds it in memory and passes it only into the sender's prompt argv; it enters no log line, span, stamp, step line, error or fixture that verify writes itself.
The hidden hook capture arm gained a names-only mode that files only closed tool words per tool hook and no payload byte; the ledger stamp gained a measured cross_session_probe block of closed words and one latency number; the diag-line schema subject enum gained verify-sender.
The recorded claude 2.1.328 fixture set was re-recorded whole (32 to 33 files, one new cross-session arrival); measured by the wrap, that arrival's prompt holds the sender's runtime-dir socket path (with the uid) and pid, which the fixture scrub and the hygiene walk's absolute-path check do not normalise or flag.
Every test and harness verify now runs with a test-owned owner-only runtime dir; the bare-session-id addressing form was retired as refused by the real CLI.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited; the capability ledger: the relied rows and their readings, the stamp merge and its measured blocks, the fixture scrub and hygiene walk, the probe plugins, the cross-session row and its uds address composer and sender prompt
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited; the claude adapter's compiled literals: the settings and allowed-tools flags, the uds scheme, the CLI socket dir name, suffix and path bound
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: edited; the hook payload reading of the claude adapter: prompt origin and the harness prefix tags of an arriving prompt
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited; unit tests of the capability ledger: tool words, the names-only capture document, the sender plugin hooks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited; unit tests of the ledger row checks: the cross-session row, sender words, the uds address and the sender prompt
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited; unit tests of the ledger probe framing turns
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited; unit tests of the ledger stamp merge: the cross_session_probe block, the relied count
@task_file n=8 path=src/cmd/hook.rs
what: edited; the viola hook command and its hidden capture arms, now with the names-only capture of closed tool words
@task_file n=9 path=src/cmd/verify.rs
what: edited; the viola verify command: its measure, the one runtime-dir env read, the verify child spawns and step lines
@task_file n=10 path=src/cmd/verify/typed.rs
what: edited; the verify typed-run inputs, now holding the runtime dir in memory
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: edited; the verify probe runs and how each run ends
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: edited; the verify probe turns: Run B's framing turns and the new cross-session turn spawning the verify-sender child
@task_file n=13 path=src/run/version_gate.rs
what: edited; tests of the start's version gate over the relied ledger count
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited; the fake agent test double of claude: the cross-session sender and receiver, the start receipt gaining pid and prompt
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: edited; the fake agent's sibling lookup, now by uds address instead of session id
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited; unit tests of the fake agent's modes: the sender and receiver
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited; unit tests of the fake agent's options against the recorded fixture set
@task_file n=18 path=schemas/diag-line.v1.json
what: edited; the diag-line v1 schema, its subject enum gaining verify-sender
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the e2e harness boot, its verify stamp now given a short owner-only runtime dir
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: edited; the e2e harness run and its expected live verify step lines
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the standing live rig: its floor rows, kept stamps and summary lines for the cross-session row and sender words
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the live rig rehearsal tests, each case's runtime dir and the env carried over a rebuilt command
@task_file n=23 path=tests/cli_verify.rs
what: edited; integration tests of viola verify: the cross-session row's pass, not-run, red and kill cases
@task_file n=24 path=tests/cli_verify/runs.rs
what: edited; integration tests of the verify probe runs
@task_file n=25 path=tests/cli_verify/stamps.rs
what: edited; integration tests of the verify ledger stamps
@task_file n=26 path=tests/cli_fake_agent.rs
what: edited; integration tests of the fake agent, its start receipt key pin
@task_file n=27 path=tests/cli_send.rs
what: edited; integration tests of viola send, a recorded session id literal
@task_file n=28 path=tests/chaos_revive.rs
what: edited; chaos tests of revive, a recorded session id literal
@task_file n=29 path=tests/contract_diag_schema.rs
what: edited; the contract test of the diag-line schema subject enum
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: edited; the contract test of fake agent drift against the recorded dialog sets
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: edited; the contract test of recorded fixture hygiene, now walking the cross-session variant
@task_file n=32 path=tests/contract_ledger_probes.rs
what: edited; the contract test of the ledger probes over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: edited; the security negative tests of env and capture, now the names-only capture keeping no canary byte
@task_file n=34 path=tests/support/home.rs
what: edited; the integration test home support, the wrapper start given a socket root runtime dir
@task_file n=35 path=tests/support/start.rs
what: edited; the integration test start support
@task_file n=36 path=tests/support/verify.rs
what: edited; the integration test verify helpers, with and without a test-owned runtime dir
@task_file n=37 path=fixtures/claude/2.1.328
what: replaced; the recorded claude 2.1.328 hook fixture set, re-recorded whole, its session ids moved
@task_file n=38 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: added; the recorded cross-session prompt arrival fixture, holding the CLI's cross-session prefix with the sender's runtime-dir socket path and pid
@task_item n=1 part=touches
The diag-line subject enum rows: verify-sender is a new subject word beside verify-probe, verify-pty-probe, agents-probe and version-probe.
@task_item n=2 part=touches
The verify child spawn subject rows: verify gains a fourth kind of spawned child, the print-mode sender verify-sender, beside verify-probe, verify-pty-probe, agents-probe and version-probe.
@task_item n=3 part=touches
The process log child and probe lifecycle contract: the sender gets its process-start and process-exit pair; the composed address and its prompt enter no line.
@task_item n=4 part=touches
The verify spawn process-exit lines (child exit status and duration) and the child spawn log subject contract now cover the sender child too.
@task_item n=5 part=touches
The hidden capture arm rows (capture arms uninstrumented, capture arms write no lines, hook stdout of the capture arms): the arm gained a names-only mode, still uninstrumented, filing only closed tool words and no payload byte.
@task_item n=6 part=touches
The environment rows (env names only never values, env vars not a configuration channel or not configuration): verify now reads XDG_RUNTIME_DIR once, held in memory, its value in no line, span or stamp and passed only into the sender's argv.
@task_item n=7 part=touches
The no endpoint, path or env value in spans rows and the verify lines carry no content rows: the composed socket address holds the runtime path and a pid and must reach no verify span or line.
@task_item n=8 part=touches
The child settings argument never logged row: Run B's argv now carries a settings argument with the inbound cross-session accept literal.
@task_item n=9 part=touches
The prompt and assistant output never in home logs rows: the sender's prompt now carries the composed address and reaches the model provider through the sender child, never a log.
@task_item n=10 part=touches
The recorded fixture rows (claude fixtures are scrub-required, fixtures not a telemetry source): the 2.1.328 cross-session arrival fixture keeps the sender's runtime-dir socket path, uid and pid inside the CLI's own prefix; the scrub rewrites only the home and the user word, so the claim that the set is scrubbed of host paths has an exception.
@task_item n=11 part=touches
The absolute-path and path-free rows (absolute paths scrubbed from external output, home-level lines path-free): the fixture hygiene walk's absolute-path check reads only home and users prefixes, so a runtime-dir path passes it.
@task_item n=12 part=touches
The wrapped claude internals are not a contract and transcripts are not a contract rows: verify now relies on the CLI's inbox socket naming (runtime dir, cc-socks, pid, .sock, a 103-byte bound) and its cross-session prompt prefix.
@task_item n=13 part=touches
The capability ledger rows (ledger contents never logged, unreadable ledger is unverified, verify the sole stamps writer): the stamp gains measured cross_session_probe; the relied set is 29 rows and a 28-row stamp is unverified.
@task_item n=14 part=touches
The verify stdout step line and summary line contracts: the relied count moved 28 to 29; a row holding the figure is stale, a row stating it by rule is not.
@task_item n=15 part=touches
The diagnostic schema additive-only rows: the subject enum grew by one word, additive.
@task_item n=16 part=touches
The CI verify against the fake agent only and real-CLI verify never in CI rows: the fake agent now plays the cross-session sender and receiver, resolving the sibling by uds address, so CI stamps the 29th row without the real CLI.
@task_item n=17 part=touches
The hook capture input bound: the names-only capture reads stdin bounded by MAX_FRAME, the same bound the MCP stdin max-frame row holds.
@task_fact n=1 slug=span-trace-coverage
fact: verify-sender is a verify child spawn subject: the print-mode claude sender spawned by Run B's cross-session turn, logged by its process-start and process-exit pair, its prompt and the composed socket address in no line.
@task_fact n=2 slug=log-coverage
fact: verify-sender is a word of the diag-line subject enum.
@task_fact n=3 slug=span-trace-coverage
fact: The hidden hook capture arm's names-only mode is uninstrumented like the other capture arms and files only closed tool words, never a payload byte.
@task_fact n=4 slug=4-span-trace-coverage
fact: verify reads XDG_RUNTIME_DIR once into memory; its value reaches only the sender child's prompt argv and never a log line, span, stamp, step line, error or fixture verify writes itself.
@task_fact n=5 slug=pii-scrubbing-compliance
fact: The recorded claude cross-session arrival fixture keeps, inside the CLI's own prefix, the recording host's runtime-dir socket path (holding the uid), the sender's pid and a numeric from-name; the fixture scrub and the hygiene walk leave it, an exception to the recorded set being scrubbed of host values.
@task_item n=1 part=untouched
The MCP server tools, their refusals and the MCP stdout frames: no MCP surface moved.
@task_item n=2 part=untouched
The channel JSON-RPC lines, corr and conn correlation, and the send, wait, last and dialog path log events: none of their fields or event orders moved.
@task_item n=3 part=untouched
Panic capture, the panic line and the panic outcome by role: no panic path changed.
@task_item n=4 part=untouched
The events log, heartbeat liveness, snapshot replay and state-recovered lines: untouched.
@task_item n=5 part=untouched
Perf budgets, the hook latency budgets and the SLO and error-budget rows: no budget or threshold moved.
@task_item n=6 part=untouched
Service identity, config.json keys, log levels, the logging stack and the diagnostics file locations and modes: no key, level or path moved.
@task_item n=7 part=untouched
Link, unlink, revive and the wheel pause and release paths: untouched.
@task_item n=8 part=untouched
The CI secret scan, artifact upload and retention rows and the banned telemetry dependencies: no dependency added and no upload changed.
