@task_prose part=session
The session landed the recorded Claude fixture set for CLI version 2.1.325, 28 files, recorded live on the standing live rig under its private config.
Two live starts were spent on claude 2.1.325 by path: R4, a clean viola verify --record at 24/24 with failing none, and R4M, the MCP tap re-record; the spare R4-2 was not used.
The live rig gained two arms: VR, a verify whose record is kept, and MT, a direct claude -p start with the test-only MCP tap in front of viola mcp. It also gained the labels R4, R4-2 and R4M, and a kept record dir under target/live-rig/<label>/record/.
The VR arm refuses, uncounted and before any ledger line, a scratch base that names a home component (refused: record-home, exit 3).
The ledger probes' stamped recorded sets went from 2 to 3 (2.1.287, 2.1.302, 2.1.325), and the recorded sets under fixtures/claude went from 4 to 5.
Only the test-only E2E crate changed. No product crate, dependency, schema, CI workflow or harness command changed, and nothing was retired.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: edited - the standing live rig's arms, plans, start and finish in the test-only E2E crate (gains the VR record-keeping arm, the MT MCP-tap arm, the record-home refusal and the record and mcp-child summary words)
@task_file n=2 path=crates/viola-e2e/src/rig/isolate.rs
what: edited - the live rig's scratch isolation: its private scratch and CLI config, first-screen keeping and teardown (gains the byte-for-byte owner-only record keep that never overwrites, and the home-component path check)
@task_file n=3 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited - the viola-live-rig test-only binary, its rehearse and live commands (gains the vr and mt rehearse arms, the --tap option and the live labels R4, R4-2 and R4M)
@task_file n=4 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited - the live rig's rehearsal integration tests over the fake agent (gain the vr record keep, the vr record-home refusal and the mt config and teardown cases)
@task_file n=5 path=tests/contract_ledger_probes.rs
what: edited - the ledger probe contract suite, which runs every ledger row's verify probe over each stamped recorded fixture set (its stamped set list gains 2.1.325)
@task_file n=6 path=fixtures/claude/2.1.325
what: added - the recorded Claude fixture set for CLI version 2.1.325: hook payloads named by hook event, screen fixtures, the agents listing and the recorded MCP tools/list exchange
@task_item n=1 part=touches
decision/record-home-names-no-home: the rule gains the rig's half. The VR arm refuses a scratch base whose path or canonical form holds a component named exactly home, with refused: record-home, exit 3, uncounted and before any ledger line.
@task_item n=2 part=touches
decision/standing-live-rig: the one standing live rig gains the VR and MT arms (words vr and mt), the live labels R4, R4-2 and R4M, the --tap option and a kept record dir per label. It is still one rig.
@task_item n=3 part=touches
decision/ledger-probe-contract-suite: the probes now read three stamped sets (2.1.287, 2.1.302, 2.1.325) instead of two. Any cell that quotes the stamped list or its size of 2 is now stale.
@task_item n=4 part=touches
decision/fixtures-per-cli-version and decision/replay-every-cli-version-fixture: a fifth recorded set (2.1.325) is added and replayed. Any cell that names the recorded versions or their count must now include it.
@task_item n=5 part=touches
decision/fixtures-recorded-by-live-verify and decision/hook-payloads-recorded-by-verify: the 2.1.325 hook payloads and screens were recorded by a live viola verify --record. That verify ran inside the rig's private scratch, and the rig kept its record and copied it to the fixture set unchanged.
@task_item n=6 part=touches
decision/mcp-exchange-recorded-by-tap and decision/mcp-tap-refuses-dirty-exchange: the MCP exchange for 2.1.325 was re-recorded by the test-only tap, started live as the rig's R4M, with tap-fixture written.
@task_item n=7 part=touches
decision/record-only-at-zero-failing-rows: the kept record came from a start at 24/24 with failing none, and the kept copy is the recorder's output unchanged.
@task_item n=8 part=touches
decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene and decision/scrubbed-fixtures: the hygiene walk now also covers the 2.1.325 set, and it read clean.
@task_item n=9 part=touches
decision/fixture-drift-byte-compare, decision/fake-agent-matches-recordings and decision/fake-agent-contract-suite: the drift suites now run over the new 2.1.325 set.
@task_item n=10 part=touches
decision/fixture-named-by-hook-event and contract/claude-fixtures/payload-path: the new files follow the per-version payload path and the hook-event naming, including SessionStart.resume-1.
@task_item n=11 part=touches
decision/compat-tests-for-cli-versions and decision/multi-version-trigger-layers: the compatibility coverage now reaches a third stamped CLI version.
@task_item n=12 part=touches
decision/real-claude-local-only, decision/real-claude-never-in-ci and decision/live-verify-never-in-ci: the two new arms are live starts of the real CLI. They run locally only, and CI is unchanged.
@task_item n=13 part=touches
decision/tests-never-read-host-sessions: the rig's VR and MT starts run under the rig's private CLI config and write nothing under the real home. A rule about the live rig and host homes is reached.
@task_item n=14 part=touches
decision/private-placement-mode-first and decision/private-filesystem-permissions: the record keep writes owner-only (0600) copies through the private placement. The MT arm's mcp.json is owner-only, and it is removed with the scratch.
@task_item n=15 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: the record keep reads within the frame cap. The MT arm's pipes and capture are read within the cap, its child is bounded by a deadline, and the tap fixture step is bounded at 20 s.
@task_item n=16 part=touches
decision/single-test-only-crate and decision/test-only-e2e-crate: all new code lives in the test-only E2E crate. The MT arm spawns the sibling test-only viola-mcp-tap binary.
@task_item n=17 part=touches
decision/live-failing-reading-not-verification: the C10 hypothesis stays a hypothesis. This run only read that Run E starts on a clean 2.1.325 recording, and nothing is claimed for 2.1.326 or 2.1.327.
@task_item n=18 part=touches
decision/relayed-fixtures-kept-superseded: the older recorded sets stay in place beside the new set.
@task_item n=19 part=touches
decision/unverified-cli-gate: the StopFailure owner is no longer the 2.1.325 recorded fixture set. The founder moved it to its own entry, A live StopFailure payload. If this row's exception names that owner, it is stale; the exception itself stays provisional.
@task_item n=20 part=touches
decision/screen-fixture-box-rows-as-literals and decision/screen-fixture-scrub-as-detector: the new set carries the Screen.modal, Screen.ready and Screen.turn fixtures for 2.1.325.
@task_fact n=1 slug=test-data-fixtures
fact: The live rig's record-keeping arm refuses a scratch base whose path or canonical form holds a component named exactly home. The refusal is refused: record-home, exit 3, uncounted, and comes before the credentials read and any ledger line, so a kept record is never made under a home.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The standing live rig keeps a clean verify's record (arm VR, labels R4 and R4-2) beside its frames under target/live-rig/<label>/record/<version>/. The copy is byte for byte and owner-only, leaves out FirstScreen files and never overwrites; that dir is the source a recorded set is copied from.
@task_fact n=3 slug=7-test-data-fixtures
fact: The MCP exchange is recorded live as the rig's MT arm (label R4M): one direct claude -p --strict-mcp-config start under the rig's private CLI config, with the test-only tap in front of viola mcp. Any child exit other than 0 leaves the start not whole.
@task_fact n=4 slug=7-test-data-fixtures
fact: A recorded set is recorded under the rig's private CLI config with the browser-tools key seeded off. It is a plain copy of the recorders' output, not a reading of a real home.
@task_item n=1 part=untouched
Product behaviour contracts: the wrapper channel, the JSON-RPC errors, the CLI exit codes, the --json documents, the human output, and viola send, wait, answer, release, list and revive. No product crate was reached.
@task_item n=2 part=untouched
The agent-run harness commands and their documents: boot, run, status, cleanup, gate, pre-push, schema-check and secret-scan. The report says the agent-run harness is unchanged.
@task_item n=3 part=untouched
CI integration, coverage floors, mutation gates, perf budgets, fuzzing and the supply-chain gates. No workflow, threshold, tool pin or dependency moved.
@task_item n=4 part=untouched
The fake agent's command line, receipts, scripts and screens. The fake agent was not changed.
@task_item n=5 part=untouched
The log format and process log lines, the event line and event data keys, and the hook exec-form contracts.
@task_item n=6 part=untouched
Stamp writing: stamps are still written only by verify against the fake agent, and the pinned product was not rebuilt or re-pinned.
