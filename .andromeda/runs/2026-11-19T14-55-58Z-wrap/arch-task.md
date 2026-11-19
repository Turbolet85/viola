@task_prose part=session
The session landed the recorded fixture set of claude 2.1.328, the fourth stamped set, as a byte copy of the live rig's kept recorder output from one clean verify run (24/24 relied rows) and the MCP tap's recorded tools/list exchange.
The ledger probe suite's stamped-set list now names 2.1.328 beside 2.1.287, 2.1.302 and 2.1.325; the drift, recorded-exchange, hygiene and verify suites are green over every set.
The test-only live rig's context arm gained one transcript lookup after the send, so a session transcript first found after the send is read from its start, and the rig prints a new closed summary word, context-transcript-after (read, absent or unread).
One live reading, recorded as a reading and not a verification: on 2.1.328 the /context local command's stdout reached the session transcript as one local-command output tag.
No product crate, CLI default version, stamped-home version, config key, CI or dependency moved, and no capability is claimed for any version.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig (viola-live-rig) and its context arm, which reads the private-config session transcript count-only under a frame bound and prints closed summary words; edited
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal over the fake agent, asserting each arm's summary words; edited
@task_file n=3 path=fixtures/claude/2.1.328
what: a recorded CLI fixture set of one claude version, the scrubbed hook payloads, screen signature rows and recorded MCP tools/list exchange that the drift and recorded-exchange suites replay; added
@task_file n=4 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite and its list of stamped recorded sets it walks; edited
@task_item n=1 part=touches
Recorded fixtures scrubbed and hygiene-walked: a new recorded set landed under that rule, passing the hygiene walk with no hand edit.
@task_item n=2 part=touches
Fixtures recorded only from a clean verify: the new set came from a verify run that exited clean with every relied row passing.
@task_item n=3 part=touches
MCP tools/list held to a recorded claude exchange: a recorded tools/list exchange for claude 2.1.328 now stands beside the earlier ones and the tap suite is green over it.
@task_item n=4 part=touches
Screen fixtures keep only signature rows: the new set carries the modal, ready and turn screen fixtures under that rule.
@task_item n=5 part=touches
Version-stamped capability ledger, the per-behaviour CLI capability ledger and the ledger as the single gate: the probe suite's stamped-set list grows to four with 2.1.328, while no capability is claimed and no ledger row changes.
@task_item n=6 part=touches
A CLI version is verified only when every relied row counts: the 2.1.328 verify read 24/24 relied rows passing.
@task_item n=7 part=touches
Verify's Run E resumes Run B's first session: the new set holds the resume session-start payload and the resume screen read box.
@task_item n=8 part=touches
viola-e2e is test-only, and test-only binaries kept out of release builds: the only crate changed is viola-e2e and the only binary reached is viola-live-rig.
@task_item n=9 part=touches
The occupied name viola-live-rig: the binary now prints one more closed summary word, context-transcript-after.
@task_item n=10 part=touches
Bounded readers at every boundary: the rig's after-send transcript read goes through the same count-only, frame-bounded tag counter.
@task_item n=11 part=touches
Local commands listed with delivery post-conditions, local commands classified by the ledger and local command confirmed by its stdout echo: the /context reading (one local-command output tag in the transcript after one send) is the reading a carried proposal to list /context as a local command rests on; nothing is listed yet.
@task_item n=12 part=touches
vt100 as a pre-send readiness gate, screen signatures as compiled literals and the input-not-ready refusal detail: a transcript row quoting a modal literal was measured to be read as a standing modal and to refuse the send as not-delivered input-not-ready; carried as a proposal, not mended.
@task_item n=13 part=touches
Real CLI probes run only locally, no live claude in CI and tests against the fake agent: the four live starts ran locally on the dev host; CI replays only the recorded sets.
@task_item n=14 part=touches
Harness prompts classified by raw prefix: the 2.1.328 paste prompt opens with the same pasted-content wrapper shape as 2.1.325's.
@task_item n=15 part=touches
Recorded fixture set count and newest stamped set: the recorded sets go from 5 to 6 and the stamped sets from 3 to 4, the newest now 2.1.328; flag only a row whose cell holds such a count or member.
@task_item n=16 part=untouched
The product crates (core, channel, state, pty, mcp, agent-claude, hyprland and the root bin) and their channel, event, snapshot, list, MCP tool and hook contracts: no product code moved.
@task_item n=17 part=untouched
CLI verbs, exit codes, refusal details and --json documents: no product output changed.
@task_item n=18 part=untouched
The CLI default version, the pinned copy, run and revive start order and the stamped home: none moved.
@task_item n=19 part=untouched
Build system, CI workflow, dependency policy, toolchain pin and coverage floors: no dependency, workflow, lint or gate change.
@task_item n=20 part=untouched
Configuration keys, environment variable names and diagnostics files: no key, variable or file shape added.
@task_item n=21 part=untouched
Stack, deployment model, project intent and the absences: no capability added or removed.
