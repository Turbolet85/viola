@task_prose part=session
The session landed the claude 2.1.325 recorded fixture set: 28 files recorded by a clean live viola verify --record start and by the MCP tap's re-record of the viola mcp exchange, both under the live rig's private config.
The test-only live rig gained two arms: VR keeps the verify record beside its frames and refuses a scratch base naming a home component, and MT spawns claude -p with an MCP config putting the tap in front of viola mcp.
The rig gained the live labels R4, R4-2 and R4M, the arm words vr and mt, the --tap option, the refusal word record-home and new summary words; all of it in the test-only e2e crate, no product crate reached.
The stamped recorded sets every 24-row ledger probe reads moved from 2 to 3 (2.1.287, 2.1.302, 2.1.325); the recorded sets under the fixtures tree moved from 4 to 5.
Measured live on 2.1.325: verify 24/24 with no failing row, the browser-tools prompt absent, Run E on a clean recording; the MCP child exited 0 and the tap fixture was written.
No viola format, schema, CLI verb, output line, exit code or MCP tool changed; nothing retired.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's arms, labels, plan and start flow: the verify-record arm and the MCP-tap arm, the record-home refusal, the summary words
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: the test-only rig's scratch isolation: keeping the verify record as private copies, and the home-component check on a path
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only viola-live-rig binary's entry: the rehearse arms vr and mt and the --tap option
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal integration tests: the record keep, the record-home refusal, the MCP-config arm's teardown
@task_file n=5 path=tests/contract_ledger_probes.rs
what: the contract ledger probe suite over every stamped recorded set; its stamped-set list now names 2.1.325
@task_file n=6 path=fixtures/claude/2.1.325
what: the recorded fixture set of claude 2.1.325: the hook payloads, screens, agents record and the recorded viola mcp tools-list exchange
@task_item n=1 part=touches
contract/viola-verify-output/record: viola verify --record wrote the 2.1.325 set the rig kept; the session relies on that contract unchanged, so a picker checks its record-dir and count words still read true.
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify/summary: the live R4 start read 24/24 with failing none; where a row's cell holds the row count 24 it still holds, no count moved.
@task_item n=3 part=touches
contract/viola-verify-refusals/fixture-not-clean: the new 2.1.325 set passed the fixture hygiene walk; a row about a recorded fixture not clean rules over the set the session added.
@task_item n=4 part=touches
contract/mcp-tools/answer, last, list, send and wait: the tools-list result of viola mcp is now recorded for 2.1.325 as well as 2.1.302; the recorded descriptions carry no usage-budget sentence; the tool set itself did not change.
@task_item n=5 part=touches
decision/mcp-verb-hidden and decision/mcp-server-without-human-surface: viola mcp was spawned by the MCP tap under a private MCP config during the live MT start; the hidden verb and its no-human-surface rule are relied on, not changed.
@task_item n=6 part=touches
contract/viola-verify-refusals/home-not-private and contract/cli-global-flags/home: the rig's arms run viola with --home on a private scratch viola home; the refusal and the flag are used as written.
@task_item n=7 part=touches
contract/viola-verify-output/stop and contract/viola-verify-refusals/stopped: the live starts ran with stop none; no stop path changed.
@task_item n=1 part=untouched
The board, viola list, the BAY line and every styling, colour, token-rendering and truncation row: no rendering or layout code changed.
@task_item n=2 part=untouched
The send, wait, last, answer, link, pause and release output and refusal rows and the readback mirror rows: no verb output or wording moved.
@task_item n=3 part=untouched
CLI exit codes, the internal-error forms, the --json documents and the exit-21 hints and details: no code, field or hint was added or changed.
@task_item n=4 part=untouched
viola run and viola revive start, preflight, list and refusal rows: no start path, refusal or output of the product changed.
@task_item n=5 part=untouched
viola plugin install, viola --help and per-verb usage rows, and the absences of a web surface and of deep links: no verb or capability was added or removed.
