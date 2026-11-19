@task_prose part=session
The session landed the recorded CLI fixture set for claude 2.1.328 whole: 28 files, a byte copy of the live rig's kept recording plus the MCP tap's tools-list fixture, under the rig's private config.
The stamped recorded sets go from 3 to 4 (2.1.287, 2.1.302, 2.1.325, 2.1.328), the recorded sets under the fixtures tree from 5 to 6, and the newest stamped set moves from 2.1.325 to 2.1.328; the default CLI version, the harness default and the stamped-home version did not move.
The test-only live rig's context arm gained an after-send transcript lookup and a new closed summary word, context-transcript-after: read|absent|unread; no product crate, CLI verb, output line, exit code or config key changed.
It measured, as a reading and not a verification, that on 2.1.328 the /context output reaches the session transcript as one local-command output tag after one send, the send itself reading not-delivered no-prompt-submitted as expected.
It also recorded, unmended and carried to the route, that the pre-send gate refuses a send with not-delivered input-not-ready when a transcript row quotes a modal literal.
No capability is claimed, nothing is retired, and no spec master was edited.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig, its context arm that reads the private-config session transcript and counts local-command output tags, and its closed summary words
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal over the fake agent, the context arm's rehearsal case and its summary words
@task_file n=3 path=fixtures/claude/2.1.328
what: a recorded CLI fixture set for one claude version: hook payloads, screen captures (ready, turn, modal), session start and end records and the MCP tools-list exchange, read by the drift, recorded-exchange and fixture-hygiene suites and by viola verify's fixture check
@task_file n=4 path=tests/contract_ledger_probes.rs
what: the contract ledger probes suite and its list of stamped recorded fixture sets
@task_item n=1 part=touches
A row whose cell holds the number of stamped recorded fixture sets, their members or the newest stamped CLI version: the stamped sets are now 4 and the newest is 2.1.328 (was 3 and 2.1.325).
@task_item n=2 part=touches
A row naming the recorded CLI fixture sets by count or by member: they are now 6 (2.1.283, 2.1.287, 2.1.288, 2.1.302, 2.1.325, 2.1.328).
@task_item n=3 part=touches
The viola verify refusal for an unclean fixture and the fixture hygiene it rests on: a new recorded set now stands and passed the hygiene walk with no hand edit.
@task_item n=4 part=touches
The refusal hint for not-delivered input-not-ready: a measured case where a transcript row quoting a modal literal refuses the send with that refusal, though no modal stands; recorded, not mended.
@task_item n=5 part=touches
The refusal hint for not-delivered no-prompt-submitted: a /context send on 2.1.328 read that reply, the expected one for a local command.
@task_item n=6 part=touches
The send readback forms for slash commands (/clear, /compact, /mcp): /context was read on 2.1.328 to post one local-command output tag to the transcript, the reading a later listing of /context as a local command is conditional on; no form was added here.
@task_item n=7 part=touches
The mcp verb's stdout and tools list: a tools-list exchange was recorded for 2.1.328 with no change to the verb's output shape.
@task_item n=8 part=touches
The fixed phraseology vocabulary, only if a row reaches the live rig's summary words: the rig gained the closed word context-transcript-after with values read, absent and unread; it is printed by a test-only tool, not by any viola verb.
@task_item n=9 part=touches
The viola verify output rows that name the stamped version or a stamped home: the stamped-home version and the default CLI version were NOT moved, so a row holding them stands.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and iconography: no glyph, colour token, column, strip or readback-box element changed.
@task_item n=2 part=untouched
The viola list board and table, the flight-progress strip and the wheel column: no board output moved.
@task_item n=3 part=untouched
CLI exit codes, exit-21 causes and their --json causes, the wrapper fault detail and the peer reply screening: no code, cause or detail word changed.
@task_item n=4 part=untouched
viola run, revive, link, unlink, pause, release, answer, wait and last output and refusals, and the strict-modes refusals: none of those verbs was touched.
@task_item n=5 part=untouched
The CLI colour decision order, colour depth, streams, catch sites, help output and the command names: no verb was added, renamed or hidden, and no name was taken or freed.
@task_item n=6 part=untouched
The web-page surface absence, the rendered-page absence and the self-validation protocol: nothing rendered was added.
