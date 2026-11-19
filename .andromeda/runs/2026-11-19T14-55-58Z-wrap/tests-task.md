@task_prose part=session
The session landed the recorded fixture set for claude CLI 2.1.328 whole: 28 files, a byte copy of the live rig's kept record (hook payloads, screens, agents listing) plus the MCP tools/list exchange recorded by the test-only tap.
The ledger probe suite's stamped list grew from 3 to 4 sets (2.1.287, 2.1.302, 2.1.325, 2.1.328); the recorded sets under the fixtures tree went from 5 to 6; the newest stamped set moved from 2.1.325 to 2.1.328.
The live rig's context arm gained an after-send transcript lookup and a new closed summary word, context-transcript-after: read|absent|unread; a transcript first found after the send is counted from its start (offset 0).
The drift, recorded-exchange, hygiene and ledger-probe suites ran green over every set, 2.1.328 included; the default CLI version, the harness default and the stamped-home version were NOT moved.
A live L2 start read, as a reading and no verification, that on 2.1.328 the /context output reaches the session transcript as one local-command output tag (context-tags 1, context-post tag).
No product crate, CI job, gate shape, config key or schema changed; no capability is claimed.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the standing committed live rig in the test-only E2E crate: its context arm that reads the session transcript under the rig's private config, counts tags only, keeps no byte and prints closed summary words
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsal over the fake agent, asserting each arm's summary words (the context arm now reads context-transcript-after absent)
@task_file n=3 path=fixtures/claude/2.1.328/
what: added — the recorded Claude fixture set for CLI version 2.1.328: hook payloads named by hook event, screen fixtures, the agents listing and the MCP tools/list exchange, replayed by the fake agent and the drift suites
@task_file n=4 path=tests/contract_ledger_probes.rs
what: edited — the ledger probe contract suite: its list of stamped recorded fixture sets that every ledger row's verify probe walks
@task_item n=1 part=touches
decision/fixtures-per-cli-version and contract/claude-fixtures/payload-path: a new per-version recorded set exists for 2.1.328 under the fixtures/claude/<cli-version>/ form; any row naming the sets or their count (now 6) is reached.
@task_item n=2 part=touches
decision/replay-every-cli-version-fixture and decision/multi-version-trigger-layers and decision/compat-tests-for-cli-versions: the replay and compatibility suites now walk the 2.1.328 set as well.
@task_item n=3 part=touches
decision/ledger-probe-contract-suite and decision/ledger-row-verify-probe: the stamped sets the probes walk went from 3 to 4, adding 2.1.328; a row holding that list or count is reached.
@task_item n=4 part=touches
decision/fixture-drift-byte-compare, decision/fake-agent-matches-recordings, decision/fake-agent-replays-recorded-bytes and decision/fake-agent-matches-verify-fixtures: the byte-for-byte drift suites hold the fake agent to the new set too (green).
@task_item n=5 part=touches
decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/scrubbed-fixtures, decision/recorder-scrub and decision/fixtures-never-from-real-prompts: the new set passed the record scrub and the hygiene walk with no hand edit.
@task_item n=6 part=touches
decision/fixtures-recorded-by-live-verify, decision/hook-payloads-recorded-locally, decision/hook-payloads-recorded-by-verify and decision/record-only-at-zero-failing-rows: the set was recorded by a local live verify start (24/24, failing none) and copied whole.
@task_item n=7 part=touches
decision/mcp-exchange-recorded-by-tap and decision/mcp-tap-refuses-dirty-exchange: the set's MCP tools/list exchange was written by the test-only tap on a separate live start.
@task_item n=8 part=touches
decision/standing-live-rig: the rig's context arm gained an after-send transcript lookup and a new closed summary word context-transcript-after (read, absent, unread).
@task_item n=9 part=touches
decision/tests-never-read-host-sessions: indirect — the rig's context arm now also reads a transcript first found after the send, still only under the rig's private config, counting tags and keeping no byte.
@task_item n=10 part=touches
decision/live-failing-reading-not-verification and decision/terminal-reply-reading-live-measured: the L2 /context reading is recorded as read, with no verification claimed.
@task_item n=11 part=touches
decision/local-command-post-condition, decision/local-command-unconfirmable and contract/driver-send/local-command: the L2 reading (one local-command output tag in the transcript after /context) is the evidence a carried proposal to list /context as a local command rests on; nothing was listed here.
@task_item n=12 part=touches
decision/real-claude-local-only, decision/live-verify-never-in-ci and decision/real-claude-never-in-ci: the four live starts on claude 2.1.328 ran locally under the rig's private config, none in CI.
@task_item n=13 part=touches
decision/test-only-e2e-crate and decision/single-test-only-crate: indirect — the code change is confined to the test-only crate's rig and its rehearsal.
@task_item n=14 part=touches
contract/fake-agent-screens/trust-dialog and decision/harness-cli-version-matches-fake: the newest stamped set is now 2.1.328 while the fake's trust form and the one default CLI version stay on 2.1.325; a row equating the newest set with the default is reached.
@task_fact n=1 slug=6-e2e-test-strategy
fact: The standing live rig's context arm prints the closed word context-transcript-after (read, absent or unread) after the send; a transcript absent before the send and found after it is counted from its start, still tags only under the rig's private config.
@task_item n=1 part=untouched
Product behaviour rows (send, answer, wait, release, revive, run start, list, verify outputs and exit codes): no product crate changed.
@task_item n=2 part=untouched
MCP server, tool result, invalid-params and channel JSON-RPC contract rows: no MCP or channel surface changed.
@task_item n=3 part=untouched
Harness command, status, boot, cleanup, gate, pre-push and run document contracts: no harness or gate surface changed.
@task_item n=4 part=untouched
CI integration rows (actions, permissions, artifacts, caches, toolchain pins, supply chain): no CI or workflow change.
@task_item n=5 part=untouched
Coverage, mutation, performance and fuzz gate rows and their thresholds: no gate value moved.
@task_item n=6 part=untouched
Log format, log line and log event corr contracts: no log field changed.
@task_item n=7 part=untouched
Security control rows over sockets, strict modes, peer checks, env strip and pinned-exe hash: none was touched.
@task_item n=8 part=untouched
Stamp-writing rows (stamps only from verify against the fake agent, one verify stamp per build): stamping and the stamped-home version did not move.
