@task_prose part=session
Added the 29th relied capability-ledger row, cross-session-prefix, reading pass or fail only; the relied set moved 28 to 29 (step lines [NN/29], 29 pass 0 fail) and a 28-row stamp now reads unverified.
verify's Run B gained a cross-session turn: a print-mode sender child (subject verify-sender) sends to the Run B CLI's composed uds socket address, built from one read of XDG_RUNTIME_DIR held in memory; Run B runs under an accept-inbound settings literal.
viola hook gained a hidden --names-only capture arm that files only closed-word tool files, no payload byte; the ledger stamp gained measured.cross_session_probe; the diag-line subject enum gained verify-sender.
The fake agent's sender now resolves its sibling by the uds address (the bare session id form was retired after the live XS red); its start receipt gained pid and prompt; the drift reader leaves out send-tool hook payloads.
Every test and harness stamping verify now gets a short owner-only test-owned runtime dir, because pre-push runs children in a clean environment where XDG_RUNTIME_DIR is unset; the rig rehearsal carries its env over when it rebuilds a command.
The 2.1.328 fixture set was re-recorded whole by the live rig (XS-2, 29/29), 32 to 33 files with one new cross-session arrival; four recorded session id literals moved; the new arrival holds the recording host's runtime-dir path and the sender pid, which the recorder scrub does not normalise.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited - the capability ledger: relied rows, verify probes and readings, stamp merge, the recorder scrub and fixture hygiene checks
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited - the claude agent crate's compiled literals (socket scheme, dir, suffix, path bound, allowed-tools flag)
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: edited - hook payload reading: the harness-prefix literals and the prompt origin of a submitted prompt
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited - ledger unit tests: tool words, capture documents, sender plugin hooks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited - ledger unit tests: per-row probe checks, the cross-session row's pass and fail table, the uds address cases
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited - ledger unit tests: verify's framing turns
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited - ledger unit tests: stamp merge and the measured block
@task_file n=8 path=src/cmd/hook.rs
what: edited - the viola hook exec-form entry points and their capture arm
@task_file n=9 path=src/cmd/verify.rs
what: edited - viola verify: the measure, its child set and the stamped step lines
@task_file n=10 path=src/cmd/verify/typed.rs
what: edited - viola verify's typed runs inputs (the held runtime dir)
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: edited - viola verify's typed runs and how each ends
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: edited - viola verify's Run B turns, the cross-session sender turn
@task_file n=13 path=src/run/version_gate.rs
what: edited - the unverified CLI version gate's tests
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited - the fake agent serving the claude boundary: its modes, receipt and cross-session send and receive
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: edited - the fake agent's agents listing and sibling lookup by socket address
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited - fake agent unit tests: the sender and receiver modes
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited - fake agent unit tests: argv options over the recorded set
@task_file n=18 path=schemas/diag-line.v1.json
what: edited - the diagnostics line schema and its closed subject enum
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: edited - the harness boot command: makes the home, stamps it by verify, starts wrappers
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: edited - the harness run command and its local-live suite
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: edited - the standing live rig: kept stamps, its floor rows and summary lines
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited - the live rig rehearsals over the fake agent
@task_file n=23 path=tests/cli_verify.rs
what: edited - viola verify integration suite: step lines, floor rows, the cross-session row cases
@task_file n=24 path=tests/cli_verify/runs.rs
what: edited - viola verify integration suite: the typed runs
@task_file n=25 path=tests/cli_verify/stamps.rs
what: edited - viola verify integration suite: stamps written only by verify
@task_file n=26 path=tests/cli_fake_agent.rs
what: edited - the fake agent's receipt and env contract suite
@task_file n=27 path=tests/cli_send.rs
what: edited - viola send integration suite: send cases copying recorded fixtures
@task_file n=28 path=tests/chaos_revive.rs
what: edited - revive chaos tests over the recorded session
@task_file n=29 path=tests/contract_diag_schema.rs
what: edited - diagnostics schema conformance check
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: edited - fake agent drift: fake transcripts compared byte for byte to the recorded dialogs
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: edited - the fixture hygiene run-time walk
@task_file n=32 path=tests/contract_ledger_probes.rs
what: edited - the ledger probe contract suite over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: edited - security negatives for env and capture: canary inputs never filed
@task_file n=34 path=tests/support/home.rs
what: edited - shared test helper: per-test home and test wrappers started as a user
@task_file n=35 path=tests/support/start.rs
what: edited - shared test helper: test wrapper starts over the recorded session
@task_file n=36 path=tests/support/verify.rs
what: edited - shared test helper: stamping verifies under a test-owned home and runtime dir
@task_file n=37 path=fixtures/claude/2.1.328
what: replaced - the recorded claude hook fixtures for one CLI version, re-recorded whole by live verify
@task_file n=38 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: added - the recorded cross-session arrival hook payload, a fixture named by hook event
@task_item n=1 part=touches
The relied ledger set moved 28 to 29 with the cross-session-prefix row; any row holding the relied count, the [NN/MM] step line or the summary counts as a figure moves (version gate needs every relied row; verify output step line and summary line).
@task_item n=2 part=touches
Every ledger row has a verify probe: the new row's probe is a Run B cross-session turn by a print-mode sender child, gated on the Run B child pid and a composed uds address.
@task_item n=3 part=touches
Unverified CLI gate: a 28-row stamp now reads unverified and a 29-row stamp verified; rows on the unverified gate and the floor reach it through the relied set.
@task_item n=4 part=touches
Hook payloads recorded by verify and replayed in CI: the recorded set gains a cross-session UserPromptSubmit variant and was re-recorded whole (32 to 33 files), moving the recorded session ids four test literals pinned.
@task_item n=5 part=touches
Fake agent held to recorded verify fixtures and fake agent serves the claude boundary: the fake sender now resolves its sibling by the uds socket address, not a bare session id, and receives cross-session only under the accept settings.
@task_item n=6 part=touches
Fake agent drift compared byte for byte: the dialog-tier reader now leaves out a hook whose payload names the send tool, an exception to the positional comparison.
@task_item n=7 part=touches
contract/fake-agent-receipt/start: the start receipt gained pid and the print prompt (nine keys); contract/fake-agent-receipt/hook gains the sender's synthetic hook lines.
@task_item n=8 part=touches
Fake-agent replay for stamps, one verify stamp per build, stamps from fake-agent verify: each stamping verify now needs a test-owned runtime dir to stamp the 29th row.
@task_item n=9 part=touches
Test verifies and runs under a test-owned user home, every test verify start tied to a test-owned home: they now also take a short owner-only test-owned runtime dir.
@task_item n=10 part=touches
pre-push children run in a clean environment: under it XDG_RUNTIME_DIR is unset, so the harness and test support supply one; the first pre-push firing read two rig rehearsals red for it.
@task_item n=11 part=touches
Harness boot makes the home, stamps it and starts wrappers: the boot stamp now runs verify with an owner-only runtime dir it owns for the stamp's life.
@task_item n=12 part=touches
Test-started viola drops inherited env and environment set per child: XDG_RUNTIME_DIR is now set per verify child by the tests, and the rig rehearsal sets one only where the host sets none.
@task_item n=13 part=touches
Test endpoint directories made private: the new test runtime dirs are owner-only directories under the OS temp dir, outside the e2e home root and its scans.
@task_item n=14 part=touches
One standing committed live rig: new labels XS and XS-2, kept stamps cross-stamp.json and cross-stamp-2.json, summary lines cross-session-prefix, cross-session, sender-tools, send-result; the rig's floor rows gained the row under its rule every relied row but four.
@task_item n=15 part=touches
Live reading recorded as a reading: XS read red 28/29 (bare session id refused) and XS-2 green 29/29; stale-socket counts recorded as readings.
@task_item n=16 part=touches
Recorder scrub and fixtures free of real data: the new arrival fixture holds the recording host's runtime-dir path (with the uid), the sender pid and a numeric from-name inside the CLI's own prefix; the scrub and the hygiene walk's absolute-path class pass it.
@task_item n=17 part=touches
Fixture hygiene walk: the walk now asserts the cross-session variant is walked for the stamped set.
@task_item n=18 part=touches
Diagnostics schema conformance and closed enums change only by decision: the diag-line subject enum gained verify-sender.
@task_item n=19 part=touches
Synthetic inputs carry a canary and one negative per security control: a new negative proves the names-only capture arm keeps no canary byte.
@task_item n=20 part=touches
viola hook exec-form entry points: a hidden names-only capture arm, valid only with capture, files closed-word tool files for pre- and post-tool-use alone.
@task_item n=21 part=touches
Ledger stamps content and stamps written only by verify: the stamp gains measured.cross_session_probe of closed words and one latency number.
@task_item n=22 part=touches
Ledger probe contract suite: the cross-session row passes over every stamped set in the e2e layer.
@task_item n=23 part=touches
Fake agent and verify fixtures are the contract, every CLI-version fixture set replayed: the drift and replay suites now read the cross-session arrival in the 2.1.328 set.
@task_item n=24 part=touches
Fake-agent receipt env holds names never values and no env value in fixtures, logs or stamps: verify's runtime-dir value reaches only the sender's prompt argv.
@task_fact n=1 slug=test-data-bootstrap
fact: Every test or harness viola verify that stamps runs with a short owner-only test-owned runtime dir, because verify reads XDG_RUNTIME_DIR once and the clean pre-push environment leaves it unset; the rig rehearsal sets its own only where the host sets none.
@task_fact n=2 slug=test-data-fixtures
fact: The fake agent's sender resolves its target by the uds socket address alone, delivers only to its sibling, refuses an address not of the uds shape, and the receiver reads its inbox only under the accept settings.
@task_fact n=3 slug=e2e-test-strategy
fact: The fake-agent dialog drift comparison leaves out hook payloads that name the send tool, so a side sender's hooks never shift recorded dialog positions.
@task_fact n=4 slug=7-test-data-fixtures
fact: The recorded cross-session arrival keeps, inside the CLI's own prefix, the recording host's runtime-dir path, the sender pid and a numeric from-name; the recorder scrub normalises only home and user and the hygiene walk flags only home-shaped absolute paths, so it passes; the runtime dir reaches no file verify writes of its own.
@task_fact n=5 slug=e2e-test-strategy
fact: A rig rehearsal that rebuilds a command from its program and args carries the original command's env over; dropping it is red only under the clean pre-push environment.
@task_fact n=6 slug=6-e2e-test-strategy
fact: The names-only hook capture arm is held by a canary negative: no payload byte reaches its filed tool files.
@task_item n=1 part=untouched
MCP server, MCP tool results, MCP invalid-params fields and MCP and CLI parity: no MCP surface moved.
@task_item n=2 part=untouched
viola send, wait, answer, release, last and list contracts, refusal orders and wheel rules: none moved.
@task_item n=3 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods, socket permissions and server identity checks: the channel endpoint and its socket dir are unchanged.
@task_item n=4 part=untouched
revive exit codes, refusal codes and start refusals: not reached.
@task_item n=5 part=untouched
CI workflow, supply-chain gates, SHA-pinned actions, MSRV, rust-cache and artifact retention: no workflow or dependency moved.
@task_item n=6 part=untouched
Coverage floors, mutation gate thresholds and verdicts, perf budgets and hyperfine gates: no threshold moved.
@task_item n=7 part=untouched
Heartbeat staleness, instance liveness, snapshot replay and torn-append healing: not reached.
@task_item n=8 part=untouched
Process log line format, log event corr contracts, status document shape and pid-file rows: no log line or key added beyond the diag subject enum.
@task_item n=9 part=untouched
Screen fixtures, MCP exchange tap and its scrub refusal codes, human-mode output literals: not reached.
