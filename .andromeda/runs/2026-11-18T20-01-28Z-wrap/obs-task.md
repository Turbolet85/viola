@task_prose part=session
The session landed a fifth recorded claude fixture set, for claude 2.1.325, recorded live by the test-only rig under its private CLAUDE_CONFIG_DIR with the browser-tools key seeded off, not a reading of a real home.
The test-only live rig gained two arms: VR (labels R4 and R4-2) keeps the recorder's scrubbed record at 0600 beside its frames and refuses, uncounted with exit 3 and the word record-home, a scratch base naming a home component; MT (label R4M) starts claude -p with the test-only MCP tap in front of viola mcp and writes the tap's fixture.
The rig's summary gained closed words: record, mcp-child, stdout-bytes, stderr-bytes, stderr-sha, capture-lines, tap-fixture; MT drops the child's stdout and only counts and hashes its stderr.
The ledger probe suite's stamped sets moved from 2 to 3 (2.1.287, 2.1.302, 2.1.325); the drift, ledger-probe and recorded MCP exchange suites now read the new set.
Only the test-only crate viola-e2e changed; no product crate, no viola format or schema, no diagnostics line, span, metric or CI workflow changed; CI on the pushed head is green 7/7.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig of the test-only crate: its arms, labels, the start ledger, the closed summary words and the MCP tap arm's direct claude -p spawn
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: the rig's private scratch isolation: the private CLAUDE_CONFIG_DIR and vhome, the kept frames and now the kept record copy, the home-component check, the teardown census
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only viola-live-rig binary: its rehearse and live commands, arm words and the new --tap option
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's integration rehearsals against the fake agent, now for the record keep, the record-home refusal and the MCP tap arm's config and teardown
@task_file n=5 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite and its STAMPED list of recorded sets every 24-row probe reads
@task_file n=6 path=fixtures/claude/2.1.325/
what: the recorded claude fixture set for 2.1.325: scrubbed hook payloads, screens, an agents row and the recorded MCP tools-list exchange, 28 files
@task_item n=1 part=touches
The rule that recorded claude fixtures are no telemetry source: a new recorded set landed, written by the recorders and the tap from a live start, never derived from diagnostics lines.
@task_item n=2 part=touches
The rule that claude fixtures are scrub-required: the 2.1.325 set is the recorders' and the tap's scrubbed output, passed the fixture hygiene walk and an identity read; landed at mode 0644 after being kept at 0600.
@task_item n=3 part=touches
The rule that real-CLI verify never runs in CI and CI verifies against the fake agent only: the live R4 start ran viola verify against real claude 2.1.325 by path in a local live round, not in CI; the rehearsals in CI use the fake agent.
@task_item n=4 part=touches
The rules that wrapped claude internals and transcripts are not a contract: the set records claude 2.1.325's hook payloads and MCP exchange as test fixtures only.
@task_item n=5 part=touches
The rules over viola mcp's stdout carrying frames only and its stdin bounded by MAX_FRAME: the MT arm puts the test-only tap in front of viola mcp and records its exchange; the rig's own reads of the record and capture are bounded through MAX_FRAME.
@task_item n=6 part=touches
The rules that stripped CLAUDE environment values and the child's env are never logged: the MT arm spawns claude -p with the rig's child env and private CLAUDE_CONFIG_DIR, its stdout dropped and stderr only counted and hashed.
@task_item n=7 part=touches
The rule that the browser-tools prompt reading is uninstrumented: R4 read browser-tools-prompt absent on 2.1.325 with the browser-tools key seeded off.
@task_item n=8 part=touches
The rules that verify log lines carry no content and verify logs only inside an instance: the R4 start ran viola verify under the rig's private vhome, 24 of 24 ledger rows.
@task_item n=9 part=touches
The rules that local run outputs are never uploaded and test homes sit under the scanned root: the rig's kept record dir target/live-rig/<label>/record/ is gitignored with the frames and stays local; the MT arm's 0600 mcp.json lives in its scratch and is removed with it.
@task_item n=10 part=touches
The rule that drift reports go to diagnostics only: the drift suites now read the 2.1.325 set; no drift report path changed.
@task_item n=11 part=touches
The rules naming the components left uninstrumented: the test-only rig and the tap stay outside product telemetry; their summary words are a closed test-harness vocabulary, not diagnostics lines.
@task_item n=12 part=touches
Any row whose cell holds the count of recorded sets under the fixtures or the stamped sets the ledger probes read: the recorded sets moved 4 to 5, the stamped sets 2 to 3.
@task_item n=13 part=touches
The Linux-only target and the hand read of the token window: the live round ran on the Linux host; the builder read the real credentials file's expiry once by hand outside the rig, nothing kept or logged.
@task_item n=1 part=untouched
The product diagnostics line schema, its field names, event enum, detail codes and process-exit codes: no product crate and no emitted line changed.
@task_item n=2 part=untouched
Span and trace coverage, span names, attributes and kinds of viola run, send, wait, dialog and hook paths: no span changed.
@task_item n=3 part=untouched
Metric coverage, derived counters, gauges and histograms, and the SLO and perf budgets of the hooks: no budget or threshold moved.
@task_item n=4 part=untouched
Error capture and panic handling by role, the panic hook, catch sites and the detail-file error line: unchanged.
@task_item n=5 part=untouched
PII scrubbing of diagnostics lines, veil redaction, the field allow-list and the secret-scan hit report: unchanged.
@task_item n=6 part=untouched
CI integration gates, the schema conformance, zero-panics and bare-instrument gates, artifact retention and the CI workflow: no gate or workflow changed.
@task_item n=7 part=untouched
The channel, send, wait, link and wheel log contracts, the heartbeat, config.json keys and the agent-run logs and status contracts: unchanged.
@task_item n=8 part=untouched
The logging stack, subscriber, log file locations, modes and home resolution of the product roles: unchanged.
