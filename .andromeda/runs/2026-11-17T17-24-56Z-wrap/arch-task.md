@task_prose part=session
Test-only session in viola-e2e: no product crate, no src file, no IPC method, endpoint, socket, port, env var, crate or dependency added.
The standing live rig (viola-live-rig) gained wake arms (R3B, R3, R3D) and an imports arm (IM), the options --wake and --stand, a no-key group-kill stop, and a hidden capture --out verb that records the rig's own UserPromptSubmit hook payload as key names and a closed source word, never a value.
The rig's user-config isolation witness moved from a whole-file stamp to a mark witness: two rig-made needles searched in the moved file, read under MAX_FRAME, no link followed out of the home, only a closed word kept (unchanged, absent, changed-by-others, changed-by-rig, unread); the word changed is retired.
The test harness boot gained a per-instance ready deadline (BootOptions ready_deadline, default 20 s) and the ci nextest profile's 20 s kill line now covers three viola-e2e binaries (filter binaries 25 to 28).
Measured live on claude 2.1.325 and 2.1.326: a fired /loop scheduled wake's UserPromptSubmit carries no source key and the wrapper logs its prompt-submitted line as origin human; the host's own Claude sessions rewrite the user config about once a minute.
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the live rig's hidden capture verb: reads one UserPromptSubmit hook payload from stdin bounded by MAX_FRAME and appends one private line of listed key names and a closed source word, no value
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the live rig's scheduled-wake arms: drive a wrapped claude through viola send, read events.ndjson turn-ended scheduled_wakes and the capture lines
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the live rig's isolated child scratch and its user-config isolation witness (now the mark witness), plus the capture and imports seeds of the private settings
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the live rig's arms, labels, session (learned session id, no-key group-kill stop) and run summary
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the viola-live-rig test-only binary's command line: the hidden capture verb and the --wake and --stand options
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the live rig's rehearsals against the fake agent (r3b, r3, im arms and the user-config mark cases)
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness boot of a wrapped instance and its readiness timeout (BootOptions ready_deadline, INSTANCE_DEADLINE made public)
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the harness perf run, now naming the shared instance deadline at its boot options
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the end-to-end CLI verb tests, now naming the shared instance deadline at their boot options
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle tests, including the never-ready boot case bounded by the ready deadline
@task_file n=11 path=.config/nextest.toml
what: edited; the nextest profiles; the ci profile's 20 s kill-line override filter now holds harness_lifecycle, cli and harness_tied
@task_item n=1 part=touches
name/binary/viola-live-rig: the test-only binary gained a hidden capture verb, the --wake and --stand options and new arm labels; a row naming its surface may need them beside it.
@task_item n=2 part=touches
name/crate/viola-e2e and decision/e2e-test-only: the crate gained the modules rig capture and rig wake, all test-only; check the rows still hold.
@task_item n=3 part=touches
decision/test-binaries-never-released, decision/release-build-viola-only and decision/release-carries-viola-only: the live rig binary grew new verbs; release-check still reads viola only.
@task_item n=4 part=touches
decision/bounded-readers and contract/wrapper-channel-frames/max-frame: the rig's capture reader and its user-config reader both read bounded by MAX_FRAME and refuse an over-bound input as unread.
@task_item n=5 part=touches
decision/serde-json-recursion-limit, decision/serde-json-unbounded-depth-banned and decision/no-unbounded-json-depth: the capture parses the hook payload with serde's default depth.
@task_item n=6 part=touches
decision/private-mode-before-write: the capture appends its line through the private append helper and the wake arms write their texts through the private replace helper.
@task_item n=7 part=touches
contract/event-prompt-submitted/origin, contract/event-data/prompt-submitted-origin and decision/human-prompt-confirms-wheel-take: live, a fired /loop scheduled wake's prompt-submitted line read origin human on 2.1.325 and 2.1.326; the planned rule that a fired scheduled wake keeps the wheel is carried to the route, not changed here.
@task_item n=8 part=touches
contract/hook-to-kind-map/userpromptsubmit and contract/event-line/source: a fired scheduled wake's UserPromptSubmit payload carries no source key on both CLIs and both wake forms; the event line's own source field is a separate, still-true claim.
@task_item n=9 part=touches
contract/event-data/turn-ended-scheduled-wakes: the wake arms read the turn-ended scheduled_wakes count live (one per fired send on R3B and R3D); the contract is read, not changed.
@task_item n=10 part=touches
decision/probe-capture-arms-inert, contract/hook-capture-arm/payload-file and contract/probe-capture/payload-file: the rig's own capture is a different, test-only capture that keeps no value; check none of these rows is read as covering it.
@task_item n=11 part=touches
absence/status-line-interception and decision/global-statusline-untouched: the rig's wake arms seed a private settings.json in the scratch home with one UserPromptSubmit hook; that is the rig's isolated child, not a product settings layer.
@task_item n=12 part=touches
decision/real-cli-probe-local-only, decision/real-claude-local-only and decision/no-live-claude-in-ci: the live starts ran a real claude on the dev host only; CI ran the rehearsals against the fake agent.
@task_item n=13 part=touches
decision/root-waits-share-one-bound and decision/test-evidence-survives-kill: the harness boot now waits each instance to a ready deadline in its options (20 s default) and the ci 20 s kill line covers three viola-e2e binaries.
@task_item n=14 part=touches
decision/tests-against-fake-agent, decision/ci-linux-fake-agent and decision/perf-gate: the ci nextest profile and the harness perf run changed under these rules; no rule's value moved.
@task_item n=15 part=touches
decision/recorded-fixtures-scrubbed and decision/process-logs-hold-codes-only: the rig's capture and witness keep closed words and key names only, the same content-free discipline applied to new test evidence.
@task_fact n=1 slug=occupied-resources
fact: The live rig captures its own UserPromptSubmit hook payload through a hidden capture verb as one private line holding only key names matching a closed pattern, a count of the others, a closed source word (schedule_wakeup, loop_wakeup, other, absent) and a time, never a value; an unreadable or over-bound payload is written as unread.
@task_fact n=2 slug=occupied-resources
fact: The live rig witnesses its isolation from the user's ~/.claude.json by a mark witness: when the file's stamp moved it reads the file bounded by MAX_FRAME, follows no link out of the home, searches for two rig-made needles and keeps only one closed word (unchanged, absent, changed-by-others, changed-by-rig, unread), because the host's own Claude sessions rewrite that file about once a minute.
@task_fact n=3 slug=occupied-resources
fact: viola-live-rig occupies a hidden capture verb (capture --out) and the options --wake (interval or dynamic) and --stand, all test-only.
@task_item n=1 part=untouched
Product crates and their rules: viola-core, viola-agent-claude, viola-channel, viola-state, viola-pty, viola-mcp, viola-hyprland and the root viola bin are untouched; no wrapper, hook, channel or MCP behaviour changed.
@task_item n=2 part=untouched
Channel, CLI verb, exit-code, refusal, MCP tool and snapshot contracts: no method, verb, code, field or document shape changed.
@task_item n=3 part=untouched
Stack and build-system decisions: no dependency added or bumped, no feature, toolchain, lint, licence or cargo-deny rule moved.
@task_item n=4 part=untouched
The capability ledger, stamps and viola verify rows: no ledger row, stamp or verify step changed; the live verify --record was moved out of this session.
@task_item n=5 part=untouched
Occupied product names: no product command, env var, socket, endpoint name, crate or MCP tool name was taken or freed.
@task_item n=6 part=untouched
Deployment, release and CI workflow rows (actions, permissions, uploads, fuzz, mutation, coverage floors): no workflow or coverage floor moved.
