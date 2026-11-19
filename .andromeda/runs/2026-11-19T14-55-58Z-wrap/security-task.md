@task_prose part=session
The session landed the recorded fixture set for claude 2.1.328: 28 files copied byte for byte from the live rig's kept record (R4-2) and its test-only MCP tap (R4M), all under the rig's private config, scrubbed by the recorder and passed by the hygiene walk with no hand edit.
The ledger probe suite's stamped sets go from three to four, now naming 2.1.328; the drift and recorded-exchange suites run green over every set. The product's default CLI version and the stamped home's version did not move.
The test-only live rig's context arm gained an after-send transcript lookup: when no session transcript was held before the send, it looks once more after the send and counts tags of a transcript found then from its start (offset 0), through the same bounded, count-only read, keeping no byte.
The rig prints a new closed summary word, context-transcript-after read, absent or unread. One live L2 start on 2.1.328 read the transcript absent before the send and present after it, with one local-command output tag, as a reading and no verification.
No product crate, dependency, config key, schema, scrub shape, CI workflow or gate changed; nothing was retired and no capability is claimed.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live round rig (viola-live-rig) and its context arm, which reads the session transcript under the rig's private config, count-only, and prints closed summary words; edited to add the after-send transcript lookup and its unit cases
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal over the fake agent, which checks the rig's summary words; edited to assert the new context-transcript-after word reads absent over the fake
@task_file n=3 path=fixtures/claude/2.1.328
what: a recorded fixture set of the real claude CLI (hook payloads, screen signature fixtures, the MCP tools-list exchange recorded through the test-only tap, the agents record), scrubbed at recording and held under the fixture hygiene walk; added whole, 28 files
@task_file n=4 path=tests/contract_ledger_probes.rs
what: the contract ledger probe suite and its list of stamped recorded sets the probes walk; edited to add 2.1.328 to that list
@task_item n=1 part=touches
decision/live-rig-context-arm-keeps-no-transcript: its rule of counting only tags past the kept length gains an exception, for a transcript first found after the send the arm counts from its start; still no transcript byte kept.
@task_item n=2 part=touches
The transcript read stays bounded and count-only: the new after-send lookup reads through the same bounded tag count under the MAX_FRAME cap, so rows on a bounded, count-only transcript read and on reads capped at MAX_FRAME rule over it.
@task_item n=3 part=touches
The live rig's closed summary words gain one, context-transcript-after with read, absent or unread; rows that keep the rig's captures to closed words only rule over it.
@task_item n=4 part=touches
The live rig child stays isolated from the user's config: every start ran under the rig's private config; the L2 start printed user-config changed-by-others (the host's own sessions), and the rig's own check that it changed nothing in the user's Claude config held.
@task_item n=5 part=touches
Recorded fixtures scrubbed or refused whole and the fixture hygiene walk: a new recorded set landed under them, passed with no hand edit.
@task_item n=6 part=touches
Fixtures reviewed before commit: the new set was read for identity (a landing read, clean) before it was committed.
@task_item n=7 part=touches
MCP fixtures recorded through a test-only tap: the new set's MCP tools-list exchange was recorded by that tap on a live start.
@task_item n=8 part=touches
Screen fixtures keep only signature rows: the new set carries its own ready, turn and modal screen fixtures.
@task_item n=9 part=touches
Real claude CLI only locally and CI holding no real claude: the live starts ran the real 2.1.328 CLI locally by path; CI replays only the recorded set.
@task_item n=10 part=touches
Probe prompts are synthetic and live session screens kept local: the recorded user-prompt and paste payloads come from the rig's own synthetic prompts, and the rig's kept record stays under the local build tree until copied into the set.
@task_item n=11 part=touches
Test seams and test-only binaries kept out of release: the change sits wholly in the test-only rig binary and its test crate, so rows refusing test-only binaries and features in the release build rule over it.
@task_fact n=1 slug=data-protection
fact: The live rig's context arm, when no session transcript was held before the send, looks the transcript up once more after the send and counts a transcript found then from its start, count-only, printing only the closed word context-transcript-after read, absent or unread.
@task_item n=1 part=untouched
Product crates and the wrapper, hook, MCP server and CLI: no product code changed, so their validation, refusal and error contracts are untouched.
@task_item n=2 part=untouched
Authentication and authorization: the socket directory, peer euid checks, strict-modes checks and server verification are unchanged.
@task_item n=3 part=untouched
Dependency security: no dependency, pin, toolchain, licence or advisory list moved, and no workflow or CI gate changed.
@task_item n=4 part=untouched
Secret management: the credential strip, hermetic child environment, canary scan and the rig's credentials copy are unchanged.
@task_item n=5 part=untouched
The version gate, the stamps file and viola verify: the product's default CLI version and the stamped home's version did not move; the count of stamped recorded sets is held in no row of this plan.
@task_item n=6 part=untouched
Absences: no network surface, release stage, secret or personal-data class was added or removed.
