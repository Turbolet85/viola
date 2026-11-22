@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty, and found no loss: 12 of 12 keys kept at 0, 20 and 100 ms on claude 2.1.330 and on 2.1.328.
The reading is no loss in 12 rounds at these delays on these versions, never that claude never loses a key; it measures the CLI unwrapped, not viola's own poll lag or the driver's paste.
No product change followed: the fix half was not needed, and no product crate, no viola-pty file and no fake-agent file changed.
It added a KR arm to the test-only live-round rig: it resizes the child's own pty, types one x and one DEL after each resize, and reads the input box row's cell attributes only, never its text.
The rig's rehearse verb gained two rehearse-only options, a quiet span of 100 to 400 ms and a KR seam that withholds or delays a redraw; the live verb refuses both before any ledger line.
The rig's after-key settle now counts quiet only from a byte after the key, for KR alone; every other arm keeps its unchanged settle.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the live-round rig's core in the test-only viola-e2e crate: its arms, the session that drives one child in its own pty, the settle, the rehearse plan and the closed-word summary keys; edited
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the live-round rig's key grant: which keys a start may type into its own pty and whether it may resize that pty; edited
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live-round rig's own vt100 screen model, read for the input box row's cell attributes, never text; edited
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the viola-live-rig test-only binary's command line: its live, rehearse and census verbs and their options; edited
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live-round rig's rehearsal integration tests, run over the fake agent; edited
@task_item n=1 part=touches
The rule that no human key is held behind a resize (the key-after-a-resize wait rule and its restatements): the Linux measurement finds no key lost after a resize, so no wait or hold is added, and the rule's exception now names the KR rehearsal beside its sibling.
@task_item n=2 part=touches
Rows that say screen content is never parsed or that screen row text is read only by verify: the KR arm reads the input box row's cell attributes and never its text, an exception that names KR beside the earlier attribute-only arms.
@task_item n=3 part=touches
The occupied name viola-live-rig: the binary gains the KR live arm (labels KR, KR-2, KR8, KR8-2) and two rehearse-only options, quiet-ms and kr-del-seam; live refuses both with a usage refusal, exit 2.
@task_item n=4 part=touches
The test-only viola-e2e crate (its name row and the rule that it is test-only and no product crate depends on it): every change of the session lies in that crate.
@task_item n=5 part=touches
Rows that keep real claude local only and out of CI: the KR live arm runs real claude on the dev host only, under a cap and a spare, and is refused under CI.
@task_item n=6 part=touches
The rule that test-only binaries stay out of release builds: the rig binary changed and stays test-only.
@task_item n=7 part=touches
vt100 as the screen model: the rig's own screen model gains a resize that keeps its box and reads at the new width.
@task_item n=8 part=touches
Rows that run tests against the fake agent: five KR rehearsals over the fake agent join the integration layer, including a late redraw, a box still dirty on its re-read, and the refused seams.
@task_item n=9 part=touches
Rows on test seams (fake-agent seam variables, test seams are no configuration channel): the new KR seam is a rehearse-only option of the test-only rig, not an environment variable and not configuration.
@task_item n=10 part=touches
Rows on kept screen products and scrubbed fixtures: the KR starts' frames are kept owner-only under a gitignored rig directory, never read into evidence, a message or the tree, and no fixture is recorded.
@task_item n=11 part=touches
Rows on the measured CLI versions and the capability ledger: the reading covers claude 2.1.330 and 2.1.328, but it adds no relied ledger row and no stamp.
@task_fact n=1 slug=established-decisions
fact: On Linux, real claude kept a key typed 0, 20 or 100 ms after a resize of its own pty in 12 of 12 rounds on 2.1.330 and on 2.1.328, so viola adds no wait after a resize; the key-loss reading is a Windows-ConPTY measurement and says nothing of viola's poll lag or the driver's paste.
@task_item n=1 part=untouched
Product crates and their runtime rules, including the pty seam and portable-pty: viola-pty and every product crate are unchanged, and the rig resizes only its own child's pty.
@task_item n=2 part=untouched
The send path, the bracketed paste and the send time bound: the paste's missing time bound is a later route entry, and nothing about sending changed.
@task_item n=3 part=untouched
The wrapper channel, MCP tools, hook tiers, event kinds and every channel, event or MCP contract: none moved.
@task_item n=4 part=untouched
viola's CLI verbs, exit codes, JSON documents and refusal details: no product verb or code changed.
@task_item n=5 part=untouched
Configuration, config.json keys and environment-variable names (VIOLA_ and AGENT_RUN_ prefixes): no key, variable or product flag was added.
@task_item n=6 part=untouched
The cross-session prefix and inbox-socket address rows: the cross-session fixture's runtime-dir path is an accepted residual that belongs to the security plan, and those rows stay true.
@task_item n=7 part=untouched
CI workflows, coverage floors, pre-push, mutation, fuzz, cargo-deny and dependency rows: no gate, threshold, workflow or dependency changed.
@task_item n=8 part=untouched
The wheel, dialogs, liveness, revive, verify and state-file rows: no behaviour of the wrapper changed.
