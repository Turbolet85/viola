@task_prose part=session
The session took the first live readings on the standing live rig (test-only, viola-e2e; no product crate, no dependency, no IPC or env surface added).
It added wake arms R3B and R3 (labels R3B-2, R3-2, R3-326, R3D; options --wake interval or dynamic and --stand) and an imports arm IM, a hidden rig capture verb that records the rig's own UserPromptSubmit hook payload as key names and a closed source word only, and a no-key group-kill stop for the imports arm.
It replaced the rig's stamp-only user-config witness with a mark witness (two rig-made needles, closed words unchanged, absent, changed-by-others, changed-by-rig, unread, and a needles count) after measuring a host writer that rewrites the user config about once a minute; the summary word changed is retired.
It gave harness boot a per-boot ready deadline (default the 20 s instance deadline, which the CLI boot keeps) so the never-ready case ends in 2.487 s instead of 20.644 s, and put the harness_lifecycle, cli and harness_tied binaries on the ci profile's 20 s kill line (override filter binaries 25 to 28).
Live it measured a fired scheduled wake's count (scheduled_wakes 1), a hook-latency window of 1 to 3 ms, the imports modal with the box down, and that a fired loop wake's prompt hook carries no source key on CLI 2.1.325 and 2.1.326 while the wrapper logs it origin human; the live verify --record (R4) left the chunk unrun.
Unit rig cases grew 29 to 48 and rig_rehearsal cases 9 to 14.
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the standing live rig's capture of its own UserPromptSubmit hook payload into one bounded, private, value-free line per payload (closed source words, listed key names)
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the standing live rig's scheduled-wake arms (R3B, R3) over a wrapped claude, reading the fired wake's count, source and timing through viola send and the event log
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the standing live rig's isolation of its claude child (private scratch home, seeded config) and its user-config witness, now the mark witness and the capture and imports seeds
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the standing live rig's arms, labels, session (learned session id, stop with and without a key) and summary words
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the standing live rig's committed binary and its command line (live labels, rehearsal words, --wake, --stand, the hidden capture verb)
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the rig rehearsal integration binary, the standing live rig's guard run against the fake agent
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the harness boot command (home, stamps, wrapper starts and the readiness wait with its per-instance deadline)
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the harness performance-budget run that boots instances and times the hooks
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the viola-e2e harness CLI integration binary, now on the ci profile's 20 s kill line
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle integration binary (boot, readiness failure, cleanup), now on the ci profile's 20 s kill line
@task_file n=11 path=.config/nextest.toml
what: edited; the nextest profiles, the ci profile's per-test kill-line override filter
@task_item n=1 part=touches
decision/standing-live-rig is stale: the rig gains wake arms and an imports arm, the capture verb, the no-key group-kill stop, the R3D label, --wake and --stand, the mark witness with its needles count and closed words; its guard binary rig_rehearsal now holds 14 cases.
@task_item n=2 part=touches
decision/standing-live-rig's isolation witness word changed is retired, replaced by changed-by-others, changed-by-rig and unread; any row naming the user-config words unchanged, changed, absent is stale.
@task_item n=3 part=touches
decision/ci-stamped-home-kill-line is stale: the ci profile's 20 s kill line now also covers the harness_lifecycle, cli and harness_tied viola-e2e binaries; the override filter's binary count moved 25 to 28.
@task_item n=4 part=touches
decision/verify-driven-tests-20s-kill is stale for the same reason: three viola-e2e binaries joined the per-test 20 s kill.
@task_item n=5 part=touches
contract/harness-boot/timeout: the readiness deadline is now a per-boot option defaulting to the 20 s instance deadline; the harness CLI boot keeps 20 s; test callers may set it shorter.
@task_item n=6 part=touches
Readiness rows over the boot wait (contract/harness-boot-reason/readiness-timeout, contract/harness-boot/readiness-failure, decision/readiness-stall-report, decision/boot-readiness-file-probe, decision/boot-deadline-not-a-budget): the wait now ends at the boot's own deadline; the never-ready case ends in 2.487 s.
@task_item n=7 part=touches
decision/root-wait-bound, decision/within-7s-wait-bound and decision/viola-e2e-30s-mutants-kill: wait bounds versus kill lines over binaries that now sit on the 20 s line; the rehearsals keep an 18 s WITHIN.
@task_item n=8 part=touches
decision/test-layer-filtersets and decision/ci-profile-no-retries: rules over the nextest ci profile whose override filter was edited.
@task_item n=9 part=touches
decision/tests-never-read-host-sessions: the rig's mark witness now reads the user's own claude config bytes after a moved stamp, searching for two rig-made needles and keeping only a word.
@task_item n=10 part=touches
Bounded-reader rows (decision/bounded-readers, decision/bounded-readers-keep-the-cap, decision/no-unbounded-json-depth, decision/no-serde-json-unbounded-depth): the capture verb and the user-config read take at most MAX_FRAME plus one byte with serde default depth; over-bound reads unread.
@task_item n=11 part=touches
Privacy and no-secrets rows (decision/logs-carry-no-secrets, decision/no-secrets-in-logs, decision/fixtures-never-from-real-prompts, decision/scrubbed-fixtures): the capture line keeps key names matching a closed pattern, a count of others and a closed source word, never a value.
@task_item n=12 part=touches
Private-placement rows (decision/private-filesystem-permissions, decision/private-placement-mode-first): the capture file is appended privately and the wake arms' settings with one UserPromptSubmit hook are written private with absolute shell-plain paths.
@task_item n=13 part=touches
Real-CLI rows (decision/real-claude-local-only, decision/real-claude-never-in-ci, decision/no-real-cli-in-ci, decision/live-verify-never-in-ci): the rig drove claude 2.1.325 and 2.1.326 by path, locally; rehearsals in CI run against the fake agent.
@task_item n=14 part=touches
contract/turn-ended/scheduled-wakes and contract/dialog-data/scheduled-wakes: the wake arms read scheduled_wakes live (1 after each send) and against the fake agent (0, its replayed Stop carries no crons).
@task_item n=15 part=touches
Wheel rows for injected turns (decision/harness-turn-keeps-the-wheel, decision/injected-turn-keeps-wheel, contract/prompt-submitted/origin-harness, contract/events/harness-prompt): every fired scheduled wake read origin human on the wrapper's prompt-submitted line; whether a fired wake keeps the wheel is planned (v2-36) and its fate is asked at route-resolve.
@task_item n=16 part=touches
Screen rows (decision/screen-signatures-only, decision/no-screen-parsing, decision/never-parse-rendered-child-screen): the imports arm reads its first frame as modal imports and box down by compiled signatures; whether the placeholder hint is told apart by a cell attribute stays unmeasured.
@task_item n=17 part=touches
Kill rows (decision/pid-start-time-verified-kill, decision/cleanup-ctrl-c-repeated-until-exit): the rig gained a no-key stop by process-group kill beside its Ctrl-C stop into its own pty.
@task_item n=18 part=touches
Test-only crate and release rows (decision/single-test-only-crate, decision/test-only-e2e-crate, decision/release-build-viola-only, decision/no-test-binary-in-release): two new modules and a hidden verb in the test-only crate and rig binary; release-check still reads viola only.
@task_item n=19 part=touches
Untied-spawn rows (decision/no-untied-harness-boot, decision/tied-verify-kill-witness): the harness lifecycle and CLI binaries now name the instance deadline at their boot literals.
@task_item n=20 part=touches
Hook-latency rows (decision/hook-deadline-timing, decision/performance-budget-tests-for-hook-deadlines): the live hook-latency window read 1 to 3 ms on the rig (a ledger note, no budget moved); the perf run only names the boot deadline.
@task_fact n=1 slug=6-e2e-test-strategy
fact: The standing live rig witnesses its isolation from the user's claude config by marks, not by a whole-file stamp: on a moved stamp it searches the file for two rig-made needles (its scratch path, its session id) and keeps only needles and a closed word unchanged, absent, changed-by-others, changed-by-rig or unread, never content, because a whole-file stamp of a shared user file cannot attribute a change on a host with other writers.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The standing live rig captures its own UserPromptSubmit hook payload through a hidden capture verb into one private line per payload holding only key names matching a closed pattern up to 40 chars, a count of other keys, a closed source word (schedule_wakeup, loop_wakeup, other, absent) and ms, or unread and oversize words; no value is kept.
@task_fact n=3 slug=6-e2e-test-strategy
fact: The rig's keys: none names its argv grant into dialogs and box text, never its stop: wake arms stop by Ctrl-C into the rig's own pty, and the imports arm stops by a process-group kill with no key typed (stop killed-no-key).
@task_fact n=4 slug=6-e2e-test-strategy
fact: A rig hook path is registered into the claude child's private settings only when absolute and free of shell metacharacters.
@task_fact n=5 slug=6-e2e-test-strategy
fact: Measured live on claude 2.1.325 and 2.1.326: a fired scheduled loop wake's UserPromptSubmit payload carries no source key in either the interval or the dynamic form, so a test cannot tell a fired wake from a human prompt by that key.
@task_item n=1 part=untouched
Product contracts and refusals (CLI exit codes and JSON documents, send, wait, answer, release, list, revive, MCP tools, the wrapper channel and its JSON-RPC errors): no product file or behaviour changed.
@task_item n=2 part=untouched
The event line's own source field and event-data keys: the product event log is unchanged; the measured missing source key is the CLI hook payload's, not the event line's.
@task_item n=3 part=untouched
Recorded fixtures per CLI version and the live verify recording rule: the live verify --record left this chunk and no fixture was recorded or replaced.
@task_item n=4 part=untouched
Coverage floors, mutation gates and thresholds, fuzzing and corpus replay: no floor, threshold or target moved.
@task_item n=5 part=untouched
Supply-chain, cargo-deny, toolchain and workflow rows (SHA pins, permissions, zizmor, advisories, CI artifacts): no dependency, workflow or tool version changed.
@task_item n=6 part=untouched
Process log format, log-line fields and corr rows: no log field or event code added.
@task_item n=7 part=untouched
The fake agent's command line, receipts, scripts and screens: the fake agent was not changed.
